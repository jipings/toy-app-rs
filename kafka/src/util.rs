use apache_avro::Schema;
use opentelemetry::propagation::{Extractor, Injector};
use rdkafka::message::{BorrowedHeaders, Headers, OwnedHeaders};
use schema_registry_converter::{
    async_impl::schema_registry::{post_schema, SrSettings},
    avro_common::get_supplied_schema,
    error::SRCError,
    schema_registry_common::{RegisteredSchema, SchemaType, SuppliedSchema},
};

pub struct HeaderInjector<'a>(pub &'a mut OwnedHeaders);

impl<'a> Injector for HeaderInjector<'a> {
    fn set(&mut self, key: &str, value: String) {
        let mut new = OwnedHeaders::new().insert(rdkafka::message::Header {
            key,
            value: Some(&value),
        });

        for header in self.0.iter() {
            let s = String::from_utf8(header.value.unwrap().to_vec()).unwrap();
            new = new.insert(rdkafka::message::Header {
                key: header.key,
                value: Some(&s),
            });
        }

        self.0.clone_from(&new);
    }
}

pub struct HeaderExtractor<'a>(pub &'a BorrowedHeaders);

impl<'a> Extractor for HeaderExtractor<'a> {
    fn get(&self, key: &str) -> Option<&str> {
        for i in 0..self.0.count() {
            if let Ok(val) = self.0.get_as::<str>(i) {
                if val.key == key {
                    return val.value;
                }
            }
        }
        None
    }

    fn keys(&self) -> Vec<&str> {
        self.0.iter().map(|kv| kv.key).collect::<Vec<_>>()
    }
}

pub async fn register_schema(
    mut schema_registry_url: String,
    subject: String,
    name: String,
    avro_schema: String,
) -> Result<RegisteredSchema, SRCError> {
    println!("register_schema: {:?}", subject);
    let supplied_schema: SuppliedSchema = SuppliedSchema {
        name: Some(name),
        schema_type: SchemaType::Avro,
        schema: avro_schema,
        references: vec![],
    };

    schema_registry_url = schema_registry_url + "/subjects" + &subject + "/versions";
    let sr_settings = SrSettings::new(schema_registry_url);

    post_schema(&sr_settings, subject, supplied_schema).await
}
