CREATE TABLE persons (
  id SERIAL PRIMARY KEY,
  uuid uuid  NOT NULL DEFAULT uuid_generate_v4() UNIQUE,
  email VARCHAR NOT NULL UNIQUE,
  first_name VARCHAR NOT NULL,
  last_name VARCHAR NOT NULL,
  created_at timestamptz DEFAULT now() NOT NULL,
  updated_at timestamptz DEFAULT now() NOT NULL
)