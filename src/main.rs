use map_macro::hash_map;

use dit_as_91896::menu;

fn main() {
    // let menu_items = menu::Items::new(vec![String::from("wow")], vec![], vec![]);
    // println!("{menu_items}");
    println!("Hello, world!\n\n");

    let items: menu::Items = menu::Items {
        food: hash_map! {
            String::from("0") => 0.0,
        },
        drinks: hash_map! {
            String::from("1") => 1.0,
        },
        sides: hash_map! {
            String::from("2") => 2.0,
        },
    };
    println!(r#"{items}"#)
}
