use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use models::commen::ConfigObj;
use service::sea_orm::DatabaseConnection;

mod config;
mod database;
mod guards;
mod handlers;
mod models;
mod routes;
mod utils;

use crate::routes::*;
use database::establish_connection;

pub struct AppState {
    db_conn: DatabaseConnection,
    config: ConfigObj,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load config
    let loaded_config = config::load_config();
    //
    let conn = establish_connection(loaded_config.db_url.clone())
        .await
        .unwrap();
    //
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // start server
    println!("Server started on http://0.0.0.0:8080");
    let _ = HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("> %r status: [%s] took: %T s"))
            .app_data(web::Data::new(AppState {
                db_conn: conn.clone(),
                config: loaded_config.clone(),
            }))
            .route("/", web::get().to(handlers::health_check::healthy))
            .service(load_students_routes())
            .service(load_teachers_routes())
            .service(load_details_routes())
            .service(load_parents_routes())
            .service(load_levels_routes())
            .service(load_scans_routes())
            .service(load_auth_routes())
            .service(load_subjects_routes())
            .service(load_groups_routes())
            .service(load_rooms_routes())
            .service(load_classes_routes())
            .service(load_attendance_routes())
            .service(load_timetable_routes())
            .service(load_assignments_routes())
            .service(load_grades_routes())
            .service(load_disciplinary_actions_routes())
            .default_service(web::to(|| HttpResponse::NotFound()))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await;
    Ok(())
}
