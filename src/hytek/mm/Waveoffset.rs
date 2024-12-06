// 
// 
// CREATE TABLE [WaveOffset]
// (
// [Event_ptr] Long Integer,
// [Heat_no] Integer,
// [Rnd_ltr] Text (2),
// [Wave_offset] Single
// );
#[derive(Serialize,Deserialize,Debug)]
struct Waveoffset {
    event_ptr : Option<u64>,
    heat_no : Option<u16>,
    rnd_ltr : Option<String>,
    wave_offset : Option<f32>
}
impl Waveoffset {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Waveoffset>, Box<dyn Error>> {
       let mut vec: Vec<Waveoffset> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Waveoffset = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Waveoffset>, Error> {
        let mut vec: Vec<Waveoffset> = Vec::new();
        match conn.execute("SELECT * FROM wave offset", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let event_ptr = read_u64(_batch.at(0, row_index));
                        let heat_no = read_u16(_batch.at(1, row_index));
                        let rnd_ltr = read_string(_batch.at(2, row_index));
                        let wave_offset = read_f32(_batch.at(3, row_index));
                        let obj = Waveoffset {
                            event_ptr,
                            heat_no,
                            rnd_ltr,
                            wave_offset
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
