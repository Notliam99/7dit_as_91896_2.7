use core::fmt;
use std::fmt::Display;

pub struct Items {
    pub food: Vec<String>,
    pub drinks: Vec<String>,
    pub sides: Vec<String>,
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
    ///     use dit_as_91896::menu::Items;
    ///
    ///     let food: Vec<String> = vec![String::from("0")];
    ///     let drinks: Vec<String> = vec![String::from("1")];
    ///     let sides: Vec<String> = vec![String::from("2")];
    ///
    ///     let items: Items = Items::new(food.clone(), drinks.clone(), sides.clone());
    ///
    ///     assert!(items.food == food);
    ///     assert!(items.drinks == drinks );
    ///     assert!(items.sides == sides);
    /// ```
    pub fn new(food: Vec<String>, drinks: Vec<String>, sides: Vec<String>) -> Self {
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
    ///     use dit_as_91896::menu::Items;
    ///
    ///     let items: Items = Items {
    ///         food: vec![String::from("0")],
    ///         drinks: vec![String::from("1")],
    ///         sides: vec![String::from("2")],
    ///     };
    ///
    ///     assert!(format!("{items}").as_str() == "Item: (food: [0], drinks: [1], sides: [2])");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output: String = String::from("Item: (food: [");

        if self.food.len() != 0usize {
            for i in &self.food {
                output.push_str(format!("{}, ", i.as_str()).as_str());
            }
            output.replace_range((output.len() - 2).., "")
        }
        output.push_str("], drinks: [");

        if self.drinks.len() != 0usize {
            for i in &self.drinks {
                output.push_str(format!("{}, ", i.as_str()).as_str())
            }
            output.replace_range((output.len() - 2).., "")
        }
        output.push_str("], sides: [");

        if self.drinks.len() != 0usize {
            for i in &self.sides {
                output.push_str(format!("{}, ", i.as_str()).as_str());
            }
            output.replace_range((output.len() - 2).., "")
        }
        output.push_str("])");

        write!(f, "{output}")
    }
}
