#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(deprecated)]
use csv::{ReaderBuilder, WriterBuilder};
use std::error::Error;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, NaiveDateTime};

use serde::de::{self, Deserializer};
use odbc_api::Connection;

use std::fmt;

use rust_decimal::Decimal;
use anyhow::anyhow;
use odbc_api::{buffers::TextRowSet, Cursor, Environment, ConnectionOptions, ResultSetMetadata};
use std::{
    ffi::CStr,
    io::{stdout, Write},
    path::PathBuf,
};

#[derive(Debug)]
struct FetchError(&'static str);

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


// / Maximum number of rows fetched with one row set. Fetching batches of rows is usually much
// / faster than fetching individual rows.
const BATCH_SIZE: usize = 5000;

impl std::error::Error for FetchError {}

use std::convert::TryFrom;


fn deserialize_bool_from_0_1<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize as an integer (0 or 1)
    let value: u8 = u8::deserialize(deserializer)?;

    // Convert 0 to false, and 1 to true
    match value {
        1 => Ok(true),
        0 => Ok(false),
        _ => Err(de::Error::custom("Invalid boolean value, expected 0 or 1")),
    }
}
fn deserialize_optional_datetime<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(date_str) => {
            // Try to parse the date string; if it's empty, return None
            if date_str.is_empty() {
                Ok(None)
            } else {
                let out =
                    NaiveDateTime::parse_from_str(&date_str, "%m/%d/%Y %H:%M:%S")
                        .map(Some)
                        .map_err(de::Error::custom);
                out
            }
        }
        None => Ok(None), // If the field is missing, return None
    }
}

include!("Util.rs");
include!("hytek/tm/Athlete.rs");
include!("hytek/tm/Rresult.rs");


fn main() -> Result<(), Box<dyn Error>> {
    let db :String = "C:\\Users\\john\\sandbox\\mdb\\fssl\\result.csv".to_string();
    let env = Environment::new()?;

    // Define the connection string
    let driver = "Driver={Microsoft Access Driver (*.mdb, *.accdb)}";
    let db = "DBQ=C:\\Users\\john\\sandbox\\mdb\\fssl\\2023.mdb";
    let password = "Uid=;Pwd=5hY-tek;";

    let connection_string = [driver, db, password].join(";");

    // Establish the connection
    let conn = env.connect_with_connection_string(&*connection_string, ConnectionOptions::default())?;




/*
    let results  = RResult::from_mdb(conn);
    match results {
        Ok(results) => {
            println!("{:?}", results.len());
        },
        Err(e) => {
            eprintln!("Error reading CSV: {}", e);
        }
    }


    let db :String = "C:\\Users\\john\\sandbox\\mdb\\fssl\\athlete.csv".to_string();

    match Athlete::all(db) {
        Ok(results) => {
               println!("{}", results.len());
        },
        Err(e) => {
            eprintln!("Error reading CSV: {}", e);
        }

    }

*/

    Ok(())
}
