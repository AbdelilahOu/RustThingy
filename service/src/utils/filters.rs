use crate::entities::{
    parents::Column as ParentCol, students::Column as StudentCol, teachers::Column as TeacherCol,
};
use crate::query::Filters;
use sea_orm::{ColumnTrait, Condition};

pub fn generate_student_filters(filters: Vec<Filters>) -> Condition {
    filters
        .into_iter()
        .fold(Condition::all(), |conditions, filter| {
            match filter.feild.as_str() {
                "full_name" => conditions.add(StudentCol::FullName.contains(filter.value)),
                "group_id" => conditions.add(StudentCol::GroupId.eq(filter.value)),
                _ => conditions,
            }
        })
}

pub fn generate_teacher_filters(filters: Vec<Filters>) -> Condition {
    filters
        .into_iter()
        .fold(Condition::all(), |conditions, filter| {
            match filter.feild.as_str() {
                "full_name" => conditions.add(TeacherCol::FullName.contains(filter.value)),
                _ => conditions,
            }
        })
}

pub fn generate_parent_filters(filters: Vec<Filters>) -> Condition {
    filters
        .into_iter()
        .fold(Condition::all(), |conditions, filter| {
            match filter.feild.as_str() {
                "full_name" => conditions.add(ParentCol::FullName.contains(filter.value)),
                _ => conditions,
            }
        })
}
