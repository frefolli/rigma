create table symbols (
    id SERIAL PRIMARY KEY,
    asset INTEGER REFERENCES assets(id) NOT NULL,
    name VARCHAR NOT NULL,
    terminality BOOLEAN NOT NULL
)
