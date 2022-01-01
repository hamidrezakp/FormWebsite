CREATE TYPE Role AS ENUM ('Admin', 'Editor', 'User');
CREATE TYPE FamilyRole AS ENUM ('Father', 'Mother', 'Children', 'NA');
CREATE TYPE ActionStatus AS ENUM ('Todo', 'Doing', 'Done');

CREATE TABLE users (
	id UUID PRIMARY KEY,
	username VARCHAR(30) NOT NULL,
	first_name VARCHAR(30) NOT NULL,
	last_name VARCHAR(30) NOT NULL,
	password_hash VARCHAR(64) NOT NULL
);

CREATE TABLE cases (
	id UUID PRIMARY KEY,
	number SERIAL,
	active BOOLEAN NOT NULL,
	registration_date TIMESTAMP NOT NULL,
	editor UUID NOT NULL REFERENCES users(id),
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
	case_id UUID NOT NULL REFERENCES cases(id),
	is_leader BOOLEAN DEFAULT FALSE NOT NULL,
	family_role FamilyRole Default 'NA' NOT NULL,
	description TEXT NULL,

	education_field VARCHAR(100) NULL,
	education_location VARCHAR(100) NULL
);

CREATE TABLE person_jobs (
	id UUID PRIMARY KEY,
	person_id UUID NOT NULL REFERENCES persons(id),
	title VARCHAR(100) NOT NULL,
	income INTEGER NULL,
	location VARCHAR(100) NULL,

	UNIQUE (id, person_id)
);

CREATE TABLE person_skills (
	id UUID PRIMARY KEY,
	person_id UUID NOT NULL REFERENCES persons(id),
	skill VARCHAR(200) NOT NULL
);

CREATE TABLE person_requirements (
	id UUID PRIMARY KEY,
	person_id UUID NOT NULL REFERENCES persons(id),
	description TEXT NOT NULL
);

CREATE TABLE person_default_job (
	person_id UUID NOT NULL,
	person_job_id UUID NOT NULL,

	PRIMARY KEY (person_id, person_job_id),
	FOREIGN KEY (person_id, person_job_id)
		REFERENCES person_jobs(person_id, id)
);

CREATE TABLE case_actions (
	id UUID PRIMARY KEY,
	case_id UUID NOT NULL REFERENCES cases(id),
	action TEXT NOT NULL,
	action_date TIMESTAMP NULL
);
