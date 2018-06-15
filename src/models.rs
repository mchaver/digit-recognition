#[macro_use]

extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct DigitImage {
    imageBase64: String,
}
