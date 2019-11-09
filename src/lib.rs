use std::fmt;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Argument { description: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(ref err) => write!(f, "IO error: {}", err),
            Error::Argument { description } => write!(f, "Argument error: {}", description),
        }
    }
}

pub struct Changeme {
    pub foo: String,
}

impl Changeme {
    pub fn new(arg: &String) -> Changeme {
        Changeme {
            foo: arg.to_string(),
        }
    }

    pub fn list(&self, arg: &str) -> Result<(), Error> {
        Ok(())
    }

    pub fn get(&self) -> Result<(), Error> {
        Ok(())
    }
}
