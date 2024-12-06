// 
// 
// CREATE TABLE [EventGetTimes]
// (
// [Event_ptr] Long Integer,
// [Heat_no] Integer,
// [Vendor_abbr] Text (10),
// [Race_number] Text (12),
// [Get_Date] DateTime,
// [Get_Time] Long Integer,
// [Now_Date] DateTime,
// [Now_Time] Long Integer,
// [Rnd_ltr] Text (2)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Eventgettimes {
    event_ptr : Option<u64>,
    heat_no : Option<u16>,
    vendor_abbr : Option<String>,
    race_number : Option<String>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    get__date : Option<NaiveDateTime>,
    get__time : Option<u64>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    now__date : Option<NaiveDateTime>,
    now__time : Option<u64>,
    rnd_ltr : Option<String>
}
impl Eventgettimes {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Eventgettimes>, Box<dyn Error>> {
       let mut vec: Vec<Eventgettimes> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Eventgettimes = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Eventgettimes>, Error> {
        let mut vec: Vec<Eventgettimes> = Vec::new();
        match conn.execute("SELECT * FROM event get times", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let event_ptr = read_u64(_batch.at(0, row_index));
                        let heat_no = read_u16(_batch.at(1, row_index));
                        let vendor_abbr = read_string(_batch.at(2, row_index));
                        let race_number = read_string(_batch.at(3, row_index));
                        let get__date = read_datetime(_batch.at(4, row_index));
                        let get__time = read_u64(_batch.at(5, row_index));
                        let now__date = read_datetime(_batch.at(6, row_index));
                        let now__time = read_u64(_batch.at(7, row_index));
                        let rnd_ltr = read_string(_batch.at(8, row_index));
                        let obj = Eventgettimes {
                            event_ptr,
                            heat_no,
                            vendor_abbr,
                            race_number,
                            get__date,
                            get__time,
                            now__date,
                            now__time,
                            rnd_ltr
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
