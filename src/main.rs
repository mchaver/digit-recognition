extern crate rand;
use rand::{Rng, thread_rng};
use rand::distributions::Uniform;


use std::fs::File;
use std::io::prelude::*;

use std::fs;

#[derive(Debug, Clone)]
struct Neuron {
    weights: Vec<f64>
}

fn mk_neuron(n: usize) -> Neuron {
    let mut rng = thread_rng();
    let uniform = Uniform::new(0.0, 1.0);

    let mut weights: Vec<f64> = Vec::with_capacity(n + 1);
    for _i in 0..n+1 {
        weights.push(rng.sample(uniform));
    }
    Neuron { weights: weights }
}

fn sum(n: Neuron, i: Vec<f64>) -> f64 {
    n.weights.iter().zip(i).collect::<Vec<(&f64,f64)>>().iter().map(|(a,b)| *a * b).collect::<Vec<_>>().iter().sum()
}


#[derive(Debug, Clone)]
struct NeuronLayer {
    neurons: Vec<Neuron>
}

// fun mk_neuralnetwork(n_inputs: int, n_outputs: int, n_neurons_to_hl: int, n_hidden_layers: int) {

// }

const threshold: f64 = 0.5;
// const threshold: i64 = 1;
// const learningRate: f64 = 0.1;
const learningRate: f64 = 1.;

fn dot(xs: Vec<f64>, ys: Vec<f64>) -> f64 {
    // xs.iter().zip(ys).collect::<Vec<(&f64,f64)>>().iter().map(|(a,b)| *a * b).collect::<Vec<_>>().iter().sum()
    xs.iter().zip(ys.iter()).map(|(x, y)| x*y).sum()
}

fn step_function(x: f64) -> i64 {
    if x > 0.0 {
        1
    } else {
        0
    }
}

fn perceptron_output(x: Vec<f64>, weights: Vec<f64>, bias: f64) -> i64 {
    step_function(dot(weights, x) + bias)
}

// smooth variation of step_function
fn sigmoid(t: f64) -> f64 {
    1.0 / (1.0 + std::f64::consts::E.powf(-t))
}

fn neuron_output(weights: Vec<f64>, inputs: Vec<f64>) -> f64 {
    sigmoid(dot(weights, inputs))
}


// fn feed_forward(neural_network: Vec<Vec<Vec<f64>>>, input_vec: Vec<f64>) -> Vec<Vec<f64>> {
//     let mut outputs = vec![];
//     let mut input_vector = input_vec.clone();

//     for layer in neural_network {
//         input_vector.push(1.0);
//         let input_with_bias = input_vector.clone();
//         let mut output = vec![];
//         for neuron in layer {
//             output.push(neuron_output(neuron, input_with_bias.clone()));
//         }
//         outputs.push(output.clone());
//         input_vector = output.clone();
//     }
//     outputs
// }

fn feed_forward(neural_network: (Vec<Vec<f64>>,Vec<Vec<f64>>), input_vec: Vec<f64>) -> (Vec<f64>, Vec<f64>) {
    let (hidden_layers, output_layer) = neural_network;
    let mut hidden_outputs = vec![];
    let mut outputs = vec![];

    let mut input_vector = input_vec.clone();
    input_vector.push(1.0);
    let input_with_bias = input_vector.clone();
    for neuron in hidden_layers {
        hidden_outputs.push(neuron_output(neuron, input_with_bias.clone()));
    }

    input_vector = hidden_outputs.clone();
    input_vector.push(1.0);
    let input_with_bias = input_vector.clone();
    for neuron in output_layer {
        outputs.push(neuron_output(neuron, input_with_bias.clone()));
    }

    (hidden_outputs, outputs)
}


//fn backpropagate(network: Vec<Vec<Vec<f64>>>, input_vector: Vec<f64>, targets: Vec<f64>, output_layer: Vec<Vec<f64>>) {
    // return two things
