use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UpdateUserModel {
    pub name_first: String,
    pub name_last: String,
}
