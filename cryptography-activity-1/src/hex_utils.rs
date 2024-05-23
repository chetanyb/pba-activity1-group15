pub struct HexConverter;

impl HexConverter {
    pub fn hex_as_bytes(hex: &str) -> Vec<u8> {
        hex.as_bytes()
            .chunks(2)
            .map(|chunk| {
                u8::from_str_radix(unsafe { std::str::from_utf8_unchecked(chunk) }, 16).unwrap()
            })
            .collect()
    }

    pub fn bytes2hex(bytes: &[u8]) -> String {
        bytes
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>()
            .join("")
    }
}
