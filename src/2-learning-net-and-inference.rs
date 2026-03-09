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
