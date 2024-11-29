use crate::data::admin;
/// Hashing generating functions
///
/// in order to create order_id, and, user_id hashed values with
/// unicity and atomicity operative properties.
use crate::data::market::Order;
use std::hash::{DefaultHasher, Hash, Hasher};

impl Hash for admin::User {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.first_name.hash(state);
        self.last_name.hash(state);
        self.email.hash(state);
    }
}

fn hash_user(user: &admin::User) -> u64 {
    let mut hasher = DefaultHasher::new();
    user.hash(&mut hasher);
    hasher.finish()
}
