BEGIN TRANSACTION;

UPDATE files
SET file_type = CASE
    WHEN is_image = 1 THEN 'image'
    ELSE 'file'
END
WHERE file_type IS NULL;

COMMIT;
