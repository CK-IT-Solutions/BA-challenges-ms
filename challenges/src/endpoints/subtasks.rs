use std::sync::Arc;

use chrono::Utc;
use entity::{
    challenges_subtask_reports, challenges_subtasks, challenges_tasks, challenges_user_subtasks,
    sea_orm_active_enums::ChallengesRating,
};
use lib::{
    auth::{AdminAuth, VerifiedUserAuth},
    config::Config,
    services::shop::AddCoinsError,
    SharedState,
};
use poem::web::Data;
use poem_ext::{db::DbTxn, response, responses::ErrorResponse};
use poem_openapi::{
    param::{Path, Query},
    payload::Json,
    OpenApi,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseTransaction, EntityTrait, QueryFilter, Set, Unchanged,
};
use uuid::Uuid;

use super::Tags;
use crate::{
    schemas::subtasks::{CreateReportRequest, PostFeedbackRequest, Report, UpdateSubtaskRequest},
    services::{
        subtasks::{get_user_subtask, update_user_subtask, UserSubtaskExt},
        tasks::{get_specific_task, Task},
    },
};

pub struct Subtasks {
    pub state: Arc<SharedState>,
    pub config: Arc<Config>,
}

#[OpenApi(tag = "Tags::Subtasks")]
impl Subtasks {
    /// Unlock a subtask by paying its fee.
    #[oai(path = "/tasks/:task_id/subtasks/:subtask_id/access", method = "post")]
    pub async fn unlock_subtask(
        &self,
        task_id: Path<Uuid>,
        subtask_id: Path<Uuid>,
        db: Data<&DbTxn>,
        auth: VerifiedUserAuth,
    ) -> UnlockSubtask::Response<VerifiedUserAuth> {
        let Some((subtask, _)) = get_subtask(&db, task_id.0, subtask_id.0).await? else {
            return UnlockSubtask::subtask_not_found();
        };
        if !auth.0.admin && auth.0.id != subtask.creator && !subtask.enabled {
            return UnlockSubtask::subtask_not_found();
        }

        let user_subtask = get_user_subtask(&db, auth.0.id, subtask.id).await?;
        if user_subtask.check_access(&auth.0, &subtask) {
            return UnlockSubtask::ok();
        }

        match self
            .state
            .services
            .shop
            .add_coins(auth.0.id, -subtask.fee, "Quiz", false)
            .await?
        {
            Ok(_) => {}
            Err(AddCoinsError::NotEnoughCoins) => {
                return UnlockSubtask::not_enough_coins();
            }
        }

        update_user_subtask(
            &db,
            user_subtask.as_ref(),
            challenges_user_subtasks::ActiveModel {
                user_id: Set(auth.0.id),
                subtask_id: Set(subtask.id),
                unlocked_timestamp: Set(Some(Utc::now().naive_utc())),
                ..Default::default()
            },
        )
        .await?;

        UnlockSubtask::unlocked()
    }

    /// Update a subtask.
    #[oai(path = "/tasks/:task_id/subtasks/:subtask_id", method = "patch")]
    pub async fn update_subtask(
        &self,
        task_id: Path<Uuid>,
        subtask_id: Path<Uuid>,
        data: Json<UpdateSubtaskRequest>,
        db: Data<&DbTxn>,
        auth: VerifiedUserAuth,
    ) -> UpdateSubtask::Response {
        let Some((subtask, task)) = get_subtask(&db, task_id.0, subtask_id.0).await? else {
            return UpdateSubtask::subtask_not_found();
        };
        if !auth.0.admin && auth.0.id != subtask.creator {
            return UpdateSubtask::permission_denied();
        }

        let Some(specific) = get_specific_task(&db, &task).await? else {
            return UpdateSubtask::subtask_not_found();
        };
        if matches!(specific, Task::CourseTask(_))
            && *data.0.fee.get_new(&(subtask.fee as _)) > self.config.challenges.quizzes.max_fee
            && !auth.0.admin
        {
            return UpdateSubtask::fee_limit_exceeded(self.config.challenges.quizzes.max_fee);
        }

        challenges_subtasks::ActiveModel {
            id: Unchanged(subtask.id),
            task_id: Unchanged(subtask.task_id),
            creator: Unchanged(subtask.creator),
            creation_timestamp: Unchanged(subtask.creation_timestamp),
            xp: Unchanged(subtask.xp),
            coins: Unchanged(subtask.coins),
            fee: data.0.fee.map(|x| x as _).update(subtask.fee),
            enabled: Unchanged(subtask.enabled),
        }
        .update(&***db)
        .await?;

        UpdateSubtask::ok()
    }

