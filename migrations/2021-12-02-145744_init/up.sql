CREATE TYPE Role AS ENUM ('Admin', 'Editor', 'User');

CREATE TABLE users (
	id UUID PRIMARY KEY,
	username VARCHAR(30) NOT NULL,
	first_name VARCHAR(30) NOT NULL,
	last_name VARCHAR(30) NOT NULL,
	password_hash VARCHAR(64) NOT NULL
);

CREATE TABLE cases (
	id UUID PRIMARY KEY,
	active BOOLEAN NOT NULL,
	registration_date TIMESTAMP NOT NULL,
	editor UUID NOT NULL REFERENCES users(id),
	address VARCHAR(300)
);

CREATE TABLE persons (
	id UUID PRIMARY KEY,
	first_name VARCHAR(30) NOT NULL,
	last_name VARCHAR(30) NOT NULL,
	father_name VARCHAR(30) NOT NULL,
	bithday DATE NOT NULL,
	national_number CHAR(10) NOT NULL,
	phone_number CHAR(13) NOT NULL,

	education_field VARCHAR(100) NULL,
	education_location VARCHAR(100) NULL
);

CREATE TABLE person_jobs (
	id UUID PRIMARY KEY,
	person_id UUID REFERENCES persons(id),
	title VARCHAR(100) NOT NULL,
	income MONEY NULL,
	location VARCHAR(100) NULL,

	UNIQUE (id, person_id)
);

CREATE TABLE person_skills (
	id UUID PRIMARY KEY,
	person_id UUID REFERENCES persons(id),
	skill VARCHAR(200) NOT NULL
);

CREATE TABLE person_default_job (
	person_id UUID NOT NULL,
	person_job_id UUID NOT NULL,

	PRIMARY KEY (person_id, person_job_id),
	FOREIGN KEY (person_id, person_job_id)
		REFERENCES person_jobs(person_id, id)
);
