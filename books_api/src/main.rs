mod entity;
mod http_server;
mod repository;
mod service;
mod dto;

use std::fs;
use std::path::PathBuf;

use crate::repository::Repository;
use database::get_connection;
use http_server::start_http_server;
use kafka::util::register_schema;
use opentelemetry::global;
use service::{book_created_producer::BookCreatedProducer, Service};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    opentelemetry::global::set_text_map_propagator(opentelemetry_zipkin::Propagator::new());
    let tracer = opentelemetry_zipkin::new_pipeline()
        .with_service_name("books_api".to_owned())
        .with_service_address("127.0.0.1:8080".parse().unwrap())
        .with_collector_endpoint("http://localhost:9411/api/v2/spans")
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("unable to install zipkin tracer");
    let tracer = tracing_opentelemetry::layer().with_tracer(tracer.clone());

    let subscriber = tracing_subscriber::fmt::layer().json();

    let level = EnvFilter::new("debug".to_owned());

    tracing_subscriber::registry()
        .with(subscriber)
        .with(level)
        .with(tracer)
        .init();
    let database_connection =
        get_connection("postgres://postgres:postgres@localhost:5433/toy-app-db").await?;
    let repository = Repository::new(database_connection.clone())
        .await
        .expect("Error creating repository");
    let schema_registry_url = "http://localhost:8081".to_owned();
    let book_created_producer =
        BookCreatedProducer::new("localhost:9092".to_owned(), schema_registry_url.clone());
    let service = Service::new(repository, book_created_producer);

    let avro_source = "../avro/books/CreatedBook.avsc";
    let avro_str = fs::read_to_string(PathBuf::from(avro_source))?;

    let registered_avro = register_schema(
        schema_registry_url,
        "CreatedBook".to_string(),
        avro_str,
    )
    .await
    .expect("Error while registering schema");

    if !registered_avro.id.is_some() {
        panic!("avro schema register failed, not found registered id: CreatedBook!");
    } else {
        info!("CreatedBook registered success, register id: {}", registered_avro.id.unwrap());
    }

    start_http_server(service).await;
    global::shutdown_tracer_provider();
    Ok(())
}
