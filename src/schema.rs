table! {
    cases (id) {
        id -> Uuid,
        number -> Int4,
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
        person_id -> Uuid,
        title -> Varchar,
        income -> Nullable<Int4>,
        location -> Nullable<Varchar>,
    }
}

table! {
    person_requirements (id) {
        id -> Uuid,
        person_id -> Uuid,
        description -> Text,
    }
}

table! {
    person_skills (id) {
        id -> Uuid,
        person_id -> Uuid,
        skill -> Varchar,
    }
}

table! {
    persons (id) {
        id -> Uuid,
        first_name -> Varchar,
        last_name -> Varchar,
        father_name -> Varchar,
        birthday -> Timestamp,
        national_number -> Bpchar,
        phone_number -> Bpchar,
        case_id -> Uuid,
        is_leader -> Bool,
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
joinable!(person_requirements -> persons (person_id));
joinable!(person_skills -> persons (person_id));
joinable!(persons -> cases (case_id));

allow_tables_to_appear_in_same_query!(
    cases,
    person_default_job,
    person_jobs,
    person_requirements,
    person_skills,
    persons,
    users,
);
