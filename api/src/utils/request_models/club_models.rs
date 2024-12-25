use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ClubModel {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct TransferOwnerModel {
    pub new_owner_id: i32,
}
