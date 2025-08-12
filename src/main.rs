// Import necessary modules from the standard library.
// `std::net::UdpSocket` is used for UDP network communication.
use std::net::UdpSocket;
// `std::fs::File` is used for file system operations, specifically creating and opening files.
use std::fs::File;
// `std::io::Write` trait provides the `write_all` method for writing data to a file.
use std::io::Write;
// `std::io::Result` is a type alias for `Result<T, std::io::Error>`, used for error handling in I/O operations.
use std::io;

/// The main function is the entry point of the Rust program.
fn main() -> io::Result<()> {
    // Define the address and port to which the UDP socket will bind.
    // "127.0.0.1:8080" means it will listen on the local loopback interface (your computer)
    // on port 8080. You can change this to "0.0.0.0:8080" to listen on all available
    // network interfaces.
    let bind_address = "127.0.0.1:8080";

    // Attempt to bind the `UdpSocket` to the specified address.
    // `UdpSocket::bind` returns a `Result`.
    // `.expect()` is used here for simplicity; in a production application, you'd use
    // more robust error handling (e.g., a `match` statement or `?` operator).
    let socket = UdpSocket::bind(bind_address)
        .expect(&format!("Couldn't bind to address {}", bind_address));

    // Print a message indicating that the server is listening.
    println!("UDP Listener started on {}", bind_address);
    println!("Incoming packets will be logged to 'udp_packets.log'");

    // Create or open the file where UDP packet data will be stored.
    // `File::create` will create a new file or truncate an existing one.
    let mut file = File::create("udp_packets.log")
        .expect("Couldn't create or open 'udp_packets.log'");

    // Define a buffer to hold incoming data.
    // A buffer of 1500 bytes is common, as it's a typical Ethernet MTU (Maximum Transmission Unit)
    // size, meaning most single UDP packets won't exceed this.
    let mut buf = [0; 1500];

    // Start an infinite loop to continuously receive UDP packets.
    // `loop {}` creates an infinite loop.
    loop {
        // Attempt to receive a datagram into the buffer.
        // `socket.recv_from(&mut buf)` returns a `Result` containing the number of bytes
        // received and the source address (`SocketAddr`).
        match socket.recv_from(&mut buf) {
            Ok((number_of_bytes, src_addr)) => {
                // If reception is successful:
                // Extract the actual data from the buffer based on the `number_of_bytes`.
                // borrowed slice `&buf[..number_of_bytes]` contains only the received data.
                // scope for borrowed slice is limited to this block.

                let received_data = &buf[..number_of_bytes];

                // Convert the received data to a string for logging (if it's valid UTF-8).
                // `String::from_utf8_lossy` converts bytes to a string, replacing invalid
                // UTF-8 sequences with a Unicode replacement character. This is good for
                // displaying potentially mixed data.
                // JAA: Remove this one, because ours is not a string!
                // whats cow for the data type? 
                
                let data_str = String::from_utf8_lossy(received_data);

                // Print information about the received packet to the console.
                println!("Received {} bytes from {}: {}", number_of_bytes, src_addr, data_str);

                // Prepare the log entry string.
                // It includes the timestamp, source address, and the received data.
                // Change log entry, we only want the data
                let log_entry = format!(
                    "[{}] Received from {}: {}\n",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"), // Add a timestamp with milliseconds
                    src_addr,
                    data_str
                );

                // Write the log entry to the file.
                // `file.write_all()` attempts to write the entire byte slice to the file.
                // `.expect()` is used for basic error handling here.
                file.write_all(log_entry.as_bytes())
                    .expect("Couldn't write to file");

                // Ensure the data is immediately written to disk, not just buffered.
                // This is important for real-time logging and crash recovery.
                file.flush().expect("Couldn't flush file buffer");
            },
            Err(e) => {
                // If an error occurs during reception, print an error message.
                eprintln!("Error receiving packet: {}", e);
                // In a production scenario, you might want to handle specific errors
                // differently or decide whether to continue the loop.
            }
        }
    }
}
