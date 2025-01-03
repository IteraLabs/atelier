use crate::data::admin::User;
/// Hashing generating functions
///
/// in order to create order_id, and, user_id hashed values with
/// unicity and atomicity operative properties.
use crate::data::market::Order;
use std::hash::{DefaultHasher, Hash, Hasher};

// ---------------------------------------------------------------- USER ID HASHING -- //
// ---------------------------------------------------------------- --------------- -- //

impl Hash for User {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.email.hash(state);
    }
}

fn hash_user(user: &User) -> u64 {
    let mut hasher = DefaultHasher::new();
    user.hash(&mut hasher);
    hasher.finish()
}

// --------------------------------------------------------------- ORDER ID HASHING -- //
// --------------------------------------------------------------- ---------------- -- //

impl Hash for Order {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.order_id.hash(state);
    }
}

fn hash_order(order: &Order) -> u64 {
    let mut hasher = DefaultHasher::new();
    order.hash(&mut hasher);
    hasher.finish()
}
