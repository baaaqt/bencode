#[cfg(test)]
mod parse_tests {
    use std::collections::HashMap;

    use crate::parser::Parser;
    use crate::value::BencodeValue;

    #[test]
    fn test_int_parsing() {
        let data = b"i42e";
        let mut parser = Parser::new(data.to_vec());
        let result = parser.parse();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BencodeValue::Integer(42));
    }

    #[test]
    fn test_string_parsing() {
        let data = b"4:spam";
        let mut parser = Parser::new(data.to_vec());
        let result = parser.parse();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BencodeValue::ByteString(b"spam".to_vec()));
    }

    #[test]
    fn test_empty_list_parsing() {
        let data = b"le";
        let mut parser = Parser::new(data.to_vec());
        let result = parser.parse();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BencodeValue::List(vec![]));
    }

    #[test]
    fn test_list_parsing() {
        let data = b"l4:spam4:eggsi42e4:spame";
        let mut parser = Parser::new(data.to_vec());
        let result = parser.parse();
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            BencodeValue::List(vec![
                BencodeValue::ByteString(b"spam".to_vec()),
                BencodeValue::ByteString(b"eggs".to_vec()),
                BencodeValue::Integer(42),
                BencodeValue::ByteString(b"spam".to_vec())
            ])
        );
    }

    #[test]
    fn test_int_list_parsing() {
        let data = b"li42ei43ee";
        let mut parser = Parser::new(data.to_vec());
        let result = parser.parse();
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            BencodeValue::List(vec![BencodeValue::Integer(42), BencodeValue::Integer(43)])
        );
    }

    #[test]
    fn test_dict_parsing() {
        let data = b"d4:spami42ee";
        let mut parser = Parser::new(data.to_vec());
        let result = parser.parse();
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            BencodeValue::Dict(HashMap::from([(
                b"spam".to_vec(),
                BencodeValue::Integer(42)
            )]))
        );
    }

    #[test]
    fn test_nested_list_parsing() {
        let data = b"ll4:spamei99ee";
        let mut parser = Parser::new(data.to_vec());
        let result = parser.parse();
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            BencodeValue::List(vec![
                BencodeValue::List(vec![BencodeValue::ByteString(b"spam".to_vec())]),
                BencodeValue::Integer(99)
            ])
        );
    }

    #[test]
    fn test_empty_dict_parsing() {
        let data = b"de";
        let mut parser = Parser::new(data.to_vec());
        let result = parser.parse();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BencodeValue::Dict(HashMap::new()));
    }

    #[test]
    fn test_dict_with_multiple_entries() {
        let data = b"d3:foo3:bar3:bazi123ee";
        let mut parser = Parser::new(data.to_vec());
        let result = parser.parse();
        assert_eq!(
            result.unwrap(),
            BencodeValue::Dict(HashMap::from([
                (b"foo".to_vec(), BencodeValue::ByteString(b"bar".to_vec())),
                (b"baz".to_vec(), BencodeValue::Integer(123))
            ]))
        );
    }

    #[test]
    fn test_dict_with_list_value() {
        let data = b"d4:listl4:spami42eee";
        let mut parser = Parser::new(data.to_vec());
        let result = parser.parse();
        assert_eq!(
            result.unwrap(),
            BencodeValue::Dict(HashMap::from([(
                b"list".to_vec(),
                BencodeValue::List(vec![BencodeValue::ByteString(b"spam".to_vec()), BencodeValue::Integer(42)])
            )]))
        );
    }

    #[test]
    fn test_invalid_bencode_returns_err() {
        let data = b"i42"; // missing 'e' at the end
        let mut parser = Parser::new(data.to_vec());
        let result = parser.parse();
        assert!(result.is_err());
    }

    #[test]
    fn test_zero_length_string() {
        let data = b"0:";
        let mut parser = Parser::new(data.to_vec());
        let result = parser.parse();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), BencodeValue::ByteString(vec![]));
    }

    #[test]
    fn test_dict_with_empty_string_key() {
        let data = b"d0:3:fooee";
        let mut parser = Parser::new(data.to_vec());
        let result = parser.parse();
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            BencodeValue::Dict(HashMap::from([(
                vec![],
                BencodeValue::ByteString(b"foo".to_vec())
            )]))
        );
    }
}
