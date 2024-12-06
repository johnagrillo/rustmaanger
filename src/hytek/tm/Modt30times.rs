// 
// 
// CREATE TABLE [modt30times]
// (
// [athlete] Long Integer,
// [event_no] Integer,
// [course_yls] Text (2),
// [event_dist] Long Integer,
// [event_time] Long Integer,
// [shave_type] Text (2)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Modt30times {
    athlete : Option<u64>,
    event_no : Option<u16>,
    course_yls : Option<String>,
    event_dist : Option<u64>,
    event_time : Option<u64>,
    shave_type : Option<String>
}
impl Modt30times {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Modt30times>, Box<dyn Error>> {
       let mut vec: Vec<Modt30times> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Modt30times = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Modt30times>, Box<dyn Error>> {
        let mut vec: Vec<Modt30times> = Vec::new();
        match conn.execute("SELECT * FROM modt30times", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let athlete = read_u64(_batch.at(0, row_index))?;
                        let event_no = read_u16(_batch.at(1, row_index))?;
                        let course_yls = read_string(_batch.at(2, row_index))?;
                        let event_dist = read_u64(_batch.at(3, row_index))?;
                        let event_time = read_u64(_batch.at(4, row_index))?;
                        let shave_type = read_string(_batch.at(5, row_index))?;
                        let obj = Modt30times {
                            athlete,
                            event_no,
                            course_yls,
                            event_dist,
                            event_time,
                            shave_type
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
