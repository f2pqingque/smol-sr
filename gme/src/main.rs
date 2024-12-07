use smol::io::{AsyncReadExt, AsyncWriteExt};
use smol::{io, Async};
use std::net::{TcpListener, TcpStream};

mod conn;
mod handlers;
mod pk;
mod util;

async fn mephistopheles(mut stream: Async<TcpStream>) -> io::Result<()> {
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
            let (cmd, body) = pk::decode_packet(complete_message).unwrap();

            // Encode the response packet
            let rsp = conn::ping_pong(cmd, body);

            if rsp.len() == 0 {
                println!("unhandled {}\n", cmd);
                // Don't send it
                buffer.drain(..pos + 4);
                continue;
            };

            // Send response packet
            stream.write_all(&rsp).await?;

            buffer.drain(..pos + 4);
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
            smol::spawn(mephistopheles(stream)).detach();
        }
    })
}
