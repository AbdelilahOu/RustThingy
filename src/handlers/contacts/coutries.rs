use crate::{models::commen::*, AppState};
use actix_web::{
    http::{header::ContentType, StatusCode},
    web::{Data as ActData, Json as ActJson, Path as ActPath, Query as ActQuery},
    HttpResponse,
};

use service::*;
use uuid::Uuid;

// i like my functions to stay inline
type TFiltersBody = ActJson<FiltersBody>;
type TQueries = ActQuery<ListQuery>;
type State = ActData<AppState>;
type IdParam = ActPath<Uuid>;
type CtBody = ActJson<CCountry>;

pub async fn create_country(body: CtBody, state: State) -> HttpResponse {
    let res = ServiceMutation::create_country(&state.db_conn, body.into_inner()).await;
    match res {
        Ok(id) => HttpResponse::Ok()
            .status(StatusCode::CREATED)
            .content_type(ContentType::json())
            .json(ResultResponse {
                error: None,
                message: Some("COUNTRYCCountry created successfully".to_string()),
                data: Some(id.to_string()),
            }),
        Err(e) => HttpResponse::Ok()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .content_type(ContentType::json())
            .json(ResultResponse::<Option<String>> {
                error: Some(e.to_string()),
                message: None,
                data: None,
            }),
    }
}

pub async fn delete_country(id: IdParam, state: State) -> HttpResponse {
    let delete_res = ServiceMutation::delete_country(&state.db_conn, id.into_inner()).await;

    match delete_res {
        Ok(i) => HttpResponse::Created()
            .content_type(ContentType::json())
            .json(ResultResponse {
                error: None,
                message: Some("COUNTRYCCountry deleted successfully".to_string()),
                data: Some(i.to_string()),
            }),
        Err(e) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .json(ResultResponse::<Option<String>> {
                error: Some(e),
                message: None,
                data: None,
            }),
    }
}

// pub async fn get_country(id: IdParam, state: State) -> HttpResponse {
//     let selected_country = ServiceQuery::get_country(id.into_inner(), &state.db_conn).await;

//     match selected_country {
//         Ok(i) => HttpResponse::Created()
//             .content_type(ContentType::json())
//             .json(ResultResponse {
//                 error: None,
//                 message: Some("COUNTRYCCountry selected successfully".to_string()),
//                 data: Some(i),
//             }),
//         Err(e) => HttpResponse::InternalServerError()
//             .content_type(ContentType::json())
//             .json(ResultResponse::<Option<String>> {
//                 error: Some(e),
//                 message: None,
//                 data: None,
//             }),
//     }
// }

// pub async fn get_countrys(queries: TQueries, body: TFiltersBody, state: State) -> HttpResponse {
//     let countrys = ServiceQuery::list_countrys(
//         QueriesFilters {
//             queries: queries.into_inner(),
//             filters: body.clone().filters,
//         },
//         &state.db_conn,
//     )
//     .await;

//     match countrys {
//         Ok(i) => HttpResponse::Created()
//             .content_type(ContentType::json())
//             .json(ResultResponse {
//                 error: None,
//                 message: Some("COUNTRYCCountrys selected successfully".to_string()),
//                 data: Some(i),
//             }),
//         Err(e) => HttpResponse::InternalServerError()
//             .content_type(ContentType::json())
//             .json(ResultResponse::<Option<String>> {
//                 error: Some(e),
//                 message: None,
//                 data: None,
//             }),
//     }
// }

pub async fn update_country(id: IdParam, body: CtBody, state: State) -> HttpResponse {
    let update_res =
        ServiceMutation::update_country(&state.db_conn, id.into_inner(), body.into_inner()).await;
    match update_res {
        Ok(i) => HttpResponse::Created()
            .content_type(ContentType::json())
            .json(ResultResponse {
                error: None,
                message: Some("COUNTRYCCountry updated successfully".to_string()),
                data: Some(i),
            }),
        Err(e) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .json(ResultResponse::<Option<String>> {
                error: Some(e),
                message: None,
                data: None,
            }),
    }
}
