use std::collections::HashMap;

use pyo3::IntoPyObject;

#[derive(Debug, PartialEq, Eq, IntoPyObject)]
pub enum BencodeValue {
    Integer(i64),
    ByteString(Vec<u8>),
    List(Vec<BencodeValue>),
    Dict(HashMap<Vec<u8>, BencodeValue>),
}
