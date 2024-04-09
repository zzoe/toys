use serde::{Deserialize, Serialize};
use speedy::{Readable, Writable};

#[derive(Debug, Readable, Writable, Serialize, Deserialize)]
pub struct SignReq {
    #[speedy(default_on_eof)]
    pub name: String,
    pub email: String,
    pub password: String,
}
