use std::cmp::min;
use std::sync::Arc;
use std::time::Duration;

use memo::utils::{IdPrefix, generate_prefixed_id};
use snafu::ResultExt;
use tokio::time::sleep;
use turso::Row;

use crate::db_pool::DbPool;
use crate::error::{DbPrepareSnafu, DbStatementSnafu};
use crate::turso_decode::{FromTursoRow, collect_row, row_integer, row_text};
use crate::turso_params::{integer_param, new_query_params, text_param};
use crate::{Error, Result};
use memo::note::NoteDto;

impl FromTursoRow for NoteDto {
    fn from_row(row: &Row) -> Result<Self> {
        Ok(Self {
            id: row_text(row, 0)?,
            file_id: row_text(row, 1)?,
            content: row_text(row, 2)?,
            created_at: row_integer(row, 3)?,
        })
    }
}

pub struct NoteRepo {
    db_pool: Arc<DbPool>,
}

impl NoteRepo {
    pub fn new(db_pool: Arc<DbPool>) -> Self {
        Self { db_pool }
    }

    pub async fn create_revision(&self, file_id: String, content: String) -> Result<NoteDto> {
        let note = NoteDto {
            id: generate_prefixed_id(IdPrefix::Note),
            file_id,
            content,
            created_at: chrono::Utc::now().timestamp(),
        };

        let query = r#"
            INSERT INTO notes
            (
                id,
                file_id,
                content,
                created_at,
            )
            VALUES
            (
                :id,
                :file_id,
                :content,
                :created_at,
            )
        "#;

        let mut q_params = new_query_params();
        q_params.push(text_param(":id", note.id.clone()));
        q_params.push(text_param(":file_id", note.file_id.clone()));
        q_params.push(text_param(":content", note.content.clone()));
        q_params.push(integer_param(":created_at", note.created_at));

        let conn = self.db_pool.acquire().await?;
        let mut stmt = conn.prepare(query).await.context(DbPrepareSnafu)?;
        stmt.execute(q_params).await.context(DbStatementSnafu)?;

        Ok(note)
    }

    pub async fn retry_create_revision(
        &self,
        file_id: String,
        content: String,
        max_retries: usize,
    ) -> Result<NoteDto> {
        let mut attempts = 0;
        let mut delay = Duration::from_millis(100);
        let max_delay = Duration::from_secs(2);

        loop {
            match self.create_revision(file_id.clone(), content.clone()).await {
                Ok(result) => return Ok(result),
                Err(Error::DbStatement { source }) => match source {
                    turso::Error::Busy(..) => {
                        attempts += 1;
                        if attempts >= max_retries {
                            return Err(Error::DbStatement { source });
                        }

                        sleep(delay).await;
                        delay = min(delay.saturating_mul(2), max_delay);
                        // Retries...
                    }
                    _ => {
                        return Err(Error::DbStatement { source });
                    }
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }

    /// Get the latest note revision for a given file_id
    pub async fn get_note(&self, file_id: &str) -> Result<Option<NoteDto>> {
        let query = r#"
            SELECT
                id,
                file_id,
                content,
                created_at,
            FROM notes
            WHERE file_id = :file_id
            ORDER BY id DESC
            LIMIT 1
        "#
        .to_string();

        let mut q_params = new_query_params();
        q_params.push(text_param(":file_id", file_id.to_owned()));

        let conn = self.db_pool.acquire().await?;
        let mut stmt = conn.prepare(query).await.context(DbPrepareSnafu)?;
        let row_result = stmt.query_row(q_params).await;
        let dto: Option<NoteDto> = collect_row(row_result)?;
        Ok(dto)
    }
}
