use actix_web::web;

mod home;
mod login;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/home", web::get().to(home::route))
        .route("/login", web::get().to(login::route));
}
