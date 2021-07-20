use std::fmt::Write;
fn main() {
    println!("{:?}", multi_table(2));
    
}


fn multi_table(n: u64) -> String {
    let mut number_to_multiply_with = 1;
    let mut table = String::new();

    loop {
        write!(table, "{} * {} = {}\n", number_to_multiply_with, n, number_to_multiply_with * n).unwrap();
        number_to_multiply_with += 1;
        if number_to_multiply_with == 10 {
            write!(table, "{} * {} = {}", number_to_multiply_with, n, number_to_multiply_with * n).unwrap();
            break table;
        } else {
            continue;
        }
    }
}


// fn multi_table(n: u64) -> String {
//     let mut number_to_multiply_with = 1;

//     loop {
//         println!("{} * {} = {}\n" , number_to_multiply_with, n, number_to_multiply_with * n);
//         number_to_multiply_with += 1;
//         if number_to_multiply_with == 11 {
//             break String::from("Saam {}");
//         } else {
//             continue;
//         }
//     }
// }