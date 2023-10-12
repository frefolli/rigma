create table branches (
    id SERIAL PRIMARY KEY,
    production INTEGER REFERENCES productions(id) NOT NULL,
    symbol INTEGER REFERENCES symbols(id) NOT NULL,
    index INTEGER NOT NULL CHECK (index >= 0),
    UNIQUE (production, index)
);
