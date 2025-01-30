#[cfg(test)]
mod tests {
    use crate::kuznechik;

    #[test]
    fn test_xor_blocks() {
        let mut a = [0b10101010, 0b11110000];
        let b = [0b11001100, 0b00110011];

        kuznechik::xor_blocks(&mut a, b);

        assert_eq!(a, [0b01100110, 0b11000011]);
    }

}