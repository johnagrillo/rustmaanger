#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#![allow(deprecated)]
use std::fmt;

use rust_decimal::Decimal;
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

include!("util.rs");
include!("db/tm/AthInfo.rs");
include!("db/tm/Athlete.rs");
include!("db/tm/ATHRECR.rs");
include!("db/tm/Attendance.rs");
include!("db/tm/COACHES.rs");
include!("db/tm/CODE.rs");
include!("db/tm/CustomLayout.rs");
include!("db/tm/CustomLayoutFields.rs");
include!("db/tm/CustomLayoutValues.rs");
include!("db/tm/CUSTOMRPTS.rs");
include!("db/tm/DELETEENTRY.rs");
include!("db/tm/Energy.rs");
include!("db/tm/ESPLITS.rs");
include!("db/tm/FAVORITES.rs");
include!("db/tm/HYTEKAGEGROUP.rs");
include!("db/tm/JOURNAL.rs");
include!("db/tm/MemCirName.rs");
include!("db/tm/MemCirSets.rs");
include!("db/tm/MemSets.rs");
include!("db/tm/Model.rs");
include!("db/tm/ModelParam.rs");
include!("db/tm/MSDecline.rs");
include!("db/tm/MTEVENT.rs");
include!("db/tm/MTEVENTE.rs");
include!("db/tm/OMEINVITE.rs");
include!("db/tm/OMEOPTIONS.rs");
include!("db/tm/OPTIONS.rs");
include!("db/tm/RECNAME.rs");
include!("db/tm/RECORDS.rs");
include!("db/tm/RELAY.rs");
include!("db/tm/RESULT.rs");
include!("db/tm/SESSIONS.rs");
include!("db/tm/SPLITS.rs");
include!("db/tm/STDNAME.rs");
include!("db/tm/StrokeCategory.rs");
include!("db/tm/TEAM.rs");
include!("db/tm/TestData.rs");
include!("db/tm/TestLine.rs");
include!("db/tm/TestSet.rs");
include!("db/tm/TestT30.rs");
include!("db/tm/WkParam.rs");
include!("db/tm/WorkCategory.rs");
include!("db/tm/Workout.rs");
include!("db/tm/CUSTOMAGEGROUP.rs");
include!("db/tm/ENTRY.rs");
include!("db/tm/MEET.rs");
include!("db/tm/Modt30times.rs");
include!("db/tm/PREENTER.rs");
include!("db/tm/SetDescriptions.rs");

use csv::WriterBuilder;
use serde::Serialize;
use serde_derive::Serialize;

#[derive(Serialize)]
struct Person {
    name: String,
    age: u32,
    city: String,
}

fn write_csv() -> Result {
    let people = vec![
        Person {
            name: "John".to_string(),
            age: 30,
            city: "New York".to_string(),
        },
        Person {
            name: "Alice".to_string(),
            age: 25,
            city: "Los Angeles".to_string(),
        },
    ];

    // Create a CSV writer
    let mut wtr = WriterBuilder::new().from_path("people.csv")?;

    // Write the CSV headers (it will use the struct field names)
    wtr.write_record(&["name", "age", "city"])?;

    // Serialize each person into the CSV format
    for person in people {
        wtr.serialize(person)?; // This serializes the struct into a CSV row
    }

    wtr.flush()?; // Ensure all data is written to the file
    Ok(())
}

fn main() -> std::result::Result<(), Error> {
    write_csv();
    Ok(())
}




fn odbc() {

    // If you do not do anything fancy it is recommended to have only one Environment in the
    // entire process.
    let env = Environment::new()?;

    // Define the connection string
    let driver = "Driver={Microsoft Access Driver (*.mdb, *.accdb)}";
    let db = "DBQ=C:\\Users\\john\\sandbox\\mdb\\fssl\\2023.mdb";
    let password = "Uid=;Pwd=5hY-tek;";

    let connection_string = [driver, db, password].join(";");

    // Establish the connection
    let conn = env.connect_with_connection_string(&*connection_string, ConnectionOptions::default())?;


    // Call the all function to fetch the athletes
    //let athletes =

    // Iterate over the results
 //   for athlete in db::tm::TmAthlete::all(conn)? {
   //     println!("Athlete ID: {}", athlete.athlete);
     //   // Print other fields as needed
       // println!("Name: {} {}, Group: {}", athlete.first, athlete.last, athlete.group);
    //}
}


