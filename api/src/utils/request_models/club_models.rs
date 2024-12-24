use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateClubModel {
    pub name: String,
}
