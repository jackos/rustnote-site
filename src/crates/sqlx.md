# Sqlx
## Install for Postgres
cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres

## Cargo.toml example for Postgres
```toml
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
```

- `runtime-actix-rustls` use `actix` as runtime and `rustls` as `TLS` backend
- `macros` gives access to `sqlx::query` and `sqlx::query_as!`
- `postgres` makes all the postgres types available
- `uuid` Allows generating uuid's from the `uuid` crate
- `chrono` allows `timestamptz` to be mapped to `DateTime<T>`
- `migrate` manage migrations from rust code

## Environment variables
- `DATABASE_URL` `postgres://postgres:password@localhost:5432/newsletter`

## Command examples
- `sqlx database create` create the database in the connection string
- `sqlx migrate add create_subscriptions_table`:
sqlx migrate add will create a sql file placeholder in `migrations` folder, add your script there:
```sql
-- Add migration script here
CREATE TABLE subscriptions(
	id uuid NOT NULL,
	PRIMARY KEY (id),
	email TEXT NOT NULL UNIQUE,
	name TEXT NOT NULL,
	subscribed_at timestamptz NOT NULL
);
```
- `sqlx migrate run` run all the migrations in `migrations` folder
