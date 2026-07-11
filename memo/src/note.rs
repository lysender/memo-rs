use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateNoteDto {
    #[validate(length(min = 1))]
    pub name: String,

    #[validate(length(min = 2, max = 10_000))]
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteDto {
    pub id: String,
    pub file_id: String,
    pub content: String,
    pub created_at: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_note_validation() {
        let data = CreateNoteDto {
            name: "Shopping list".to_string(),
            content: "Milk".to_string(),
        };
        assert!(data.validate().is_ok());

        let data = CreateNoteDto {
            name: "Shopping list".to_string(),
            content: "x".repeat(10_000),
        };
        assert!(data.validate().is_ok());

        let data = CreateNoteDto {
            name: "".to_string(),
            content: "Milk".to_string(),
        };
        assert!(data.validate().is_err());

        let data = CreateNoteDto {
            name: "Shopping list".to_string(),
            content: "x".to_string(),
        };
        assert!(data.validate().is_err());

        let data = CreateNoteDto {
            name: "Shopping list".to_string(),
            content: "x".repeat(10_001),
        };
        assert!(data.validate().is_err());
    }
}
