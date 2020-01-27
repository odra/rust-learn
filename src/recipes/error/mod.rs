use serde_derive::*;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Transaction {
    from: String,
    to: String,
    amount: u64
}

#[derive(Debug)]
enum TransactionError {
    LoadError(std::io::Error),
    ParseError(serde_json::Error),
    Mess(&'static str) //avoids weird panic error if any
}

impl From<std::io::Error> for TransactionError {
    fn from(e: std::io::Error) -> Self {
        TransactionError::LoadError(e)
    }
}

impl From<serde_json::Error> for TransactionError {
    fn from(e: serde_json::Error) -> Self {
        TransactionError::ParseError(e)
    }   
}

impl From<&'static str> for TransactionError {
    fn from(e: &'static str) -> Self {
        TransactionError::Mess(e)
    }
} 

fn get_transactions_match(path: &str) -> Result<Vec<Transaction>, String> {
    let s = match std::fs::read_to_string(path) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string())
    };
    let t: Vec<Transaction> = match serde_json::from_str(&s) {
        Ok(v) => v,
        Err(e) => return Err(e.to_string())
    };

    Ok(t)
}

fn get_transactions_if(path: &str) -> Result<Vec<Transaction>, String> {
    let res = std::fs::read_to_string(path);
    if res.is_err() {
        return Err(res.unwrap_err().to_string());
    }

    let s = res.unwrap();
    let res = serde_json::from_str(&s);
    if res.is_err() {
        return Err(res.unwrap_err().to_string());
    }

    Ok(res.unwrap())
}

fn get_transactions_map(path: &str) -> Result<Vec<Transaction>, String> {
    std::fs::read_to_string(path)
        .map_err(|e| e.to_string())
        .and_then(|ld| serde_json::from_str(&ld).map_err(|e| e.to_string()))
}

fn get_transactions_custom_error(path: &str) -> Result<Vec<Transaction>, TransactionError> {
    std::fs::read_to_string(path)
        .map_err(|e| e.into())
        .and_then(|ld| serde_json::from_str(&ld).map_err(|e| e.into()))
}

fn get_transactions_one_liner(path: &str) -> Result<Vec<Transaction>, TransactionError> {
    Ok(serde_json::from_str(&std::fs::read_to_string(path)?)?)
}

fn get_transaction_from(path: &str, name: &str) -> Option<Transaction> {
    let transactions = get_transactions_one_liner(path).ok()?;
    
    for transaction in transactions {
        if transaction.from == name {
            return Some(transaction)
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_transactions_success() {
        let transactions = get_transactions_match("test_data/transactions.json").unwrap();
        assert_eq!(2, transactions.len());

        let transactions = get_transactions_if("test_data/transactions.json").unwrap();
        assert_eq!(2, transactions.len());

        let transactions = get_transactions_map("test_data/transactions.json").unwrap();
        assert_eq!(2, transactions.len());

        let transactions = get_transactions_one_liner("test_data/transactions.json").unwrap();
        assert_eq!(2, transactions.len());
    }

    #[test]
    fn test_get_transactions_error() {
        let err = get_transactions_match("test_data/transactions.404.json").unwrap_err();
        assert_eq!("No such file or directory (os error 2)", err);

        let err = get_transactions_if("test_data/transactions.404.json").unwrap_err();
        assert_eq!("No such file or directory (os error 2)", err);

        let err = get_transactions_map("test_data/transactions.404.json").unwrap_err();
        assert_eq!("No such file or directory (os error 2)", err);
    }

    #[test]
    fn test_get_transactions_error_custom() {
        let err = get_transactions_custom_error("test_data/transactions.404.json").unwrap_err();
        assert_eq!("LoadError(Os { code: 2, kind: NotFound, message: \"No such file or directory\" })", format!("{:?}", err));

        let err = get_transactions_custom_error("test_data/transactions.invalid.json").unwrap_err();
        assert_eq!("ParseError(Error(\"expected ident\", line: 9, column: 16))", format!("{:?}", err));

        let err = get_transactions_one_liner("test_data/transactions.404.json").unwrap_err();
        assert_eq!("LoadError(Os { code: 2, kind: NotFound, message: \"No such file or directory\" })", format!("{:?}", err));

        let err = get_transactions_one_liner("test_data/transactions.invalid.json").unwrap_err();
        assert_eq!("ParseError(Error(\"expected ident\", line: 9, column: 16))", format!("{:?}", err));
    }

    #[test]
    fn test_get_transaction_from() {
        let transaction = get_transaction_from("test_data/transactions.json", "from1").unwrap();
        assert_eq!("from1", transaction.from);

        let transaction = get_transaction_from("test_data/transactions.json", "from10");
        assert_eq!(None, transaction);

        let transaction = get_transaction_from("test_data/transactions.404.json", "from1");
        assert_eq!(None, transaction);
    }
}