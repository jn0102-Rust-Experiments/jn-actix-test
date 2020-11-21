use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TestResponse {
    pub some_value0: String,
    pub some_value1: i64,
    pub string_vec: Vec<String>,
}
