create table if not exists study (
    study_id serial primary key,
    protocol_id varchar(255),
    protocol_description varchar(2000)
);
