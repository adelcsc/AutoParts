pub mod auth_controller {
    use actix_web::HttpResponse;

    pub(crate) async fn loginUser() -> HttpResponse {
        HttpResponse::Ok().body("Hello,World")
    }
}
