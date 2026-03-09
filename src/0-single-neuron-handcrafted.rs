/*
A single neuron.
This is the fundamental building block of neural networks.
There is no learning here, all weights and bias are manually chosen (handcrafted).

It computes a weighted sum of inputs plus a bias:
output = x1*w1 + x2*w2 + x3*w3 + bias
*/

fn main() {
    let inputs = vec![1.1, 2.2, 3.3];
    let weights = vec![4.1, 8.3, 9.1];
    let bias = 3.0;

    let output = inputs[0] * weights[0] + inputs[1] * weights[1] + inputs[2] * weights[2] + bias;

    println!("{output}");
}
