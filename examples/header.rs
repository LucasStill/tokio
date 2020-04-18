pub mod header {

    use std::net::SocketAddr;
    use std::time::SystemTime;

    #[derive(Debug)]
    pub struct Header {
        pub addr: SocketAddr,
        pub destination: SocketAddr
    }


    // impl Header {
    pub fn new(addr: SocketAddr, destination: SocketAddr) {    //Need to add more instances st. hop_count etc

        let version: u8 = 001;  //version
        let protocol_type: u8 = 1;
        let tokens: u8 = 1; //TO CHANGE
        let hop_count: u8 = 0;  //Need to increment each time it passes a node by 1, is limit
        let length_message: u32 = 150;

        let timestamp = SystemTime::now();


        let origin_address = addr;  //Need to check size format
        let target_address = destination;   //TO FIND
        let mut sequence_number = 0;    //COULD be persistent during restarts. As header is created, sets value to 0

        //let message_id =          //Need way to create unique IDs

        //OK  let service_type = 1; //Sticks to 1 for the moment because it is only messages

        if test_menu(origin_address.to_string(), hop_count) {
            println!("Header, {}, {}, {}, {}, {}, {:?}, {}, {}, {}", version, protocol_type, tokens, hop_count, length_message, timestamp, origin_address, target_address, sequence_number);
            //return header
        }
    }

    fn test_menu(addr: String, hop_count: u8) -> bool {
        if test_is_valid_ip(addr) && is_hop_valid(hop_count) {
            println!("Header can be created. Information is valid");
            return true
        } else {
            println!("Header can not be created. Information is not valid");
            return false;
        }
    }

    fn test_is_valid_ip(addr: String) -> bool {
        let vec = addr.split(|c| c == '.' || c == ':').collect::<Vec<&str>>();

        /* for s in 0..vec.len() {
             println!("s: {}", vec[s]);
         }*/

        //size of address taking into account suffix after ":"
        if vec.len() != 5 {
            return false;
        }

        let mut check = true;

        for s in 0..vec.len() - 1 {
            let m: i32 = vec[s].parse().unwrap();
            //println!("{}", m);

            if m < 0 || m >= 256 {
                check = false;
            }
        }
        return check;
    }

    fn is_hop_valid(hop_count: u8) -> bool {
        if 0 <= hop_count && hop_count <= 255 { return true; } else { return false; }
    }

    //Increments need be placed in the system loop: TO DO
    fn increment_hop_count(hop_count: u8) -> u8 {
        return hop_count + 1;
    }

    fn increment_sequence_number(sequence_number: u32) -> u32 {
        //increments by one or resets to zero when it reaches the limit of 255

        let new_sequence_number: u32;

        if sequence_number == 255 {
            new_sequence_number = 0;
        } else {
            new_sequence_number = sequence_number + 1;
        }
        return new_sequence_number;
    }
//}
}