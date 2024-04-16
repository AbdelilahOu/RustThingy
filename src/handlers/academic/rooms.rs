use crate::types::shared::{ResponseData, State};
use actix_web::{
    web::{Json, Path, Query},
    HttpResponse,
};
use service::{
    models::{Room, RoomQuery},
    mutation::MutationService,
    query::QueryService,
    uuid::Uuid,
};
//
type Body = Json<Room>;
pub async fn create(body: Body, state: State) -> HttpResponse {
    let res = MutationService::create_room(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("Room created successfully".to_string()),
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
    let delete_res = MutationService::delete_room(&state.db_conn, id.into_inner()).await;
    match delete_res {
        Ok(i) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Room deleted successfully".to_string()),
            data: Some(i.to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list(q: Query<RoomQuery>, state: State) -> HttpResponse {
    let rooms = QueryService::list_rooms(&state.db_conn, q.into_inner()).await;
    match rooms {
        Ok(i) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Rooms selected successfully".to_string()),
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
        MutationService::update_room(&state.db_conn, id.into_inner(), body.into_inner()).await;
    match update_res {
        Ok(i) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Room updated successfully".to_string()),
            data: Some(i),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
