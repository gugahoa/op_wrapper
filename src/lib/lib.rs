extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[derive(Debug, Deserialize)]
pub struct Item {
    pub details: Detail
}

#[derive(Debug, Deserialize)]
pub struct Detail {
    pub fields: Vec<Field>
}

#[derive(Debug, Deserialize)]
pub struct Field {
    pub designation: String,
    pub value: String
}
