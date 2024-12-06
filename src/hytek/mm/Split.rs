// 
// 
// CREATE TABLE [Split]
// (
// [Event_ptr] Long Integer,
// [Ath_no] Long Integer,
// [Team_no] Long Integer,
// [Team_ltr] Text (2),
// [Rnd_ltr] Text (2),
// [Split_no] Integer,
// [Split_Time] Single,
// [Relay_no] Long Integer
// );
#[derive(Serialize,Deserialize,Debug)]
struct Split {
    event_ptr : Option<u64>,
    ath_no : Option<u64>,
    team_no : Option<u64>,
    team_ltr : Option<String>,
    rnd_ltr : Option<String>,
    split_no : Option<u16>,
    split__time : Option<f32>,
    relay_no : Option<u64>
}
impl Split {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Split>, Box<dyn Error>> {
       let mut vec: Vec<Split> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Split = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Split>, Error> {
        let mut vec: Vec<Split> = Vec::new();
        match conn.execute("SELECT * FROM split", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let event_ptr = read_u64(_batch.at(0, row_index));
                        let ath_no = read_u64(_batch.at(1, row_index));
                        let team_no = read_u64(_batch.at(2, row_index));
                        let team_ltr = read_string(_batch.at(3, row_index));
                        let rnd_ltr = read_string(_batch.at(4, row_index));
                        let split_no = read_u16(_batch.at(5, row_index));
                        let split__time = read_f32(_batch.at(6, row_index));
                        let relay_no = read_u64(_batch.at(7, row_index));
                        let obj = Split {
                            event_ptr,
                            ath_no,
                            team_no,
                            team_ltr,
                            rnd_ltr,
                            split_no,
                            split__time,
                            relay_no
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
