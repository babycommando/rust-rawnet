// A layer of neurons.
// Each neuron receives the same input vector, but has its own weights and bias.
// Each neuron produces its own output.
// Shows how a neural network layer is formed by multiple neurons operating in parallel.
// Still no learning, parameters are manually chosen.

// Outputs are grouped into a vector:
// layer_output = [output1, output2, output3]

fn main() {
    let inputs = vec![1.0, 2.0, 3.0];

    let weights1 = vec![0.2, 0.8, -0.5];
    let weights2 = vec![0.5, -0.9, 0.3];
    let weights3 = vec![-0.4, 0.1, 0.6];

    let bias1 = 0.1;
    let bias2 = 0.2;
    let bias3 = -0.1;

    let neuron1 =
        inputs[0] * weights1[0] + inputs[1] * weights1[1] + inputs[2] * weights1[2] + bias1;

    let neuron2 =
        inputs[0] * weights2[0] + inputs[1] * weights2[1] + inputs[2] * weights2[2] + bias2;

    let neuron3 =
        inputs[0] * weights3[0] + inputs[1] * weights3[1] + inputs[2] * weights3[2] + bias3;

    let layer_output = vec![neuron1, neuron2, neuron3];

    println!("{:?}", layer_output);
}
