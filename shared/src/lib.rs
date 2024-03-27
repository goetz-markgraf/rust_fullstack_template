use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MessageDto {
    pub text: String,
}
