CREATE TABLE input_table (
   id SERIAL PRIMARY KEY,
   title VARCHAR NOT NULL,
   body TEXT NOT NULL,
   published BOOLEAN NOT NULL DEFAULT 'f'
);

CREATE TABLE output_table (
     id SERIAL PRIMARY KEY,
     title VARCHAR NOT NULL,
     body TEXT NOT NULL,
     published BOOLEAN NOT NULL DEFAULT 'f'
)