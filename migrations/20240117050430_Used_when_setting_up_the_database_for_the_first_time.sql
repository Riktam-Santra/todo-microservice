-- Add migration script here
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE DATABASE IF NOT EXISTS todos;

CREATE TABLE IF NOT EXISTS todos ( id uuid NOT NULL DEFAULT gen_random_uuid(),
                                                            title VARCHAR NOT NULL,
                                                                          task_completed BOOLEAN NOT NULL,
                                                                                                 date_created DATE NOT NULL DEFAULT now());