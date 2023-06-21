//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "challenges_subtasks")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub task_id: Uuid,
    pub creator: Uuid,
    pub creation_timestamp: DateTime,
    pub xp: i64,
    pub coins: i64,
    pub fee: i64,
    pub enabled: bool,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::challenges_coding_challenges::Entity")]
    ChallengesCodingChallenges,
    #[sea_orm(has_many = "super::challenges_matchings::Entity")]
    ChallengesMatchings,
    #[sea_orm(has_many = "super::challenges_multiple_choice_quizes::Entity")]
    ChallengesMultipleChoiceQuizes,
    #[sea_orm(has_many = "super::challenges_questions::Entity")]
    ChallengesQuestions,
    #[sea_orm(has_many = "super::challenges_subtask_reports::Entity")]
    ChallengesSubtaskReports,
    #[sea_orm(
        belongs_to = "super::challenges_tasks::Entity",
        from = "Column::TaskId",
        to = "super::challenges_tasks::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    ChallengesTasks,
    #[sea_orm(has_many = "super::challenges_user_subtasks::Entity")]
    ChallengesUserSubtasks,
}

impl Related<super::challenges_coding_challenges::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChallengesCodingChallenges.def()
    }
}

impl Related<super::challenges_matchings::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChallengesMatchings.def()
    }
}

impl Related<super::challenges_multiple_choice_quizes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChallengesMultipleChoiceQuizes.def()
    }
}

impl Related<super::challenges_questions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChallengesQuestions.def()
    }
}

impl Related<super::challenges_subtask_reports::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChallengesSubtaskReports.def()
    }
}

impl Related<super::challenges_tasks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChallengesTasks.def()
    }
}

impl Related<super::challenges_user_subtasks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChallengesUserSubtasks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
