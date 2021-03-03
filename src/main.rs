use std::str;

const LOOKUP_TABLE: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
    'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
    'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
    'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'
];

const PADDING: char = '=';

fn encode_chunks(chunks: &[u8]) -> Vec<char> {
    let mut v = Vec::new();
    match chunks.len() {
        3 => {
            v.push(LOOKUP_TABLE[(chunks[0] >> 2) as usize]);
            v.push(LOOKUP_TABLE[(((chunks[0] & 0b00000011) << 4) | chunks[1] >> 4) as usize]);
            v.push(LOOKUP_TABLE[(((chunks[1] & 0b00001111) << 2) | ((chunks[2] & 0b11000000) >> 6)) as usize]);
            v.push(LOOKUP_TABLE[(chunks[2] & 0b00111111) as usize]);
        },
        2 => {
            v.push(LOOKUP_TABLE[(chunks[0] >> 2) as usize]);
            v.push(LOOKUP_TABLE[(((chunks[0] & 0b00000011) << 4) | chunks[1] >> 4) as usize]);
            v.push(LOOKUP_TABLE[((chunks[1] & 0b00001111) << 2) as usize]);
            v.push(PADDING);
        },
        1 => {
            v.push(LOOKUP_TABLE[(chunks[0] >> 2) as usize]);
            v.push(LOOKUP_TABLE[((chunks[0] & 0b00000011) << 4) as usize]);
            v.push(PADDING);
            v.push(PADDING);
        },
        _ => {}
    }
    v
}

fn decode_string(input: &str) -> Vec<u8> {
    let filtered = input.chars().map(|c| if c == PADDING { 'A' } else { c }).collect::<Vec<char>>();
    let mut v: Vec<u8> = Vec::new();

    for chunk in filtered.chunks(4) {
        let mut n = LOOKUP_TABLE.iter().position(|&x| x == chunk[0]).unwrap() << 18;
        let n1 = LOOKUP_TABLE.iter().position(|&x| x == chunk[1]).unwrap() << 12;
        let n2 = LOOKUP_TABLE.iter().position(|&x| x == chunk[2]).unwrap() << 6;
        let n3 = LOOKUP_TABLE.iter().position(|&x| x == chunk[3]).unwrap();

        n = n + n1 + n2 + n3;
        v.push(((n >> 16) & 0xFF) as u8);
        v.push(((n >> 8) & 0xFF) as u8);
        v.push((n & 0xFF) as u8);
    }

    v
}

fn base64_decode(data: &str) -> String {
    if data.len() % 4 != 0 {
        String::from("Incompatible String")
    } else {
        let v: Vec<u8> = decode_string(data);
        str::from_utf8(&v).unwrap().to_owned()
    }
}

fn base64(data: &str) -> String {
    let byte_array: &[u8] = data.as_bytes();
    let mut v: Vec<char> = Vec::new();
    for octet_array in byte_array.chunks(3) {
        v.extend(encode_chunks(octet_array))
    }
    return v.into_iter().collect::<String>();
}

fn main() {
    println!("Hello, World!");
}
