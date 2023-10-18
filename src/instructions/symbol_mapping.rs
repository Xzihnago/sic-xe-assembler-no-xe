use std::fmt;

pub struct SymbolMapping {
    pub symbol: String,
    pub addr: usize,
}

impl fmt::Display for SymbolMapping {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\t{:04X}", self.symbol, self.addr)
    }
}

impl SymbolMapping {
    pub fn new(symbol: impl Into<String>, addr: usize) -> Self {
        Self {
            symbol: symbol.into(),
            addr,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_mapping() {
        let mapping = SymbolMapping::new("LOOP", 0);
        assert_eq!(format!("{}", mapping), "LOOP\t0000");
    }
}
