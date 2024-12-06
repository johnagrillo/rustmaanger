// 
// 
// CREATE TABLE [RelayNames]
// (
// [Event_ptr] Long Integer,
// [Team_no] Long Integer,
// [Team_ltr] Text (510),
// [Ath_no] Long Integer,
// [Pos_no] Integer,
// [Event_round] Text (2),
// [Relay_no] Long Integer
// );
#[derive(Serialize,Deserialize,Debug)]
struct Relaynames {
    event_ptr : Option<u64>,
    team_no : Option<u64>,
    team_ltr : Option<String>,
    ath_no : Option<u64>,
    pos_no : Option<u16>,
    event_round : Option<String>,
    relay_no : Option<u64>
}
impl Relaynames {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Relaynames>, Box<dyn Error>> {
       let mut vec: Vec<Relaynames> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Relaynames = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Relaynames>, Error> {
        let mut vec: Vec<Relaynames> = Vec::new();
        match conn.execute("SELECT * FROM relay names", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let event_ptr = read_u64(_batch.at(0, row_index));
                        let team_no = read_u64(_batch.at(1, row_index));
                        let team_ltr = read_string(_batch.at(2, row_index));
                        let ath_no = read_u64(_batch.at(3, row_index));
                        let pos_no = read_u16(_batch.at(4, row_index));
                        let event_round = read_string(_batch.at(5, row_index));
                        let relay_no = read_u64(_batch.at(6, row_index));
                        let obj = Relaynames {
                            event_ptr,
                            team_no,
                            team_ltr,
                            ath_no,
                            pos_no,
                            event_round,
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
