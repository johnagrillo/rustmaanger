// 
// 
// CREATE TABLE [Regions]
// (
// [Reg_no] Long Integer,
// [Reg_abbr] Text (4),
// [Reg_name] Text (40),
// [fem_size] Integer,
// [male_size] Integer,
// [combined_size] Integer
// );
#[derive(Serialize,Deserialize,Debug)]
struct Regions {
    reg_no : Option<u64>,
    reg_abbr : Option<String>,
    reg_name : Option<String>,
    fem_size : Option<u16>,
    male_size : Option<u16>,
    combined_size : Option<u16>
}
impl Regions {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Regions>, Box<dyn Error>> {
       let mut vec: Vec<Regions> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Regions = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Regions>, Error> {
        let mut vec: Vec<Regions> = Vec::new();
        match conn.execute("SELECT * FROM regions", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let reg_no = read_u64(_batch.at(0, row_index));
                        let reg_abbr = read_string(_batch.at(1, row_index));
                        let reg_name = read_string(_batch.at(2, row_index));
                        let fem_size = read_u16(_batch.at(3, row_index));
                        let male_size = read_u16(_batch.at(4, row_index));
                        let combined_size = read_u16(_batch.at(5, row_index));
                        let obj = Regions {
                            reg_no,
                            reg_abbr,
                            reg_name,
                            fem_size,
                            male_size,
                            combined_size
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
