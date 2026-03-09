// 4-backpropagation-network.rs
//
// Trainable hidden-layer neural network.
//
// Architecture:
// input -> 2 hidden neurons -> 1 output neuron
//
// This file introduces backpropagation.
// The network learns its weights and biases from a tiny dataset.
//
// This is the first file where the network is truly multi-layer and trainable.

use std::io;

fn sigmoid(value: f64) -> f64 {
    1.0 / (1.0 + (-value).exp())
}

fn sigmoid_derivative(output: f64) -> f64 {
    output * (1.0 - output)
}

fn main() {
    let training_inputs = [1.0, 2.0];
    let expected_outputs = [0.0, 1.0];

    let mut hidden_weight_1 = 0.5;
    let mut hidden_bias_1 = 0.0;

    let mut hidden_weight_2 = -0.5;
    let mut hidden_bias_2 = 0.0;

    let mut output_weight_1 = 0.5;
    let mut output_weight_2 = 0.5;
    let mut output_bias = 0.0;

    let learning_rate = 0.5;

    for _ in 0..10000 {
        for index in 0..training_inputs.len() {
            let input_value = training_inputs[index];
            let expected_output = expected_outputs[index];

            let hidden_output_1 = sigmoid(input_value * hidden_weight_1 + hidden_bias_1);

            let hidden_output_2 = sigmoid(input_value * hidden_weight_2 + hidden_bias_2);

            let predicted_output = sigmoid(
                hidden_output_1 * output_weight_1 + hidden_output_2 * output_weight_2 + output_bias,
            );

            let output_error = predicted_output - expected_output;
            let output_delta = output_error * sigmoid_derivative(predicted_output);

            let hidden_error_1 = output_delta * output_weight_1;
            let hidden_error_2 = output_delta * output_weight_2;

            let hidden_delta_1 = hidden_error_1 * sigmoid_derivative(hidden_output_1);

            let hidden_delta_2 = hidden_error_2 * sigmoid_derivative(hidden_output_2);

            output_weight_1 -= learning_rate * output_delta * hidden_output_1;
            output_weight_2 -= learning_rate * output_delta * hidden_output_2;
            output_bias -= learning_rate * output_delta;

            hidden_weight_1 -= learning_rate * hidden_delta_1 * input_value;
            hidden_bias_1 -= learning_rate * hidden_delta_1;

            hidden_weight_2 -= learning_rate * hidden_delta_2 * input_value;
            hidden_bias_2 -= learning_rate * hidden_delta_2;
        }
    }

    println!("training finished");
    println!("hidden_weight_1: {hidden_weight_1}");
    println!("hidden_bias_1: {hidden_bias_1}");
    println!("hidden_weight_2: {hidden_weight_2}");
    println!("hidden_bias_2: {hidden_bias_2}");
    println!("output_weight_1: {output_weight_1}");
    println!("output_weight_2: {output_weight_2}");
    println!("output_bias: {output_bias}");

    let mut user_input = String::new();

    println!("enter a number:");

    io::stdin()
        .read_line(&mut user_input)
        .expect("failed to read input");

    let input_number: f64 = user_input
        .trim()
        .parse()
        .expect("please type a valid number");

    let hidden_output_1 = sigmoid(input_number * hidden_weight_1 + hidden_bias_1);

    let hidden_output_2 = sigmoid(input_number * hidden_weight_2 + hidden_bias_2);

    let coolness_score = sigmoid(
        hidden_output_1 * output_weight_1 + hidden_output_2 * output_weight_2 + output_bias,
    );

    println!("coolness_score: {coolness_score}");

    if coolness_score > 0.5 {
        println!("cool");
    } else {
        println!("not cool");
    }
}
