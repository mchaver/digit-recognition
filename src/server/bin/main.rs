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

use std::io;
use std::io::prelude::*;
use std::fs::File;

use std::path::{Path, PathBuf};
use rocket::response::NamedFile;


extern crate image;
use image::GenericImage;
use image::FilterType;
#[derive(Serialize, Deserialize, Debug)]
pub struct DigitImage {
    image_base64: String,
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

/*
image_b64 = request.values['imageBase64']
		image_encoded = image_b64.split(',')[1]
		image = base64.decodebytes(image_encoded.encode('utf-8'))		
prediction = model.predict(image) 
*/

#[post("/image", format = "application/json", data = "<image>")]
fn image(image: Json<DigitImage>) -> &'static str {
    println!("image called");
    let split = image.image_base64.split(",");
    let vec = split.collect::<Vec<&str>>();
    if vec.len() > 0 {
        match decode(vec[1]) {
            Ok(img_data) => {


                //let buffer: &[u8] = ...; // Generate the image data

                // Save the buffer as "image.png"
                // image::save_buffer(&Path::new("image.png"), buffer, 800, 600, image::RGBA(8))
                // image::save_buffer(&Path::new("test2.jpg"), &img_data, 200, 200, image::RGB(8));
                
                match File::create("test1.png") {
                    Ok(mut buffer) => {
                        buffer.write(&img_data);
                        println!("open test1.png");
                        match image::open("test1.png") {
                            Ok(imgg) => {
                                println!("opened");
                                let ref mut ab = imgg.resize_exact(28,28, FilterType::Nearest);
                                // let ref mut fout = File::create("test2.png").unwrap();
                                // Write the contents of this image to the Writer in PNG format.
                                ab.save("test2.png").unwrap();
                                ab.save("test2.jpg").unwrap();
                                ()
                            },
                            Err(err) => println!("{}", err),
                        }
                    },
                    Err(err) => println!("{}", err),
                }},
            Err(err) => println!("{}", err),
        }
        // Writes some prefix of the byte string, not necessarily all of it.
    }
    "hello"
}

fn main() {
    rocket::ignite().mount("/", routes![index, image, files]).launch();
}
