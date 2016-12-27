# Rust API Example
Playing with Rust, Nickel, Postgres, Basic API Example

### About
 - I'm teaching myself rust; this is a very basic example with Nickel with Diesel utilizing Postgres

### Pre-requisites
 - [Install Rust](https://www.rust-lang.org/en-US/install.html)
 - Rustup to nightly build required
  - curl -s https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly
  - rustup update
  - rustup install nightly
    - rustup run nightly rustc --version
    - rustup default nightly
  - cargo update
 - Postgres
  - CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
 - cargo install diesel_cli
  - diesel setup --database-url='postgres://localhost/postgres'
  - DATABASE_URL='postgres://localhost/postgres' diesel migration redo

### Routes
```
METHOD  - ROUTE

GET     - http://127.0.0.1:4001/
GET     - http://127.0.0.1:4001/person/:uuid/lookup
PUT     - http://127.0.0.1:4001/person/:uuid/update
DELETE  - http://127.0.0.1:4001/person/:uuid/destroy
POST    - http://127.0.0.1:4001/person/create

```

### Environment Variables
  - PORT
  - HOST
  - DATABASE_URI


## Build
  - PORT=4001 HOST=127.0.0.1 DATABASE_URI=postgresql://127.0.0.1:5432/postgres cargo build

## Run
 - PORT=4001 HOST=127.0.0.1 DATABASE_URI=postgresql://127.0.0.1:5432/postgres cargo run
