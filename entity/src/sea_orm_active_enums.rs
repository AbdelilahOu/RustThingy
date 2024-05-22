//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.14

use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "announcement_categories")]
pub enum AnnouncementCategories {
    #[sea_orm(string_value = "academic")]
    Academic,
    #[sea_orm(string_value = "event")]
    Event,
    #[sea_orm(string_value = "general")]
    General,
}

impl Into<AnnouncementCategories> for String {
    fn into(self) -> AnnouncementCategories {
        match self.as_str() {
            "academic" => AnnouncementCategories::Academic,
            "event" => AnnouncementCategories::Event,
            _ => AnnouncementCategories::General,
        }
    }
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

impl Into<Audiences> for String {
    fn into(self) -> Audiences {
        match self.as_str() {
            "groups" => Audiences::Groups,
            "parents" => Audiences::Parents,
            "students" => Audiences::Students,
            "teachers" => Audiences::Teachers,
            _ => Audiences::All,
        }
    }
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

impl Into<PerformanceLevels> for String {
    fn into(self) -> PerformanceLevels {
        match self.as_str() {
            "exceeds_expectations" => PerformanceLevels::ExceedsExpectations,
            "meets_expectations" => PerformanceLevels::MeetsExpectations,
            "below_expectations" => PerformanceLevels::BelowExpectations,
            "needs_improvement" => PerformanceLevels::NeedsImprovement,
            _ => PerformanceLevels::NotYetMeetingExpectations,
        }
    }
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

impl Into<String> for Roles {
    fn into(self) -> String {
        match self {
            Roles::Admin => "admin".to_string(),
            Roles::Assistant => "assistant".to_string(),
            Roles::Student => "student".to_string(),
            Roles::Teacher => "teacher".to_string(),
            Roles::Parent => "parent".to_string(),
            Roles::NotDefined => "not-defined".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "time_table_item_categories")]
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

pub struct WeekDayNumber(pub u32);

impl Into<Option<WeekDays>> for WeekDayNumber {
    fn into(self) -> Option<WeekDays> {
        match self {
            WeekDayNumber(0) => Some(WeekDays::Sunday),
            WeekDayNumber(1) => Some(WeekDays::Monday),
            WeekDayNumber(2) => Some(WeekDays::Tuesday),
            WeekDayNumber(3) => Some(WeekDays::Wednesday),
            WeekDayNumber(4) => Some(WeekDays::Thursday),
            WeekDayNumber(5) => Some(WeekDays::Friday),
            WeekDayNumber(6) => Some(WeekDays::Saturday),
            _ => None,
        }
    }
}
