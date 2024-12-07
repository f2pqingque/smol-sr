// Convert le bytes into hsr packet
pub fn decode_packet(bytes: &[u8]) -> Result<(u16, Vec<u8>), &str> {
    use std::convert::TryInto;

    if bytes.len() < 16 {
        return Err("byte array too short lmao");
    }

    // Validate the head magic (first 4 bytes)
    let head_magic = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
    assert_eq!(head_magic, 0x9D74C714, "wrong head magic bucko");

    // Extract the cmd id (2 bytes)
    let cmd = u16::from_be_bytes(bytes[4..6].try_into().unwrap());

    // Extract head size (2 bytes)
    let head_size = u16::from_be_bytes(bytes[6..8].try_into().unwrap());

    // Extract body size (4 bytes)
    let body_size = u32::from_be_bytes(bytes[8..12].try_into().unwrap()) as usize;

    // Extract the head data
    let head_start = 12;
    let head_end = head_start + head_size as usize;
    let _head = &bytes[head_start..head_end];

    // Extract the body data
    let body_start = head_end;
    let body_end = body_start + body_size;
    if body_end > bytes.len() {
        return Err("body longer than the byte array??");
    }
    let body = bytes[body_start..body_end].to_vec();

    // Validate the tail magic (last 4 bytes)
    let tail_magic = u32::from_be_bytes(bytes[body_end..body_end + 4].try_into().unwrap());
    assert_eq!(tail_magic, 0xD7A152C8, "go get an actual tail");

    // Log the details of the decoded packet
    // println!("head_magic: 0x{:X}", head_magic);
    println!("got cmd: {}", cmd);
    // println!("head_size: {}", head_size);
    // println!("body_size: {}", body_size);
    // println!("head: {:?}", head);
    println!("the body: {:?}", body);
    // println!("tail_magic: 0x{:X}", tail_magic);

    // Return the command and body data
    Ok((cmd, body))
}

// Convert le cmd & proto into packet
pub fn encode_packet(cmd_id: u16, data: Vec<u8>) -> Vec<u8> {
    // The total length of the packet consists of a fixed header, body, and tail.
    let packet_len = 12 + data.len() + 4;

    // Create a mutable buffer to hold the encoded packet
    let mut buffer = Vec::with_capacity(packet_len);

    // Write head_magic (4 bytes)
    buffer.extend_from_slice(&0x9D74C714u32.to_be_bytes());

    // Write cmd_id (2 bytes)
    buffer.extend_from_slice(&cmd_id.to_be_bytes());

    // Write 2 empty bytes (reserved)
    buffer.extend_from_slice(&0u16.to_be_bytes());

    // Write body_size (4 bytes, the length of the body)
    buffer.extend_from_slice(&(data.len() as u32).to_be_bytes());

    // Write the body (actual data)
    buffer.extend_from_slice(&data);

    // Write tail_magic (4 bytes)
    buffer.extend_from_slice(&0xD7A152C8u32.to_be_bytes());

    println!("sending: {:?}", buffer);
    println!("dummy: {}\n", data.len() == 0);
    buffer
}
