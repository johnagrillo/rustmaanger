// 
// 
// CREATE TABLE [TestData]
// (
// [test_no] Long Integer,
// [athlete] Long Integer,
// [heart_1] Integer,
// [heart_2] Integer,
// [heart_3] Integer,
// [ind_rmk] Text (80)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Testdata {
    test_no : Option<u64>,
    athlete : Option<u64>,
    heart1 : Option<u16>,
    heart2 : Option<u16>,
    heart3 : Option<u16>,
    ind_rmk : Option<String>
}
impl Testdata {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Testdata>, Box<dyn Error>> {
       let mut vec: Vec<Testdata> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Testdata = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Testdata>, Box<dyn Error>> {
        let mut vec: Vec<Testdata> = Vec::new();
        match conn.execute("SELECT * FROM test data", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let test_no = read_u64(_batch.at(0, row_index))?;
                        let athlete = read_u64(_batch.at(1, row_index))?;
                        let heart1 = read_u16(_batch.at(2, row_index))?;
                        let heart2 = read_u16(_batch.at(3, row_index))?;
                        let heart3 = read_u16(_batch.at(4, row_index))?;
                        let ind_rmk = read_string(_batch.at(5, row_index))?;
                        let obj = Testdata {
                            test_no,
                            athlete,
                            heart1,
                            heart2,
                            heart3,
                            ind_rmk
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
