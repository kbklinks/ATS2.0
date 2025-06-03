CREATE TYPE application_stage AS ENUM (
    'applied',
    'screening',
    'interview',
    'offer',
    'hired',
    'rejected'
);

DROP TYPE IF EXISTS application_stage;
