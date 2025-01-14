use entity::{
    challenges_coding_challenge_result, challenges_coding_challenge_submissions,
    challenges_subtasks, sea_orm_active_enums::ChallengesVerdictVariant,
};
use lib::services::Services;
use schemas::challenges::leaderboard::{Leaderboard, Rank};
use sea_orm::{
    sea_query::{Alias, Expr, Query, SelectStatement, SimpleExpr},
    ColumnTrait, DatabaseTransaction, Iden, Value,
};
use uuid::Uuid;
use chrono::NaiveDateTime;

use super::{get_leaderboard, get_leaderboard_user};

fn get_base_query(
    language: &str,
    date_range: Option<(NaiveDateTime, NaiveDateTime)>,
) -> SelectStatement {
    let mut inner_query = Query::select()
        .expr_as(
            Expr::col((
                challenges_coding_challenge_submissions::Entity,
                challenges_coding_challenge_submissions::Column::Creator,
            )),
            Alias::new("user_id"),
        )
        .expr_as(
            Expr::col(challenges_coding_challenge_submissions::Column::SubtaskId),
            Alias::new("subtask_id"),
        )
        .expr_as(
            Expr::col((
                challenges_coding_challenge_submissions::Entity,
                challenges_coding_challenge_submissions::Column::CreationTimestamp,
            ))
            .max(),
            Alias::new("last_update"),
        )
        .from(challenges_coding_challenge_result::Entity)
        .inner_join(
            challenges_coding_challenge_submissions::Entity,
            Expr::col((
                challenges_coding_challenge_result::Entity,
                challenges_coding_challenge_result::Column::SubmissionId,
            ))
            .equals((
                challenges_coding_challenge_submissions::Entity,
                challenges_coding_challenge_submissions::Column::Id,
            )),
        )
        .inner_join(
            challenges_subtasks::Entity,
            Expr::col((challenges_subtasks::Entity, challenges_subtasks::Column::Id))
                .equals((
                    challenges_coding_challenge_submissions::Entity,
                    challenges_coding_challenge_submissions::Column::SubtaskId,
                )),
        )
        .and_where(
            Expr::col(challenges_coding_challenge_submissions::Column::Environment)
                .eq(language),
        )
        .and_where(
            Expr::col(challenges_coding_challenge_result::Column::Verdict).eq(
                SimpleExpr::Constant(Value::String(Some(
                    ChallengesVerdictVariant::Ok.to_string().into(),
                ))),
            ),
        );

    // Add date range filter if provided
    if let Some((start_date, end_date)) = date_range {
        inner_query = inner_query.and_where(
            Expr::col((
                challenges_coding_challenge_submissions::Entity,
                challenges_coding_challenge_submissions::Column::CreationTimestamp,
            ))
            .between(start_date, end_date),
        );
    }

    inner_query = inner_query.group_by_columns([
        (
            challenges_coding_challenge_submissions::Entity,
            challenges_coding_challenge_submissions::Column::Creator,
        ),
        (
            challenges_coding_challenge_submissions::Entity,
            challenges_coding_challenge_submissions::Column::SubtaskId,
        ),
    ]);

    Query::select()
        .column(Alias::new("user_id"))
        .expr_as(
            challenges_subtasks::Column::Xp
                .sum()
                .cast_as(Alias::new("int8")),
            Alias::new("xp"),
        )
        .expr_as(
            Expr::col(Alias::new("last_update")).max(),
            Alias::new("last_update"),
        )
        .from_subquery(inner_query, Alias::new("x"))
        .inner_join(
            challenges_subtasks::Entity,
            Expr::col(Alias::new("subtask_id")).equals(challenges_subtasks::Column::Id),
        )
        .group_by_col(Alias::new("user_id"))
        .to_owned()
}

pub async fn get_language_leaderboard(
    db: &DatabaseTransaction,
    services: &Services,
    language: &str,
    limit: u64,
    offset: u64,
    date_range: Option<(NaiveDateTime, NaiveDateTime)>,
) -> anyhow::Result<Leaderboard> {
    let base_query = get_base_query(language, date_range);
    get_leaderboard(db, services, base_query, limit, offset).await
}

pub async fn get_language_leaderboard_user(
    db: &DatabaseTransaction,
    language: &str,
    user_id: Uuid,
    date_range: Option<(NaiveDateTime, NaiveDateTime)>,
) -> anyhow::Result<Rank> {
    let base_query = get_base_query(language, date_range);
    get_leaderboard_user(db, base_query, user_id).await
}
