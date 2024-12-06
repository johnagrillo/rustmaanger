#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(deprecated)]
use std::fmt;

// include!("db/mmAgegroup.rs");
use chrono::NaiveDateTime;
use anyhow::Error;
use anyhow::anyhow;
use odbc_api::{buffers::TextRowSet, Cursor, Environment, ConnectionOptions, ResultSetMetadata};
use odbc_api::Connection;
use std::{
    ffi::CStr,
    io::{stdout, Write},
    path::PathBuf,
};

/// Maximum number of rows fetched with one row set. Fetching batches of rows is usually much
/// faster than fetching individual rows.
const BATCH_SIZE: usize = 5000;

#[derive(Debug)]
struct Person {
    name: Vec<u8>,
    age1: u32,
    age2: u64,
    email: Vec<u8>,
}

#[derive(Debug)]
struct FetchError(&'static str);

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for FetchError {}

use std::convert::TryFrom;

fn fetch_u16(bytes: &[u8]) -> Result<u16, FetchError> {
    if bytes.is_empty() {
        return Ok(0); // Handle NULL case
    }

    if bytes.len() < 2 {
        return Err(FetchError("Insufficient bytes for conversion"));
    }

    // Convert the first 2 bytes to a u16
    let mut arr = [0u8; 2];
    arr.copy_from_slice(&bytes[0..2]);

    // Convert the array to u16
    let result = u16::from_ne_bytes(arr); // Use native endianness
    Ok(result)
}
fn fetch_u32(bytes: &[u8]) -> Result<u32, FetchError> {
    if bytes.is_empty() {
        return Ok(0); // Handle NULL case
    }

    if bytes.len() < 4 {
        return Err(FetchError("Insufficient bytes for conversion"));
    }

    // Convert the first 4 bytes to a u32
    let mut arr = [0u8; 4];
    arr.copy_from_slice(&bytes[0..4]);

    // Convert the array to u32
    let result = u32::from_ne_bytes(arr); // Use native endianness
    Ok(result)
}



fn fetch_u64(bytes: &[u8]) -> Result<u64, FetchError> {
    if bytes.is_empty() {
        return Ok(0); // Handle NULL case
    }

    if bytes.len() < 8 {
        return Err(FetchError("Insufficient bytes for conversion"));
    }

    // Convert the first 8 bytes to a u64
    let mut arr = [0u8; 8];
    arr.copy_from_slice(&bytes[0..8]);

    // Convert the array to u64
    let result = u64::from_ne_bytes(arr); // Use native endianness
    Ok(result)
}



fn fetch_f64(bytes: &[u8]) -> Result<f64, FetchError> {
    if bytes.is_empty() {
        return Ok(0.0); // Handle NULL case
    }

    if bytes.len() < 8 {
        return Err(FetchError("Insufficient bytes for conversion"));
    }

    // Convert the first 8 bytes to a f64
    let mut arr = [0u8; 8];
    arr.copy_from_slice(&bytes[0..8]);

    // Convert the array to f64
    let result = f64::from_ne_bytes(arr); // Use native endianness
    Ok(result)
}


fn fetch_f32(bytes: &[u8]) -> Result<f32, FetchError> {
    if bytes.is_empty() {
        return Ok(0.0); // Handle NULL case
    }

    if bytes.len() < 4 {
        return Err(FetchError("Insufficient bytes for conversion"));
    }

    // Convert the first 4 bytes to a f32
    let mut arr = [0u8; 4];
    arr.copy_from_slice(&bytes[0..4]);

    // Convert the array to f32
    let result = f32::from_ne_bytes(arr); // Use native endianness
    Ok(result)
}


fn fetch_datetime(bytes: &[u8]) -> Result<NaiveDateTime, FetchError> {
    if bytes.is_empty() {
        return Err(FetchError("Received empty bytes for datetime"));
    }

    if bytes.len() < 8 {
        return Err(FetchError("Insufficient bytes for conversion"));
    }

    // Convert the bytes to an i64 (timestamp in milliseconds)
    let millis = i64::from_ne_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5], bytes[6], bytes[7],
    ]);

    // Create a NaiveDateTime from the timestamp in milliseconds
    let datetime = NaiveDateTime::from_timestamp_millis(millis)
        .ok_or(FetchError("Invalid timestamp"))?; // Handle possible invalid timestamps

    Ok(datetime)
}



include!("db/tmSetDescriptions.rs");


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Write csv to standard out
    let out = stdout();
    let mut writer = csv::Writer::from_writer(out);

    // If you do not do anything fancy it is recommended to have only one Environment in the
    // entire process.
    let environment = Environment::new()?;

    // Connect using a DSN. Alternatively we could have used a connection string
    let connection : Connection= environment.connect(
        "DataSourceName",
        "Username",
        "Password",
        ConnectionOptions::default(),
    )?;
    // Create a vector to hold the Person objects
    let mut persons: Vec<Person> = Vec::new();

    
    // Execute a one of query without any parameters.
    match connection.execute("SELECT * FROM TableName", ())? {
        Some(mut cursor) => {
            // Write the column names to stdout
            let headline : Vec<String> = cursor.column_names()?.collect::<Result<_,_>>()?;
            writer.write_record(headline)?;

            // Use schema in cursor to initialize a text buffer large enough to hold the largest
            // possible strings for each column up to an upper limit of 4KiB.
            let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
            // Bind the buffer to the cursor. It is now being filled with every call to fetch.
            let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;

            // Iterate over batches
            while let Ok(Some(batch)) = row_set_cursor.fetch() {
		//let name: Vec<u8> = row_set_cursor.get(1)?; 
                // Within a batch, iterate over every row
                for row_index in 0..batch.num_rows() {

		    let name = batch.at(0, row_index).unwrap_or(&[]).to_vec(); // Fetch name

		    // Fetch age as a &[u8]

		    let age_bytes = batch.at(1, row_index).unwrap_or(&[]);
		    let age1: u32 = fetch_u32(age_bytes)?;

		    let age2: u64 = 10;
		    
                    let email = batch.at(3, row_index).unwrap_or(&[]).to_vec(); // Fetch email

		    let person = Person {
			
			name: name,   // Assuming first column is name
			age1: age1,    // Assuming second column is age
			age2: age2,    // Assuming second column is age
			email: email,  // Assuming third column is email
		    };
		    persons.push(person); 
		    
                }
            }
        }
        None => {
            eprintln!(
                "Query came back empty. No output has been created."
            );
        }
    }

    Ok(())
}



