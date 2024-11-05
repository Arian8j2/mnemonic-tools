mod english_wordlist;
pub use english_wordlist::ENGLISH_WORD_LIST;

mod u11;
pub use u11::U11;

use sha2::Digest;

const BITS_PER_BYTE: usize = 8;

const ENTROPY_BITS: usize = 128;

pub const ENTROPY_BYTES: usize = ENTROPY_BITS / BITS_PER_BYTE;

const CHECKSUM_BITS: usize = 4;

pub const WORD_LEN: usize = (ENTROPY_BITS + CHECKSUM_BITS) / U11::SIZE;

pub fn indices_to_mnemonic_words(indices: &[U11]) -> Vec<&'static str> {
    indices
        .iter()
        .map(|index| {
            *english_wordlist::ENGLISH_WORD_LIST
                .get(index.0 as usize)
                .unwrap()
        })
        .collect()
}

/// takes entropy and calculates checksum and turns the bytes to wordlist indices
pub fn entropy_to_indices(entropy: &[u8]) -> Vec<U11> {
    entropy_and_checksum_to_indices(entropy, calculate_entropy_checksum(entropy))
}

/// turns entropy + checksum bytes to wordlist indices
pub fn entropy_and_checksum_to_indices(entropy: &[u8], checksum: u8) -> Vec<U11> {
    assert_eq!(entropy.len(), ENTROPY_BYTES);
    let mut entropy_with_checksum = entropy.to_vec();
    entropy_with_checksum.push(checksum);
    U11::from_bytes(&entropy_with_checksum)
}

pub fn calculate_entropy_checksum(entropy: &[u8]) -> u8 {
    let entropy_hash = sha2::Sha256::digest(entropy);
    entropy_hash[0] & 0xf0
}

/// # Returns
/// original entropy bytes and single byte containing four bits of checksum
pub fn mnemonic_words_to_entropy(words: &[&str]) -> anyhow::Result<(Vec<u8>, u8)> {
    let all_words = english_wordlist::ENGLISH_WORD_LIST;

    let indices = words
        .iter()
        .map(|word| {
            all_words
                .iter()
                .position(|w| w == word)
                .map(|x| U11(x as u16))
        })
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| anyhow::anyhow!("Invalid word"))?;

    let buffer = U11::slice_to_bytes(&indices);
    Ok((buffer[..ENTROPY_BYTES].to_vec(), buffer[ENTROPY_BYTES]))
}

pub fn is_mnemonic_valid(words: &[&str]) -> bool {
    let Ok((entropy, checksum)) = mnemonic_words_to_entropy(words) else {
        return false;
    };
    calculate_entropy_checksum(&entropy) == checksum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_entropy_to_indices() {
        let entropy = [
            173, 187, 102, 254, 134, 8, 40, 224, 45, 61, 48, 132, 130, 242, 129, 77,
        ];
        let indices = entropy_to_indices(&entropy);
        assert_eq!(indices[0], U11(1389));
        assert_eq!(indices[1], U11(1753));
    }

    #[test]
    fn test_entropy_to_mnemonic() {
        let entropy = [
            42, 23, 157, 78, 23, 28, 210, 247, 84, 188, 186, 146, 239, 208, 84, 94,
        ];
        let expected_mnemonic = [
            "claw", "rude", "fatal", "comfort", "snake", "knife", "fancy", "nuclear", "napkin",
            "leader", "bench", "rug",
        ];
        let indices = entropy_to_indices(&entropy);
        assert_eq!(indices_to_mnemonic_words(&indices), expected_mnemonic);
    }

    #[test]
    fn test_mnemonic_to_entropy() {
        let mnemonic = [
            "claw", "rude", "fatal", "comfort", "snake", "knife", "fancy", "nuclear", "napkin",
            "leader", "bench", "rug",
        ];
        let expected_entropy = [
            42, 23, 157, 78, 23, 28, 210, 247, 84, 188, 186, 146, 239, 208, 84, 94,
        ];
        let (entropy, checksum) = mnemonic_words_to_entropy(&mnemonic).unwrap();
        assert_eq!(entropy, expected_entropy);
        assert_eq!(checksum, calculate_entropy_checksum(&entropy));
    }

    #[test]
    fn test_mnemonic_valid() {
        assert_eq!(
            is_mnemonic_valid(&[
                "claw", "rude", "fatal", "comfort", "snake", "knife", "fancy", "nuclear", "napkin",
                "leader", "bench", "rug",
            ]),
            true
        );
        assert_eq!(
            is_mnemonic_valid(&[
                "lion", "rude", "fatal", "comfort", "snake", "knife", "fancy", "nuclear", "napkin",
                "leader", "bench", "rug",
            ]),
            false
        );
        assert_eq!(
            is_mnemonic_valid(&[
                "harsh", "system", "vendor", "betray", "cloud", "firm", "space", "federal", "loud",
                "disorder", "edit", "airport"
            ]),
            true
        )
    }

    #[test]
    fn test_generate_similar_mnemonic() {
        let mut words = "satisfy spend denial mammal salon trade monster echo until stand say aunt"
            .split(" ")
            .collect::<Vec<_>>();

        // change 'mammal' to 'lion'
        words[3] = "lion";

        assert_eq!(is_mnemonic_valid(&words), false);
        let (new_entropy, _) = mnemonic_words_to_entropy(&words).unwrap();
        let indices = entropy_to_indices(&new_entropy);
        let new_words = indices_to_mnemonic_words(&indices);
        assert_eq!(is_mnemonic_valid(&new_words), true);
        assert_eq!(words[..words.len() - 1], new_words[..new_words.len() - 1]);
        assert_ne!(words.last().unwrap(), new_words.last().unwrap());
    }
}
