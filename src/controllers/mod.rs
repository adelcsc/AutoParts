pub mod auth_controller {
    use std::fmt::format;
    use actix_web::{HttpResponse, web};
    use actix_web::guard::Head;
    use jsonwebtoken::{EncodingKey, Header};
    use serde::{Deserialize,Serialize};
    #[derive(Deserialize,Serialize)]
    pub struct LoginRequest {
        username : String,
        password : String
    }
    pub(crate) async fn loginUser(
        info : web::Json<LoginRequest>
    ) -> HttpResponse {
        let token = jsonwebtoken::encode(&Header{
            typ: None,
            alg: Default::default(),
            cty: None,
            jku: None,
            jwk: None,
            kid: None,
            x5u: None,
            x5c: None,
            x5t: None,
            x5t_s256: None
        },&LoginRequest{
            username: "".to_string(),
            password: "".to_string()
        },&EncodingKey::from_secret(("lol").as_ref())).unwrap();
        HttpResponse::Ok().body(format!("{}",token))
    }
}
