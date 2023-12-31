use actix_web::{web, Scope};

use crate::handlers::details;

pub fn load_states_routes() -> Scope {
    web::scope("/states")
        .route("/filters", web::post().to(details::list_states))
        .route("/", web::post().to(details::create_state))
        .route("/{id}", web::put().to(details::update_state))
        .route("/{id}", web::delete().to(details::delete_state))
}
