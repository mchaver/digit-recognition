#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate nn;

extern crate rocket;
extern crate rocket_contrib;
use rocket::response::content;
use rocket::response::status;
use rocket_contrib::Json;


#[macro_use]

extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate base64;
use std::u8;
use self::base64::{decode};

use std::io;
use std::io::prelude::*;
use std::fs::File;

use std::path::{Path, PathBuf};
use rocket::response::NamedFile;


extern crate image;
use image::FilterType;

use std::fs;

#[macro_use] extern crate lazy_static;
use regex::Regex;
extern crate regex;

#[derive(Serialize, Deserialize, Debug)]
pub struct DigitImage {
    image_base64: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DigitImageForTraining {
    image_base64: String,
    tag: String,
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[post("/predict", format = "application/json", data = "<image>")]
fn predict(image: Json<DigitImage>) -> content::Json<String> {
    content::Json("{ 'error': 'image payload was incorrect' }".to_string())   
}

#[post("/train", format = "application/json", data = "<image>")]
fn train(image: Json<DigitImageForTraining>) -> content::Json<String> {
    let img_string = image.image_base64.split(",").collect::<Vec<&str>>();
    if img_string.len() > 0 {
        match decode(img_string[1]) {
            Ok(img_data) => {
                match File::create("temp.png") {
                    Ok(mut buffer) => {
                        match buffer.write(&img_data) {
                            Ok(_write_res) => {
                                match image::open("temp.png") {
                                    Ok(img) => {
                                        let mut count: u32 = 0;
                                        lazy_static! {
                                            static ref COUNT: Regex = Regex::new(r"^\d{1}-(\d+)").unwrap();
                                        }
                                        let paths = fs::read_dir("./").unwrap();
                                        // path.unwrap().path().display()
                                        for path in paths {
                                            let path_name = path.unwrap().path().display().to_string();
                                            let cap = COUNT.captures(&path_name).unwrap();
                                            if cap.len() > 0 {
                                                let loc_count: u32 = cap[1].parse().unwrap();
                                                if loc_count > count {
                                                    count = loc_count;
                                                }
                                            }
                                        }
                                        let ref mut img_resized = img.resize_exact(28,28, FilterType::Nearest);
                                        let file_name = format!("{}-{}.png", image.tag, count);
                                        img_resized.save(file_name).unwrap();
                                        content::Json("{ 'result': 'image has been saved' }".to_string())
                                    },
                                    Err(err) => {
                                        let err_str = format!("'error':'{}'", err.to_string());
                                        content::Json(err_str)
                                    },
                                }
                            },
                            Err(err) => {
                                let err_str = format!("'error':'{}'", err.to_string());
                                content::Json(err_str)
                            },
                        }
                    },
                    Err(err) => {
                        let err_str = format!("'error':'{}'", err.to_string());
                        content::Json(err_str)
                    },
                }},
            Err(err) => {
                let err_str = format!("'error':'{}'", err.to_string());
                content::Json(err_str)
            },
        }
        // Writes some prefix of the byte string, not necessarily all of it.
    } else {
        content::Json("{ 'error': 'image payload was incorrect' }".to_string())
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index, files, predict, train]).launch();
}
