# Manually steup DB

## Prepare

- Cd into the foler contains `docker-compose.yml` file.
- `docker-compose up`

## Use PSQL connect to it

- `sudo apt install postgresql-client-common postgresql-client`
- `PGPASSWORD=postgres psql -h localhost -U postgres`
  - Use this command to connect to local db.

## Use `.sql` file to init tables in your database

- `PGPASSWORD=postgres psql -h localhost -U postgres -d rustweb -f db/init.sql`

# Use SQLx crate to do migration

## Prepare

- `cargo install sqlx-cli`
- `sqlx migrate add -r questions_table`

  - This will create a migration folder within the place you run this command.
  - Fill the generated two `.sql` files with commands you need. One is `***.up.sql`, another is the `***.down.sql` for reverse operation.

- Execute migration like:
  `sqlx migrate run --database-url postgres://postgres:postgres@localhost:5432/rustweb --source db/migrations`
- Execute migrate revert like:
  `sqlx migrate revert --database-url postgres://postgres:postgres@localhost:5432/rustweb --source db/migrations`
  - Each revert will trigger the last migration and try to run the `***.down.sql`.
