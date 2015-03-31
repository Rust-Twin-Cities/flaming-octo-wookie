#![feature(convert)]

use std::convert::AsRef;

#[derive(PartialEq, Eq, Debug)]
pub struct RibonucleicAcid {
    // ...
}

impl RibonucleicAcid {
    pub fn new(nucleotides: &str) -> RibonucleicAcid {
        // ...
    }
}

impl AsRef<str> for RibonucleicAcid {
    fn as_ref(&self) -> &str {
        // ...
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct DeoxyribonucleicAcid {
        // ...
}


impl DeoxyribonucleicAcid {
    pub fn new(nucleotides: &str) -> DeoxyribonucleicAcid {
        // ...
    }

    pub fn to_rna(&self) -> RibonucleicAcid {
        // ...
    }
}
