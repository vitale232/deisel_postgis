CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE locations (
    id SERIAL PRIMARY KEY NOT NULL,
    loc geography(Point, 4326) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT 't',
    updated_at TIMESTAMP NOT NULL
);

SELECT
    diesel_manage_updated_at('locations');