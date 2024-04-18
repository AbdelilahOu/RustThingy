use crate::types::shared::{ResponseData, State};
use actix_web::{
    web::{Json, Path, Query},
    HttpResponse as Response,
};
use service::{
    models::{Class, ClassQuery},
    mutation::MutationService,
    query::QueryService,
    uuid::Uuid,
};
//
pub async fn create(body: Json<Class>, state: State) -> Response {
    let res = MutationService::create_class(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => Response::Created().json(ResponseData {
            error: None,
            message: Some("Class created successfully".to_string()),
            data: Some(id.to_string()),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn delete(id: Path<Uuid>, state: State) -> Response {
    let res = MutationService::delete_class(&state.db_conn, id.into_inner()).await;
    match res {
        Ok(delete_count) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("Class deleted successfully".to_string()),
            data: Some(delete_count.to_string()),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list(query: Query<ClassQuery>, state: State) -> Response {
    let res = QueryService::list_classes(&state.db_conn, query.into_inner()).await;
    match res {
        Ok(classes) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("Classs selected successfully".to_string()),
            data: Some(classes),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn update(id: Path<Uuid>, body: Json<Class>, state: State) -> Response {
    let res =
        MutationService::update_class(&state.db_conn, id.into_inner(), body.into_inner()).await;
    match res {
        Ok(id) => Response::Ok().json(ResponseData {
            error: None,
            message: Some("Class updated successfully".to_string()),
            data: Some(id),
        }),
        Err(e) => Response::InternalServerError().json(ResponseData::<String> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
