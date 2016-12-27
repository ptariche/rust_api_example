CREATE TABLE persons (
  id SERIAL PRIMARY KEY,
  email VARCHAR NOT NULL UNIQUE,
  first_name VARCHAR NOT NULL,
  last_name VARCHAR NOT NULL,
  uuid uuid  NOT NULL DEFAULT uuid_generate_v4() UNIQUE,
  created_at timestamptz DEFAULT now() NOT NULL,
  updated_at timestamptz DEFAULT now() NOT NULL
)