# psql-diesel-sample

## Table of Contents

+ [Summary](#summary)
+ [References](#references)
+ [How to use](#how-to-use)

## Summary

Repository demonstrating how to apply CRUD operations on psql database tables with Rust and required third-party crates. Duplicates: [Diesel PostgreSql full sample used in this setup](https://github.com/diesel-rs/diesel/tree/2.1.x/examples/postgres/getting_started_step_3/src)

## References

- [Diesel - Getting Started](https://diesel.rs/guides/getting-started)
- [Diesel PostgreSql full sample used in this setup](https://github.com/diesel-rs/diesel/tree/2.1.x/examples/postgres/getting_started_step_3/src)

## How to use

**Precondition**: On Ubuntu 20.04 `sudo apt-get install -y libpq-dev`

Afterwards following steps were executed considering migrations and binaries containing functions utilizing diesel ORM in order to apply CRUD operations on a PosgreSql migrated database table. Following steps have been executed as a **precondition and shall not be run afterwards** unless intended (e.g. updated migrations to be tested):

```bash
sudo apt-get install libpq-dev -y
cargo install diesel_cli --no-default-features --features postgres
echo DATABASE_URL=postgres://user:password@localhost/diesel-demo > .env
sudo docker compose up -d --build
cargo new lib --lib
cd lib
# update Cargo.toml 
diesel setup
# update in migrations 2023-10-03-174636_create_posts folder the `down.sql` and `up.sql` (See: https://diesel.rs/guides/getting-started)
diesel migration run # runs up.sql
# diesel migration redo # runs down.sql
# Update content based on folowing example: https://github.com/diesel-rs/diesel/tree/2.1.x/examples/postgres/getting_started_step_3/src
```

Following steps can be executed **repeatedly**:

```bash
sudo docker compose up -d --build
cd lib
source ../.env
cargo build
cargo run --bin write_post # write post
cargo run --bin publish_post <id, e.g. 1> 
cargo run --bin show_posts
cargo run --bin delete_post <pattern, e.g. test> 
```

In order to clear resources: 

```bash
sudo docker rm -f $(sudo docker ps -qa)
sudo docker system prune --volumes --force
sudo rm -rf lib/target
```