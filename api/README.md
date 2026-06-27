# memo-api: Make Memories

`memo-api` is a simple file storage service and is the backend service for `memo-website`.

It is designed for personal use and not indended for large number of concurrent users.
The goal of the service is to provide an economical way to store and retrieve
files in the cloud.

Uses cases:
- Store personal files and documents
- Online photo album

## Features 

- [x] JSON API endpoints to manage files
- [x] Multi-tenant
- [x] AWS S3
- [x] Turso database
- [x] JWT with Role-based authorization

## Workflow

- Organizations and users a managed by YAAS - https://github.com/lysender-lab/yaas
- Each organizations will have their directory in a unified bucket
- Users will share access to files/notes within the organization
- Files are categorized as photos, documents and videos
- Notes are backed by database but will be treated as files as well
- Files are organized by directories (like albums for photos)
- Each directory have the following sub-directories for image files:
    - orig - original image
    - preview - web optimized image preview
    - thumb - web optimized image thumbnail
- File metadata is collected
  - Content type
  - Size
  - Image dimension for each version
  - Date picture is taken

## Models

Dir:
- id
- org_id
- dir_type (Photos, Videos, Documents, Notes)
- name
- label
- created_at
- updated_at

File:
- id
- org_id
- dir_id
- file_type (File, Image, Video, Note)
- name
- filename
- content_type
- size
- is_image
- img_versions
- img_taken_at
- created_at
- updated_at

Note:
- id
- org_id
- dir_id
- file_id
- content
- created_at
- updated_at
- deleted_at

