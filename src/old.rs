#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(non_camel_case_types)]

// include!("db/mmAgegroup.rs");
// include!("db/mmAltScoring.rs");
// include!("db/mmAthlete.rs");
// include!("db/mmCCrank.rs");
// include!("db/mmCCtime.rs");
// include!("db/mmCheckList.rs");
// include!("db/mmCombinedEvent.rs");
// include!("db/mmDivisions.rs");
// include!("db/mmDualteams.rs");
// include!("db/mmEntry.rs");
// include!("db/mmEvent.rs");
// include!("db/mmEventGetTimes.rs");
// include!("db/mmMasters.rs");
// include!("db/mmMeet.rs");
// include!("db/mmMEETMOBILE2OPTIONS.rs");
// include!("db/mmMemorizedReports.rs");
// include!("db/mmMultiage.rs");
// include!("db/mmMultiageScnd.rs");
// include!("db/mmOfficials.rs");
// include!("db/mmOMEOPTIONS.rs");
// include!("db/mmRecords.rs");
// include!("db/mmRecordsApp.rs");
// include!("db/mmRecordsbyEvent.rs");
// include!("db/mmRegions.rs");
// include!("db/mmRelay.rs");
// include!("db/mmRelayNames.rs");
// include!("db/mmScoreLanes.rs");
// include!("db/mmScoring.rs");
// include!("db/mmScoringImprovement.rs");
// include!("db/mmSession.rs");
// include!("db/mmSessitem.rs");
// include!("db/mmSplit.rs");
// include!("db/mmStdLanes.rs");
// include!("db/mmTeam.rs");
// include!("db/mmTeamCoaches.rs");
// include!("db/mmTimeStd.rs");
// include!("db/mmWaveOffset.rs");
// include!("db/mmRecordTags.rs");
// include!("db/mmTagNames.rs");
// include!("db/tm2016FSSL.rs");
// include!("db/tmAthInfo.rs");
// include!("db/tmAthlete.rs");
// include!("db/tmATHRECR.rs");
// include!("db/tmAttendance.rs");
// include!("db/tmCOACHES.rs");
// include!("db/tmCODE.rs");
// include!("db/tmCustomLayout.rs");
// include!("db/tmCustomLayoutFields.rs");
// include!("db/tmCustomLayoutValues.rs");
// include!("db/tmCUSTOMRPTS.rs");
// include!("db/tmDELETEENTRY.rs");
// include!("db/tmEnergy.rs");
// include!("db/tmESPLITS.rs");
// include!("db/tmFAVORITES.rs");
// include!("db/tmHYTEKAGEGROUP.rs");
// include!("db/tmJOURNAL.rs");
// include!("db/tmMemCirName.rs");
// include!("db/tmMemCirSets.rs");
// include!("db/tmMemSets.rs");
// include!("db/tm_model.rs");
// include!("db/tmModelParam.rs");
// include!("db/tmMSDecline.rs");
// include!("db/tmMTEVENT.rs");
// include!("db/tmMTEVENTE.rs");
// include!("db/tmOMEINVITE.rs");
// include!("db/tmOMEOPTIONS.rs");
// include!("db/tmOPTIONS.rs");
// include!("db/tmRECNAME.rs");
// include!("db/tmRECORDS.rs");
// include!("db/tmRELAY.rs");
// include!("db/tmRESULT.rs");
// include!("db/tmSESSIONS.rs");
// include!("db/tmSPLITS.rs");
// include!("db/tmSTDNAME.rs");
// include!("db/tmStrokeCategory.rs");
// include!("db/tmTEAM.rs");
// include!("db/tmTestData.rs");
// include!("db/tmTestLine.rs");
// include!("db/tmTestSet.rs");
// include!("db/tmTestT30.rs");
// include!("db/tmWkParam.rs");
// include!("db/tmWorkCategory.rs");
// include!("db/tmWorkout.rs");
// include!("db/tm2023_ALL.rs");
// include!("db/tmCUSTOMAGEGROUP.rs");
// include!("db/tmENTRY.rs");
// include!("db/tmMEET.rs");
// include!("db/tm_modt30times.rs");
// include!("db/tmPREENTER.rs");
// include!("db/tmSetDescriptions.rs");
use anyhow::Error;
use odbc_api::{buffers::TextRowSet, Cursor, Environment, ConnectionOptions, ResultSetMetadata};
use std::{
    ffi::CStr,
    io::{stdout, Write},
    path::PathBuf,
};

/// Maximum number of rows fetched with one row set. Fetching batches of rows is usually much
/// faster than fetching individual rows.
const BATCH_SIZE: usize = 5000;
use std::env;
fn main() -> Result<(), Error> {
    // Write csv to standard out
    let out = stdout();
    let mut writer = csv::Writer::from_writer(out);

    // If you do not do anything fancy it is recommended to have only one Environment in the
    // entire process.
    let environment = Environment::new()?;

    let db_path = "C:/Users/john/sandbox/mdb/fssl/2023.mdb";

    let password = "Pwd=5hY-tek";
    
    // Use the connection string, username, and password in the connect call
    let connection = environment.connect(
	"{Microsoft Access Driver (*.mdb, *.accdb)}",
	db_path,
	password,
        ConnectionOptions::default(),
    )?;

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
            while let Some(batch) = row_set_cursor.fetch()? {
                // Within a batch, iterate over every row
                for row_index in 0..batch.num_rows() {
                    // Within a row iterate over every column
                    let record = (0..batch.num_cols()).map(|col_index| {
                        batch
                            .at(col_index, row_index)
                            .unwrap_or(&[])
                    });
                    // Writes row as csv
                    writer.write_record(record)?;
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


