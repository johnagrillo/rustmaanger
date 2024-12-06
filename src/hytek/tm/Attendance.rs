// 
// 
// CREATE TABLE [Attendance]
// (
// [athlete] Long Integer,
// [attend_date] DateTime,
// [morn_aft] Text (2),
// [attn_stat] Text (2),
// [actual_dist] Long Integer,
// [yd_mtr] Text (2)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Attendance {
    athlete : Option<u64>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    attend_date : Option<NaiveDateTime>,
    morn_aft : Option<String>,
    attn_stat : Option<String>,
    actual_dist : Option<u64>,
    yd_mtr : Option<String>
}
impl Attendance {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Attendance>, Box<dyn Error>> {
       let mut vec: Vec<Attendance> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Attendance = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Attendance>, Box<dyn Error>> {
        let mut vec: Vec<Attendance> = Vec::new();
        match conn.execute("SELECT * FROM attendance", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let athlete = read_u64(_batch.at(0, row_index))?;
                        let attend_date = read_datetime(_batch.at(1, row_index))?;
                        let morn_aft = read_string(_batch.at(2, row_index))?;
                        let attn_stat = read_string(_batch.at(3, row_index))?;
                        let actual_dist = read_u64(_batch.at(4, row_index))?;
                        let yd_mtr = read_string(_batch.at(5, row_index))?;
                        let obj = Attendance {
                            athlete,
                            attend_date,
                            morn_aft,
                            attn_stat,
                            actual_dist,
                            yd_mtr
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
