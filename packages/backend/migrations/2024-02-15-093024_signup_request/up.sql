CREATE TABLE signup_request (
    id uuid primary key not null,
    bath_username text unique not null,
    created_at Timestamp not null default now(),
    expires Timestamp not null,
    secret_hash text not null
);
