use super::hex_utils::HexConverter;

pub struct XorCipher;

impl XorCipher {
    pub fn xor_hex_strings(a: &str, b: &str) -> Vec<u8> {
        Self::xor_bytes(
            &HexConverter::hex_as_bytes(a),
            &HexConverter::hex_as_bytes(b),
        )
    }

    pub fn xor_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
        a.iter().zip(b.iter()).map(|(x, y)| x ^ y).collect()
    }

    pub fn encrypt(key: &[u8], msg: &str) -> String {
        let c = Self::xor_bytes(key, msg.as_bytes());
        HexConverter::bytes2hex(&c)
    }

    pub fn decrypt(key: &[u8], cipher: &str) -> String {
        let cipher_bytes = HexConverter::hex_as_bytes(cipher);
        String::from_utf8_lossy(&Self::xor_bytes(key, &cipher_bytes)).to_string()
    }
}
