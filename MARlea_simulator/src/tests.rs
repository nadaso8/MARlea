#[cfg(test)]
mod test {
    use super::super::*;
    use std::str::FromStr;
    #[test]
    fn test_query_from_string() {
        assert_eq!(Query::from_str("simulate").unwrap(), Query::Simulate);
        assert_eq!(Query::from_str("help").unwrap(), Query::Help);
        assert!(Query::from_str("invalid").is_err());
    }
}
