use crate::{guards::auth::check_token, handlers::students};
use actix_web::{guard, web, Scope};

pub fn load_students_routes(secret: String) -> Scope {
    web::scope("/students")
        .guard(guard::fn_guard(move |ctx| {
            return check_token(ctx, secret.clone());
        }))
        .route("/", web::post().to(students::create_student))
        .route("/{id}", web::get().to(students::get_student))
        .route("/{id}", web::put().to(students::update_student))
        .route("/filters", web::post().to(students::list_students))
        .route("/{id}", web::delete().to(students::delete_student))
}
