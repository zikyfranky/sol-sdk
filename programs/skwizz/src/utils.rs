pub fn sqrt(value: u64) -> u64 {
    // Use fixed-point arithmetic to approximate square root using the Newton-Raphson method
    let mut guess = value / 2;
    for _ in 0..10 {
        guess = (guess + value / guess) / 2;
    }

    guess
}
