use crate::types::shared::*;
use actix_web::{web::Json, HttpResponse};
use service::{models::Scan, mutation::*, query::*};

type Body = Json<Scan>;
pub async fn create(body: Body, state: State) -> HttpResponse {
    let res = MutationsService::create_scan(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Created().json(ResponseData {
            error: None,
            message: Some("scan created successfully".to_string()),
            data: Some(id.to_string()),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}

pub async fn list(q: TQueries, body: TFiltersBody, state: State) -> HttpResponse {
    let scans = QueriesService::list_scans(
        &state.db_conn,
        QueriesFilters {
            queries: q.into_inner(),
            filters: body.clone().filters,
        },
    )
    .await;

    match scans {
        Ok(i) => HttpResponse::Ok().json(ResponseData {
            error: None,
            message: Some("Scans selected successfully".to_string()),
            data: Some(i),
        }),
        Err(e) => HttpResponse::InternalServerError().json(ResponseData::<Option<String>> {
            error: Some(e.to_string()),
            message: None,
            data: None,
        }),
    }
}
