//!
//! This module contains logics and types related to companies
//!
//! Author: Omar Aouini
//!
//! 20/02/2022
//!

use std::fmt::{Display, Formatter};
use log::debug;
use serde::{Serialize, Deserialize};
use sqlx::{Error, MySql, MySqlPool, query_as};
use sqlx::mysql::MySqlQueryResult;

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub vat_code: String,
    pub address: String,
    pub email: String,
    pub phone_number: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCompanyDTO {
    pub name: String,
    pub vat_code: String,
    pub address: String,
    pub email: String,
    pub phone_number: String
}

impl From<CreateCompanyDTO> for Company {
    fn from(c: CreateCompanyDTO) -> Self {
        Company{
            id: 0,
            name: c.name,
            vat_code: c.vat_code,
            address: c.address,
            email: c.email,
            phone_number: c.phone_number
        }
    }
}

impl Company {
    pub async fn find(pool: &MySqlPool, id: &i32) -> Result<Company, String> {
        let res :Result<Company, Error> =
            sqlx::query_as::<_, Company>("SELECT * FROM companies WHERE id = ?")
                .bind(id)
                .fetch_one(pool).await;
        res.map_err(|err| err.into_database_error().unwrap().to_string())
    }
    pub async fn find_all(pool: &MySqlPool) -> Result<Vec<Company>, String> {
        let res :Result<Vec<Company>, Error> =
            sqlx::query_as::<_, Company>("SELECT * FROM companies").fetch_all(pool).await;
        res.map_err(|err| err.into_database_error().unwrap().to_string())
    }
    pub async fn add(pool: &MySqlPool, company : &CreateCompanyDTO) -> Result<MySqlQueryResult, String> {
        let query = format!(
            "INSERT INTO companies(name, vat_code, address, email, phone_number) VALUES ({:?}, {:?}, {:?}, {:?}, {:?})",
            company.name, company.vat_code, company.address, company.email, company.phone_number);
        let res = sqlx::query(&query).execute(pool).await;
        res.map_err(|err| err.into_database_error().unwrap().to_string())
    }
    pub async fn update(pool: &MySqlPool, company : &Company) -> Result<u64, String> {
        let query = format!(
            "UPDATE companies SET name = {:?}, vat_code = {:?}, address = {:?}, email = {:?}, phone_number = {:?} WHERE id = {:?}",
            company.name, company.vat_code, company.address, company.email, company.phone_number, company.id);
        let res = sqlx::query(&query).execute(pool).await;
        res.map(|result| result.rows_affected()).map_err(|err| err.into_database_error().unwrap().to_string())
    }
    pub async fn delete(pool: &MySqlPool, id : &i32) -> Result<u64, String> {
        let res =
            sqlx::query("DELETE FROM companies WHERE id = ?").bind(id).execute(pool).await;
        res.map(|result| result.rows_affected() as u64).map_err(|err| err.into_database_error().unwrap().to_string())
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