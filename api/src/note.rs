use snafu::{ResultExt, ensure};
use validator::Validate;

use crate::{
    Result,
    error::{DbSnafu, ValidationSnafu},
    state::AppState,
};
use db::file::MAX_FILES;
use memo::{
    dir::DirDto,
    file::{FileDto, FileType},
    note::CreateNoteDto,
    utils::{IdPrefix, generate_prefixed_id, truncate_string},
    validators::flatten_errors,
};

pub async fn create_note_svc(
    state: &AppState,
    dir: &DirDto,
    data: &CreateNoteDto,
) -> Result<FileDto> {
    let errors = data.validate();
    ensure!(
        errors.is_ok(),
        ValidationSnafu {
            msg: flatten_errors(&errors.unwrap_err()),
        }
    );

    let count = state
        .db
        .files
        .count_by_dir(&dir.id)
        .await
        .context(DbSnafu)?;
    ensure!(count < MAX_FILES as i64, ValidationSnafu {
        msg: "Directory already has maximum files".to_string(),
    });

    let existing = state
        .db
        .files
        .find_by_name(&dir.id, &data.name)
        .await
        .context(DbSnafu)?;
    ensure!(
        existing.is_none(),
        ValidationSnafu {
            msg: format!("{} already exists", truncate_string(&data.name, 20)),
        }
    );

    let now = chrono::Utc::now().timestamp();
    let file = FileDto {
        id: generate_prefixed_id(IdPrefix::File),
        org_id: dir.org_id.clone(),
        dir_id: dir.id.clone(),
        file_type: FileType::Note,
        name: data.name.clone(),
        filename: data.name.clone(),
        content_type: "text/plain".to_string(),
        size: data.content.len() as i64,
        url: None,
        img_versions: None,
        img_taken_at: None,
        created_at: now,
        updated_at: now,
    };

    let file = state
        .db
        .files
        .retry_create(file, 5)
        .await
        .context(DbSnafu)?;

    if let Err(error) = state
        .db
        .notes
        .retry_create_revision(file.id.clone(), data.content.clone(), 5)
        .await
        .context(DbSnafu)
    {
        state.db.files.delete(&file.id).await.context(DbSnafu)?;
        return Err(error);
    }

    Ok(file)
}
