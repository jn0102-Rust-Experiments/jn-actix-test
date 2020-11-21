extern crate jn_actix_initializer_macros;

use super::super::model::test_response::TestResponse;
use actix_web::{get, HttpResponse};
use jn_actix_initializer_macros::register_handler;

#[get("/")]
#[register_handler]
pub fn test_json() -> HttpResponse {
    HttpResponse::Ok().json(TestResponse {
        some_value0: "value_0".to_string(),
        some_value1: 2,
        string_vec: vec![
            "test-0a".to_string(),
            "test-0b".to_string(),
            "test-0c".to_string(),
        ],
    })
}

#[get("/test2")]
#[register_handler]
pub async fn test_json2() -> HttpResponse {
    HttpResponse::Ok().json(("TEST 2", 5, 8, "LAST"))
}
