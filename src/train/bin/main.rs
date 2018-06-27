extern crate nn;
use nn::{backpropagate, feed_forward, tof64};

extern crate rand;
use rand::{Rng, thread_rng};
use rand::distributions::Uniform;

use std::fs;

extern crate image;
use image::GenericImage;
use image::DynamicImage::ImageRgb8;
use image::DynamicImage;
use image::FilterType;
use image::imageops;

extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;

fn normalize_pixel(rgba: image::Rgba<u8>) -> f64 {
    let res = ((rgba.data[0] as f64) + (rgba.data[1] as f64) + (rgba.data[2] as f64) + (rgba.data[3] as f64) - (std::u8::MIN as f64)) / ((std::u8::MAX as f64) * 4.0);
    if res > 0.9 {
        0.0
    } else {
        1.0
    }
    // res
}


fn bounding_box(d: &Vec<u8>, w: usize, h: usize) -> ((u32,u32),(u32,u32)) {
    let mut x0 = 0;
    let mut y0 = 0;

    for h0 in 0..h {
        let mut should_break = false;
        for w0 in 0..w {
            if d[(h0 * w) + w0] < 255 {
                y0 = h0 as u32;
                should_break = true;
                break;
            }
        }
        if should_break {
            break;
        }
    }

    for w0 in 0..w {
        let mut should_break = false;
        for h0 in 0..h {
            if d[(h0 * w) + w0] < 255 {
                x0 = w0 as u32;
                should_break = true;
                break;
            }
        }
        if should_break {
            break;
        }
    }

    let mut x1 = w as u32;
    let mut y1 = h as u32;

    let mut h0 = h;
    while h0 > 0 {
        let mut should_break = false;
        let mut w0 = w;
        while w0 > 0 {
            if d[((h0 - 1) * w) + (w0 - 1)] < 255 {
                y1 = h0 as u32;
                should_break = true;
                break;
            }
            w0 -= 1;
        }
        if should_break {
            break;
        }
        h0 -= 1;
    }

    let mut w0 = w;
    while w0 > 0 {
        let mut should_break = false;
        let mut h0 = h;
        while h0 > 0 {
            if d[((h0 - 1) * w) + (w0 - 1)] < 255 {
                x1 = w0 as u32;
                should_break = true;
                break;
            }
            h0 -= 1;
        }
        if should_break {
            break;
        }
        w0 -= 1;
    }

    ((x0,y0),(x1 - x0,y1 - y0))
}

// fn normalize_pixel(rgba: image::Rgb<u8>) -> f64 {
//     let res = (((rgba.data[0] as f64) + (rgba.data[1] as f64) + (rgba.data[2] as f64)) - (std::u8::MIN as f64)) / ((std::u8::MAX as f64) * 3.0);
//     if res > 0.9 {
//         1.0
//     } else {
//         0.0
//     }
//     // res
// }

// fn normalize_pixel(val: u8) -> f64 {
//     if val == 255 {
//         1.0
//     } else {
//         0.0
//     }
// }



fn mk_target(target: usize) -> Vec<f64> {
    let mut x = std::iter::repeat(0.0).take(10).collect::<Vec<_>>();
    x[target] = 1.0;
    x
}

fn mk_trained_data_files(hidden_layers: &Vec<Vec<f64>>, output_layers: &Vec<Vec<f64>>) {
    let mut hfile = File::create("hidden_layers.json").unwrap();
    let h = serde_json::to_string(hidden_layers).unwrap();
    hfile.write_all(h.as_bytes()).unwrap();


    let mut ofile = File::create("output_layers.json").unwrap();
    let o = serde_json::to_string(output_layers).unwrap();
    ofile.write_all(o.as_bytes()).unwrap();    
}

fn avg(xs: &Vec<f64>) -> f64 {
    let res : f64 = xs.iter().sum();
    res / (xs.len() as f64)
}

