use smol::io::{AsyncReadExt, AsyncWriteExt};
use smol::{io, Async};
use std::net::{TcpListener, TcpStream};

// Decode a byte slice into a command and body, validating the packet structure.
async fn decode_bytes(bytes: &[u8]) -> Result<(u16, Vec<u8>), &str> {
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
    let head = &bytes[head_start..head_end];

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
    println!("head_magic: 0x{:X}", head_magic);
    println!("cmd: {}", cmd);
    println!("head_size: {}", head_size);
    println!("body_size: {}", body_size);
    println!("head: {:?}", head);
    println!("body: {:?}", body);
    println!("tail_magic: 0x{:X}", tail_magic);

    // Return the command and body data
    Ok((cmd, body))
}

// Encode cmd id and protobuf
fn encode_packet(cmd_id: u16, data: Vec<u8>) -> Vec<u8> {
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

    buffer
}

// Echo, takes client messages' cmd & body
// then encodes and resends it.
async fn echo(mut stream: Async<TcpStream>) -> io::Result<()> {
    // The full buffer
    let mut buffer = Vec::new();
    // Temporary buffer for reading data from stream
    let mut temp_buffer = vec![0; 1024];

    loop {
        // Read data from the stream and store it in the temporary buffer
        let n = stream.read(&mut temp_buffer).await?;

        if n == 0 {
            // If no data is read, it means the client has closed the connection
            println!("cya later bud");
            break;
        }

        // Append the received data to the main buffer
        buffer.extend_from_slice(&temp_buffer[..n]);

        // Check if the buffer contains a full message by looking for the tail magic
        if let Some(pos) = buffer
            .windows(4)
            .position(|window| window == b"\xD7\xA1\x52\xC8")
        {
            // We found the tail magic, so we can process the message
            let complete_message = &buffer[..pos + 4]; // Include the tail magic

            // Attempt to decode the message
            let (cmd, body) = match decode_bytes(complete_message).await {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("L + ratio: {}", e);
                    buffer.drain(..pos + 4); // Discard the processed portion of the buffer
                    continue;
                }
            };

            // Log the command and body
            println!("{:?}, {:?}", cmd, body);
            println!("got this: {:?}", complete_message);

            // Encode the response packet and send it back to the client
            let buffera: &[u8] = &encode_packet(cmd, body);

            // Log the outgoing response packet
            println!("send this: {:?}", buffera);
            stream.write_all(buffera).await?;

            // Remove the processed message from the buffer
            buffer.drain(..pos + 4); // Discard everything up to and including the tail magic
        }
    }

    Ok(())
}

// Main entry point for the server.
fn main() -> io::Result<()> {
    smol::block_on(async {
        // Bind the server to a local address and port
        let listener = Async::<TcpListener>::bind(([127, 0, 0, 1], 7000))?;
        println!("party at 10pm @ {}", listener.get_ref().local_addr()?);

        // Accept incoming client connections in a loop
        loop {
            let (stream, peer_addr) = listener.accept().await?;
            println!("hi there, {}", peer_addr);

            // Spawn a task to handle the client connection
            smol::spawn(echo(stream)).detach();
        }
    })
}
