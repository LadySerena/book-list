-- auto-generated definition
create table book
(
    id      serial
        primary key,
    title   text not null,
    authors text[]
);

alter table book
    owner to postgres;

