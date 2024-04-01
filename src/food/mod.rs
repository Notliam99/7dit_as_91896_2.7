use core::fmt;
use std::{collections::HashMap, fmt::Display};

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
        let mut output: String = String::from("food: {\n");

        if self.food.keys().len() != 0usize {
            for (item, price) in &self.food {
                output.push_str(format!("    {}: costs ${:.2},\n", item.as_str(), price).as_str());
            }
            output.replace_range((output.len() - 2).., "\n")
        }
        output.push_str("},\ndrinks: {\n");

        if self.drinks.keys().len() != 0usize {
            for (item, price) in &self.drinks {
                output.push_str(format!("    {}: costs ${:.2},\n", item.as_str(), price).as_str())
            }
            output.replace_range((output.len() - 2).., "\n")
        }
        output.push_str("},\nsides: {\n");

        if self.drinks.keys().len() != 0usize {
            for (item, price) in &self.sides {
                output.push_str(format!("    {}: costs ${:.2},\n", item.as_str(), price).as_str());
            }
            output.replace_range((output.len() - 2).., "\n")
        }
        output.push_str("},");

        write!(f, "{output}")
    }
}
