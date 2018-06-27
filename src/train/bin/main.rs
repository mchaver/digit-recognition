extern crate nn;
use nn::{backpropagate, feed_forward, tof64};

extern crate rand;
use rand::{Rng, thread_rng};
use rand::distributions::Uniform;

use std::fs;

extern crate image;
use image::GenericImage;


extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;

fn normalize_pixel(rgba: image::Rgba<u8>) -> f64 {
    let res = ((rgba.data[0] as f64) + (rgba.data[1] as f64) + (rgba.data[2] as f64) + (rgba.data[3] as f64) - (std::u8::MIN as f64)) / ((std::u8::MAX as f64) * 4.0);
        if res > 0.6 {
            1.0
        } else {
            0.0
        }
}

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
    //         backpropagate(&mut hidden_layer, &mut output_layer, pair.0.clone(), pair.1.clone());
    //     }
    // }

    // let r0 = feed_forward(&hidden_layer, &output_layer, &inputs_f64[0]);
    // // println!("{:?}", r0.1);

    // let j = serde_json::to_string(&r0.1).unwrap();
    // println!("{:?}", j);
    // let p: Vec<f64> = serde_json::from_str(&j).unwrap();
    // println!("{:?}", p);

    

    let input_size = 28 * 28; // each input is a vector of length 25
    let num_hidden = 8;       // we'll have 5 neurons in the hidden layer
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
        for path in paths {
            let path_str = path.unwrap().path().display().to_string();
            match image::open(path_str) {
                Ok(img) => {
                    let pixels = img.pixels().collect::<Vec<(u32,u32,image::Rgba<u8>)>>();
                    let normalized_pixels = pixels.iter().map(|(_,_,p)| normalize_pixel(*p)).collect::<Vec<_>>();
                    inputs.push(normalized_pixels);
                },
                Err(err) => {
                    println!("error: {}", err.to_string());
                },
            }
        }
        input_layer.push(inputs);
    }

    for _j in 0..500 {
        for i in 0 .. output_size {
            let target = mk_target(i);
            for input in &input_layer[i] {
                backpropagate(&mut hidden_layer, &mut output_layer, input, &target);
            }
        }
    }

    // store hidden_layer
    // store outp
    let img = image::open("./training_data/2/0.png").unwrap();    
    let pixels = img.pixels().collect::<Vec<(u32,u32,image::Rgba<u8>)>>();
    let normalized_pixels = pixels.iter().map(|(_,_,p)| normalize_pixel(*p)).collect::<Vec<_>>();
    let r1 = feed_forward(&hidden_layer, &output_layer, &normalized_pixels);
    println!("{:?}", r1.1);

    mk_trained_data_files(&hidden_layer, &output_layer);    
}