//    let rs = feed_forward(network.clone(), input_vector.clone());
fn backpropagate(network: (Vec<Vec<f64>>,Vec<Vec<f64>>) , input_vector: Vec<f64>, targets: Vec<f64>) -> (Vec<Vec<f64>>,Vec<Vec<f64>>) {
    let (hidden_outputs, outputs) = feed_forward(network.clone(), input_vector.clone());

    // the output * (1 - output) is from the derivative of sigmoid
    let mut output_deltas = vec![];

    for (output,target) in outputs.iter().zip(targets.iter()) {
        output_deltas.push(output * (1.0 - output) * (output * target));
    }

    let mut mut_output_neurons = vec![];
    // adjust weights for output layer, one neuron at a time
    for (i, output_neuron) in network.1.iter().enumerate() {
        let mut mut_output_neuron = output_neuron.clone();
        let mut hidden_outputs_p: Vec<f64> = hidden_outputs.clone();
        hidden_outputs_p.push(1.0);
        for (j, hidden_output) in hidden_outputs_p.iter().enumerate() {
            mut_output_neuron[j] -= output_deltas[i] * hidden_output;
        }
        mut_output_neurons.push(mut_output_neuron);
    }

    // back-proppagate errors to hidden layer
    let mut hidden_deltas = vec![];
    for (i, hidden_output) in hidden_outputs.iter().enumerate() {
        let os = network.1.iter().map(|x| x[i]).collect();
        hidden_deltas.push(hidden_output * (1.0 - hidden_output) * dot(output_deltas.clone(), os))
    }
    
    let mut mut_hidden_neurons = vec![];
    // adjust weights for hidden layer, one neuron at a time
    let mut input_vector_p: Vec<f64> = input_vector.clone();
    input_vector_p.push(1.0);
        
    for (i, hidden_neuron) in network.0.iter().enumerate() {
        let mut mut_hidden_neuron = hidden_neuron.clone();
        
        for (j, input) in input_vector_p.iter().enumerate() {
            mut_hidden_neuron[j] -= hidden_deltas[i] * input
        }
        mut_hidden_neurons.push(mut_hidden_neuron);
    }

    (mut_hidden_neurons, mut_output_neurons)
}

fn tof64(v: Vec<Vec<i64>>) -> Vec<Vec<f64>> {
    let mut out: Vec<Vec<f64>> = Vec::with_capacity(v.len());
    for (i, subv) in v.iter().enumerate() {
        let mut hh: Vec<f64> = Vec::with_capacity(subv.len());
        for vv in subv.iter() {
            hh.push(vv.clone() as f64);
        }
        out.push(hh);
    }
    out           
}

