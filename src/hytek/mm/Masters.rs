// 
// 
// CREATE TABLE [Masters]
// (
// [timer_type] Integer,
// [pool_name] Text (90),
// [pool_city] Text (40),
// [pool_state] Text (6),
// [pool_lmsc] Text (6),
// [Meet_type] Integer,
// [ref_name] Text (80),
// [sub_name] Text (80),
// [sub_address] Text (80),
// [sub_city] Text (40),
// [sub_state] Text (6),
// [sub_zip] Text (20),
// [sub_phone] Text (40),
// [sub_email] Text (100),
// [sendto_name] Text (80),
// [sendto_address] Text (80),
// [sendto_city] Text (40),
// [sendto_state] Text (6),
// [sendto_zip] Text (20),
// [sendto_email] Text (100),
// [record_name] Text (24)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Masters {
    timer_type : Option<u16>,
    pool_name : Option<String>,
    pool_city : Option<String>,
    pool_state : Option<String>,
    pool_lmsc : Option<String>,
    meet_type : Option<u16>,
    ref_name : Option<String>,
    sub_name : Option<String>,
    sub_address : Option<String>,
    sub_city : Option<String>,
    sub_state : Option<String>,
    sub_zip : Option<String>,
    sub_phone : Option<String>,
    sub_email : Option<String>,
    sendto_name : Option<String>,
    sendto_address : Option<String>,
    sendto_city : Option<String>,
    sendto_state : Option<String>,
    sendto_zip : Option<String>,
    sendto_email : Option<String>,
    record_name : Option<String>
}
impl Masters {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Masters>, Box<dyn Error>> {
       let mut vec: Vec<Masters> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Masters = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Masters>, Error> {
        let mut vec: Vec<Masters> = Vec::new();
        match conn.execute("SELECT * FROM masters", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let timer_type = read_u16(_batch.at(0, row_index));
                        let pool_name = read_string(_batch.at(1, row_index));
                        let pool_city = read_string(_batch.at(2, row_index));
                        let pool_state = read_string(_batch.at(3, row_index));
                        let pool_lmsc = read_string(_batch.at(4, row_index));
                        let meet_type = read_u16(_batch.at(5, row_index));
                        let ref_name = read_string(_batch.at(6, row_index));
                        let sub_name = read_string(_batch.at(7, row_index));
                        let sub_address = read_string(_batch.at(8, row_index));
                        let sub_city = read_string(_batch.at(9, row_index));
                        let sub_state = read_string(_batch.at(10, row_index));
                        let sub_zip = read_string(_batch.at(11, row_index));
                        let sub_phone = read_string(_batch.at(12, row_index));
                        let sub_email = read_string(_batch.at(13, row_index));
                        let sendto_name = read_string(_batch.at(14, row_index));
                        let sendto_address = read_string(_batch.at(15, row_index));
                        let sendto_city = read_string(_batch.at(16, row_index));
                        let sendto_state = read_string(_batch.at(17, row_index));
                        let sendto_zip = read_string(_batch.at(18, row_index));
                        let sendto_email = read_string(_batch.at(19, row_index));
                        let record_name = read_string(_batch.at(20, row_index));
                        let obj = Masters {
                            timer_type,
                            pool_name,
                            pool_city,
                            pool_state,
                            pool_lmsc,
                            meet_type,
                            ref_name,
                            sub_name,
                            sub_address,
                            sub_city,
                            sub_state,
                            sub_zip,
                            sub_phone,
                            sub_email,
                            sendto_name,
                            sendto_address,
                            sendto_city,
                            sendto_state,
                            sendto_zip,
                            sendto_email,
                            record_name
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
