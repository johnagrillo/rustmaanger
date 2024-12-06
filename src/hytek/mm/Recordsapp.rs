// 
// 
// CREATE TABLE [RecordsApp]
// (
// [timer_type] Integer,
// [app_title] Text (180),
// [app_date] Text (22),
// [sendto_name] Text (80),
// [sendto_address1] Text (80),
// [sendto_address2] Text (80),
// [sendto_city] Text (40),
// [sendto_state] Text (6),
// [sendto_zip] Text (20),
// [sendto_email] Text (100),
// [record_name] Text (24),
// [app_topnote1] Text (220),
// [app_bottomnote1] Text (220),
// [app_topnote2] Text (220),
// [app_bottomnote2] Text (220),
// [app_topnote3] Text (220),
// [app_bottomnote3] Text (220),
// [app_topnote4] Text (220),
// [app_bottomnote4] Text (220),
// [referree_name] Text (80),
// [adminref_name] Text (80)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Recordsapp {
    timer_type : Option<u16>,
    app_title : Option<String>,
    app_date : Option<String>,
    sendto_name : Option<String>,
    sendto_address1 : Option<String>,
    sendto_address2 : Option<String>,
    sendto_city : Option<String>,
    sendto_state : Option<String>,
    sendto_zip : Option<String>,
    sendto_email : Option<String>,
    record_name : Option<String>,
    app_topnote1 : Option<String>,
    app_bottomnote1 : Option<String>,
    app_topnote2 : Option<String>,
    app_bottomnote2 : Option<String>,
    app_topnote3 : Option<String>,
    app_bottomnote3 : Option<String>,
    app_topnote4 : Option<String>,
    app_bottomnote4 : Option<String>,
    referree_name : Option<String>,
    adminref_name : Option<String>
}
impl Recordsapp {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Recordsapp>, Box<dyn Error>> {
       let mut vec: Vec<Recordsapp> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Recordsapp = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Recordsapp>, Error> {
        let mut vec: Vec<Recordsapp> = Vec::new();
        match conn.execute("SELECT * FROM records app", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let timer_type = read_u16(_batch.at(0, row_index));
                        let app_title = read_string(_batch.at(1, row_index));
                        let app_date = read_string(_batch.at(2, row_index));
                        let sendto_name = read_string(_batch.at(3, row_index));
                        let sendto_address1 = read_string(_batch.at(4, row_index));
                        let sendto_address2 = read_string(_batch.at(5, row_index));
                        let sendto_city = read_string(_batch.at(6, row_index));
                        let sendto_state = read_string(_batch.at(7, row_index));
                        let sendto_zip = read_string(_batch.at(8, row_index));
                        let sendto_email = read_string(_batch.at(9, row_index));
                        let record_name = read_string(_batch.at(10, row_index));
                        let app_topnote1 = read_string(_batch.at(11, row_index));
                        let app_bottomnote1 = read_string(_batch.at(12, row_index));
                        let app_topnote2 = read_string(_batch.at(13, row_index));
                        let app_bottomnote2 = read_string(_batch.at(14, row_index));
                        let app_topnote3 = read_string(_batch.at(15, row_index));
                        let app_bottomnote3 = read_string(_batch.at(16, row_index));
                        let app_topnote4 = read_string(_batch.at(17, row_index));
                        let app_bottomnote4 = read_string(_batch.at(18, row_index));
                        let referree_name = read_string(_batch.at(19, row_index));
                        let adminref_name = read_string(_batch.at(20, row_index));
                        let obj = Recordsapp {
                            timer_type,
                            app_title,
                            app_date,
                            sendto_name,
                            sendto_address1,
                            sendto_address2,
                            sendto_city,
                            sendto_state,
                            sendto_zip,
                            sendto_email,
                            record_name,
                            app_topnote1,
                            app_bottomnote1,
                            app_topnote2,
                            app_bottomnote2,
                            app_topnote3,
                            app_bottomnote3,
                            app_topnote4,
                            app_bottomnote4,
                            referree_name,
                            adminref_name
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
