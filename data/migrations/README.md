# SQL Migrations

This directory contains all migrations needed to go from an empty object
database to one filled with all the relevant data.

The migrations contain **both** the structure of the database, **and** the data
stored in the database.

You can use [`sqlx`] to generate new migrations:

1. First, install the `sqlx-cli` binary:

   ```shell
   cargo install sqlx-cli --no-default-features --features sqlite
   ```

2. Next, create a new migration:

   ```shell
   cargo sqlx --database-url sqlite://data/database.sqlite3 migrate --source data/migrations add add_my_table
   ```

3. Then, open the new migration file created in `data/migrations` and modify it
   to your liking.

4. Finally, run the migration:

   ```shell
   cargo sqlx --database-url sqlite://data/database.sqlite3 migrate --source data/migrations run
   ```
