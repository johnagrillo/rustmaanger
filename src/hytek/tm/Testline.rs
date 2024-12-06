// 
// 
// CREATE TABLE [TestLine]
// (
// [test_no] Long Integer,
// [athlete] Long Integer,
// [rep_no] Integer,
// [rep_time] Long Integer
// );
#[derive(Serialize,Deserialize,Debug)]
struct Testline {
    test_no : Option<u64>,
    athlete : Option<u64>,
    rep_no : Option<u16>,
    rep_time : Option<u64>
}
impl Testline {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Testline>, Box<dyn Error>> {
       let mut vec: Vec<Testline> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Testline = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Testline>, Box<dyn Error>> {
        let mut vec: Vec<Testline> = Vec::new();
        match conn.execute("SELECT * FROM test line", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let test_no = read_u64(_batch.at(0, row_index))?;
                        let athlete = read_u64(_batch.at(1, row_index))?;
                        let rep_no = read_u16(_batch.at(2, row_index))?;
                        let rep_time = read_u64(_batch.at(3, row_index))?;
                        let obj = Testline {
                            test_no,
                            athlete,
                            rep_no,
                            rep_time
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
