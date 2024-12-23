use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UpdateUserModel {
    pub name_first: Option<String>,
    pub name_last: Option<String>,
    pub email: Option<String>,
}
