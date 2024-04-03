use crate::food::Items;
use std::{collections::HashMap, fmt::Display};

#[derive(Clone)]
pub struct Order {
    pub order: Items,
    pub menu_items: Items,
    pub cost_for_items: Items,
}

pub struct OrdersVec(pub Vec<Order>);

impl Order {
    /**
    # This New Func Creates A New Instance

    # Example
    ```rust
        use std::collections::HashMap;
        use dit_as_91896::{
            order::Order,
            food::Items
        };

        let my_order: Order = Order::new(
            Items::new(HashMap::new(), HashMap::new(), HashMap::new()),
            Items::new(HashMap::new(), HashMap::new(), HashMap::new())
        );
    ```
    */
    pub fn new(menu_items: Items, cost_for_items: Items) -> Self {
        Self {
            order: Items {
                food: HashMap::new(),
                drinks: HashMap::new(),
                sides: HashMap::new(),
            },
            menu_items,
            cost_for_items,
        }
    }

    /**
    # Add To The Order

    # Example
    ```rust
        use map_macro::hash_map;
        use std::collections::HashMap;

        use dit_as_91896::{
            order::Order,
            food::Items
        };

        let menu = Items::new(
            hash_map! {
                String::from("test_food") => 2.0
            },
            HashMap::new(),
            HashMap::new()
        );

        let cost_menu = Items::new(
            hash_map! {
                String::from("test_food") => 1.0
            },
            HashMap::new(),
            HashMap::new()
        );

        let mut my_order: Order = Order::new(
            menu,
            cost_menu
        );

        // adds two of the food.
        my_order.order_add(String::from("test_food")).unwrap();

        my_order.order_add(String::from("test_food")).unwrap();

        let order_output = Items::new(hash_map! { String::from("test_food") => 4.0 }, HashMap::new(), HashMap::new());

        assert!(my_order.order.food == order_output.food);
        assert!(my_order.order.drinks == order_output.drinks);
        assert!(my_order.order.sides == order_output.sides);


    ```
    */
    pub fn order_add(&mut self, item_name: String) -> Result<(), &'static str> {
        if self.menu_items.food.contains_key(&item_name) {
            // checks if item_name is in food

            if self.order.food.contains_key(&item_name) {
                // checks if theres allredy one in order

                let price: f64 = self.order.food.get(&item_name).unwrap()
                    + *self.menu_items.food.get(&item_name.clone()).unwrap();

                self.order.food.insert(item_name.clone(), price);
                Ok(())
            } else {
                self.order.food.insert(
                    item_name.clone(),
                    *self.menu_items.food.get(&item_name.clone()).unwrap(),
                );
                Ok(())
            }
        } else if self.menu_items.drinks.contains_key(&item_name) {
            // checks if item_name is in drinks

            if self.order.drinks.contains_key(&item_name) {
                // checks if theres allredy one in order

                let price: f64 = self.order.drinks.get(&item_name).unwrap()
                    + *self.menu_items.drinks.get(&item_name.clone()).unwrap();

                self.order.drinks.insert(item_name.clone(), price);
                Ok(())
            } else {
                self.order.drinks.insert(
                    item_name.clone(),
                    *self.menu_items.drinks.get(&item_name.clone()).unwrap(),
                );
                Ok(())
            }
        } else if self.menu_items.sides.contains_key(&item_name) {
            // checks if item_name is in sides

            if self.order.sides.contains_key(&item_name) {
                // checks if theres allredy one in order

                let price: f64 = self.order.sides.get(&item_name).unwrap()
                    + *self.menu_items.sides.get(&item_name.clone()).unwrap();

                self.order.sides.insert(item_name.clone(), price);
                Ok(())
            } else {
                self.order.sides.insert(
                    item_name.clone(),
                    *self.menu_items.sides.get(&item_name.clone()).unwrap(),
                );
                Ok(())
            }
        } else {
            Err("Error: The Item Could Not Be Found On The Menu.")
        }
    }
}

impl Display for Order {
    /**
    # Allows Order To Be Printed
    */
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.order)
    }
}

impl Display for OrdersVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::from("orders: [");

        for (x, order) in self.0.iter().enumerate() {
            output.push_str(format!("\n    {}: {{ {} }},", x, order).as_str())
        }

        output.replace_range((output.len() - 1).., "\n]");

        write!(f, "{output}")
    }
}
