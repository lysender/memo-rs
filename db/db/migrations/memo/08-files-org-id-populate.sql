BEGIN TRANSACTION;

UPDATE files
SET org_id = (
    SELECT d.org_id
    FROM dirs d
    WHERE d.id = files.dir_id
)
WHERE org_id IS NULL;

COMMIT;
