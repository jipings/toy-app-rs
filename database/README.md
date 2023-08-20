
## [Migration](https://www.sea-ql.org/SeaORM/docs/migration/setting-up-migration/)

> $ cargo install sea-orm-cli

> $ cd books_api && sea-orm-cli migrate init

> $ DATABASE_URL=postgres://postgres:postgres@localhost:5433/toy-app-db cargo run

> $ sea-orm-cli generate entity -u postgres://postgres:postgres@localhost:5433/toy-app-db --with-serde both -o ./src/entity
