
CREATE TABLE users (
    id BIGSERIAL PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TABLE notes (
    id BIGSERIAL PRIMARY KEY,
    content TEXT NOT NULL,
    geom GEOMETRY(POINT, 4326),
    user_id BIGINT NOT NULL REFERENCES USERS(ID),
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

SELECT create_hypertable('notes', by_range('created_at'));

SELECT *
FROM notes
WHERE created_at BETWEEN '2024-02-01 00:00:00' AND '2024-02-15 23:59:59'
  AND content = '';

SELECT *
FROM notes
WHERE created_at BETWEEN '2024-02-01 00:00:00'
  AND content = '';
