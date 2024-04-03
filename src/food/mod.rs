use core::fmt;
use std::{collections::HashMap, fmt::Display};

#[derive(Clone)]
pub struct Items {
    pub food: HashMap<String, f64>,
    pub drinks: HashMap<String, f64>,
    pub sides: HashMap<String, f64>,
}

impl Items {
    /// # New instance function
    /// @prams:
    ///     food: vector of food items
    ///     drinks: vector of drinks
    ///     sides: vector of sides
    /// @return: instance of type Items
    ///
    /// # Panics
    ///
    /// The function panics if the second argument is zero.
    ///
    /// ```rust
    ///     use std::collections::HashMap;
    ///     use map_macro::hash_map;
    ///     use dit_as_91896::food::Items;
    ///
    ///     let food: HashMap<String, f64> = hash_map! {
    ///         String::from("0") => 0.0,
    ///     };
    ///     let drinks: HashMap<String, f64> = hash_map! {
    ///         String::from("1") => 1.0,
    ///     };
    ///     let sides: HashMap<String, f64> = hash_map! {
    ///         String::from("2") => 2.0,
    ///     };
    ///
    ///     let items: Items = Items::new(food.clone(), drinks.clone(), sides.clone());
    ///
    ///     assert!(items.food == food);
    ///     assert!(items.drinks == drinks );
    ///     assert!(items.sides == sides);
    /// ```
    pub fn new(
        food: HashMap<String, f64>,
        drinks: HashMap<String, f64>,
        sides: HashMap<String, f64>,
    ) -> Self {
        Self {
            food,
            drinks,
            sides,
        }
    }

    pub fn menu_view(&self) -> String {
        let mut food: String = String::new();
        let mut drinks: String = String::new();
        let mut sides: String = String::new();

        if self.food.keys().len() != 0usize {
            for (item, price) in &self.food {
                food.push_str(format!("{}: costs ${:.2},", item.as_str(), price).as_str());
            }
            food.replace_range((food.len() - 1).., "")
        }

        if self.drinks.keys().len() != 0usize {
            for (item, price) in &self.drinks {
                drinks.push_str(format!("{}: costs ${:.2},", item.as_str(), price).as_str())
            }
            drinks.replace_range((drinks.len() - 1).., "")
        }

        if self.sides.keys().len() != 0usize {
            for (item, price) in &self.sides {
                sides.push_str(format!("{}: costs ${:.2},", item.as_str(), price).as_str());
            }
            sides.replace_range((sides.len() - 1).., "")
        }

        String::from(format!(
            "Food\n    {food}\n\nDrinks\n    {drinks}\n\nSides\n    {sides}"
        ))
    }
}

impl Display for Items {
    /// Fmt function allows the struct/type to be printed to the console
    ///
    /// ```rust
    ///     use map_macro::hash_map;
    ///     use dit_as_91896::food::Items;
    ///
    ///     let items: Items = Items {
    ///     food: hash_map! {
    ///         String::from("0") => 0.0,
    ///     },
    ///     drinks: hash_map! {
    ///         String::from("1") => 1.0,
    ///     },
    ///     sides: hash_map! {
    ///         String::from("2") => 2.0,
    ///     },
    /// };
    ///
    ///     assert!(format!("{items}").as_str() == "food: {\n    0: costs $0.00\n},\ndrinks: {\n    1: costs $1.00\n},\nsides: {\n    2: costs $2.00\n},");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut food: String = String::new();
        let mut drinks: String = String::new();
        let mut sides: String = String::new();

        if self.food.keys().len() != 0usize {
            for (item, price) in &self.food {
                food.push_str(format!("{}: costs ${:.2},", item.as_str(), price).as_str());
            }
            food.replace_range((food.len() - 1).., "")
        }

        if self.drinks.keys().len() != 0usize {
            for (item, price) in &self.drinks {
                drinks.push_str(format!("{}: costs ${:.2},", item.as_str(), price).as_str())
            }
            drinks.replace_range((drinks.len() - 1).., "")
        }

        if self.sides.keys().len() != 0usize {
            for (item, price) in &self.sides {
                sides.push_str(format!("{}: costs ${:.2},", item.as_str(), price).as_str());
            }
            sides.replace_range((sides.len() - 1).., "")
        }

        write!(
            f,
            "food: {{{food}}}, drinks: {{{drinks}}}, sides: {{{sides}}}"
        )
    }
}
