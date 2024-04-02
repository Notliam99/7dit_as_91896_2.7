use std::{
    io::{stdout, Write},
    process,
};

use crossterm::{
    cursor::MoveUp,
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use ctrlc;
use dit_as_91896::{
    food::Items,
    order::{Order, OrdersVec},
};
use map_macro::hash_map;
use text_io::read;

fn user_order(user_menu: &Items, cost_menu: &Items) -> Order {
    let mut order = Order::new(user_menu.clone(), cost_menu.clone());
    println!("MENU\n{}\n\n\nCURENT ORDER\n{order}", user_menu.menu_view());

    loop {
        print!("\nAdd To Order: ");
        stdout().flush().unwrap();
        let add: String = read!("{}\n");

        order.order_add(add).unwrap();

        print!("\n\n Do You Want To Add More [Y/N]");

        let exit: String = read!("{}");

        if exit.to_lowercase() == String::from("y") {
            break;
        }
    }

    order
}

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

    // enters a seprate sestion kinda
    println!("ENTERING SCREEN ...");
    execute!(stdout(), EnterAlternateScreen, Clear(ClearType::All)).unwrap();

    ctrlc::set_handler(move || {
        execute!(stdout(), LeaveAlternateScreen).unwrap();
        process::exit(1)
    })
    .unwrap();

    let mut orders: Vec<Order> = Vec::new();

    loop {
        orders.push(user_order(&food, &cost_of_food));
        break;
    }

    println!("{}", OrdersVec(orders.clone()));

    // on exit it closes out
    execute!(
        stdout(),
        LeaveAlternateScreen,
        MoveUp(1),
        Clear(ClearType::CurrentLine)
    )
    .unwrap();

    println!("{}", OrdersVec(orders));
}