fn main() {
    // let zero_digit =
    //     vec![1,1,1,1,1,
    //          1,0,0,0,1,
    //          1,0,0,0,1,
    //          1,0,0,0,1,
    //          1,1,1,1,1];

    // let one_digit =
    //     vec![0,0,1,0,0,
    //          0,0,1,0,0,
    //          0,0,1,0,0,
    //          0,0,1,0,0,
    //          0,0,1,0,0];

    // let two_digit =
    //     vec![1,1,1,1,1,
    //          0,0,0,0,1,
    //          1,1,1,1,1,
    //          1,0,0,0,0,
    //          1,1,1,1,1];

    // let three_digit =
    //     vec![1,1,1,1,1,
    //          0,0,0,0,1,
    //          1,1,1,1,1,
    //          0,0,0,0,1,
    //          1,1,1,1,1];

    // let four_digit =
    //     vec![1,0,0,0,1,
    //          1,0,0,0,1,
    //          1,1,1,1,1,
    //          0,0,0,0,1,
    //          0,0,0,0,1];

    // let five_digit =
    //     vec![1,1,1,1,1,
    //          1,0,0,0,0,
    //          1,1,1,1,1,
    //          0,0,0,0,1,
    //          1,1,1,1,1];

    // let six_digit =
    //     vec![1,1,1,1,1,
    //          1,0,0,0,0,
    //          1,1,1,1,1,
    //          1,0,0,0,1,
    //          1,1,1,1,1];


    // let seven_digit =
    //     vec![1,1,1,1,1,
    //          0,0,0,0,1,
    //          0,0,0,0,1,
    //          0,0,0,0,1,
    //          0,0,0,0,1];

    // let eight_digit =
    //     vec![1,1,1,1,1,
    //          1,0,0,0,1,
    //          1,1,1,1,1,
    //          1,0,0,0,1,
    //          1,1,1,1,1];


    // let nine_digit =
    //     vec![1,1,1,1,1,
    //          1,0,0,0,1,
    //          1,1,1,1,1,
    //          0,0,0,0,1,
    //          1,1,1,1,1];

    // let inputs =
    //     vec![zero_digit,one_digit,two_digit,three_digit,four_digit,five_digit,six_digit,seven_digit,eight_digit,nine_digit];
    
    // let targets = vec![
    //     vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    //     vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
    //     vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
    //     vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
    //     vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
    //     vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
    //     vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
    //     vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
    //     vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    //     vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1]];

    // let input_size = 25;  // each input is a vector of length 25
    // let num_hidden = 5;   // we'll have 5 neurons in the hidden layer
    // let output_size = 10; // we need 10 outputs for each input

    // let mut rng = thread_rng();
    // let uniform = Uniform::new(0.0, 1.0);

    // let mut hidden_layer: Vec<Vec<f64>> = Vec::with_capacity(num_hidden);
    // for _i in 0..num_hidden {
    //     let mut hh: Vec<f64> = Vec::with_capacity(input_size + 1);
    //     for _j in 0..input_size + 1 {
    //         hh.push(rng.sample(uniform));
    //     }
    //     hidden_layer.push(hh);
    // }

    // let mut output_layer: Vec<Vec<f64>> = Vec::with_capacity(output_size);
    // for _i in 0..output_size {
    //     let mut hh: Vec<f64> = Vec::with_capacity(num_hidden + 1);
    //     for _j in 0..num_hidden + 1 {
    //         hh.push(rng.sample(uniform));
    //     }
    //     output_layer.push(hh);
    // }

    // let inputs_f64 = tof64(inputs);
    // let targets_f64 = tof64(targets);

    // for _i in 0..10000 {
    //     for pair in inputs_f64.iter().zip(targets_f64.iter()) {
    //         backpropagate(&mut hidden_layer, &mut output_layer, &pair.0, &pair.1);
    //     }
    // }


    // let crooked_three_digit =
    //     vec![0.,1.,1.,1.,1.,
    //          0.,0.,0.,0.,1.,
    //          0.,1.,1.,1.,1.,
    //          0.,0.,0.,0.,1.,
    //          0.,1.,1.,1.,1.];

    // let crooked_eight_digit =
    //     vec![1.,1.,1.,1.,1.,
    //          1.,0.,0.,1.,1.,
    //          1.,1.,1.,1.,1.,
    //          1.,0.,0.,1.,1.,
    //          1.,1.,1.,1.,1.];

    // let r0 = feed_forward(&hidden_layer, &output_layer, &inputs_f64[0]);
    // println!("{:?}", r0.1);
    // let j = serde_json::to_string(&r0.1).unwrap();
    // println!("{:?}", j);
    // let p: Vec<f64> = serde_json::from_str(&j).unwrap();
    // println!("{:?}", p);
    // let r0 = feed_forward(&hidden_layer, &output_layer, &crooked_three_digit);
    // let j = serde_json::to_string(&r0.1).unwrap();
    // println!("{:?}", j);
    // let p: Vec<f64> = serde_json::from_str(&j).unwrap();
    // println!("{:?}", p);
    

    let input_size = 28 * 28; // each input is a vector of length 25
    let num_hidden = 2;       // we'll have 5 neurons in the hidden layer
    let output_size = 10;     // we need 10 outputs for each input

    let mut rng = thread_rng();
    let uniform = Uniform::new(0.0, 1.0);

    let mut hidden_layer: Vec<Vec<f64>> = Vec::with_capacity(num_hidden);
    for _i in 0..num_hidden {
        let mut hh: Vec<f64> = Vec::with_capacity(input_size + 1);
        for _j in 0..input_size + 1 {
            hh.push(rng.sample(uniform));
        }
        hidden_layer.push(hh);
    }

    let mut output_layer: Vec<Vec<f64>> = Vec::with_capacity(output_size);
    for _i in 0..output_size {
        let mut hh: Vec<f64> = Vec::with_capacity(num_hidden + 1);
        for _j in 0..num_hidden + 1 {
            hh.push(rng.sample(uniform));
        }
        output_layer.push(hh);
    }

    let mut input_layer: Vec<Vec<Vec<f64>>> = Vec::with_capacity(output_size);

    for i in 0 .. output_size {
        let i_string = i.to_string();
        let dir = format!("./training_data/{}", i_string);

        let paths = fs::read_dir(dir).unwrap();
        let mut inputs: Vec<Vec<f64>> = Vec::new();
        for (j, path) in paths.enumerate() {
            let path_str = path.unwrap().path().display().to_string();

            //  if let ImageRgb8(img) = image::open("res/day3.png").unwrap() {
            //let Result<ImageRgb8(oimg)> = image::open(path_str);
            match image::open(path_str) {
                Ok(mut img) => {
                    let pixels = img.pixels().collect::<Vec<(u32,u32,image::Rgba<u8>)>>();
                    let normalized_pixels = pixels.iter().map(|(_,_,p)| normalize_pixel(*p)).collect::<Vec<_>>();
                    // let pixels = img.grayscale().raw_pixels();
                    // let normalized_pixels = pixels.iter().map(|p| normalize_pixel(*p)).collect::<Vec<_>>();
                    // println!("{:?}", normalized_pixels);
                    // println!("avg: {}", avg(&normalized_pixels));
                    let ((x0,y0),(x1,y1)) = bounding_box(&img.grayscale().raw_pixels(), 28, 28);
                    println!("{},{},{},{}", x0, y0, x1, y1);
                    
                    let img11 = img.crop(x0, y0, x1, y1); //.to_image();
                    let img2 = imageops::resize(&img11, 28,28, FilterType::Nearest);
                    // let img3 = img2.resize_exact(28,28, FilterType::Nearest);
                    let target_path = format!("./training_data_resized/{}/{}.png", i_string, j.to_string());

                    img2.save(target_path).unwrap();

                    inputs.push(normalized_pixels);
                },
                Err(err) => {
                    println!("error: {}", err.to_string());
                },
            }
        }
        input_layer.push(inputs.clone());
        // println!("size of {}: {}", i, inputs.len());
    }

    

    // for _j in 0..600 {
    //     for i in 0 .. output_size {
    //         let target = mk_target(i);
    //         for input in &input_layer[i] {
    //             backpropagate(&mut hidden_layer, &mut output_layer, input, &target);
    //         }
    //     }
    // }

    let fp = format!("./training_data/0/0.png");
    let img = image::open(fp).unwrap();
    let pixels = img.pixels().collect::<Vec<(u32,u32,image::Rgba<u8>)>>();
    let gray_pixels = img.grayscale().raw_pixels();
    // println!("{:?}", pixels);
    // println!("{:?}", gray_pixels);
    // for i in 0 .. output_size {
    //     let i_string = i.to_string();
    //     let fp = format!("./training_data/{}/0.png", i_string);
    //     let img = image::open(fp).unwrap();
    //     // let normalized_pixels = img.grayscale().raw_pixels().iter().map(|x| *x as f64).collect::<Vec<_>>();

    //     // let pixels = img.grayscale().raw_pixels();
    //     // let normalized_pixels = pixels.iter().map(|p| normalize_pixel(*p)).collect::<Vec<_>>();
    //     let pixels = img.pixels().collect::<Vec<(u32,u32,image::Rgba<u8>)>>();
    //     // let ImageRgb8(img) = image::open(fp).unwrap();    
    //     // let pixels = img.raw_pixels().collect::<Vec<u8>>();
    //     let normalized_pixels = pixels.iter().map(|(_,_,p)| normalize_pixel(*p)).collect::<Vec<_>>();
        
    //     let r1 = feed_forward(&hidden_layer, &output_layer, &normalized_pixels);

    //     println!("Guess for {}: {:?}", i_string, r1.1);
    // }

    mk_trained_data_files(&hidden_layer, &output_layer);    
}



