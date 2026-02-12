

pub fn read_uint32(data: &[u8], offset: usize) -> u32 {
    let value:u32 =  data[offset] as u32
        | ((data[offset + 1] as u32) << 8)
        | ((data[offset + 2] as u32) << 16)
        | ((data[offset + 3] as u32) << 24);
    return value;
}

pub fn read_uint32_be(data: &[u8], offset: usize) -> u32 {
    let value:u32 =  ((data[offset] as u32) << 24)
        | ((data[offset + 1] as u32) << 16)
        | ((data[offset + 2] as u32) << 8)
        | (data[offset + 3] as u32);
    return value;
}

pub fn read_uint64(data: &[u8], offset: usize) -> u64 {
    let value:u64 =  data[offset] as u64
        | ((data[offset + 1] as u64) << 8)
        | ((data[offset + 2] as u64) << 16)
        | ((data[offset + 3] as u64) << 24)
        | ((data[offset + 4] as u64) << 32)
        | ((data[offset + 5] as u64) << 40)
        | ((data[offset + 6] as u64) << 48)
        | ((data[offset + 7] as u64) << 56);
    return value;
}

#[warn(dead_code)]
pub fn write_uint32(data: &mut [u8], offset: usize, value: u32) {
    data[offset] = (value & 0xFF) as u8;
    data[offset + 1] = ((value >> 8) & 0xFF) as u8;
    data[offset + 2] = ((value >> 16) & 0xFF) as u8;
    data[offset + 3] = ((value >> 24) & 0xFF) as u8;
}

pub fn read_uleb128(data: &[u8], offset: &mut usize) -> u128 {
    let mut result: u128 = 0;
    let mut shift: u32 = 0;
    loop {
        let byte = data[*offset];
        *offset += 1;
        result |= ((byte & 0x7F) as u128) << shift;
        if (byte & 0x80) == 0 {
            break;
        }
        shift += 7;
    }
    return result;
}

pub fn read_sleb128(data: &[u8], offset: &mut usize) -> i128 {
    let mut result: i128 = 0;
    let mut shift: u32 = 0;
    let mut byte: u8;
    loop {
        byte = data[*offset];
        *offset += 1;
        result |= ((byte & 0x7F) as i128) << shift;
        shift += 7;
        if (byte & 0x80) == 0 {
            break;
        }
    }
    // Sign extend if necessary
    if byte & 0x40 != 0 {
        result -= 1 << shift;
    }
    return result;
}