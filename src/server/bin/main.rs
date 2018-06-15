#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
use rocket::http::RawStr;
extern crate rocket_contrib;

use rocket_contrib::Json;


#[macro_use]

extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate base64;
use std::u8;
use self::base64::{decode};


use std::io::prelude::*;
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
pub struct DigitImage {
    image_base64: String,
}

/*
image_b64 = request.values['imageBase64']
		image_encoded = image_b64.split(',')[1]
		image = base64.decodebytes(image_encoded.encode('utf-8'))		
prediction = model.predict(image) 
*/

#[post("/image", format = "application/json", data = "<image>")]
fn image(image: Json<DigitImage>) -> &'static str {
    let split = image.image_base64.split(",");
    let vec = split.collect::<Vec<&str>>();
    if vec.len() > 0 {
        match decode(vec[1]) {
            Ok(img_data) => {
                match File::create("test.jpg") {
                    Ok(mut buffer) => { buffer.write(&img_data); () },
                    Err(err) => (),
                }},
            Err(err) => (),
        }
        // Writes some prefix of the byte string, not necessarily all of it.
    }
    "hello"
}

// extern crate models;
// use models::{DigitImage};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: &RawStr) -> String {
    format!("Hello, {}!", name.as_str())
}

#[get("/hyo/<name>/<age>/<cool>")]
fn halo(name: String, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index, hello, halo, image]).launch();
}