fn main() {
    // let mut weights: Vec<f64> = vec![];

    // let rcontents = fs::read_to_string("test.txt");
    // match rcontents {
    //     Result::Ok(contents) => {
    //         let lines = contents.lines().collect::<Vec<_>>();
    //         let weight_strings = lines[0].split(',').collect::<Vec<_>>();
    //         for weight_string in weight_strings {
    //              weights.push(weight_string.parse::<f64>().unwrap());
    //         }
            
    //         let mut outputs = vec![];
    //         let mut inputs: Vec<Vec<f64>> = vec![];

    //         for i in 1..(lines.len()) {
    //             let inputs_and_outputs = lines[i].split(';').collect::<Vec<_>>();
    //             outputs.push(inputs_and_outputs[1].parse::<i64>().unwrap());
    //             let input_strings = inputs_and_outputs[0].split(',').collect::<Vec<_>>();

    //             inputs.push(vec![]);
    //             for input_string in input_strings {
    //                 inputs[i-1].push(input_string.parse::<f64>().unwrap());
    //             }
    //         }

    //         let mut all_correct = false;
    //         let mut error = 0;

    //         for x in 0..20 {
    //             let mut error_count = 0;

    //             // println!("print errors");
    //             for i in 0..(inputs.len()) {

    //                 let result = dot(inputs[i].clone(), weights.clone());
    //                 if result * (outputs[i] as f64) <= 0. {
    //                     for j in 0..(weights.len()) {
    //                         weights[j] = weights[j] + learningRate * inputs[i][j] * (outputs[i] as f64);
    //                     }
    //                 }
    //                 println!("{:?}", result);
    //                 let mut result = 0;
    //                 let t: i64 = inputs[i].iter().sum();
    //                 // println!("{:?}", t);
    //                 if (t as f64) > threshold {
    //                     result = 1;
    //                 }
    //                 error = outputs[i] - result;
    //                 println!("{}", error);
    //                 if error > 0 {
    //                     println!("got errro");
    //                     error_count = error_count + 1;
    //                     for j in 0..3 {
    //                         weights[j] = weights[j] + learningRate * (error as f64) * (inputs[i][j] as f64);
    //                     }
    //                 }
                    
    //             }

    //             if error_count == 0 {
    //                 all_correct = true
    //             }
    //         }

    //         println!("{:?}", weights.clone());

    //         println!("{:?}", dot(vec![-2.,4., -1.], weights.clone()));
    //         println!("{:?}", dot(vec![4.,1., -1.], weights.clone()));
    //         println!("{:?}", dot(vec![1.,6., -1.], weights.clone()));
    //         println!("{:?}", dot(vec![2.,4., -1.], weights.clone()));
    //         println!("{:?}", dot(vec![6.,2., -1.], weights.clone()));
    //     },
    //     Result::Err(err) => ()
    // }

    // let dataset: Vec<Vec<f64>> =
    //     vec![
    //         vec![2.7810836,   2.550537003,  0.0],
    //         vec![1.465489372, 2.362125076,  0.0],
    //         vec![3.396561688, 4.400293529,  0.0],
    //         vec![1.38807019,  1.850220317,  0.0],
    //         vec![3.06407232,  3.005305973,  0.0],
    //         vec![7.627531214, 2.759262235,  1.0],
    //         vec![5.332441248, 2.088626775,  1.0],
    //         vec![6.922596716, 1.77106367,   1.0],
    //         vec![8.675418651, -0.242068655, 1.0],
    //         vec![7.673756466, 3.508563011,  1.0]];

    // let l_rate = 0.3;
    // let n_epoch = 100;
    // let coef = coefficients_sgd(dataset, l_rate, n_epoch);
    // println!("{:?}",coef);

    // let xor_network =
    //     vec![ // hidden layer
    //         vec![
    //             vec![20.0, 20.0, -30.0], // and neuron
    //             vec![20.0, 20.0, -10.0]], // or neuron
    //         // output layer
    //         vec![
    //             vec![-60.0, 60.0, -30.0]]]; // 2nd input but not 1st input neuron

    let zero_digit =
        vec![1,1,1,1,1,
         1,0,0,0,1,
         1,0,0,0,1,
         1,0,0,0,1,
         1,1,1,1,1];

    let one_digit =
        vec![0,0,1,0,0,
         0,0,1,0,0,
         0,0,1,0,0,
         0,0,1,0,0,
         0,0,1,0,0];

    let two_digit =
        vec![1,1,1,1,1,
         0,0,0,0,1,
         1,1,1,1,1,
         1,0,0,0,0,
         1,1,1,1,1];

    let three_digit =
        vec![1,1,1,1,1,
         0,0,0,0,1,
         1,1,1,1,1,
         0,0,0,0,1,
         1,1,1,1,1];

    let four_digit =
        vec![1,0,0,0,1,
         1,0,0,0,1,
         1,1,1,1,1,
         0,0,0,0,1,
         0,0,0,0,1];

    let five_digit =
        vec![1,1,1,1,1,
         1,0,0,0,0,
         1,1,1,1,1,
         0,0,0,0,1,
         1,1,1,1,1];

    let six_digit =
        vec![1,1,1,1,1,
         1,0,0,0,0,
         1,1,1,1,1,
         1,0,0,0,1,
         1,1,1,1,1];


    let seven_digit =
        vec![1,1,1,1,1,
         0,0,0,0,1,
         0,0,0,0,1,
         0,0,0,0,1,
         0,0,0,0,1];

    let eight_digit =
        vec![1,1,1,1,1,
         1,0,0,0,1,
         1,1,1,1,1,
         1,0,0,0,1,
         1,1,1,1,1];


    let nine_digit =
        vec![1,1,1,1,1,
         1,0,0,0,1,
         1,1,1,1,1,
         0,0,0,0,1,
         1,1,1,1,1];

    let inputs =
        vec![zero_digit,one_digit,two_digit,three_digit,four_digit,five_digit,six_digit,seven_digit,eight_digit,nine_digit];
    
    let targets = vec![
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 1]];

    let input_size = 25;  // each input is a vector of length 25
    let num_hidden = 5;   // we'll have 5 neurons in the hidden layer
    let output_size = 10; // we need 10 outputs for each input

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

    // let mut weights: Vec<f64> = Vec::with_capacity(n + 1);
    // for _i in 0..n+1 {
    //     weights.push(rng.sample(uniform));
    // }

    let inputs_f64 = tof64(inputs);
    let targets_f64 = tof64(targets);

    println!("get started");
    for _i in 0..5000 {
        for pair in inputs_f64.iter().zip(targets_f64.iter()) {
            let p = backpropagate((hidden_layer.clone(), output_layer.clone()), pair.0.clone(), pair.1.clone());
            hidden_layer = p.0;
            output_layer = p.1;
        }
    }

    let r0 = feed_forward((hidden_layer.clone(), output_layer.clone()), inputs_f64[0].clone());
    println!("{:?}", r0.1);

    let r1 = feed_forward((hidden_layer.clone(), output_layer.clone()), inputs_f64[1].clone());
    println!("{:?}", r1.1);

    let r8 = feed_forward((hidden_layer.clone(), output_layer.clone()), inputs_f64[8].clone());
    println!("{:?}", r8.1);
}


