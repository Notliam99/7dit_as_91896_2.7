use std::{
    io::{stdout, Write},
    process,
};

use crossterm::{
    cursor::MoveUp,
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};

use dit_as_91896::{
    food::Items,
    order::{Order, OrdersVec},
};
use map_macro::hash_map;
use text_io::read;

fn user_order(user_menu: &Items, cost_menu: &Items, number_order: usize) -> Order {
    let mut order = Order::new(user_menu.clone(), cost_menu.clone());

    execute!(stdout(), Clear(ClearType::All)).unwrap();

    println!(
        "MENU\n{}\n\n\nCURENT ORDER: ({number_order})\n{order}",
        user_menu.menu_view()
    );

    loop {
        print!("\nAdd To Order: ");
        stdout().flush().unwrap();
        let add: String = read!("{}\n");

        let add_vec: Vec<&str> = add.trim().split_whitespace().collect();

        let found: bool = match order.order_add(add_vec.join(" ").to_lowercase()) {
            Ok(_) => true,
            Err(_) => false,
        };

        execute!(stdout(), Clear(ClearType::All)).unwrap();

        println!(
            "MENU\n{}\n\n\nCURENT ORDER: ({number_order})\n{order}",
            user_menu.menu_view()
        );

        if !found {
            println!("\n---?---\nDidn't Reconise Item\n---?---");
            continue;
        }

        print!("\n\n Do You Want To Add More? [Y/N]");

        let exit: String = read!("{}\n");

        if exit.to_lowercase() != String::from("y") {
            break;
        }
    }

    order
}

fn main() {
    let food: Items = Items {
        food: hash_map! {
            String::from("black bean beef burrito") => 16.0,
            String::from("extra spicy beff barrito") => 14.5,
            String::from("corn beff barrito") => 16.0,
        },
        drinks: hash_map! {
            String::from("coke") => 6.0,
            String::from("sprite") => 6.0,
            String::from("fanta") => 6.0
        },
        sides: hash_map! {
            String::from("rice") => 3.5,
            String::from("corn Chips") => 5.0,
            String::from("guacamole") => 2.5
        },
    };

    let cost_of_food: Items = Items {
        food: hash_map! {
            String::from("black bean beef burrito") => 7.5,
            String::from("extra spicy beff barrito") => 7.0,
            String::from("corn beff barrito") => 7.5,
        },
        drinks: hash_map! {
            String::from("coke") => 3.0,
            String::from("sprite") => 3.0,
            String::from("fanta") => 3.0
        },
        sides: hash_map! {
            String::from("rice") => 1.5,
            String::from("corn Chips") => 2.0,
            String::from("guacamole") => 0.5
        },
    };

    // enters a seprate sestion kinda
    println!("ENTERING SCREEN ...");
    execute!(stdout(), EnterAlternateScreen, Clear(ClearType::All)).unwrap();

    ctrlc::set_handler(move || {
        execute!(
            stdout(),
            LeaveAlternateScreen,
            MoveUp(1),
            Clear(ClearType::CurrentLine)
        )
        .unwrap();
        process::exit(1)
    })
    .unwrap();

    let mut orders: Vec<Order> = Vec::new();

    loop {
        orders.push(user_order(&food, &cost_of_food, orders.len()));

        execute!(stdout(), Clear(ClearType::All)).unwrap();
        println!("{}", OrdersVec(orders.clone()));

        print!("\n\n Do You Want To Add A Order? [Y/N]");

        let exit: String = read!("{}\n");

        if exit.to_lowercase() != String::from("y") {
            break;
        }
    }

    println!("{}", OrdersVec(orders.clone()));

    let mut total_cost_profit: Vec<f64> = vec![0.0, 0.0];

    for i in orders.iter() {
        let cost_profit: Vec<f64> = i.cost_profit();

        total_cost_profit[0] = total_cost_profit[0] + cost_profit[0];
        total_cost_profit[1] = total_cost_profit[1] + cost_profit[1];
    }

    // on exit it closes out
    execute!(
        stdout(),
        LeaveAlternateScreen,
        MoveUp(1),
        Clear(ClearType::CurrentLine)
    )
    .unwrap();

    println!(
        "{}\n\nTotal Sales (gross): ${:.2}\n\nTotal profit: ${:.2}",
        OrdersVec(orders),
        total_cost_profit[0],
        total_cost_profit[1]
    );
}
