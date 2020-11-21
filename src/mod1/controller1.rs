extern crate jn_actix_initializer_macros;

use actix_web::{get, HttpRequest, HttpResponse};
use jn_actix_initializer_macros::register_handler;

#[get("/controller1")]
#[register_handler]
pub fn controller1_0(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json((
        "CONTROLLER 1",
        format!("HOST: '{}'", req.clone().connection_info().host()),
        "RESPONSE",
    ))
}
