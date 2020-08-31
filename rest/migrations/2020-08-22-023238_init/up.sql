-- Your SQL goes here
-- Initialize database tables
CREATE TABLE items (
    id integer not null primary key autoincrement,
    name_en varchar(100) not null,
    name_jp varchar(100) not null,
    pri_type integer not null,
    sub_type integer not null default 0
);
CREATE TABLE owneditems (
    id integer not null primary key autoincrement,
    user_id integer not null,
    item_id integer not null,
    extra integer not null default 0
);
CREATE TABLE users (
    id integer not null primary key autoincrement,
    username varchar(50) not null,
    passwd varchar(100) not null,
    token varchar(50) not null default '0',
    seen varchar(20) not null default '0'
);
CREATE TABLE primarytypes (
    id integer not null primary key autoincrement,
    name varchar(50) not null
);
CREATE TABLE subtypes (
    id integer not null primary key autoincrement,
    name varchar(50) not null
);