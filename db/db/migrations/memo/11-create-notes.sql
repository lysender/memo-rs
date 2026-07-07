CREATE TABLE notes (
    id TEXT PRIMARY KEY,
    file_id TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (file_id) REFERENCES files(id)
) STRICT;

CREATE INDEX idx_notes_file_id_id ON notes(file_id, id DESC);
