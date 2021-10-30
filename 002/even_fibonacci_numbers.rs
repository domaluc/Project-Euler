fn fibonacci_even(limit: u32) -> u32 {
    let mut even_sum = 0u32;
    let mut x;
    let mut y = 1u32;
    let mut z = 2u32;
    while z < limit {
        even_sum += z;
        x = y + z;
        y = x + z;
        z = x + y;
    }
    even_sum
}

fn main() {
    println!("{}", fibonacci_even(4_000_000));
}
