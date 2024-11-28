use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Eq, Clone)]
pub enum TransactionType {
    Bought,
    Sold,
}

impl From<String> for TransactionType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Bought" => TransactionType::Bought,
            "Sold" => TransactionType::Sold,
            _ => panic!("Invalid value for TransactionType"),
        }
    }
}
