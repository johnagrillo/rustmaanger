// 
// 
// CREATE TABLE [MemCirName]
// (
// [cir_no] Long Integer,
// [cir_name] Text (180),
// [tot_lines] Integer,
// [tot_dist] Long Integer,
// [tot_seconds] Long Integer
// );
#[derive(Serialize,Deserialize,Debug)]
struct Memcirname {
    cir_no : Option<u64>,
    cir_name : Option<String>,
    tot_lines : Option<u16>,
    tot_dist : Option<u64>,
    tot_seconds : Option<u64>
}
impl Memcirname {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Memcirname>, Box<dyn Error>> {
       let mut vec: Vec<Memcirname> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Memcirname = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Memcirname>, Box<dyn Error>> {
        let mut vec: Vec<Memcirname> = Vec::new();
        match conn.execute("SELECT * FROM mem cir name", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let cir_no = read_u64(_batch.at(0, row_index))?;
                        let cir_name = read_string(_batch.at(1, row_index))?;
                        let tot_lines = read_u16(_batch.at(2, row_index))?;
                        let tot_dist = read_u64(_batch.at(3, row_index))?;
                        let tot_seconds = read_u64(_batch.at(4, row_index))?;
                        let obj = Memcirname {
                            cir_no,
                            cir_name,
                            tot_lines,
                            tot_dist,
                            tot_seconds
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
