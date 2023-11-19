use ::entity::parents::{ActiveModel as ParentActiveModel, Entity as Parent};
use ::entity::students::{ActiveModel as StudentActiveModel, Entity as Student};
use ::entity::teachers::{ActiveModel as TeacherActiveModel, Entity as Teacher};
use sea_orm::{prelude::Uuid, *};

use crate::{CParent, CStudent, CTeacher};

pub struct ServiceMutation;

impl ServiceMutation {
    // students entity
    pub async fn create_student(db: &DbConn, data: CStudent) -> Result<Uuid, DbErr> {
        let c_student = StudentActiveModel {
            first_name: Set(data.first_name),
            last_name: Set(data.last_name),
            level: Set(data.level),
            ..Default::default()
        };
        match Student::insert(c_student).exec(db).await {
            Ok(res) => Ok(res.last_insert_id),
            Err(err) => return Err(err),
        }
    }
    pub async fn delete_student(db: &DbConn, id: Uuid) -> Result<u64, String> {
        let selected_student = Student::find_by_id(id).one(db).await;
        match selected_student {
            Ok(student_opt) => match student_opt {
                Some(student_model) => match student_model.delete(db).await {
                    Ok(delete_a) => Ok(delete_a.rows_affected),
                    Err(err) => Err(err.to_string()),
                },
                None => Err(String::from("student doesnt exist")),
            },
            Err(err) => Err(err.to_string()),
        }
    }
    pub async fn update_student(db: &DbConn, id: Uuid, data: CStudent) -> Result<CStudent, String> {
        let selected_student = Student::find_by_id(id).one(db).await;
        match selected_student {
            Ok(student_opt) => match student_opt {
                Some(student_model) => {
                    let mut student_model: StudentActiveModel = student_model.into();
                    student_model.first_name = Set(data.first_name);
                    student_model.last_name = Set(data.last_name);
                    student_model.level = Set(data.level);
                    student_model.id = Set(id);
                    match student_model.update(db).await {
                        Ok(updated_student) => Ok(CStudent {
                            first_name: updated_student.first_name,
                            last_name: updated_student.last_name,
                            level: updated_student.level,
                        }),
                        Err(err) => Err(err.to_string()),
                    }
                }
                None => Err(String::from("student doesnt exist")),
            },
            Err(err) => Err(err.to_string()),
        }
    }
    // teachers entity
    pub async fn create_teacher(db: &DbConn, data: CTeacher) -> Result<Uuid, DbErr> {
        let c_teacher = TeacherActiveModel {
            first_name: Set(data.first_name),
            last_name: Set(data.last_name),
            ..Default::default()
        };
        match Teacher::insert(c_teacher).exec(db).await {
            Ok(res) => Ok(res.last_insert_id),
            Err(err) => return Err(err),
        }
    }
    pub async fn delete_teacher(db: &DbConn, id: Uuid) -> Result<u64, String> {
        let selected_teacher = Teacher::find_by_id(id).one(db).await;
        match selected_teacher {
            Ok(teacher_opt) => match teacher_opt {
                Some(teacher_model) => match teacher_model.delete(db).await {
                    Ok(delete_a) => Ok(delete_a.rows_affected),
                    Err(err) => Err(err.to_string()),
                },
                None => Err(String::from("teacher doesnt exist")),
            },
            Err(err) => Err(err.to_string()),
        }
    }
    pub async fn update_teacher(db: &DbConn, id: Uuid, data: CTeacher) -> Result<CTeacher, String> {
        let selected_teacher = Teacher::find_by_id(id).one(db).await;
        match selected_teacher {
            Ok(teacher_opt) => match teacher_opt {
                Some(teacher_model) => {
                    let mut teacher_model: TeacherActiveModel = teacher_model.into();
                    teacher_model.first_name = Set(data.first_name);
                    teacher_model.last_name = Set(data.last_name);
                    teacher_model.id = Set(id);
                    match teacher_model.update(db).await {
                        Ok(updated_teacher) => Ok(CTeacher {
                            first_name: updated_teacher.first_name,
                            last_name: updated_teacher.last_name,
                        }),
                        Err(err) => Err(err.to_string()),
                    }
                }
                None => Err(String::from("teacher doesnt exist")),
            },
            Err(err) => Err(err.to_string()),
        }
    }
    // parents entity
    pub async fn create_parent(db: &DbConn, data: CParent) -> Result<Uuid, DbErr> {
        let c_parent = ParentActiveModel {
            first_name: Set(data.first_name),
            last_name: Set(data.last_name),
            ..Default::default()
        };
        match Parent::insert(c_parent).exec(db).await {
            Ok(res) => Ok(res.last_insert_id),
            Err(err) => return Err(err),
        }
    }
    pub async fn delete_parent(db: &DbConn, id: Uuid) -> Result<u64, String> {
        let selected_parent = Parent::find_by_id(id).one(db).await;
        match selected_parent {
            Ok(parent_opt) => match parent_opt {
                Some(parent_model) => match parent_model.delete(db).await {
                    Ok(delete_a) => Ok(delete_a.rows_affected),
                    Err(err) => Err(err.to_string()),
                },
                None => Err(String::from("parent doesnt exist")),
            },
            Err(err) => Err(err.to_string()),
        }
    }
    pub async fn update_parent(db: &DbConn, id: Uuid, data: CParent) -> Result<CParent, String> {
        let selected_parent = Parent::find_by_id(id).one(db).await;
        match selected_parent {
            Ok(parent_opt) => match parent_opt {
                Some(parent_model) => {
                    let mut parent_model: ParentActiveModel = parent_model.into();
                    parent_model.first_name = Set(data.first_name);
                    parent_model.last_name = Set(data.last_name);
                    parent_model.id = Set(id);
                    match parent_model.update(db).await {
                        Ok(updated_parent) => Ok(CParent {
                            first_name: updated_parent.first_name,
                            last_name: updated_parent.last_name,
                        }),
                        Err(err) => Err(err.to_string()),
                    }
                }
                None => Err(String::from("parent doesnt exist")),
            },
            Err(err) => Err(err.to_string()),
        }
    }
    //
}
