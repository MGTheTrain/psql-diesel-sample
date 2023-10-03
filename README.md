# psql-diesel-sample

## Table of Contents

+ [Summary](#summary)
+ [References](#references)
+ [How to use](#how-to-use)

## Summary

Repository demonstrating how to apply CRUD operations on psql database tables with Rust and required third-party crates.

## References

- [Diesel - Getting Started](https://diesel.rs/guides/getting-started)

## How to use

**Precondition**: On Ubuntu 20.04 

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
diesel migration run


```

TBD