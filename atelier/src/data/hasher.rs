use crate::data::admin::User;
/// Hashing generating functions
///
/// in order to create order_id, and, user_id hashed values with
/// unicity and atomicity operative properties.
use crate::data::market::Order;
use std::hash::{DefaultHasher, Hash, Hasher};

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
