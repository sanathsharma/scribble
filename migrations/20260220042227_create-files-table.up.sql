-- Add up migration script here
CREATE TABLE files (
	id INTEGER PRIMARY KEY AUTOINCREMENT,
	directory TEXT NOT NULL DEFAULT '',
	filename TEXT NOT NULL,
	storage_dir TEXT CHECK (
		storage_dir IS NOT NULL
		AND storage_dir != ''
	)
);

CREATE INDEX idx_files_directory ON files (directory);

CREATE INDEX idx_files_filename ON files (filename);

create index idx_files_storage_dir on files (storage_dir);

CREATE UNIQUE INDEX idx_files_storage_dir_filename_dir ON files (storage_dir, filename, directory);
