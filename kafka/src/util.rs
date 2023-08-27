use opentelemetry::propagation::{Extractor, Injector};
use rdkafka::message::{BorrowedHeaders, Headers, OwnedHeaders};
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterAvro {
    pub id: Option<u32>,
    #[serde(default)]
    pub schema: String,
    #[serde(default)]
    pub schema_type: String,
}

pub async fn register_schema(
    mut schema_registry_url: String,
    subject: String,
    avro_schema: String,
) -> Result<RegisterAvro, reqwest::Error> {
    schema_registry_url = schema_registry_url + "/subjects/" + &subject + "/versions";

    let register_avro = RegisterAvro {
        id: None,
        schema: avro_schema.clone(),
        schema_type: "AVRO".to_owned(),
    };

    let register_avro_res: RegisterAvro = reqwest::Client::new()
        .post(schema_registry_url)
        .json(&register_avro)
        .send()
        .await?
        .json()
        .await?;
    
    Ok(register_avro_res)
}
