extern crate rand;
use rand::{Rng, thread_rng};
use rand::distributions::Uniform;

fn dot(xs: Vec<f64>, ys: Vec<f64>) -> f64 {
    xs.iter().zip(ys.iter()).map(|(x, y)| x*y).sum()
}

// smooth variation of step_function
fn sigmoid(t: f64) -> f64 {
    1.0 / (1.0 + std::f64::consts::E.powf(-t))
}

fn neuron_output(weights: Vec<f64>, inputs: Vec<f64>) -> f64 {
    sigmoid(dot(weights, inputs))
}

fn feed_forward(neural_network: (Vec<Vec<f64>>,Vec<Vec<f64>>), input_vec: Vec<f64>) -> (Vec<f64>, Vec<f64>) {
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

fn backpropagate(network: (Vec<Vec<f64>>,Vec<Vec<f64>>) , input_vector: Vec<f64>, targets: Vec<f64>) -> (Vec<Vec<f64>>,Vec<Vec<f64>>) {
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

fn tof64(v: Vec<Vec<i64>>) -> Vec<Vec<f64>> {
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

fn main() {
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

    let inputs_f64 = tof64(inputs);
    let targets_f64 = tof64(targets);


    for _i in 0..10000 {
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



    // println!("get started");
    // println!("{:?}", inputs_f64[0].clone());
    // let r0 = feed_forward((hidden_layer.clone(), output_layer.clone()), inputs_f64[0].clone());
    // println!("{:?}", r0);

    // println!("{}", neuron_output(vec![0.2,0.99,0.4],vec![0.,1.,1.]));

    // println!("{}", dot(vec![0.5,1.,1.75,3.],vec![3.,5.,1.,2.])); // 14.25

    // println!("{}", sigmoid(1.));
    // println!("{}", sigmoid(0.5));
// println!("{}", sigmoid(0.25));



    // let r1 = feed_forward((hidden_layer.clone(), output_layer.clone()), inputs_f64[1].clone());
    // println!("{:?}", r1.1);

    // let r8 = feed_forward((hidden_layer.clone(), output_layer.clone()), inputs_f64[8].clone());
    // println!("{:?}", r8.1);

    // r8.1.iter().cloned().fold(0./0., f64::max)
