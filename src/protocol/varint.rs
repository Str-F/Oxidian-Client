use bytes::{Buf, BufMut, BytesMut};

pub enum VarIntError {
    NotEnoughBytes,
    TooBig,
}

pub fn encode(output: &mut BytesMut, value: i32) {
    let mut value = value as u32;

    while value > 0x7F {
        let byte = (value & 0x7F) as u8;
        output.put_u8(byte | 0x80);
        value >>= 7;
    }
    output.put_u8(value as u8);
}

pub fn decode(input: &BytesMut) -> Result<(i32, usize), VarIntError> {
    let mut output = 0;
    let mut bytes_count = 0;

    loop {
        if input.len() <= bytes_count {
            return Err(VarIntError::NotEnoughBytes);
        }
        let byte = input[bytes_count];
        let data = (byte & 0x7F) as i32;

        output |= data << (bytes_count * 7);

        bytes_count += 1;

        if bytes_count > 5 {
            return Err(VarIntError::TooBig);
        }

        if (byte & 0x80) == 0 {
            break;
        }
    }
    Ok((output, bytes_count))
}

pub fn decode_mut(input: &mut BytesMut) -> Result<(i32), VarIntError> {
    let (ouput, bytes_count) = decode(input)?;
    input.advance(bytes_count);
    Ok(ouput)
}
