use dit_as_91896::menu;

fn main() {
    let menu_items = menu::Items::new(vec![String::from("wow")], vec![], vec![]);
    println!("{menu_items}");
    println!("Hello, world!\n\n");

    let items: menu::Items = menu::Items {
        food: vec![String::from("0")],
        drinks: vec![String::from("1")],
        sides: vec![String::from("2")],
    };
    println!("{items}")
}
