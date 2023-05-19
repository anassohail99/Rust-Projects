fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_Param, bool_Param) = pair;
    return (bool_Param, int_Param);
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32, f32);

fn main() {
    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );

    // Values can be extracted from the tuple using tuple indexing.
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // Tuples can be tuple members.
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("Tuple inside a tuple: {:?}", tuple_of_tuples);

    let tuple_string = format!("{:?}", tuple_of_tuples);
    println!("Tuple inside a tuple: {}", tuple_string);
}
