use std::usize;

#[derive(Debug)]
pub enum Qtype {
    A,
    CNAME,
    UNKNOWN,
}

#[allow(dead_code)]
impl Qtype {
    pub fn from_num(n: u16) -> Qtype {
        match n {
            1 => Qtype::A,
            5 => Qtype::CNAME,
            _ => Qtype::UNKNOWN,
        }
    }

    pub fn to_num(self) -> u16 {
        match self {
            Qtype::A => 1,
            Qtype::CNAME => 5,
            Qtype::UNKNOWN => 0,
        }
    }
}

#[allow(unused_variables)]
#[allow(dead_code)]
#[derive(Debug)]
pub struct Question {
    pub name: String,
    pub qtype: Qtype,
    pub qclass: u16,
}

impl Question {
    pub fn new(inp: &[u8; 512]) -> Question {
        let quesslice: &[u8] = &inp[12..]; // Skipping the first 12 bytes (DNS header)

        // Mutable variables
        let mut name = String::new();
        let mut start: usize = 0;
        // let mut jump = false;
        // let mut jumperror = false;
        // let mut seen_offsets: Vec<usize> = Vec::new(); // Keep track of visited offsets to prevent circular jumps

        loop {
            // Detect a pointer (0xC0 indicates a pointer)
            // if quesslice[start] & 0xC0 == 0xC0 {
            //     // Calculate the pointer offset
            //     let offset =
            //         ((quesslice[start] as usize & 0x3F) << 8) | quesslice[start + 1] as usize;
            //
            //     // Detect circular jumps
            //     if seen_offsets.contains(&offset) {
            //         jumperror = true;
            //         println!("Error: Circular jump detected! Exiting to prevent a loop.");
            //         break;
            //     }
            //
            //     // Track the offsets we've seen
            //     seen_offsets.push(offset);
            //
            //     // Jump to the pointer location
            //     start = offset;
            //     jump = true;
            //
            //     continue;
            // } else {
            // Extract label length

            let label_length = quesslice[start] as usize;

            if label_length == 0 {
                break; // End of the name
            }

            start += 1;

            // Extract and add the label to the name
            for i in 0..label_length {
                name.push(quesslice[start + i] as char);
            }

            // Move the pointer after the label
            start += label_length;

            // Add a dot between labels, but not after the last label
            if quesslice[start] != 0 && (quesslice[start] & 0xC0) != 0xC0 {
                name.push('.');
            }
            // }
        }

        // Move past the null byte (end of name)
        start += 1;

        // Safely extract the qtype and qclass, handling potential out-of-bounds access
        let num = if start + 2 <= quesslice.len() {
            u16::from_be_bytes(quesslice[start..start + 2].try_into().unwrap())
        } else {
            0 // Invalid case, should handle this more gracefully
        };

        let qtype: Qtype = Qtype::from_num(num);

        start += 2;

        let qclass = if start + 2 <= quesslice.len() {
            u16::from_be_bytes(quesslice[start..start + 2].try_into().unwrap())
        } else {
            0 // Invalid case, should handle this more gracefully
        };

        // Return the constructed Question
        Question {
            name,
            qtype,
            qclass,
        }
    }
}
