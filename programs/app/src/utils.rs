pub fn sqrt(x: u64) -> u64 {
    // Use fixed-point arithmetic to approximate square root using the Newton-Raphson method
    let mut z = (x + 1) / 2;
    let mut y = x;

    while z < y {
        y = z;
        z = (x / z + z) / 2;
    }

    z
}
