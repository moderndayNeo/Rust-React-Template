-- Your SQL goes here
CREATE TABLE new_ai_tools (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	company TEXT NOT NULL,
	name TEXT NOT NULL,
	description TEXT NOT NULL,
	image_url TEXT
);

INSERT INTO new_ai_tools SELECT * FROM ai_tools;
DROP TABLE ai_tools;
ALTER TABLE new_ai_tools RENAME TO ai_tools;
