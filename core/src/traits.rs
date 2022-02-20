//!
//! This module contains traits related for common operations of the project
//!
//! Author: Omar Aouini
//!
//! 20/02/2022
//!


/// Add Database crud operations functionality to specified types
/// it takes as generic param the type to work with (T) and the Connection pool type (P)
///
/// # Examples
///
/// ```
/// impl Crud<Company, MySqlPool> for Company {
///    fn find(pool: MySqlPool, id: &i32) -> Result<Company, String> {
///       "...query logic..."
///    }
///
/// ```
pub trait Crud<T, P> {
    fn find(pool:&P,id :&i32) -> Result<T, String>;
    fn findAll(pool:&P) -> Result<Vec<T>, String>;
    fn add(pool:&P, elem :&T)  -> Result<bool, String>;
    fn delete(pool:&P, id :&i32)  -> Result<bool, String>;
    fn update(pool:&P, elem :&T)  -> Result<bool, String>;
}