use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct SignUpResponseDto {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
