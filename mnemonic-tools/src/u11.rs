use super::BITS_PER_BYTE;

/// structure representing **11 bit unsigned integer**
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct U11(pub u16);

impl U11 {
    pub const SIZE: usize = 11;

    pub fn from_bytes(bytes: &[u8]) -> Vec<Self> {
        let mut bit_index = 0;
        let mut indices = Vec::new();

        while bit_index + U11::SIZE < bytes.len() * BITS_PER_BYTE {
            let byte_index = bit_index / BITS_PER_BYTE;
            let starting_bit = bit_index % BITS_PER_BYTE;

            let mut buffer = u32::from_be_bytes([
                bytes[byte_index],
                bytes.get(byte_index + 1).copied().unwrap_or_default(),
                bytes.get(byte_index + 2).copied().unwrap_or_default(),
                bytes.get(byte_index + 3).copied().unwrap_or_default(),
            ]);
            buffer <<= starting_bit;
            buffer >>= 32 - U11::SIZE;
            indices.push(U11(buffer as u16));
            bit_index += U11::SIZE;
        }
        indices
    }

    pub fn slice_to_bytes(u11s: &[Self]) -> Vec<u8> {
        let expected_length = u11s.len() * U11::SIZE / BITS_PER_BYTE + 1;
        let mut buffer = vec![0u8; expected_length + 4];
        let mut bit_index = 0;

        for u11 in u11s {
            let byte_index = bit_index / 8;
            let starting_bit = bit_index % 8;

            let buffer_4bytes = &mut buffer[byte_index..byte_index + 4];
            let mut buffer_4bytes_u32 = u32::from_be_bytes(buffer_4bytes.try_into().unwrap());

            let mut mask: u32 = u11.0.into();
            mask <<= 32 - U11::SIZE;
            mask >>= starting_bit;
            buffer_4bytes_u32 |= mask;

            buffer_4bytes.copy_from_slice(&buffer_4bytes_u32.to_be_bytes());
            bit_index += U11::SIZE;
        }

        buffer.resize(expected_length, 0);
        buffer
    }
}
