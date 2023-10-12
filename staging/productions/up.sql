create table productions (
    id SERIAL PRIMARY KEY,
    asset INTEGER REFERENCES assets(id),
    left INTEGER NOT NULL,
    right [INTEGER] NOT NULL
)
