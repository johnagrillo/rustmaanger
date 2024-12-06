
fn read_decimal(bytes: Option<&[u8]>) -> std::result::Result<Decimal, Box<dyn Error>> {
    let byte_array = bytes.ok_or_else(|| anyhow::anyhow!("Failed to fetch decimal"))?;

    // Check if there's enough bytes (e.g., assuming 16 bytes for a Decimal)
    if byte_array.len() < 16 {
        return Err(anyhow::anyhow!("Insufficient bytes for Decimal").into());
    }

    // Convert the byte array into an array of bytes suitable for Decimal
    let array: [u8; 16] = byte_array.try_into().map_err(|_| {
        anyhow::anyhow!("Invalid byte slice for Decimal: expected 16 bytes")
    })?;

    // Convert the byte array to i128 (assuming little-endian)
    let value = i128::from_le_bytes(array);

    // Create a Decimal from the i128 value
    let decimal = Decimal::from(value);
    
    Ok(decimal)
}


fn read_bool(bytes: Option<&[u8]>) -> std::result::Result<bool, Box<dyn Error>> {
    // Attempt to retrieve the byte slice, returning an error if None
    let byte_array = bytes.ok_or_else(|| anyhow::anyhow!("Failed to fetch bool"))?;

    // Check if there's at least one byte to read
    if byte_array.len() < 1 {
        return Err(anyhow::anyhow!("Insufficient bytes for bool").into());
    }

    // Read the first byte and interpret it as a bool
    let byte = byte_array[0];
    match byte {
        0 => Ok(false), // 0 represents false
        1 => Ok(true),  // 1 represents true
        _ => Err(anyhow::anyhow!("Invalid byte for bool").into()), // Handle unexpected byte values
    }
}




fn read_f32(bytes: Option<&[u8]>) -> Result<f32, Box<dyn Error>> {
    let float_bytes = bytes.ok_or_else(|| anyhow::anyhow!("Failed to fetch f32"))?.to_vec();

    // Assuming the float is stored as f32 in bytes
    if float_bytes.len() < 4 {
        return Err(anyhow::anyhow!("Insufficient bytes for f32").into());
    }
    let array: [u8; 4] = float_bytes.try_into().map_err(|_| anyhow::anyhow!("Invalid byte slice for f32"))?;
    Ok(f32::from_le_bytes(array))
}

fn read_f64(bytes: Option<&[u8]>) -> Result<f64, Box<dyn Error>>{
    let float_bytes = bytes.ok_or_else(|| anyhow::anyhow!("Failed to fetch f64"))?.to_vec();

    // Assuming the float is stored as f64 in bytes
    if float_bytes.len() < 8 {
        return Err(anyhow::anyhow!("Insufficient bytes for f64").into());
    }
    let array: [u8; 8] = float_bytes.try_into().map_err(|_| anyhow::anyhow!("Invalid byte slice for f64"))?;
    Ok(f64::from_le_bytes(array))
}

fn read_u8(bytes: Option<&[u8]>) -> Result<u8, Box<dyn Error>> {
    let age_bytes = bytes.ok_or_else(|| anyhow::anyhow!("Failed to fetch u8"))?
        .to_vec();

    // stored as u8 in bytes
    if age_bytes.len() < 1 {
        return Err(anyhow::anyhow!("Insufficient bytes for 8").into());
    }
    let array: [u8; 1] = age_bytes.try_into().map_err(|_| anyhow::anyhow!("Invalid byte slice for u8"))?;
    Ok(u8::from_le_bytes(array))
}


fn read_u16(bytes: Option<&[u8]>) -> Result<u16, Box<dyn Error>>{
    let age_bytes = bytes.ok_or_else(|| anyhow::anyhow!("Failed to fetch u32"))?
        .to_vec();

    // stored as u16 in bytes
    if age_bytes.len() < 2 {
        return Err(anyhow::anyhow!("Insufficient bytes for u16").into());
    }
    let array: [u8; 2] = age_bytes.try_into().map_err(|_| anyhow::anyhow!("Invalid byte slice for u16"))?;
    Ok(u16::from_le_bytes(array))
}

fn read_u32(bytes: Option<&[u8]>) -> Result<u32, Box<dyn Error>> {
    let age_bytes = bytes.ok_or_else(|| anyhow::anyhow!("Failed to fetch u32"))?
        .to_vec();

    // Assuming age is stored as u32 in bytes
    if age_bytes.len() < 4 {
        return Err(anyhow::anyhow!("Insufficient bytes for u32").into());
    }
    let array: [u8; 4] = age_bytes.try_into().map_err(|_| anyhow::anyhow!("Invalid byte slice for u32"))?;
    Ok(u32::from_le_bytes(array))
}



fn read_u64(bytes: Option<&[u8]>) -> Result<u64, Box<dyn Error>> {
    // Ensure the bytes are present
    //let athlete_id_bytes = &[0..8];


    let age_bytes = bytes.unwrap_or(&[]);

    println!("{}", age_bytes.len());

    // Check that the byte slice has the required length of 8 bytes
    if age_bytes.len() != 8 {
        return Err(anyhow::anyhow!("Insufficient bytes for u64, expected 8 bytes").into());
    }


    // Convert to a fixed-size array
    let array: [u8; 8] = age_bytes.try_into().map_err(|_| anyhow::anyhow!("Invalid byte slice for u64"))?;

    // Return the u64 value, assuming the bytes are in little-endian order
    Ok(u64::from_le_bytes(array))
}



fn read_string(bytes: Option<&[u8]>) -> Result<String, Box<dyn Error>> {
    
    let str_bytes = bytes.ok_or_else(|| anyhow!("Failed to fetch string"))?
	.to_vec();
    
    let str = String::from_utf8(str_bytes)
	.unwrap_or_else(|_| String::from("Invalid UTF-8"));
    
    Ok(str)
}

fn read_datetime(option_bytes: Option<&[u8]>) -> Result<NaiveDateTime, FetchError> {
    let bytes = match option_bytes {
        Some(b) => b,
        None => return Err(FetchError("Received None for datetime")),
    };

    if bytes.len() < 8 {
        return Err(FetchError("Insufficient bytes for conversion"));
    }

    // Convert the first 8 bytes to an i64 (timestamp in milliseconds)
    let millis = i64::from_ne_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5], bytes[6], bytes[7],
    ]);

    // Create a NaiveDateTime from the timestamp in milliseconds
    NaiveDateTime::from_timestamp_millis(millis)
        .ok_or(FetchError("Invalid timestamp")) // Handle possible invalid timestamps
}



fn read_datetime2(bytes: &[u8]) -> Result<NaiveDateTime, FetchError> {
    if bytes.is_empty() {
        return Err(FetchError("Received empty bytes for datetime"));
    }

    if bytes.len() < 8 {
        return Err(FetchError("Insufficient bytes for conversion"));
    }

    // Convert the bytes to an i64 (timestamp in milliseconds)
    let millis = i64::from_ne_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5], bytes[6], bytes[7],
    ]);

    // Create a NaiveDateTime from the timestamp in milliseconds
    let datetime = NaiveDateTime::from_timestamp_millis(millis)
        .ok_or(FetchError("Invalid timestamp"))?; // Handle possible invalid timestamps

    Ok(datetime)
}



