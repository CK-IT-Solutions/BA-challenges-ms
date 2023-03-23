//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "challenges_coding_challenges")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub subtask_id: Uuid,
    pub time_limit: i64,
    pub memory_limit: i64,
    #[sea_orm(column_type = "Text")]
    pub evaluator: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::challenges_coding_challenge_example::Entity")]
    ChallengesCodingChallengeExample,
    #[sea_orm(
        belongs_to = "super::challenges_subtasks::Entity",
        from = "Column::SubtaskId",
        to = "super::challenges_subtasks::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    ChallengesSubtasks,
}

impl Related<super::challenges_coding_challenge_example::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChallengesCodingChallengeExample.def()
    }
}

impl Related<super::challenges_subtasks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ChallengesSubtasks.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