// fn feed_forward(neural_network: (Vec<Vec<f64>>,Vec<Vec<f64>>), input_vec: Vec<f64>) -> (Vec<f64>, Vec<f64>) {

/*
# feed_forward produces the outputs of every neuron
# feed_forward[-1] is the outputs of the output-layer neurons print x, y, feed_forward(xor_network,[x, y])[-1]
    # 0 0 [9.38314668300676e-14]
    # 0 1 [0.9999999999999059]
    # 1 0 [0.9999999999999059]
    # 1 1 [9.383146683006828e-14]

*/


fn predict(row: Vec<f64>, coefficients: Vec<f64>) -> f64 {
    let mut yhat = coefficients[0];
    for i in 0..(row.len() - 1) {
        yhat += coefficients[i + 1] * row[i];
    }
    1.0 / (1.0 + (-yhat).exp())
}

fn coefficients_sgd(train: Vec<Vec<f64>>, l_rate: f64, n_epoch: usize) -> Vec<f64> {
    let mut coef = vec![0.0; train[0].clone().len()];
    for _epoch in 0..n_epoch {
        for row in train.clone() {
            let yhat = predict(row.clone(), coef.clone());
            let error = row[row.len()-1] - yhat;
            coef[0] = coef[0] + l_rate * error * yhat * (1.0 - yhat);
            for i in 0..(row.len() - 1) {
                coef[i+1] = coef[i+1] + l_rate * error * yhat * (1.0 - yhat) * row[i]; 
            }
        }
    }
    coef
}

fn logistics_regression(train: Vec<Vec<f64>>, test: Vec<Vec<f64>>, l_rate: f64, n_epoch: usize) -> Vec<i64> {
    let mut predictions = vec![];
    let coef = coefficients_sgd(train, l_rate, n_epoch);
    for row in test {
        let yhat = predict(row, coef.clone()).round() as i64;
        predictions.push(yhat);
    }
    predictions
}
