// 
// 
// CREATE TABLE [model]
// (
// [wk_ptr] Long Integer,
// [wk_line] Integer,
// [wk_type] Text (2),
// [circuit_line] Integer,
// [circuit_tot] Integer,
// [circuit_reps] Integer,
// [set_reps] Long Integer,
// [set_dist] Long Integer,
// [set_interval] Long Integer,
// [set_desc] Text (64),
// [set_note] Text (110),
// [set_energyno] Integer,
// [set_type] Text (8),
// [set_categoryabbr] Text (6),
// [tot_yards] Long Integer,
// [set_starttime] Long Integer,
// [set_finishtime] Long Integer
// );
#[derive(Serialize,Deserialize,Debug)]
struct Model {
    wk_ptr : Option<u64>,
    wk_line : Option<u16>,
    wk_type : Option<String>,
    circuit_line : Option<u16>,
    circuit_tot : Option<u16>,
    circuit_reps : Option<u16>,
    set_reps : Option<u64>,
    set_dist : Option<u64>,
    set_interval : Option<u64>,
    set_desc : Option<String>,
    set_note : Option<String>,
    set_energyno : Option<u16>,
    set_type : Option<String>,
    set_categoryabbr : Option<String>,
    tot_yards : Option<u64>,
    set_starttime : Option<u64>,
    set_finishtime : Option<u64>
}
impl Model {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Model>, Box<dyn Error>> {
       let mut vec: Vec<Model> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Model = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Model>, Box<dyn Error>> {
        let mut vec: Vec<Model> = Vec::new();
        match conn.execute("SELECT * FROM model", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let wk_ptr = read_u64(_batch.at(0, row_index))?;
                        let wk_line = read_u16(_batch.at(1, row_index))?;
                        let wk_type = read_string(_batch.at(2, row_index))?;
                        let circuit_line = read_u16(_batch.at(3, row_index))?;
                        let circuit_tot = read_u16(_batch.at(4, row_index))?;
                        let circuit_reps = read_u16(_batch.at(5, row_index))?;
                        let set_reps = read_u64(_batch.at(6, row_index))?;
                        let set_dist = read_u64(_batch.at(7, row_index))?;
                        let set_interval = read_u64(_batch.at(8, row_index))?;
                        let set_desc = read_string(_batch.at(9, row_index))?;
                        let set_note = read_string(_batch.at(10, row_index))?;
                        let set_energyno = read_u16(_batch.at(11, row_index))?;
                        let set_type = read_string(_batch.at(12, row_index))?;
                        let set_categoryabbr = read_string(_batch.at(13, row_index))?;
                        let tot_yards = read_u64(_batch.at(14, row_index))?;
                        let set_starttime = read_u64(_batch.at(15, row_index))?;
                        let set_finishtime = read_u64(_batch.at(16, row_index))?;
                        let obj = Model {
                            wk_ptr,
                            wk_line,
                            wk_type,
                            circuit_line,
                            circuit_tot,
                            circuit_reps,
                            set_reps,
                            set_dist,
                            set_interval,
                            set_desc,
                            set_note,
                            set_energyno,
                            set_type,
                            set_categoryabbr,
                            tot_yards,
                            set_starttime,
                            set_finishtime
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
