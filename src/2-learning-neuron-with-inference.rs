/*
A trainable neuron with inference.
This goes in two stages: 1 - training, 2 - inference.

it learns that the number 1 is not cool, and the number 2 is cool.
input → expected output
1 → 0  (not cool)
2 → 1  (cool)

The neuron learns parameters (weight, bias) from a dataset.
The training looo consists of:
predict output
compute error
compute gradient
adjust weight and bias
Repeated many times until the model converges.

input_number ----\
                  > neuron -> sigmoid -> prediction
bias ------------/

After training, the neuron is used to classify new inputs.
prediction > 0.5 → cool
prediction ≤ 0.5 → not cool
*/
use std::io;

fn sigmoid(value: f64) -> f64 {
    1.0 / (1.0 + (-value).exp())
}

fn main() {
    // dataset
    let training_inputs = [1.0, 2.0];
    let expected_outputs = [0.0, 1.0];

    // model parameters
    let mut weight = 0.0;
    let mut bias = 0.0;

    let learning_rate = 0.1;

    // training
    for _ in 0..10000 {
        let mut weight_gradient = 0.0;
        let mut bias_gradient = 0.0;

        for index in 0..training_inputs.len() {
            let input_value = training_inputs[index];
            let expected_output = expected_outputs[index];

            let predicted_output = sigmoid(input_value * weight + bias);

            let prediction_error = predicted_output - expected_output;

            let sigmoid_derivative = predicted_output * (1.0 - predicted_output);

            let output_gradient = prediction_error * sigmoid_derivative;

            weight_gradient += output_gradient * input_value;
            bias_gradient += output_gradient;
        }

        weight -= learning_rate * weight_gradient;
        bias -= learning_rate * bias_gradient;
    }

    println!("trained weight: {weight}");
    println!("trained bias: {bias}");

    // inference
    let mut user_input = String::new();

    println!("enter a number:");

    io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read input");

    let input_number: f64 = user_input
        .trim()
        .parse()
        .expect("please type a valid number");

    let coolness_score = sigmoid(input_number * weight + bias);

    if coolness_score > 0.5 {
        println!("cool");
    } else {
        println!("not cool");
    }
}

//---

/*
it can be represented in a simpler variable name format like
can you write me this again but with clearer variable names, not enshorts

use std::io;

fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

fn main() {

    // dataset
    let inputs = [1.0, 2.0];
    let targets = [0.0, 1.0];

    // parameters
    let mut w = 0.0;
    let mut b = 0.0;

    let lr = 0.1;

    // training
    for _ in 0..10000 {

        let mut dw = 0.0;
        let mut db = 0.0;

        for i in 0..inputs.len() {

            let x = inputs[i];
            let y = targets[i];

            let pred = sigmoid(x * w + b);

            let error = pred - y;

            let grad = error * pred * (1.0 - pred);

            dw += grad * x;
            db += grad;
        }

        w -= lr * dw;
        b -= lr * db;
    }

    println!("trained weight: {}", w);
    println!("trained bias: {}", b);

    // inference
    let mut input = String::new();

    println!("enter a number:");

    io::stdin().read_line(&mut input).unwrap();

    let x: f64 = input.trim().parse().unwrap();

    let score = sigmoid(x * w + b);

    if score > 0.5 {
        println!("cool");
    } else {
        println!("not cool");
    }
}
*/
