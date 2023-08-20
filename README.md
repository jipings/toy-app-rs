
## cargo 

### Library crates
> $ cargo new --vcs=none --lib common

> $ cargo new --vcs=none --lib database

> $ cargo new --vcs=none --lib kafka

 ### Application 
> $ cargo new --vcs=none books_api

> $ cargo new --vcs=none books_analytics

## avro

https://betterprogramming.pub/working-with-apache-avro-in-rust-e18a20048c67

> $ cargo install rsgen-avro

> $ rsgen-avro --derive-builders resources/post.avsc src/avro.rs

start docker-compose

> $ docker-compose up

create topic 
> kafka-topics --create --bootstrap-server localhost:9092 --topic my-topic --replication-factor 1 --partitions 1 --if-not-exists

schema-registry
https://github.com/confluentinc/schema-registry

```sh
curl -X POST -H "Content-Type: application/vnd.schemaregistry.v1+json" \
    --data '{"schema": "{\"namespace\": \"io.confluent.examples.clients.basicavro\", \"type\": \"record\",\"name\": \"Payment\",\"fields\": [{\"name\": \"id\", \"type\": \"string\"},{\"name\": \"amount\", \"type\": \"double\"}]}"}' \
    http://localhost:8081/subjects/Payment/versions {"id":2}
```

## [Migration](https://www.sea-ql.org/SeaORM/docs/migration/setting-up-migration/)

> $ cargo install sea-orm-cli

> $ cd books_api && sea-orm-cli migrate init

> $ DATABASE_URL=postgres://postgres:postgres@localhost:5433/toy-app-db cargo run

> $ sea-orm-cli generate entity -u postgres://postgres:postgres@localhost:5433/toy-app-db --with-serde both -o ./src/entity

## TODO

* protocol

* gRPC

* sql Migration


