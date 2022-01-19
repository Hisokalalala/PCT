use std::vec::Vec;
use primitive::{ QueryId, EncodedValue, HashValue, AdditionalValue };
use constant::*;

/* Type EncodedQueryRep */
#[derive(Clone, Default, Debug)]
pub struct EncodedQueryRep {
    pub id: QueryId,
    pub parameters: Vec<HashValue>,
    pub addi_parameters: Vec<AdditionalValue>,
}

impl EncodedQueryRep {
    pub fn new() -> Self {
        EncodedQueryRep {
            id: 0,
            parameters: Vec::with_capacity(QUERY_SIZE),
            addi_parameters: Vec::with_capacity(QUERY_SIZE),
        }
    }
}