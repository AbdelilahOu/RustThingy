use crate::types::shared::{ResponseData, State};
use actix_web::{
    web::{Json, Path, Query},
    HttpResponse,
};
use service::{
    models::{Subject, SubjectQuery},
    mutation::MutationService,
    query::QueryService,
    uuid::Uuid,
};

//
type Body = Json<Subject>;
pub async fn create(body: Body, state: State) -> HttpResponse {
    let res = MutationService::create_subject(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("Subject created successfully".to_string()),
            data: Some(id.to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn delete(id: Path<Uuid>, state: State) -> HttpResponse {
    let delete_res = MutationService::delete_subject(&state.db_conn, id.into_inner()).await;
    match delete_res {
        Ok(i) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Subject deleted successfully".to_string()),
            data: Some(i.to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list_by_level_id(id: Path<Uuid>, state: State) -> HttpResponse {
    let selected_subject =
        QueryService::list_level_subjects(&state.db_conn, id.into_inner()).await;
    match selected_subject {
        Ok(i) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Subject selected by level id successfully".to_string()),
            data: Some(i),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list(q: Query<SubjectQuery>, state: State) -> HttpResponse {
    let subjects = QueryService::list_subjects(&state.db_conn, q.into_inner()).await;
    match subjects {
        Ok(i) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Subjects selected successfully".to_string()),
            data: Some(i),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn update(id: Path<Uuid>, body: Body, state: State) -> HttpResponse {
    let update_res =
        MutationService::update_subject(&state.db_conn, id.into_inner(), body.into_inner()).await;
    match update_res {
        Ok(i) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Subject updated successfully".to_string()),
            data: Some(i),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
