use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UpdateUserModel {
    pub name_first: Option<String>,
    pub name_last: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUserClubModel {
    pub club_id: u32,
}

#[derive(Serialize, Deserialize)]
pub struct UpdatePasswordModel {
    pub old_password: String,
    pub new_password: String,
}
