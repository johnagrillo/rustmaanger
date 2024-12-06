// 
// 
// CREATE TABLE [MemSets]
// (
// [set_no] Long Integer,
// [set_reps] Long Integer,
// [set_dist] Long Integer,
// [set_interval] Long Integer,
// [set_desc] Text (64),
// [set_energyno] Integer,
// [set_type] Text (8),
// [set_categoryabbr] Text (6)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Memsets {
    set_no : Option<u64>,
    set_reps : Option<u64>,
    set_dist : Option<u64>,
    set_interval : Option<u64>,
    set_desc : Option<String>,
    set_energyno : Option<u16>,
    set_type : Option<String>,
    set_categoryabbr : Option<String>
}
impl Memsets {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Memsets>, Box<dyn Error>> {
       let mut vec: Vec<Memsets> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Memsets = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Memsets>, Box<dyn Error>> {
        let mut vec: Vec<Memsets> = Vec::new();
        match conn.execute("SELECT * FROM mem sets", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let set_no = read_u64(_batch.at(0, row_index))?;
                        let set_reps = read_u64(_batch.at(1, row_index))?;
                        let set_dist = read_u64(_batch.at(2, row_index))?;
                        let set_interval = read_u64(_batch.at(3, row_index))?;
                        let set_desc = read_string(_batch.at(4, row_index))?;
                        let set_energyno = read_u16(_batch.at(5, row_index))?;
                        let set_type = read_string(_batch.at(6, row_index))?;
                        let set_categoryabbr = read_string(_batch.at(7, row_index))?;
                        let obj = Memsets {
                            set_no,
                            set_reps,
                            set_dist,
                            set_interval,
                            set_desc,
                            set_energyno,
                            set_type,
                            set_categoryabbr
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
