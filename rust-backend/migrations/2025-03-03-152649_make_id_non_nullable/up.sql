-- Your SQL goes here
CREATE TABLE new_ai_tools (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	company TEXT NOT NULL,
	description TEXT NOT NULL,
	image_url TEXT
);

INSERT INTO new_ai_tools SELECT * FROM ai_tools;
DROP TABLE ai_tools;
CREATE TABLE ai_tools (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	name TEXT NOT NULL,
	company TEXT NOT NULL,
	description TEXT NOT NULL,
	image_url TEXT
);
INSERT INTO ai_tools SELECT * FROM new_ai_tools;
DROP TABLE new_ai_tools;
