use sea_orm::{prelude::Uuid, *};

use crate::{
    entities::*,
    models::{
        CActivity, CEvent, CLecture, CUser, ParentWithAddress, StudentWithAddress,
        TeacherWithAddress,
    },
    utils::convert_to_enum::to_day_of_week,
};

pub struct TransactionsService;

type TxnRes<T> = Result<T, TransactionError<DbErr>>;

impl TransactionsService {
    pub async fn create_student(db: DbConn, data: StudentWithAddress) -> TxnRes<()> {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // create details first
                let details = PersonDetailsActiveModel {
                    email: Set(data.details.email),
                    phone_number: Set(data.details.phone),
                    country_id: Set(data.details.country_id),
                    state_id: Set(data.details.state_id),
                    city_id: Set(data.details.city_id),
                    street_id: Set(data.details.street_id),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                // create person
                let person = PersonActiveModel {
                    details_id: Set(Some(details.id)),
                    person_type: Set("student".to_owned()),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                // create student
                let _student = StudentActiveModel {
                    first_name: Set(data.student.first_name),
                    last_name: Set(data.student.last_name),
                    group_id: Set(data.student.group_id),
                    person_id: Set(Some(person.id)),
                    ..Default::default()
                }
                .insert(txn)
                .await?;

                Ok(())
            })
        })
        .await
    }
    pub async fn create_teacher(db: DbConn, data: TeacherWithAddress) -> TxnRes<()> {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // create details first
                let details = PersonDetailsActiveModel {
                    email: Set(data.details.email),
                    phone_number: Set(data.details.phone),
                    country_id: Set(data.details.country_id),
                    state_id: Set(data.details.state_id),
                    city_id: Set(data.details.city_id),
                    street_id: Set(data.details.street_id),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                // create person
                let person = PersonActiveModel {
                    details_id: Set(Some(details.id)),
                    person_type: Set("parent".to_owned()),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                // create student
                let _teacher = TeacherActiveModel {
                    first_name: Set(data.teacher.first_name),
                    last_name: Set(data.teacher.last_name),
                    person_id: Set(Some(person.id)),
                    ..Default::default()
                }
                .insert(txn)
                .await?;

                Ok(())
            })
        })
        .await
    }
    pub async fn create_parent(db: DbConn, data: ParentWithAddress) -> TxnRes<()> {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // create details first
                let details = PersonDetailsActiveModel {
                    email: Set(data.details.email),
                    phone_number: Set(data.details.phone),
                    country_id: Set(data.details.country_id),
                    state_id: Set(data.details.state_id),
                    city_id: Set(data.details.city_id),
                    street_id: Set(data.details.street_id),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                // create person
                let person = PersonActiveModel {
                    details_id: Set(Some(details.id)),
                    person_type: Set("parent".to_owned()),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                // create student
                let _parent = ParentActiveModel {
                    first_name: Set(data.parent.first_name),
                    last_name: Set(data.parent.last_name),
                    person_id: Set(Some(person.id)),
                    ..Default::default()
                }
                .insert(txn)
                .await?;

                Ok(())
            })
        })
        .await
    }
    pub async fn upsert_user(db: &DbConn, data: CUser) -> TxnRes<Uuid> {
        db.transaction::<_, Uuid, DbErr>(|txn| {
            Box::pin(async move {
                // check if user exists
                let user = User::find()
                    .filter(users::Column::Email.eq(&data.email))
                    .one(txn)
                    .await?;

                if user.is_some() {
                    println!("user already exists");
                    // upsert the user first
                    return Ok(user.unwrap().id.to_owned());
                }
                // create details first
                let c_person = PersonActiveModel {
                    person_type: Set("NOT DEFINED".to_owned()),
                    ..Default::default()
                }
                .insert(txn)
                .await?;

                let c_user = UserActiveModel {
                    first_name: Set(data.first_name),
                    last_name: Set(data.last_name),
                    email: Set(data.email),
                    picture: Set(data.picture),
                    person_id: Set(c_person.id),
                    ..Default::default()
                }
                .insert(txn)
                .await?;

                Ok(c_user.id)
            })
        })
        .await
    }
    pub async fn create_event(db: &DbConn, data: CEvent) -> TxnRes<()> {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // create time table
                let timetable_active_modal = TimeTableActiveModel {
                    item_type: Set(TimeTableItemType::Lecture),
                    full_date: Set(Some(data.full_date)),
                    start_time: Set(Some(data.start_time)),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                // create create event
                EventActiveModel {
                    time_table_id: Set(Some(timetable_active_modal.id)),
                    event_title: Set(Some(data.event_title)),
                    event_description: Set(Some(data.event_description)),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                Ok(())
            })
        })
        .await
    }
    pub async fn create_activity(db: &DbConn, data: CActivity) -> TxnRes<()> {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // create time table
                let timetable_active_modal = TimeTableActiveModel {
                    item_type: Set(TimeTableItemType::Activity),
                    start_time: Set(Some(data.start_time)),
                    end_time: Set(Some(data.end_time)),
                    day_of_week: Set(to_day_of_week(data.day_of_week)),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                // create create activity
                ActivityActiveModel {
                    time_table_id: Set(Some(timetable_active_modal.id)),
                    activity_title: Set(Some(data.activity_title)),
                    activity_description: Set(Some(data.activity_description)),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                Ok(())
            })
        })
        .await
    }
    pub async fn create_lecture(db: &DbConn, data: CLecture) -> TxnRes<()> {
        db.transaction::<_, (), DbErr>(|txn| {
            Box::pin(async move {
                // create time table
                let timetable_active_modal = TimeTableActiveModel {
                    item_type: Set(TimeTableItemType::Lecture),
                    start_time: Set(Some(data.start_time)),
                    end_time: Set(Some(data.end_time)),
                    day_of_week: Set(to_day_of_week(data.day_of_week)),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                // create create lecture
                LectureActiveModel {
                    time_table_id: Set(Some(timetable_active_modal.id)),
                    class_id: Set(Some(data.class_id)),
                    ..Default::default()
                }
                .insert(txn)
                .await?;
                Ok(())
            })
        })
        .await
    }
    //
}
