create table productions (
    id SERIAL PRIMARY KEY,
    asset INTEGER REFERENCES assets(id) NOT NULL,
    "left" INTEGER  REFERENCES symbols(id) NOT NULL
)
