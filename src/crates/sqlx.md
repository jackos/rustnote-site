# Sqlx
## Install for Postgres
cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres

## Cargo.toml example for Postgres
[dependencies.sqlx]
version = "0.5.7"
default-features = false
features = [
	"runtime-actix-rustls",
	"macros",
	"postgres",
	"uuid",
	"chrono",
	"migrate",
]

- `runtime-actix-rustls` use `actix` as runtime and `rustls` as `TLS` backend
- `macros` gives access to `sqlx::query` and `sqlx::query_as!`
- `postgres` makes all the postgres types available
- `uuid` Allows generating uuid's from the `uuid` crate
- `chrono` allows `timestamptz` to be mapped to `DateTime<T>`
- `migrate` manage migrations from rust code

