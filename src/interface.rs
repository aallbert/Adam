pub fn file_to_i8(c: char) -> i8 {
    (c as i8 - b'a' as i8) + 1
}

pub fn rank_to_i8(c: char) -> i8 {
    (c as i8 - b'1' as i8) + 1
}

pub fn i8_to_file(i: i8) -> char {
    (b'a' + (i - 1) as u8) as char
}

pub fn i8_to_rank(i: i8) -> char {
    (b'1' + (i - 1) as u8) as char
}

pub fn abs_diff_u16(a: u16, b: u16) -> u16 {
    let diff = a.wrapping_sub(b);
    let mask = ((a < b) as u16).wrapping_neg(); // 0 wenn a >= b, sonst 0xFFFFFFFF
    (diff ^ mask).wrapping_sub(mask)
}
