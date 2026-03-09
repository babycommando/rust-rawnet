// 5-vectorized-network.rs
//
// Vector-style hidden-layer neural network.
//
// Architecture:
// input -> hidden layer -> output neuron
//
// This file keeps the same network as before,
// but stores weights and biases in vectors instead of
// hardcoding each neuron by name.
//
// This is a small step toward scalable neural network code.

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

    let mut hidden_weights = vec![0.5, -0.5];
    let mut hidden_biases = vec![0.0, 0.0];

    let mut output_weights = vec![0.5, 0.5];
    let mut output_bias = 0.0;

    let learning_rate = 0.5;

    for _ in 0..10000 {
        for index in 0..training_inputs.len() {
            let input_value = training_inputs[index];
            let expected_output = expected_outputs[index];

            let mut hidden_outputs = Vec::new();

            for hidden_index in 0..hidden_weights.len() {
                let hidden_output = sigmoid(
                    input_value * hidden_weights[hidden_index] + hidden_biases[hidden_index],
                );

                hidden_outputs.push(hidden_output);
            }

            let mut output_input = output_bias;

            for hidden_index in 0..hidden_outputs.len() {
                output_input += hidden_outputs[hidden_index] * output_weights[hidden_index];
            }

            let predicted_output = sigmoid(output_input);

            let output_error = predicted_output - expected_output;
            let output_delta = output_error * sigmoid_derivative(predicted_output);

            let mut hidden_deltas = Vec::new();

            for hidden_index in 0..hidden_outputs.len() {
                let hidden_error = output_delta * output_weights[hidden_index];
                let hidden_delta = hidden_error * sigmoid_derivative(hidden_outputs[hidden_index]);

                hidden_deltas.push(hidden_delta);
            }

            for hidden_index in 0..output_weights.len() {
                output_weights[hidden_index] -=
                    learning_rate * output_delta * hidden_outputs[hidden_index];
            }

            output_bias -= learning_rate * output_delta;

            for hidden_index in 0..hidden_weights.len() {
                hidden_weights[hidden_index] -=
                    learning_rate * hidden_deltas[hidden_index] * input_value;

                hidden_biases[hidden_index] -= learning_rate * hidden_deltas[hidden_index];
            }
        }
    }

    println!("training finished");
    println!("hidden_weights: {:?}", hidden_weights);
    println!("hidden_biases: {:?}", hidden_biases);
    println!("output_weights: {:?}", output_weights);
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

    let mut hidden_outputs = Vec::new();

    for hidden_index in 0..hidden_weights.len() {
        let hidden_output =
            sigmoid(input_number * hidden_weights[hidden_index] + hidden_biases[hidden_index]);

        hidden_outputs.push(hidden_output);
    }

    let mut output_input = output_bias;

    for hidden_index in 0..hidden_outputs.len() {
        output_input += hidden_outputs[hidden_index] * output_weights[hidden_index];
    }

    let coolness_score = sigmoid(output_input);

    println!("coolness_score: {coolness_score}");

    if coolness_score > 0.5 {
        println!("cool");
    } else {
        println!("not cool");
    }
}
