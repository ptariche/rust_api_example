# Rust API Example
Playing with Rust, Nickel, Postgres, Basic API Example

### About
 - I'm teaching myself rust; this is a very basic example with Nickel with Diesel utilizing Postgres

### Pre-requisites
 - Install Rust
 - Rustup to nightly build required
 - Postgres
  - CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

### Build
- cargo build
- cargon run

### Routes
```

GET     - http://127.0.0.1:4001/
GET     - http://127.0.0.1:4001/person/lookup/:uuid
DELETE  - http://127.0.0.1:4001/person/destroy/:uuid
POST    - http://127.0.0.1:4001/person/create

```

### Environment Variables
  - PORT
  - HOST
  - DATABASE_URI

## To run
 - PORT=4001 HOST=127.0.0.1 DATABASE_URI=postgresql://127.0.0.1:5432/postgres cargo run
