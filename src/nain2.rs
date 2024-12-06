

fn fetch_people(connection_string: &str) -> Result<Vec<Person>> {
    let connection = Connection::connect(connection_string)
        .context("Failed to connect to the database")?;

    let query = "SELECT name, email, age, salary FROM people";
    let mut cursor = connection.prepare(query)?.into_cursor()?;

    let mut people = Vec::new();

    // Fetching rows in batches
    while let Some(batch) = cursor.fetch()? {
        for row_index in 0..batch.num_rows() {
            let row = &batch[row_index];
            
            let name: String = row.get(0)?.to_string()?;
            let email: String = row.get(1)?.to_string()?;
            let age: u32 = row.get(2)?.to_u32()?;
            let salary: f64 = row.get(3)?.to_f64()?;

            let person = Person { name, email, age, salary };
            people.push(person);
        }
    }

    Ok(people)
}

fn func2() -> Result<()> {
    let connection_string = "DSN=your_dsn;UID=user;PWD=password";
    let people = fetch_people(connection_string)?;

    for person in people {
        println!("Name: {}, Email: {}, Age: {}, Salary: ${:.2}", 
                 person.name, person.email, person.age, person.salary);
    }

    Ok(())
}
