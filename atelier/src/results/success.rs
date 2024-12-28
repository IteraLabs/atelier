//! Custom Results responses as enums with their corresponding variants.

pub enum LevelSuccess {
    LevelCreated,
    LevelDeleted,
    LevelModified,
}

pub enum OrderSuccess {
    OrderCreated,
    OrderDeleted,
    OrderModified,
    OrderMatched,
}

