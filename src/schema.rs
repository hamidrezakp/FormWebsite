table! {
    case_actions (id) {
        id -> Uuid,
        case_id -> Uuid,
        action -> Text,
        status -> Int4,
        action_date -> Nullable<Timestamp>,
    }
}

table! {
    cases (id) {
        id -> Uuid,
        number -> Int4,
        active -> Bool,
        registration_date -> Timestamp,
        editor -> Uuid,
        address -> Nullable<Varchar>,
        description -> Nullable<Text>,
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
        birthday -> Date,
        national_number -> Bpchar,
        phone_number -> Bpchar,
        case_id -> Uuid,
        is_leader -> Bool,
        family_role -> Int4,
        description -> Nullable<Text>,
        education_field -> Nullable<Varchar>,
        education_location -> Nullable<Varchar>,
    }
}

table! {
    user_tokens (id) {
        id -> Uuid,
        user_id -> Uuid,
        subject -> Text,
        token -> Text,
        created_at -> Timestamp,
        expires_at -> Timestamp,
        payload -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        password_hash -> Bytea,
        password_salt -> Bytea,
        role -> Int4,
    }
}

joinable!(case_actions -> cases (case_id));
joinable!(cases -> users (editor));
joinable!(person_default_job -> person_jobs (person_id));
joinable!(person_jobs -> persons (person_id));
joinable!(person_requirements -> persons (person_id));
joinable!(person_skills -> persons (person_id));
joinable!(persons -> cases (case_id));
joinable!(user_tokens -> users (user_id));

allow_tables_to_appear_in_same_query!(
    case_actions,
    cases,
    person_default_job,
    person_jobs,
    person_requirements,
    person_skills,
    persons,
    user_tokens,
    users,
);
