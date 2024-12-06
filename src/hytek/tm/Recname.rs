// 
// 
// CREATE TABLE [RECNAME]
// (
// [RecFile] Text (16),
// [Year] Text (8),
// [Descript] Text (40),
// [Flag] Text (2),
// [Course] Text (2),
// [Record] Long Integer
// );
#[derive(Serialize,Deserialize,Debug)]
struct Recname {
    rec_file : Option<String>,
    year : Option<String>,
    descript : Option<String>,
    flag : Option<String>,
    course : Option<String>,
    record : Option<u64>
}
impl Recname {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Recname>, Box<dyn Error>> {
       let mut vec: Vec<Recname> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Recname = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Recname>, Box<dyn Error>> {
        let mut vec: Vec<Recname> = Vec::new();
        match conn.execute("SELECT * FROM RECNAME", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let rec_file = read_string(_batch.at(0, row_index))?;
                        let year = read_string(_batch.at(1, row_index))?;
                        let descript = read_string(_batch.at(2, row_index))?;
                        let flag = read_string(_batch.at(3, row_index))?;
                        let course = read_string(_batch.at(4, row_index))?;
                        let record = read_u64(_batch.at(5, row_index))?;
                        let obj = Recname {
                            rec_file,
                            year,
                            descript,
                            flag,
                            course,
                            record
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
