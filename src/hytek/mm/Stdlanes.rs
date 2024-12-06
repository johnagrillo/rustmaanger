// 
// 
// CREATE TABLE [StdLanes]
// (
// [tot_lanes] Integer,
// [order_01] Integer,
// [order_02] Integer,
// [order_03] Integer,
// [order_04] Integer,
// [order_05] Integer,
// [order_06] Integer,
// [order_07] Integer,
// [order_08] Integer,
// [order_09] Integer,
// [order_10] Integer,
// [order_11] Integer,
// [order_12] Integer
// );
#[derive(Serialize,Deserialize,Debug)]
struct Stdlanes {
    tot_lanes : Option<u16>,
    order01 : Option<u16>,
    order02 : Option<u16>,
    order03 : Option<u16>,
    order04 : Option<u16>,
    order05 : Option<u16>,
    order06 : Option<u16>,
    order07 : Option<u16>,
    order08 : Option<u16>,
    order09 : Option<u16>,
    order10 : Option<u16>,
    order11 : Option<u16>,
    order12 : Option<u16>
}
impl Stdlanes {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Stdlanes>, Box<dyn Error>> {
       let mut vec: Vec<Stdlanes> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Stdlanes = result?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Stdlanes>, Error> {
        let mut vec: Vec<Stdlanes> = Vec::new();
        match conn.execute("SELECT * FROM std lanes", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let tot_lanes = read_u16(_batch.at(0, row_index));
                        let order01 = read_u16(_batch.at(1, row_index));
                        let order02 = read_u16(_batch.at(2, row_index));
                        let order03 = read_u16(_batch.at(3, row_index));
                        let order04 = read_u16(_batch.at(4, row_index));
                        let order05 = read_u16(_batch.at(5, row_index));
                        let order06 = read_u16(_batch.at(6, row_index));
                        let order07 = read_u16(_batch.at(7, row_index));
                        let order08 = read_u16(_batch.at(8, row_index));
                        let order09 = read_u16(_batch.at(9, row_index));
                        let order10 = read_u16(_batch.at(10, row_index));
                        let order11 = read_u16(_batch.at(11, row_index));
                        let order12 = read_u16(_batch.at(12, row_index));
                        let obj = Stdlanes {
                            tot_lanes,
                            order01,
                            order02,
                            order03,
                            order04,
                            order05,
                            order06,
                            order07,
                            order08,
                            order09,
                            order10,
                            order11,
                            order12
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
