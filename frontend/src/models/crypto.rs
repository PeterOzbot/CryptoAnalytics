use std::fmt::{Display, Formatter, Result};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Crypto {
    pub api_key: String,
    pub precision: i16,
}

impl Display for Crypto {
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        fmt.write_str(self.api_key.as_str())?;
        Ok(())
    }
}
