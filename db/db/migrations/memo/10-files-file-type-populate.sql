BEGIN TRANSACTION;

UPDATE files
SET file_type = CASE
    WHEN is_image = 1 THEN 'Image'
    ELSE 'File'
END
WHERE file_type IS NULL;

COMMIT;
