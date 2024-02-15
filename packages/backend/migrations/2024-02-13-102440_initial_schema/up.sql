CREATE TABLE groups (
    id uuid primary key not null,
    join_code char(8) unique
);

CREATE TABLE users (
    id uuid primary key not null,
    display_name text not null,
    bath_username text unique not null,
    password_hash text not null,
    created_at timestamp not null default now(),
    dietary_requirements text not null,
    accessibility_requirements text not null,
    group_id uuid,
    constraint fk_group
        foreign key(group_id)
            references groups(id)
);
