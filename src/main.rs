use std::net::UdpSocket;
use std::time::Duration;

mod header;
use header::Header;

mod domain;
mod response;

mod question;
use question::Question;
use response::Response;

fn main() {
    let mut dns_query: [u8; 512] = [0; 512];
    let socket = UdpSocket::bind("0.0.0.0:3003").unwrap();
    socket
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    println!("DNS server listening on port 53...");

    loop {
        match socket.recv_from(&mut dns_query) {
            Ok((amt, src)) => {
                println!("Received {} bytes from {}", amt, src);

                // Process the request (parse and build response)
                let response = handle_request(&dns_query);

                let mut jnd = 511;
                while response[jnd] == 0 {
                    jnd -= 1;
                }

                // Send the processed response back to the client
                socket.send_to(&response[0..=jnd], src).unwrap();
                println!("Response sent to {}", src);
            }
            Err(e) => eprintln!("Failed to receive request: {:?}", e),
        }
    }

    // dns_query[0..29].copy_from_slice(&[
    //     0x12, 0x34, // Transaction ID
    //     0x01, 0x00, // Flags
    //     0x00, 0x01, // Questions: 1
    //     0x00, 0x00, // Answer RRs: 0
    //     0x00, 0x00, // Authority RRs: 0
    //     0x00, 0x00, // Additional RRs: 0
    //     // Question section: "example.com"
    //     0x07, 0x65, 0x78, 0x61, 0x6D, 0x70, 0x6C, 0x65, // "example"
    //     0x03, 0x63, 0x6F, 0x6D, // "com"
    //     0x00, // Null terminator
    //     0x00, 0x01, // query type
    //     0x00, 0x01, // query class
    // ]);
}

fn handle_request(dns_query: &[u8; 512]) -> [u8; 512] {
    let header = Header::new(&dns_query);
    let question = Question::new(&dns_query);

    println!("{:#?}", header);
    println!("{:#?}", question);

    let mut res: Response = Response { ansbuf: [0; 512] };
    res.new(question, *dns_query);
    println!("Answer Buffer: {:?}", res.ansbuf);

    res.ansbuf
}
