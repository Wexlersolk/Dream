use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct InvalidOptionError;

impl fmt::Display for InvalidOptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid meditation option provided")
    }
}

impl Error for InvalidOptionError {}

pub enum Meditations {
    Remember,
    Recall,
    Create,
}

pub fn parse_meditation(input: &str) -> Result<Meditations, Box<dyn Error>> {
    match input {
        "Remember" => Ok(Meditations::Remember),
        "Recall" => Ok(Meditations::Recall),
        "Create" => Ok(Meditations::Create),
        _ => Err(Box::new(InvalidOptionError)),
    }
}
