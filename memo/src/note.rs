use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteDto {
    pub id: String,
    pub file_id: String,
    pub content: String,
    pub created_at: i64,
}
