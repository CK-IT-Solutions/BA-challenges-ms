//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "jobs_skill_requirements")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub job_id: Uuid,
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub skill_id: String,
    pub level: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::jobs_jobs::Entity",
        from = "Column::JobId",
        to = "super::jobs_jobs::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    JobsJobs,
}

impl Related<super::jobs_jobs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::JobsJobs.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
