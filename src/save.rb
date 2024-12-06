/// -- ----------------------------------------------------------
/// -- MDB Tools - A library for reading MS Access database files
/// -- Copyright (C) 2000-2011 Brian Bruns and others.
/// -- Files in libmdb are licensed under LGPL and the utilities under
/// -- the GPL, see COPYING.LIB and COPYING files respectively.
/// -- Check out http://mdbtools.sourceforge.net
/// -- ----------------------------------------------------------
/// 
/// -- That file uses encoding UTF-8
/// 
/// CREATE TABLE [SetDescriptions]
/// (
/// [set_name] Text (60)
/// );

#[derive(Debug)]
struct TmSetdescriptions {
    set_name:String,
}

impl TmSetdescriptions {

    fn all(conn: Connection) -> Result<Vec<TmSetdescriptions>, Error> {
	let mut vec: Vec<TmSetdescriptions> = Vec::new();
	match conn.execute("SELECT * FROM TableName", ())? {
            Some(mut cursor) => {
		let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
		let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;
		while let Some(batch) = row_set_cursor.fetch()? {
		    for row_index in 0..batch.num_rows() {
			let set_name = read_string(batch.at(0, row_index))?;
			let obj = TmSetdescriptions {
			    set_name: set_name
			};
			vec.push(obj);
		    };
		}
            },
            None => todo!()
	}
	Ok(vec) // Return the vector of Person objects
    }
}

