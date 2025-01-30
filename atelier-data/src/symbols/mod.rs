
#[derive(Debug, Clone)]
pub struct Symbol {
    pub symbol_id: Option<String>,
    pub base: Option<String>,
    pub quote: Option<String>,
    pub decimals: Option<u32>,
}

impl Symbol {
    pub fn new() -> Symbol {
        Symbol {
            symbol_id: None,
            base: None,
            quote: None,
            decimals: None,
        }
    }

    pub fn symbol_id(mut self, symbol_id: String) -> Self {
        self.symbol_id = Some(symbol_id);
        self
    }

    pub fn base(mut self, base: String) -> Self {
        self.base = Some(base);
        self
    }

    pub fn quote(mut self, quote: String) -> Self {
        self.quote = Some(quote);
        self
    }

    pub fn decimals(mut self, decimals: u32) -> Self {
        self.decimals = Some(decimals);
        self
    }

    pub fn build(self) -> Symbol {
        self
    }
}

