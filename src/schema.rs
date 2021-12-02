table! {
    cases (id) {
        id -> Uuid,
        active -> Bool,
        registration_date -> Timestamp,
        editor -> Uuid,
        address -> Nullable<Varchar>,
    }
}

table! {
    person_default_job (person_id, person_job_id) {
        person_id -> Uuid,
        person_job_id -> Uuid,
    }
}

table! {
    person_jobs (id) {
        id -> Uuid,
        person_id -> Nullable<Uuid>,
        title -> Varchar,
        income -> Nullable<Money>,
        location -> Nullable<Varchar>,
    }
}

table! {
    person_skills (id) {
        id -> Uuid,
        person_id -> Nullable<Uuid>,
        skill -> Varchar,
    }
}

table! {
    persons (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        father_name -> Varchar,
        bithday -> Date,
        national_number -> Bpchar,
        phone_number -> Bpchar,
        education_field -> Nullable<Varchar>,
        education_location -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        password_hash -> Varchar,
    }
}

joinable!(cases -> users (editor));
joinable!(person_default_job -> person_jobs (person_id));
joinable!(person_jobs -> persons (person_id));
joinable!(person_skills -> persons (person_id));

allow_tables_to_appear_in_same_query!(
    cases,
    person_default_job,
    person_jobs,
    person_skills,
    persons,
    users,
);
