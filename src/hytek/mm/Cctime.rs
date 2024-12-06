// 
// 
// CREATE TABLE [CCtime]
// (
// [Event_ptr] Long Integer,
// [Ath_no] Long Integer,
// [Chute_rank] Long Integer,
// [Chute_no] Integer,
// [Fin_hand] Boolean NOT NULL,
// [Fin_stat] Text (2),
// [Fin_Time] Single
// );
#[derive(Serialize,Deserialize,Debug)]
struct Cctime {
    event_ptr : Option<u64>,
    ath_no : Option<u64>,
    chute_rank : Option<u64>,
    chute_no : Option<u16>,
    #[serde(deserialize_with = "deserialize_bool_from_0_1")]
    fin_hand : bool,
    fin_stat : Option<String>,
    fin__time : Option<f32>
}
impl Cctime {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Cctime>, Box<dyn Error>> {
       let mut vec: Vec<Cctime> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Cctime = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Cctime>, Error> {
        let mut vec: Vec<Cctime> = Vec::new();
        match conn.execute("SELECT * FROM c ctime", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let event_ptr = read_u64(_batch.at(0, row_index));
                        let ath_no = read_u64(_batch.at(1, row_index));
                        let chute_rank = read_u64(_batch.at(2, row_index));
                        let chute_no = read_u16(_batch.at(3, row_index));
                        let fin_hand = read_bool(_batch.at(4, row_index));
                        let fin_stat = read_string(_batch.at(5, row_index));
                        let fin__time = read_f32(_batch.at(6, row_index));
                        let obj = Cctime {
                            event_ptr,
                            ath_no,
                            chute_rank,
                            chute_no,
                            fin_hand,
                            fin_stat,
                            fin__time
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
