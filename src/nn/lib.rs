extern crate rand;
use rand::{Rng, thread_rng};
use rand::distributions::Uniform;

fn dot(xs: Vec<f64>, ys: Vec<f64>) -> f64 {
    xs.iter().zip(ys.iter()).map(|(x, y)| x*y).sum()
}

fn sigmoid(t: f64) -> f64 {
    1.0 / (1.0 + std::f64::consts::E.powf(-t))
}

fn neuron_output(weights: Vec<f64>, inputs: Vec<f64>) -> f64 {
    sigmoid(dot(weights, inputs))
}

pub fn feed_forward(neural_network: (Vec<Vec<f64>>, Vec<Vec<f64>>), input_vec: Vec<f64>) -> (Vec<f64>, Vec<f64>) {
    let (hidden_layers, output_layer) = neural_network;

    let mut hidden_outputs = vec![];
    let mut input_vector = input_vec.clone();
    input_vector.push(1.0);
    let input_with_bias = input_vector.clone();
    for neuron in hidden_layers {
        hidden_outputs.push(neuron_output(neuron, input_with_bias.clone()));
    }

    let mut outputs = vec![];
    input_vector = hidden_outputs.clone();
    input_vector.push(1.0);
    let input_with_bias = input_vector.clone();
    for neuron in output_layer {
        outputs.push(neuron_output(neuron, input_with_bias.clone()));
    }

    (hidden_outputs, outputs)
}

pub fn backpropagate(network: (Vec<Vec<f64>>,Vec<Vec<f64>>) , input_vector: Vec<f64>, targets: Vec<f64>) -> (Vec<Vec<f64>>,Vec<Vec<f64>>) {
    let (hidden_outputs, outputs) = feed_forward(network.clone(), input_vector.clone());

    // the output * (1 - output) is from the derivative of sigmoid
    let mut output_deltas = vec![];

    for (output,target) in outputs.iter().zip(targets.iter()) {
        output_deltas.push(output * (1.0 - output) * (output - target));
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

pub fn tof64(v: Vec<Vec<i64>>) -> Vec<Vec<f64>> {
    let mut out: Vec<Vec<f64>> = Vec::with_capacity(v.len());
    for subv in v.iter() {
        let mut hh: Vec<f64> = Vec::with_capacity(subv.len());
        for vv in subv.iter() {
            hh.push(vv.clone() as f64);
        }
        out.push(hh);
    }
    out           
}

