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
