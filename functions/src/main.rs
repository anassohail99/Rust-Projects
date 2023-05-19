mod associated_functions;

fn main() {
    associated_functions::main();
    // fizzbuzz_to(100);
}

// fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
//     return lhs % rhs == 0;
// }

// fn fizzbuzz(n: u32) {
//     if is_divisible_by(n, 3) {
//         println!("Fizzbuzz");
//     } else if is_divisible_by(n, 6) {
//         println!("Fizz");
//     } else if is_divisible_by(n, 5) {
//         println!("buzz");
//     } else {
//         println!("{}", n);
//     }
// }

// fn fizzbuzz_to(n: u32) {
//     for n in 1..=n {
//         fizzbuzz(n);
//     }
// }
