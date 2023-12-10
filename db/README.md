# Start

- Cd into the foler contains `docker-compose.yml` file.
- `docker-compose up`

# Use PSQL connect to it

- `sudo apt install postgresql-client-common postgresql-client`
- `PGPASSWORD=postgres psql -h localhost -U postgres -d rust_web`

# Use `.sql` file to init tables

- `PGPASSWORD=postgres psql -h localhost -U postgres -d rust_web -f <your_sql_script.sql>`
