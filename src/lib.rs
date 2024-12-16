// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

///////////////////////////////////////////// WASM
use ::std::cmp::Ordering;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub fn age_comparator(age: i8) -> String {
    let eligible_age = 18;
    match age.cmp(&eligible_age) {
        Ordering::Greater => format!("You are {} Eligible To Vote", age),
        Ordering::Less => format!("You are {} Not Eligible To Vote", age),
        Ordering::Equal => format!("Congrats You gained the Rights to Vote").to_string(),
    }
}
