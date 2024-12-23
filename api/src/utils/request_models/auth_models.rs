use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RegisterModel {
    pub user_type: String,
    pub name_first: String,
    pub name_last: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginModel {
    pub user_type: String,
    pub email: String,
    pub password: String,
}
