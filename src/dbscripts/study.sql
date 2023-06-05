CREATE TABLE public.study
(
    study_id serial NOT NULL,
    protocol_id character varying(255) NOT NULL,
    protocol_description character varying(2000),
    PRIMARY KEY (study_id)
);
