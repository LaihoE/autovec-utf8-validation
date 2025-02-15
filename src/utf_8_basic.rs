pub fn validate_utf8(bytes: &[u8]) -> bool {
    if bytes.is_empty() {
        return true;
    }
    if (bytes[0] & 0b11000000) == 0b10000000 {
        return false;
    }

    let mut is_valid = true;
    for w in bytes.windows(5) {
        let is_1_long = w[0] <= 127;
        let is_2_long = (w[0] & 0b11100000) == 0b11000000;
        let is_3_long = (w[0] & 0b11110000) == 0b11100000;
        let is_4_long = (w[0] & 0b11111000) == 0b11110000;

        let first_is_cont = (w[0] & 0b11000000) == 0b10000000;
        let second_is_cont = (w[1] & 0b11000000) == 0b10000000;
        let third_is_cont = (w[2] & 0b11000000) == 0b10000000;
        let fourth_is_cont = (w[3] & 0b11000000) == 0b10000000;
        let fift_is_cont = (w[4] & 0b11000000) == 0b10000000;

        let one_long_ok = is_1_long & !second_is_cont;
        let two_long_ok = is_2_long & second_is_cont & !third_is_cont;
        let three_long_ok = is_3_long & second_is_cont & third_is_cont & !fourth_is_cont;
        let four_long_ok = is_4_long & second_is_cont & third_is_cont & fourth_is_cont & !fift_is_cont;

        let two_overlong = is_2_long & ((w[0] & 0b00111110) == 0);
        let three_overlong = is_3_long & ((w[0] & 0b00001111 == 0) & ((w[1] & 0b00100000) == 0));
        let four_overlong = is_4_long & ((w[0] & 0b00001111 == 0) & ((w[1] & 0b00110000) == 0));
        let too_large = (is_4_long & (w[0] > 0xF4)) | (is_4_long & (w[0] == 0xF4) & (w[1] > 0b10001111));
        let is_surrogate = (w[0] == 0b11101101) & (w[1] >> 5 == 0b101);

        is_valid &= (one_long_ok | two_long_ok | three_long_ok | four_long_ok | first_is_cont)
            & !(is_surrogate | two_overlong | three_overlong | four_overlong | too_large);
    }
    // Handle tail
    let start = find_last_non_cont(&bytes[..bytes.len() - bytes.len() % 5]);
    is_valid && std::str::from_utf8(&bytes[start..]).is_ok()
}

pub fn find_last_non_cont(bytes: &[u8]) -> usize {
    let mut idx = 0;
    for i in (0..bytes.len()).rev() {
        let is_1_long = bytes[i] <= 127;
        let is_2_long = (bytes[i] & 0b11100000) == 0b11000000;
        let is_3_long = (bytes[i] & 0b11110000) == 0b11100000;
        let is_4_long = (bytes[i] & 0b11111000) == 0b11110000;
        if is_1_long | is_2_long | is_3_long | is_4_long {
            idx = i;
            break;
        }
        idx = i;
    }
    idx
}
