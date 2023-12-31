use std::collections::HashMap;

use super::types::*;
use super::utils::filters::*;
use ::entity::{groups, levels, parents, persons, prelude::*, scans, students, subjects, teachers};
use chrono::NaiveDateTime;
use sea_orm::{
    prelude::Uuid,
    sea_query::{
        extension::postgres::PgExpr, Alias, Expr, PostgresQueryBuilder, Query, SimpleExpr,
        SubQueryStatement,
    },
    *,
};
use serde_json::Value as SerdValue;

type JsonV = SerdValue;
type Values = Vec<JsonV>;

pub struct QueriesService;

impl QueriesService {
    // students entity
    pub async fn list_students(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
        let students = Student::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .filter(generate_student_filters(qf.filters))
            .into_json()
            .all(db)
            .await?;

        Ok(students)
    }
    //
    pub async fn get_student(db: &DbConn, id: Uuid) -> Result<Option<JsonV>, DbErr> {
        let student = Student::find_by_id(id).into_json().one(db).await?;
        Ok(student)
    }
    //
    pub async fn list_teachers(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
        let teachers = Teacher::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .filter(generate_teacher_filters(qf.filters))
            .into_json()
            .all(db)
            .await?;

        Ok(teachers)
    }
    //
    pub async fn get_teacher(db: &DbConn, id: Uuid) -> Result<Option<JsonV>, DbErr> {
        let teacher = Teacher::find_by_id(id).into_json().one(db).await?;
        Ok(teacher)
    }
    //
    pub async fn list_parents(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
        let parents = Parent::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .filter(generate_parent_filters(qf.filters))
            .into_json()
            .all(db)
            .await?;

        Ok(parents)
    }
    //
    pub async fn get_parent(db: &DbConn, id: Uuid) -> Result<Option<JsonV>, DbErr> {
        let parent = Parent::find_by_id(id).into_json().one(db).await?;
        Ok(parent)
    }
    //
    pub async fn get_details(db: &DbConn, id: Uuid) -> Result<Option<JsonV>, DbErr> {
        let details = PersonDetails::find_by_id(id).into_json().one(db).await?;
        Ok(details)
    }
    //
    pub async fn list_countries(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
        let countries = Country::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .into_json()
            .all(db)
            .await?;

        Ok(countries)
    }
    //
    pub async fn list_states(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
        let states = State::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .into_json()
            .all(db)
            .await?;

        Ok(states)
    }
    //
    pub async fn list_cities(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
        let cities = City::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .into_json()
            .all(db)
            .await?;

        Ok(cities)
    }
    //
    pub async fn list_districts(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
        let districts = District::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .into_json()
            .all(db)
            .await?;

        Ok(districts)
    }
    //
    pub async fn list_streets(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
        let streets = Street::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .into_json()
            .all(db)
            .await?;

        Ok(streets)
    }
    //
    pub async fn list_scans(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
        let mut filters = HashMap::<String, Filters>::new();
        qf.filters.into_iter().for_each(|f| {
            filters.insert(f.feild.clone(), f);
        });
        //
        let (sql, values) = Query::select()
            .from(Scans)
            .exprs([
                Expr::col((Scans, scans::Column::Id)),
                Expr::col((Scans, scans::Column::PersonId)),
                Expr::col((Scans, scans::Column::ScanDate)),
                Expr::col((Persons, persons::Column::PersonType)),
            ])
            // GET full_name
            .expr_as(
                Expr::case(
                    Expr::col(persons::Column::PersonType).eq("student".to_owned()),
                    SimpleExpr::SubQuery(
                        None,
                        Box::new(SubQueryStatement::SelectStatement(
                            Query::select()
                                .from(Student)
                                .column(students::Column::FullName)
                                .cond_where(
                                    Expr::col((Student, students::Column::PersonId))
                                        .equals((Scans, scans::Column::PersonId)),
                                )
                                .to_owned(),
                        )),
                    ),
                )
                .case(
                    Expr::col(persons::Column::PersonType).eq("parent".to_owned()),
                    SimpleExpr::SubQuery(
                        None,
                        Box::new(SubQueryStatement::SelectStatement(
                            Query::select()
                                .from(Parent)
                                .column(parents::Column::FullName)
                                .cond_where(
                                    Expr::col((Parent, parents::Column::PersonId))
                                        .equals((Scans, scans::Column::PersonId)),
                                )
                                .to_owned(),
                        )),
                    ),
                )
                .case(
                    Expr::col(persons::Column::PersonType).eq("teacher".to_owned()),
                    SimpleExpr::SubQuery(
                        None,
                        Box::new(SubQueryStatement::SelectStatement(
                            Query::select()
                                .from(Teacher)
                                .column(teachers::Column::FullName)
                                .cond_where(
                                    Expr::col((Teacher, teachers::Column::PersonId))
                                        .equals((Scans, scans::Column::PersonId)),
                                )
                                .to_owned(),
                        )),
                    ),
                ),
                Alias::new("full_name"),
            )
            // GET _id
            .expr_as(
                Expr::case(
                    Expr::col(persons::Column::PersonType).eq("student".to_owned()),
                    SimpleExpr::SubQuery(
                        None,
                        Box::new(SubQueryStatement::SelectStatement(
                            Query::select()
                                .from(Student)
                                .column(students::Column::Id)
                                .cond_where(
                                    Expr::col((Student, students::Column::PersonId))
                                        .equals((Scans, scans::Column::PersonId)),
                                )
                                .to_owned(),
                        )),
                    ),
                )
                .case(
                    Expr::col(persons::Column::PersonType).eq("parent".to_owned()),
                    SimpleExpr::SubQuery(
                        None,
                        Box::new(SubQueryStatement::SelectStatement(
                            Query::select()
                                .from(Parent)
                                .column(parents::Column::Id)
                                .cond_where(
                                    Expr::col((Parent, parents::Column::PersonId))
                                        .equals((Scans, scans::Column::PersonId)),
                                )
                                .to_owned(),
                        )),
                    ),
                )
                .case(
                    Expr::col(persons::Column::PersonType).eq("teacher".to_owned()),
                    SimpleExpr::SubQuery(
                        None,
                        Box::new(SubQueryStatement::SelectStatement(
                            Query::select()
                                .from(Teacher)
                                .column(teachers::Column::Id)
                                .cond_where(
                                    Expr::col((Teacher, teachers::Column::PersonId))
                                        .equals((Scans, scans::Column::PersonId)),
                                )
                                .to_owned(),
                        )),
                    ),
                ),
                Alias::new("_id"),
            )
            //
            .join(
                JoinType::Join,
                Persons,
                Expr::col((Persons, persons::Column::Id)).equals((Scans, scans::Column::PersonId)),
            )
            // FULL_NAME filter
            .conditions(
                filters.get("full_name").is_some(),
                |x| {
                    let full_name = filters.get("full_name").unwrap().value.as_str();
                    x.and_where(
                        Expr::case(
                            Expr::col(persons::Column::PersonType).eq("student".to_owned()),
                            SimpleExpr::SubQuery(
                                None,
                                Box::new(SubQueryStatement::SelectStatement(
                                    Query::select()
                                        .from(Student)
                                        .column(students::Column::FullName)
                                        .cond_where(
                                            Expr::col((Student, students::Column::PersonId))
                                                .equals((Scans, scans::Column::PersonId)),
                                        )
                                        .to_owned(),
                                )),
                            )
                            .ilike(format!("%{}%", full_name)),
                        )
                        .case(
                            Expr::col(persons::Column::PersonType).eq("parent".to_owned()),
                            SimpleExpr::SubQuery(
                                None,
                                Box::new(SubQueryStatement::SelectStatement(
                                    Query::select()
                                        .from(Parent)
                                        .column(parents::Column::FullName)
                                        .cond_where(
                                            Expr::col((Parent, parents::Column::PersonId))
                                                .equals((Scans, scans::Column::PersonId)),
                                        )
                                        .to_owned(),
                                )),
                            )
                            .ilike(format!("%{}%", full_name)),
                        )
                        .case(
                            Expr::col(persons::Column::PersonType).eq("teacher".to_owned()),
                            SimpleExpr::SubQuery(
                                None,
                                Box::new(SubQueryStatement::SelectStatement(
                                    Query::select()
                                        .from(Teacher)
                                        .column(teachers::Column::FullName)
                                        .cond_where(
                                            Expr::col((Teacher, teachers::Column::PersonId))
                                                .equals((Scans, scans::Column::PersonId)),
                                        )
                                        .to_owned(),
                                )),
                            )
                            .ilike(format!("%{}%", full_name)),
                        )
                        .into(),
                    );
                },
                |_| {},
            )
            // START scan_date
            .conditions(
                filters.get("scan_time_start").is_some(),
                |x| {
                    // get avlue
                    let start_time_feild_value =
                        filters.get("scan_time_start").unwrap().value.as_str();
                    // check
                    let start_time = NaiveDateTime::parse_from_str(
                        start_time_feild_value,
                        if start_time_feild_value.contains("T") {
                            "%Y-%m-%dT%H:%M:%S%.f"
                        } else {
                            "%Y-%m-%d %H:%M:%S%.f"
                        },
                    );
                    // parse success
                    if let Ok(start_time) = start_time {
                        x.and_where(Expr::col((Scans, scans::Column::ScanDate)).gte(start_time));
                    } else {
                        println!("error parsing date : {:?}", start_time.err());
                    }
                },
                |_| {},
            )
            // END scan_date
            .conditions(
                filters.get("scan_time_end").is_some(),
                |x| {
                    // get avlue
                    let end_time_feild_value = filters.get("scan_time_end").unwrap().value.as_str();
                    // check
                    let end_time = NaiveDateTime::parse_from_str(
                        end_time_feild_value,
                        if end_time_feild_value.contains("T") {
                            "%Y-%m-%dT%H:%M:%S%.f"
                        } else {
                            "%Y-%m-%d %H:%M:%S%.f"
                        },
                    );
                    // parse success
                    if let Ok(end_time) = end_time {
                        x.and_where(Expr::col((Scans, scans::Column::ScanDate)).lte(end_time));
                    } else {
                        println!("error parsing date : {:?}", end_time.err());
                    }
                },
                |_| {},
            )
            // FILTER BY PERSON_TYPE
            .conditions(
                filters.get("person_type").is_some(),
                |x| {
                    let person_type = filters.get("person_type").unwrap().value.as_str();
                    x.and_where(Expr::col(persons::Column::PersonType).eq(person_type));
                },
                |_| {},
            )
            //
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .to_owned()
            .build(PostgresQueryBuilder);

        let result = SelectScans::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            sql,
            values,
        ))
        .into_json()
        .all(db)
        .await?;

        Ok(result)
    }
    //
    pub async fn list_attendance(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
        let mut filters = HashMap::<String, Filters>::new();
        qf.filters.into_iter().for_each(|f| {
            filters.insert(f.feild.clone(), f);
        });
        //
        let (sql, values) = Query::select()
            .from(Scans)
            .exprs([
                Expr::col((Scans, scans::Column::Id)),
                Expr::col((Scans, scans::Column::PersonId)),
                Expr::col((Scans, scans::Column::ScanDate)),
                Expr::col((Student, students::Column::FullName)),
                Expr::col((Group, groups::Column::GroupName)),
                Expr::col((Level, levels::Column::LevelName)),
            ])
            // GET _id
            .expr_as(
                Expr::col((Student, students::Column::Id)),
                Alias::new("_id"),
            )
            //
            .join(
                JoinType::Join,
                Student,
                Expr::col((Student, students::Column::PersonId))
                    .equals((Scans, scans::Column::PersonId)),
            )
            //
            .join(
                JoinType::Join,
                Group,
                Expr::col((Group, groups::Column::Id)).equals((Student, students::Column::GroupId)),
            )
            //
            .join(
                JoinType::Join,
                Level,
                Expr::col((Level, levels::Column::Id)).equals((Group, groups::Column::LevelId)),
            )
            // FULL_NAME filter
            .conditions(
                filters.get("full_name").is_some(),
                |x| {
                    let full_name = filters.get("full_name").unwrap().value.as_str();
                    x.and_where(
                        Expr::col(students::Column::FullName).ilike(format!("%{}%", full_name)),
                    );
                },
                |_| {},
            )
            // START scan_date
            .conditions(
                filters.get("scan_time_start").is_some(),
                |x| {
                    // get avlue
                    let start_time_feild_value =
                        filters.get("scan_time_start").unwrap().value.as_str();
                    // check
                    let start_time = NaiveDateTime::parse_from_str(
                        start_time_feild_value,
                        if start_time_feild_value.contains("T") {
                            "%Y-%m-%dT%H:%M:%S%.f"
                        } else {
                            "%Y-%m-%d %H:%M:%S%.f"
                        },
                    );
                    // parse success
                    if let Ok(start_time) = start_time {
                        x.and_where(Expr::col((Scans, scans::Column::ScanDate)).gte(start_time));
                    } else {
                        println!("error parsing date : {:?}", start_time.err());
                    }
                },
                |_| {},
            )
            // END scan_date
            .conditions(
                filters.get("scan_time_end").is_some(),
                |x| {
                    // get avlue
                    let end_time_feild_value = filters.get("scan_time_end").unwrap().value.as_str();
                    // check
                    let end_time = NaiveDateTime::parse_from_str(
                        end_time_feild_value,
                        if end_time_feild_value.contains("T") {
                            "%Y-%m-%dT%H:%M:%S%.f"
                        } else {
                            "%Y-%m-%d %H:%M:%S%.f"
                        },
                    );
                    // parse success
                    if let Ok(end_time) = end_time {
                        x.and_where(Expr::col((Scans, scans::Column::ScanDate)).lte(end_time));
                    } else {
                        println!("error parsing date : {:?}", end_time.err());
                    }
                },
                |_| {},
            )
            // GROUP_ID
            .conditions(
                filters.get("group_id").is_some(),
                |x| {
                    let group_id = Uuid::parse_str(filters.get("group_id").unwrap().value.as_str());
                    if let Ok(id) = group_id {
                        x.and_where(Expr::col((Student, students::Column::GroupId)).eq(id));
                    } else {
                        println!("error parsing group_id : {:?}", group_id.err());
                    }
                },
                |_| {},
            )
            //
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .order_by(scans::Column::ScanDate, Order::Desc)
            .to_owned()
            .build(PostgresQueryBuilder);

        let result = SelectScans::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            sql,
            values,
        ))
        .into_json()
        .all(db)
        .await?;

        Ok(result)
    }
    //
    pub async fn list_levels(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
        let levels = Level::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .into_json()
            .all(db)
            .await?;

        Ok(levels)
    }
    //
    pub async fn list_subjects(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
        let subjects = Subject::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .into_json()
            .all(db)
            .await?;

        Ok(subjects)
    }
    //
    pub async fn list_level_subjects(db: &DbConn, level_id: Uuid) -> Result<Values, DbErr> {
        let level_subjects = Subject::find()
            .filter(subjects::Column::LevelId.eq(level_id.clone()))
            .into_json()
            .all(db)
            .await?;

        Ok(level_subjects)
    }
    //
    pub async fn list_groups(db: &DbConn, qf: QueriesFilters) -> Result<Values, DbErr> {
        let groups = Group::find()
            .offset((qf.queries.page - 1) * qf.queries.limit)
            .limit(qf.queries.limit)
            .into_json()
            .all(db)
            .await?;

        Ok(groups)
    }
    //
    pub async fn list_level_groups(db: &DbConn, level_id: Uuid) -> Result<Values, DbErr> {
        let level_groups = Group::find()
            .filter(groups::Column::LevelId.eq(level_id.clone()))
            .into_json()
            .all(db)
            .await?;

        Ok(level_groups)
    }
    //
    pub async fn list_rooms(db: &DbConn) -> Result<Values, DbErr> {
        let rooms = Room::find().into_json().all(db).await?;

        Ok(rooms)
    }
    //
    pub async fn list_classes(db: &DbConn) -> Result<Values, DbErr> {
        let classes = Class::find().into_json().all(db).await?;

        Ok(classes)
    }
}

// raw sql for : list_scans_related

//
// let result: Vec<SelectScans> = SelectScans::find_by_statement(
//     Statement::from_sql_and_values(
//     DbBackend::Postgres,
//     r#"
//             SELECT
//                 s.*,
//                 p.person_type,
//             CASE
//                 WHEN p.person_type = 'student' THEN (SELECT full_name FROM students where students.person_id = p.id)
//                 WHEN p.person_type = 'parent' THEN (SELECT full_name FROM parents where parents.person_id = p.id)
//                 ELSE (SELECT full_name FROM teachers where teachers.person_id = p.id)
//             END,
//             CASE
//                 WHEN p.person_type = 'student' THEN (SELECT id FROM students where students.person_id = p.id)
//                 WHEN p.person_type = 'parent' THEN (SELECT id FROM parents where parents.person_id = p.id)
//                 ELSE (SELECT id FROM teachers where teachers.person_id = p.id)
//             END as _id
//             FROM scans as s JOIN persons as p ON s.person_id = p.id LIMIT $1 OFFSET $2
//         "#,
//     [limit.into(),offset.into()]),
// )
// .all(db)
// .await?;
