BEGIN TRANSACTION;

ALTER TABLE files ADD COLUMN org_id TEXT;

CREATE INDEX idx_files_org_id ON files(org_id);

COMMIT;
