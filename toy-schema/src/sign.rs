use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SignReq {
    #[serde(default)]
    pub name: String,
    pub email: String,
    pub password: String,
}
