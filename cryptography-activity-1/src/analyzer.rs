use super::hex_utils::HexConverter;
use super::xor_chipher::XorCipher;

pub struct MessageAnalyzer<'a> {
    ciphertexts: Vec<&'a str>,
}

impl<'a> MessageAnalyzer<'a> {
    // Constructor to create a new instance of MessageAnalyzer
    pub fn new(ciphertexts: Vec<&'a str>) -> Self {
        Self { ciphertexts }
    }

    // Method to analyze the ciphertexts using crib dragging
    pub fn analyze(&self) {
        // Convert each ciphertext from hex string to bytes
        let cipher: Vec<Vec<u8>> = self
            .ciphertexts
            .iter()
            .map(|s| HexConverter::hex_as_bytes(s))
            .collect();

        // Iterate over each ciphertext as the target to be compared against others
        for j in 1..self.ciphertexts.len() {
            let mut result = Vec::new();

            // XOR each ciphertext with the target ciphertext and filter the result
            for i in 1..self.ciphertexts.len() {
                if i == j {
                    continue;
                }
                let raw_xor = XorCipher::xor_bytes(&cipher[j], &cipher[i]);
                let filtered_raw: Vec<u8> = raw_xor
                    .into_iter()
                    .map(|ch| if ch.is_ascii_alphabetic() { ch } else { b'_' })
                    .collect();
                result.push(filtered_raw.clone());
            }

            // Determine the minimum length among all ciphertexts
            let min_length = cipher.iter().map(|x| x.len()).min().unwrap();
            let mut plain_text = vec![b'_'; min_length];

            // Analyze each byte position across the results
            for i in 0..min_length {
                let mut count_underscore = 0;
                let mut count_letter = 0;
                let mut current_letter = 0u8;
                let mut is_unique = true;

                // Count the occurrences of letters and underscores
                result
                    .iter()
                    .filter(|bytes| bytes.len() > i)
                    .map(|bytes| bytes[i])
                    .for_each(|letter| {
                        if letter == b'_' {
                            count_underscore += 1;
                        } else {
                            count_letter += 1;
                            if current_letter != 0 && current_letter == letter.to_ascii_lowercase()
                            {
                                // do nothing
                            } else if current_letter != 0
                                && current_letter != letter.to_ascii_lowercase()
                            {
                                is_unique = false;
                            }
                            current_letter = letter.to_ascii_lowercase();
                        }
                    });

                // Determine the character to place in the plaintext
                if count_letter == 1 {
                    plain_text[i] = current_letter;
                } else if count_letter > 1 {
                    if is_unique {
                        plain_text[i] = current_letter;
                    }
                } else {
                    plain_text[i] = b'_';
                }
            }

            // Print the resulting plaintext guess
            println!();
            println!(
                "Guess for message[{}][0..{}]: {}",
                j,
                min_length,
                String::from_utf8_lossy(&plain_text)
            );
        }
    }
}
