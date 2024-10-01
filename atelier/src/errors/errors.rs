//! Custom Error Types 
//! Provides the definition of error types that are custom made for the atelier project.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LevelError {

    // Level not found
    #[error("Level not found")]
    LevelNotFound,

    // Level info not available
    #[error("Level info not available")]
    LevelInfoNotAvailable,
    
    // Level deletion not successful
    #[error("Level deletion not successful")]
    LevelDeletionFailed,

    // Level modification not succesful
    #[error("Level modification not successful")]
    LevelModificationFailed,
    
    // Level insertion not successful
    #[error("Level insertion not successful")]
    LevelInsertionFailed,
}

#[derive(Error, Debug)]
pub enum OrderError {

    // Order not found
    #[error("Order not found")]
    OrderNotFound,
    
    // Order info not available
    #[error("Order info not available")]
    OrderInfoNotAvailable,
    
    // Order deletion not successful
    #[error("Order deletion not successful")]
    OrderDeletionFailed,
    
    // Order modification not successful
    #[error("Order modification not successful")]
    OrderModificationFailed,
    
    // Order insertion not succesful
    #[error("Order insertion not successful")]
    OrderInsertionFailed,

}

#[derive(Error, Debug)]
pub enum GeneratorError {
    
    // Generator not found
    #[error("Generator not found")]
    GeneratorNotFound,

    // Value not supported
    #[error("The provided value is not supported by the generator")]
    GeneratorValueError,
}

