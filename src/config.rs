use actix_web::web;
use crate::controllers;
pub fn config(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope(&"/v1").
            service(
                web::resource("/auth").
                    route(web::get().to(controllers::auth_controller::loginUser))
            )
    );
}
