create table assets (
    id SERIAL PRIMARY KEY,
    document INTEGER REFERENCES documents(id) NOT NULL
)
