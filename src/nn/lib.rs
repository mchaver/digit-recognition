extern crate rand;
use rand::{Rng, thread_rng};
use rand::distributions::Uniform;

fn dot(xs: &Vec<f64>, ys: &Vec<f64>) -> f64 {
    xs.iter().zip(ys.iter()).map(|(x, y)| x*y).sum()
}

fn sigmoid(t: f64) -> f64 {
    1.0 / (1.0 + std::f64::consts::E.powf(-t))
}

fn neuron_output(weights: &Vec<f64>, inputs: &Vec<f64>) -> f64 {
    sigmoid(dot(weights, inputs))
}

pub fn feed_forward(hidden_layers: &Vec<Vec<f64>>, output_layers: &Vec<Vec<f64>>, input_vec: &Vec<f64>) -> (Vec<f64>, Vec<f64>) {
    let mut hidden_outputs = vec![];
    let mut input_with_bias = input_vec.clone();
    input_with_bias.push(1.0);
    
    for neuron in hidden_layers {
        hidden_outputs.push(neuron_output(neuron, &input_with_bias));
    }

    let mut outputs = vec![];
    input_with_bias = hidden_outputs.clone();
    input_with_bias.push(1.0);

    for neuron in output_layers {
        outputs.push(neuron_output(&neuron, &input_with_bias));
    }

    (hidden_outputs, outputs)
}

pub fn backpropagate(hidden_layers: &mut Vec<Vec<f64>>, output_layers: &mut Vec<Vec<f64>>, input_vector: Vec<f64>, targets: Vec<f64>) {
    let (hidden_outputs, outputs) = feed_forward(&hidden_layers, &output_layers, &input_vector);

    // the output * (1 - output) is from the derivative of sigmoid
    let mut output_deltas = vec![];

    for (output,target) in outputs.iter().zip(targets.iter()) {
        output_deltas.push(output * (1.0 - output) * (output - target));
    }

    // adjust weights for output layer, one neuron at a time
    for i in 0..output_layers.len() {
        let mut hidden_outputs_p: Vec<f64> = hidden_outputs.clone();
        hidden_outputs_p.push(1.0);
        for (j, hidden_output) in hidden_outputs_p.iter().enumerate() {
            output_layers[i][j] -= output_deltas[i] * hidden_output;
        }
    }

    // back-proppagate errors to hidden layer
    let mut hidden_deltas = vec![];
    for (i, hidden_output) in hidden_outputs.iter().enumerate() {
        let os = output_layers.iter().map(|x| x[i]).collect();
        hidden_deltas.push(hidden_output * (1.0 - hidden_output) * dot(&output_deltas, &os))
    }
    
    // adjust weights for hidden layer, one neuron at a time
    let mut input_vector_p: Vec<f64> = input_vector.clone();
    input_vector_p.push(1.0);
        
    for i in 0..hidden_layers.len() {
        for (j, input) in input_vector_p.iter().enumerate() {
            hidden_layers[i][j] -= hidden_deltas[i] * input
        }
    }
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

