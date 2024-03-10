# usa-kabu-jobs

## What is this?

- This is a rust project to get stock data from financialmodelingprep.com and save it to database.

## How to use

- You need to get an API key from financialmodelingprep.com and set it to the environment variable `FMP_API_KEY`.
- You need to set the environment variable `DATABASE_URL` to connect to the database.

## API Documents

https://financialmodelingprep.com/developer/docs/

## API endpoints

https://financialmodelingprep.com/api/v3/stock/list?apikey={apikey}

## How to install diesel_cli

1. Install diesel_cli

```bash
cargo install diesel_cli --no-default-features --features postgres
```

2. Set the environment variable `DATABASE_URL` to connect to the database.

```bash
diesel setup
```

3. Create a migration file

```bash
diesel migration generate stocks
```

4. Run the migration

```bash
diesel migration run
```
