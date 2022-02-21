//!
//! This module contains logics and types related to companies
//!
//! Author: Omar Aouini
//!
//! 20/02/2022
//!

use log::debug;
use serde::{Serialize, Deserialize};
use sqlx::MySqlPool;
use crate::traits::Crud;

#[derive(Serialize, Deserialize, Debug)]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub vat_code: String,
    pub address: String,
    pub email: String,
    pub phone_number: String,
    pub projects : Vec<String>,
}

impl Crud<Company, MySqlPool> for Company {
    fn find(pool: &MySqlPool, id: &i32) -> Result<Company, String> {
        debug!("SEARCH WITH ID {:?}", id);
        Ok(Company{
            id: *id,
            name: "paolino".to_string(),
            vat_code: "".to_string(),
            address: "".to_string(),
            email: "".to_string(),
            phone_number: "".to_string(),
            projects: vec![]
        })
    }

    fn find_all(pool: &MySqlPool) -> Result<Vec<Company>, String> {
        debug!("FIND ALL");
        Ok(vec![])
    }

    fn add(pool: &MySqlPool, elem: &Company) -> Result<bool, String> {
        debug!("ADD COMPANY: {:#?}", elem);
        return Err("errore nell'add".to_string())
    }

    fn delete(pool: &MySqlPool, id: &i32) -> Result<bool, String> {
        debug!("DELETE WITH ID {:?}", id);
        return Ok(true)
    }

    fn update(pool: &MySqlPool, elem: &Company) -> Result<bool, String> {
        debug!("UPDATE COMPANY: {:#?}", elem);
        return Err("errore nell'update".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(1,1)
    }

    #[test]
    fn test_find_all() {
        assert_eq!(1,1)
    }
}