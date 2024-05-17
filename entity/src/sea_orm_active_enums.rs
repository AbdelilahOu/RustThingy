//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "announcement_categories"
)]
pub enum AnnouncementCategories {
    #[sea_orm(string_value = "academic")]
    Academic,
    #[sea_orm(string_value = "event")]
    Event,
    #[sea_orm(string_value = "general")]
    General,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "audiences")]
pub enum Audiences {
    #[sea_orm(string_value = "all")]
    All,
    #[sea_orm(string_value = "groups")]
    Groups,
    #[sea_orm(string_value = "parents")]
    Parents,
    #[sea_orm(string_value = "students")]
    Students,
    #[sea_orm(string_value = "teachers")]
    Teachers,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "performance_levels")]
pub enum PerformanceLevels {
    #[sea_orm(string_value = "below_expectations")]
    BelowExpectations,
    #[sea_orm(string_value = "exceeds_expectations")]
    ExceedsExpectations,
    #[sea_orm(string_value = "meets_expectations")]
    MeetsExpectations,
    #[sea_orm(string_value = "needs_improvement")]
    NeedsImprovement,
    #[sea_orm(string_value = "not_yet_meeting_expectations")]
    NotYetMeetingExpectations,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "person_enums")]
pub enum PersonEnums {
    #[sea_orm(string_value = "not_defined")]
    NotDefined,
    #[sea_orm(string_value = "parent")]
    Parent,
    #[sea_orm(string_value = "student")]
    Student,
    #[sea_orm(string_value = "teacher")]
    Teacher,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "roles")]
pub enum Roles {
    #[sea_orm(string_value = "admin")]
    Admin,
    #[sea_orm(string_value = "assistant")]
    Assistant,
    #[sea_orm(string_value = "not_defined")]
    NotDefined,
    #[sea_orm(string_value = "parent")]
    Parent,
    #[sea_orm(string_value = "student")]
    Student,
    #[sea_orm(string_value = "teacher")]
    Teacher,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "time_table_item_categories"
)]
pub enum TimeTableItemCategories {
    #[sea_orm(string_value = "activity")]
    Activity,
    #[sea_orm(string_value = "event")]
    Event,
    #[sea_orm(string_value = "lecture")]
    Lecture,
}
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "week_days")]
pub enum WeekDays {
    #[sea_orm(string_value = "friday")]
    Friday,
    #[sea_orm(string_value = "monday")]
    Monday,
    #[sea_orm(string_value = "saturday")]
    Saturday,
    #[sea_orm(string_value = "sunday")]
    Sunday,
    #[sea_orm(string_value = "thursday")]
    Thursday,
    #[sea_orm(string_value = "tuesday")]
    Tuesday,
    #[sea_orm(string_value = "wednesday")]
    Wednesday,
}
