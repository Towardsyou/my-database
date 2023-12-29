-- Add migration script here

create table `user` (
    id integer primary key AUTOINCREMENT,
    name varchar(255) not null,
    email varchar(255) not null,
    password char(255) not null,
    role varchar(255) not null,
    status varchar(255) not null,
    created_at timestamp not null
);

create table `issue` (
    id integer primary key AUTOINCREMENT,
    title varchar(255) not null,
    description varchar(255) not null,
    creator int not null,
    reviewer varchar(255) not null,
    status varchar(255) not null,
    created_at timestamp not null
);

create table `history` (
    object_id integer not null,
    event_time timestamp not null,
    operation varchar(255) not null,
    operator int not null
);