fn find_multiples(limit: u32) -> u32 {
    let sum_of_multiple = |num: u32| -> u32 {
        let amount = (limit - 1) / num;
        num * (amount * (amount + 1)) / 2
    };
    sum_of_multiple(3) + sum_of_multiple(5) - sum_of_multiple(15)
}

fn main() {
    println!("{}", find_multiples(1000));
}
