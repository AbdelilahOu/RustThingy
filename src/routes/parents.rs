use crate::handlers::parents;
use actix_web::{web, Scope};

pub fn load_parents_routes() -> Scope {
    web::scope("/parents")
        .route("/", web::post().to(parents::create_parent))
        .route("/{id}", web::get().to(parents::get_parent))
        .route("/{id}", web::put().to(parents::update_parent))
        .route("/search", web::post().to(parents::get_parents))
        .route("/{id}", web::delete().to(parents::delete_parent))
}