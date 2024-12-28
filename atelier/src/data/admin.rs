/// Admin types and implementations

pub struct User {
    pub user_id: u32,
    pub since_ts: u128,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

pub struct Symbol {
    pub symbol_id: u32,
    quote_asset: String,
    base_asset: String,
}
