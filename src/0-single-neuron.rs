fn main() {
    let inputs = vec![1.1, 2.2, 3.3];
    let weights = vec![4.1, 8.3, 9.1];
    let bias = 3.0;

    let output = inputs[0] * weights[0] + inputs[1] * weights[1] + inputs[2] * weights[2] + bias;

    println!("{output}");
}
