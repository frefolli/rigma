create table symbols (
    id SERIAL PRIMARY KEY,
    asset INTEGER REFERENCES assets(id),
    name VARCHAR NOT NULL,
    terminality BOOLEAN NOT NULL
)