    /// Submit feedback for a subtask after solving it.
    #[oai(
        path = "/tasks/:task_id/subtasks/:subtask_id/feedback",
        method = "post"
    )]
    pub async fn post_feedback(
        &self,
        task_id: Path<Uuid>,
        subtask_id: Path<Uuid>,
        data: Json<PostFeedbackRequest>,
        db: Data<&DbTxn>,
        auth: VerifiedUserAuth,
    ) -> PostFeedback::Response<VerifiedUserAuth> {
        let Some((subtask, _)) = get_subtask(&db, task_id.0, subtask_id.0).await? else {
            return PostFeedback::subtask_not_found();
        };
        if !auth.0.admin && auth.0.id != subtask.creator && !subtask.enabled {
            return PostFeedback::subtask_not_found();
        }

        let user_subtask = get_user_subtask(&db, auth.0.id, subtask.id).await?;
        if !user_subtask.can_rate(&auth.0, &subtask) {
            return PostFeedback::permission_denied();
        }

        update_user_subtask(
            &db,
            user_subtask.as_ref(),
            challenges_user_subtasks::ActiveModel {
                user_id: Set(auth.0.id),
                subtask_id: Set(subtask.id),
                rating: Set(Some(data.0.rating)),
                rating_timestamp: Set(Some(Utc::now().naive_utc())),
                ..Default::default()
            },
        )
        .await?;

        if data.0.rating == ChallengesRating::Positive && subtask.fee > 0 {
            self.state
                .services
                .shop
                .add_coins(subtask.creator, 1, "Quiz", true)
                .await??;
        }

        PostFeedback::created()
    }

    /// Return a list of all subtask reports.
    #[oai(path = "/subtask_reports", method = "get")]
    pub async fn list_reports(
        &self,
        /// Whether to search for completed reports.
        completed: Query<Option<bool>>,
        db: Data<&DbTxn>,
        _auth: AdminAuth,
    ) -> ListReports::Response<AdminAuth> {
        let mut query = challenges_subtask_reports::Entity::find()
            .find_also_related(challenges_subtasks::Entity);
        if let Some(completed) = completed.0 {
            let col = challenges_subtask_reports::Column::CompletedBy;
            query = query.filter(match completed {
                true => col.is_not_null(),
                false => col.is_null(),
            });
        }
        ListReports::ok(
            query
                .all(&***db)
                .await?
                .into_iter()
                .filter_map(|(report, subtask)| Some(Report::from(report, subtask?.task_id)))
                .collect(),
        )
    }

    /// Report a subtask.
    #[oai(path = "/subtask_reports", method = "post")]
    pub async fn create_report(
        &self,
        data: Json<CreateReportRequest>,
        db: Data<&DbTxn>,
        auth: VerifiedUserAuth,
    ) -> CreateReport::Response<VerifiedUserAuth> {
        let Some((subtask, _)) = get_subtask(&db, data.0.task_id, data.0.subtask_id).await? else {
            return CreateReport::subtask_not_found();
        };
        if !auth.0.admin && auth.0.id != subtask.creator && !subtask.enabled {
            return CreateReport::subtask_not_found();
        }

        let user_subtask = get_user_subtask(&db, auth.0.id, subtask.id).await?;
        if !user_subtask.can_rate(&auth.0, &subtask) {
            return CreateReport::permission_denied();
        }

        let now = Utc::now().naive_utc();

        update_user_subtask(
            &db,
            user_subtask.as_ref(),
            challenges_user_subtasks::ActiveModel {
                user_id: Set(auth.0.id),
                subtask_id: Set(subtask.id),
                rating: Set(None),
                rating_timestamp: Set(Some(now)),
                ..Default::default()
            },
        )
        .await?;

        let report = challenges_subtask_reports::ActiveModel {
            id: Set(Uuid::new_v4()),
            subtask_id: Set(subtask.id),
            user_id: Set(auth.0.id),
            timestamp: Set(now),
            reason: Set(data.0.reason),
            comment: Set(data.0.comment),
            completed_by: Set(None),
            completed_timestamp: Set(None),
        }
        .insert(&***db)
        .await?;

        let subtask = challenges_subtasks::ActiveModel {
            enabled: Set(false),
            ..subtask.into()
        }
        .update(&***db)
        .await?;

        CreateReport::created(Report::from(report, subtask.task_id))
    }
}

response!(UnlockSubtask = {
    /// The user already has access to this subtask.
    Ok(200),
    /// The fee has been paid and the subtask has been unlocked successfully.
    Unlocked(201),
    /// The subtask does not exist.
    SubtaskNotFound(404, error),
    /// The user does not have enough coins to pay the fee.
    NotEnoughCoins(403, error),
});

response!(UpdateSubtask = {
    Ok(200),
    /// The subtask does not exist.
    SubtaskNotFound(404, error),
    /// The user is not allowed to modify this subtask.
    PermissionDenied(403, error),
    /// The max fee limit has been exceeded.
    FeeLimitExceeded(403, error) => u64,
});

response!(PostFeedback = {
    Created(201),
    /// The subtask does not exist.
    SubtaskNotFound(404, error),
    /// The user is not allowed to post feeback for this subtask.
    PermissionDenied(403, error),
});

response!(ListReports = {
    Ok(200) => Vec<Report>,
});

response!(CreateReport = {
    /// Subtask has been reported successfully.
    Created(201) => Report,
    /// Subtask does not exist.
    SubtaskNotFound(404, error),
    /// The user is not allowed to report this subtask.
    PermissionDenied(403, error),
});

async fn get_subtask(
    db: &DatabaseTransaction,
    task_id: Uuid,
    subtask_id: Uuid,
) -> Result<Option<(challenges_subtasks::Model, challenges_tasks::Model)>, ErrorResponse> {
    Ok(
        match challenges_subtasks::Entity::find_by_id(subtask_id)
            .find_also_related(challenges_tasks::Entity)
            .filter(challenges_subtasks::Column::TaskId.eq(task_id))
            .one(db)
            .await?
        {
            Some((subtask, Some(task))) => Some((subtask, task)),
            _ => None,
        },
    )
}
