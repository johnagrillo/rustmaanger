// 
// 
// CREATE TABLE [FAVORITES]
// (
// [FavName] Text (16),
// [FavTeam] Text (10),
// [FavGroup] Text (6),
// [FavSubGr] Text (6),
// [FavClass] Text (6),
// [FavMeetType] Text (6),
// [FavCourse] Text (10),
// [FavWMGroup] Text (6),
// [FavWMSubGr] Text (6),
// [SinceDate] DateTime,
// [UntilDate] DateTime,
// [FavTeamDiv] Text (6)
// );
#[derive(Serialize,Deserialize,Debug)]
struct Favorites {
    fav_name : Option<String>,
    fav_team : Option<String>,
    fav_group : Option<String>,
    fav_sub_gr : Option<String>,
    fav_class : Option<String>,
    fav_meet_type : Option<String>,
    fav_course : Option<String>,
    fav_w_m_group : Option<String>,
    fav_w_m_sub_gr : Option<String>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    since_date : Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_optional_datetime")]
    until_date : Option<NaiveDateTime>,
    fav_team_div : Option<String>
}
impl Favorites {

    fn from_csv_path(csv: String) -> std::result::Result<Vec<Favorites>, Box<dyn Error>> {
       let mut vec: Vec<Favorites> = Vec::new();
       let mut rdr = ReaderBuilder::new().has_headers(false).from_path(csv)?;
       for elem in rdr.deserialize() {
            let obj: Favorites = elem?;
            vec.push(obj);
       }
       Ok(vec)
    }
    fn from_mdb(conn: Connection) -> Result<Vec<Favorites>, Box<dyn Error>> {
        let mut vec: Vec<Favorites> = Vec::new();
        match conn.execute("SELECT * FROM FAVORITES", ())? {
            Some(mut cursor) => {
                let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
                let mut row_set_cursor = cursor.bind_buffer(&mut buffers);
                while let Some(_batch) = row_set_cursor.fetch()? {
                    for row_index in 0.._batch.num_rows() {
                        let fav_name = read_string(_batch.at(0, row_index))?;
                        let fav_team = read_string(_batch.at(1, row_index))?;
                        let fav_group = read_string(_batch.at(2, row_index))?;
                        let fav_sub_gr = read_string(_batch.at(3, row_index))?;
                        let fav_class = read_string(_batch.at(4, row_index))?;
                        let fav_meet_type = read_string(_batch.at(5, row_index))?;
                        let fav_course = read_string(_batch.at(6, row_index))?;
                        let fav_w_m_group = read_string(_batch.at(7, row_index))?;
                        let fav_w_m_sub_gr = read_string(_batch.at(8, row_index))?;
                        let since_date = read_datetime(_batch.at(9, row_index))?;
                        let until_date = read_datetime(_batch.at(10, row_index))?;
                        let fav_team_div = read_string(_batch.at(11, row_index))?;
                        let obj = Favorites {
                            fav_name,
                            fav_team,
                            fav_group,
                            fav_sub_gr,
                            fav_class,
                            fav_meet_type,
                            fav_course,
                            fav_w_m_group,
                            fav_w_m_sub_gr,
                            since_date,
                            until_date,
                            fav_team_div
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
