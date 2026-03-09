// 3-hidden-layer-network.rs
//
// Handcrafted hidden-layer neural network.
//
// Architecture:
// input -> 2 hidden neurons -> 1 output neuron
//
// This file demonstrates a real multi-layer forward pass.
// Nothing is being trained yet.
// All weights and biases are manually chosen.

/*
input ----\
           > hidden_1 ---\
bias_1 ---/               \
                           > output -> final score
input ----\               /
           > hidden_2 ---/
bias_2 ---/
*/

fn sigmoid(value: f64) -> f64 {
    1.0 / (1.0 + (-value).exp())
}

fn main() {
    let input = 2.0;

    let hidden_weight_1 = 1.5;
    let hidden_bias_1 = -1.0;

    let hidden_weight_2 = -1.0;
    let hidden_bias_2 = 2.0;

    let hidden_output_1 = sigmoid(input * hidden_weight_1 + hidden_bias_1);
    let hidden_output_2 = sigmoid(input * hidden_weight_2 + hidden_bias_2);

    let output_weight_1 = 2.0;
    let output_weight_2 = -1.0;
    let output_bias = -0.5;

    let output = sigmoid(
        hidden_output_1 * output_weight_1 + hidden_output_2 * output_weight_2 + output_bias,
    );

    println!("input: {input}");
    println!("hidden_output_1: {hidden_output_1}");
    println!("hidden_output_2: {hidden_output_2}");
    println!("final_output: {output}");
}
