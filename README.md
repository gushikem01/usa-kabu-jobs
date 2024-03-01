# usa-kabu-jobs

this is a project to get the stock information of the US stock market.
using the financialmodelingprep API.
the database is postgresql.
the language is rust.
the architecture is clean architecture.
but it is not a strict clean architecture.

## Description

```
project_name
│
├───src
│    ├───application # application
│    │       ├───dtos # DTO（Data Transfer Object）
│    │       ├───services # service (application layer)
│    │       └───usecases # usecase (application layer)
│    │
│    ├───domain # domain layer
│    │       ├───entities # entity (domain layer)
│    │       ├───repositories # repository (domain layer)
│    │       └───value_objects # value object (domain layer)
│    │
│    └───infrastructure # infrastructure layer
│            ├───config # configuration
│            ├───database # database
│            ├───external # external
│            └───logging
│
└────tests
```

## API Documents

https://financialmodelingprep.com/developer/docs/

## API endpoints

https://financialmodelingprep.com/api/v3/stock/list?apikey={apikey}

## command

```bash
cargo install diesel_cli --no-default-features --features postgres
```

```bash
diesel setup
```

```bash
diesel migration generate stocks
```

```bash
diesel migration run
```
