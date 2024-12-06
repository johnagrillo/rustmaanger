// 
// 
// CREATE TABLE [CheckList]
// (
// [meet_setup] Boolean NOT NULL,
// [athlete_pref] Boolean NOT NULL,
// [seeding_pref] Boolean NOT NULL,
// [report_pref] Boolean NOT NULL,
// [printer_setup] Boolean NOT NULL,
// [entry_pref] Boolean NOT NULL,
// [scoring_setup] Boolean NOT NULL,
// [entryfee_surcharges] Boolean NOT NULL,
// [divreg_names] Boolean NOT NULL,
// [dir_pref] Boolean NOT NULL,
// [event_setup] Boolean NOT NULL,
// [records_setup] Boolean NOT NULL,
// [timestd_setup] Boolean NOT NULL,
// [timing_setup] Boolean NOT NULL,
// [scbd_setup] Boolean NOT NULL,
// [session_setup] Boolean NOT NULL
// );
#[derive(Serialize,Deserialize,Debug)]
struct Checklist {
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    meet_setup : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    athlete_pref : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    seeding_pref : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    report_pref : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    printer_setup : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    entry_pref : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    scoring_setup : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    entryfee_surcharges : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    divreg_names : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    dir_pref : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    event_setup : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    records_setup : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    timestd_setup : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    timing_setup : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    scbd_setup : bool,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    session_setup : bool
}
impl Checklist {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Checklist>, Box<dyn Error>> {
       let mut vec: Vec<Checklist> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Checklist = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Checklist>, Error> {
        let mut vec: Vec<Checklist> = Vec::new();
        match conn.execute("SELECT * FROM check list", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let meet_setup = read_bool(_batch.at(0, row_index));
                        let athlete_pref = read_bool(_batch.at(1, row_index));
                        let seeding_pref = read_bool(_batch.at(2, row_index));
                        let report_pref = read_bool(_batch.at(3, row_index));
                        let printer_setup = read_bool(_batch.at(4, row_index));
                        let entry_pref = read_bool(_batch.at(5, row_index));
                        let scoring_setup = read_bool(_batch.at(6, row_index));
                        let entryfee_surcharges = read_bool(_batch.at(7, row_index));
                        let divreg_names = read_bool(_batch.at(8, row_index));
                        let dir_pref = read_bool(_batch.at(9, row_index));
                        let event_setup = read_bool(_batch.at(10, row_index));
                        let records_setup = read_bool(_batch.at(11, row_index));
                        let timestd_setup = read_bool(_batch.at(12, row_index));
                        let timing_setup = read_bool(_batch.at(13, row_index));
                        let scbd_setup = read_bool(_batch.at(14, row_index));
                        let session_setup = read_bool(_batch.at(15, row_index));
                        let obj = Checklist {
                            meet_setup,
                            athlete_pref,
                            seeding_pref,
                            report_pref,
                            printer_setup,
                            entry_pref,
                            scoring_setup,
                            entryfee_surcharges,
                            divreg_names,
                            dir_pref,
                            event_setup,
                            records_setup,
                            timestd_setup,
                            timing_setup,
                            scbd_setup,
                            session_setup
                        };
                        vec.push(obj);
                    };
                }
            }
	    None => todo!()
        }
        Ok(vec)
    }
}
