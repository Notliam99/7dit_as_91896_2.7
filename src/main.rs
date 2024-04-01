use map_macro::hash_map;

use dit_as_91896::food::Items;

fn main() {
    let food: Items = Items {
        food: hash_map! {
            String::from("cooked barito") => 15.0,
        },
        drinks: hash_map! {
            String::from("coke") => 6.0,
        },
        sides: hash_map! {
            String::from("rice") => 8.0,
        },
    };

    let cost_of_food: Items = Items {
        food: hash_map! {
            String::from("cooked barito") => 6.0,
        },
        drinks: hash_map! {
            String::from("coke") => 1.0,
        },
        sides: hash_map! {
            String::from("rice") => 0.5,
        },
    };

    println!(
        "to buy costs:\n{}\n\nto make costs:\n{}",
        food, cost_of_food
    )
}
