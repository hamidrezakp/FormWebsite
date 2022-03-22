-- CREATE TYPE Role AS ENUM ('Admin', 'Editor', 'User'); 0, 1, 2
-- CREATE TYPE FamilyRole AS ENUM ('Father', 'Mother', 'Children', 'NA'); 0, 1, 2, 3
-- CREATE TYPE ActionStatus AS ENUM ('Todo', 'Doing', 'Done'); 0, 1, 2

CREATE TABLE users (
	id UUID PRIMARY KEY,
	username VARCHAR(30) NOT NULL,
	first_name VARCHAR(30) NOT NULL,
	last_name VARCHAR(30) NOT NULL,
	password_hash BYTEA NOT NULL,
	password_salt BYTEA NOT NULL,
	role INTEGER DEFAULT 3 NOT NULL,

	UNIQUE (username)
);

CREATE TABLE cases (
	id UUID PRIMARY KEY,
	number SERIAL,
	active BOOLEAN NOT NULL,
	registration_date TIMESTAMP NOT NULL,
	editor UUID NOT NULL REFERENCES users,
	address VARCHAR(300),
	description TEXT NULL
);

CREATE TABLE persons (
	id UUID PRIMARY KEY,
	first_name VARCHAR(30) NOT NULL,
	last_name VARCHAR(30) NOT NULL,
	father_name VARCHAR(30) NOT NULL,
	birthday DATE NOT NULL,
	national_number CHAR(10) NOT NULL,
	phone_number CHAR(13) NOT NULL,
	case_id UUID NOT NULL REFERENCES cases ON DELETE CASCADE,
	is_leader BOOLEAN DEFAULT FALSE NOT NULL,
	family_role INTEGER DEFAULT 3 NOT NULL,
	description TEXT NULL,

	education_field VARCHAR(100) NULL,
	education_location VARCHAR(100) NULL
);

CREATE TABLE person_jobs (
	id UUID PRIMARY KEY,
	person_id UUID NOT NULL REFERENCES persons ON DELETE CASCADE,
	title VARCHAR(100) NOT NULL,
	income INTEGER NULL,
	location VARCHAR(100) NULL,

	UNIQUE (id, person_id)
);

CREATE TABLE person_skills (
	id UUID PRIMARY KEY,
	person_id UUID NOT NULL REFERENCES persons ON DELETE CASCADE,
	skill VARCHAR(200) NOT NULL
);

CREATE TABLE person_requirements (
	id UUID PRIMARY KEY,
	person_id UUID NOT NULL REFERENCES persons ON DELETE CASCADE,
	description TEXT NOT NULL
);

CREATE TABLE person_default_job (
	person_id UUID NOT NULL,
	person_job_id UUID NOT NULL,

	PRIMARY KEY (person_id, person_job_id),
	FOREIGN KEY (person_id, person_job_id)
		REFERENCES person_jobs(person_id, id)
		ON DELETE CASCADE
);

CREATE TABLE case_actions (
	id UUID PRIMARY KEY,
	case_id UUID NOT NULL REFERENCES cases ON DELETE CASCADE,
	action TEXT NOT NULL,
	status INTEGER DEFAULT 0 NOT NULL,
	action_date TIMESTAMP NULL
);

CREATE TABLE user_tokens (
	id UUID PRIMARY KEY,
	user_id UUID NOT NULL REFERENCES users ON DELETE CASCADE,
	subject TEXT NOT NULL,
	token TEXT NOT NULL,
	created_at TIMESTAMP NOT NULL,
	expires_at TIMESTAMP NOT NULL,
	payload TEXT NULL
);
