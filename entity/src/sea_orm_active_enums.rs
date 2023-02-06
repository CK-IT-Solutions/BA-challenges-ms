//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.7

use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "jobs_job_type")]
pub enum JobsJobType {
    #[sea_orm(string_value = "full_time")]
    FullTime,
    #[sea_orm(string_value = "internship")]
    Internship,
    #[sea_orm(string_value = "mini_job")]
    MiniJob,
    #[sea_orm(string_value = "part_time")]
    PartTime,
    #[sea_orm(string_value = "temporary")]
    Temporary,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "jobs_professional_level"
)]
pub enum JobsProfessionalLevel {
    #[sea_orm(string_value = "entry")]
    Entry,
    #[sea_orm(string_value = "junior")]
    Junior,
    #[sea_orm(string_value = "manager")]
    Manager,
    #[sea_orm(string_value = "senior")]
    Senior,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "jobs_salary_per")]
pub enum JobsSalaryPer {
    #[sea_orm(string_value = "day")]
    Day,
    #[sea_orm(string_value = "hour")]
    Hour,
    #[sea_orm(string_value = "month")]
    Month,
    #[sea_orm(string_value = "once")]
    Once,
    #[sea_orm(string_value = "task")]
    Task,
    #[sea_orm(string_value = "year")]
    Year,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Copy)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "jobs_salary_unit")]
pub enum JobsSalaryUnit {
    #[sea_orm(string_value = "euro")]
    Euro,
    #[sea_orm(string_value = "morphcoins")]
    Morphcoins,
}
