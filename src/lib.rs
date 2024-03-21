use core::fmt;
use std::fmt::Display;

pub struct Items {
    pub food: Vec<String>,
    pub drinks: Vec<String>,
    pub sides: Vec<String>,
}

impl Items {
    /// # New instance function
    /// @prams: None
    /// @return: instance of type
    pub fn new(food: Vec<String>, drinks: Vec<String>, sides: Vec<String>) -> Self {
        Self {
            food,
            drinks,
            sides,
        }
    }
}

impl Display for Items {
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
