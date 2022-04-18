use std::fmt::{Display, Formatter, Result};

#[derive(Clone, PartialEq, Debug)]
pub struct Crypto {
    pub id: String,
    pub precision: usize,
}

impl Display for Crypto {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        fmt.write_str(self.id.as_str())?;
        Ok(())
    }
}
