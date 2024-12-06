// 
// 
// CREATE TABLE [TestT30]
// (
// [athlete] Long Integer,
// [test_date] DateTime,
// [test_stroke] Integer,
// [test_dist] Long Integer,
// [test_time] Long Integer,
// [test_sly] Text (2),
// [test_rmk] Text (80),
// [heart_1] Integer,
// [heart_2] Integer,
// [heart_3] Integer,
// [use_it] Boolean NOT NULL
// );
#[derive(Serialize,Deserialize,Debug)]
struct Testt30 {
    athlete : Option<u64>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    test_date : Option<NaiveDateTime>,
    test_stroke : Option<u16>,
    test_dist : Option<u64>,
    test_time : Option<u64>,
    test_sly : Option<String>,
    test_rmk : Option<String>,
    heart1 : Option<u16>,
    heart2 : Option<u16>,
    heart3 : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    use_it : bool
}
impl Testt30 {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Testt30>, Box<dyn Error>> {
       let mut vec: Vec<Testt30> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Testt30 = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Testt30>, Box<dyn Error>> {
        let mut vec: Vec<Testt30> = Vec::new();
        match conn.execute("SELECT * FROM test t 3 0", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let athlete = read_u64(_batch.at(0, row_index))?;
                        let test_date = read_datetime(_batch.at(1, row_index))?;
                        let test_stroke = read_u16(_batch.at(2, row_index))?;
                        let test_dist = read_u64(_batch.at(3, row_index))?;
                        let test_time = read_u64(_batch.at(4, row_index))?;
                        let test_sly = read_string(_batch.at(5, row_index))?;
                        let test_rmk = read_string(_batch.at(6, row_index))?;
                        let heart1 = read_u16(_batch.at(7, row_index))?;
                        let heart2 = read_u16(_batch.at(8, row_index))?;
                        let heart3 = read_u16(_batch.at(9, row_index))?;
                        let use_it = read_bool(_batch.at(10, row_index))?;
                        let obj = Testt30 {
                            athlete,
                            test_date,
                            test_stroke,
                            test_dist,
                            test_time,
                            test_sly,
                            test_rmk,
                            heart1,
                            heart2,
                            heart3,
                            use_it
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
