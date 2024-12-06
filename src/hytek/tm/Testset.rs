// 
// 
// CREATE TABLE [TestSet]
// (
// [test_no] Long Integer,
// [test_date] DateTime,
// [tot_reps] Integer,
// [test_dist] Long Integer,
// [test_interval] Long Integer,
// [test_sly] Text (2),
// [push_off] Boolean NOT NULL,
// [test_rmk] Text (80),
// [test_stroke] Integer,
// [use_it] Boolean NOT NULL
// );
#[derive(Serialize,Deserialize,Debug)]
struct Testset {
    test_no : Option<u64>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    test_date : Option<NaiveDateTime>,
    tot_reps : Option<u16>,
    test_dist : Option<u64>,
    test_interval : Option<u64>,
    test_sly : Option<String>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    push_off : bool,
    test_rmk : Option<String>,
    test_stroke : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    use_it : bool
}
impl Testset {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Testset>, Box<dyn Error>> {
       let mut vec: Vec<Testset> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Testset = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Testset>, Box<dyn Error>> {
        let mut vec: Vec<Testset> = Vec::new();
        match conn.execute("SELECT * FROM test set", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let test_no = read_u64(_batch.at(0, row_index))?;
                        let test_date = read_datetime(_batch.at(1, row_index))?;
                        let tot_reps = read_u16(_batch.at(2, row_index))?;
                        let test_dist = read_u64(_batch.at(3, row_index))?;
                        let test_interval = read_u64(_batch.at(4, row_index))?;
                        let test_sly = read_string(_batch.at(5, row_index))?;
                        let push_off = read_bool(_batch.at(6, row_index))?;
                        let test_rmk = read_string(_batch.at(7, row_index))?;
                        let test_stroke = read_u16(_batch.at(8, row_index))?;
                        let use_it = read_bool(_batch.at(9, row_index))?;
                        let obj = Testset {
                            test_no,
                            test_date,
                            tot_reps,
                            test_dist,
                            test_interval,
                            test_sly,
                            push_off,
                            test_rmk,
                            test_stroke,
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
