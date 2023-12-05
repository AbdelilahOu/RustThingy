// use crate::handlers::scans;
use actix_web::{web, Scope};

use crate::handlers::{health_check, scans};

pub fn load_scans_routes() -> Scope {
    web::scope("/scans")
        .route("/", web::post().to(scans::create_scan))
        .route("/{id}", web::get().to(health_check::healthy))
        .route("/{id}", web::put().to(health_check::healthy))
        .route("/{id}", web::delete().to(health_check::healthy))
}
