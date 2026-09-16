use bytes::{Buf, BufMut, BytesMut};

#[derive(Debug)]
pub enum VarIntError {
    NotEnoughBytes,
    TooBig,
}

impl std::fmt::Display for VarIntError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VarIntError::NotEnoughBytes => write!(f, "Not enough bytes"),
            VarIntError::TooBig => write!(f, "Value is too big"),
        }
    }
}

impl std::error::Error for VarIntError {}

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

pub fn decode_mut(input: &mut BytesMut) -> Result<i32, VarIntError> {
    let (ouput, bytes_count) = decode(input)?;
    input.advance(bytes_count);
    Ok(ouput)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn varint_encode_normal() {
        let mut buffer = BytesMut::new();
        let value = 300;
        encode(&mut buffer, value);
        let decoded = decode(&mut buffer).unwrap().0;
        assert_eq!(value, decoded);
    }

    #[test]
    fn varint_encode_zero() {
        let mut buffer = BytesMut::new();
        let value = 0;
        encode(&mut buffer, value);
        let decoded = decode(&mut buffer).unwrap().0;
        assert_eq!(value, decoded);
    }

    #[test]
    fn varint_encode_negative() {
        let mut buffer = BytesMut::new();
        let value = -300;
        encode(&mut buffer, value);
        let decoded = decode(&mut buffer).unwrap().0;
        assert_eq!(value, decoded);
    }

    #[test]
    fn varint_encode_not_enough_bytes() {
        let mut buffer = BytesMut::new();
        let value = 300;
        encode(&mut buffer, value);
        buffer.truncate(1);
        let result = decode(&mut buffer);
        assert!(matches!(result, Err(VarIntError::NotEnoughBytes)));
    }
}
