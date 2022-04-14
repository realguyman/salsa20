fn quarter_round(mut a: u32, mut b: u32, mut c: u32, mut d: u32) -> [u32; 4] {
    b ^= a.wrapping_add(d).rotate_left(7);
    c ^= b.wrapping_add(a).rotate_left(9);
    d ^= c.wrapping_add(b).rotate_left(13);
    a ^= d.wrapping_add(c).rotate_left(18);

    [a, b, c, d]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quarter_round_test_1() {
        assert_eq!(
            quarter_round(0x00000000, 0x00000000, 0x00000000, 0x00000000),
            [0x00000000, 0x00000000, 0x00000000, 0x00000000],
        );
    }

    #[test]
    fn quarter_round_test_2() {
        assert_eq!(
            quarter_round(0x00000001, 0x00000000, 0x00000000, 0x00000000),
            [0x08008145, 0x00000080, 0x00010200, 0x20500000],
        );
    }

    #[test]
    fn quarter_round_test_3() {
        assert_eq!(
            quarter_round(0x00000000, 0x00000001, 0x00000000, 0x00000000),
            [0x88000100, 0x00000001, 0x00000200, 0x00402000],
        );
    }

    #[test]
    fn quarter_round_test_4() {
        assert_eq!(
            quarter_round(0x00000000, 0x00000000, 0x00000001, 0x00000000),
            [0x80040000, 0x00000000, 0x00000001, 0x00002000],
        );
    }

    #[test]
    fn quarter_round_test_5() {
        assert_eq!(
            quarter_round(0x00000000, 0x00000000, 0x00000000, 0x00000001),
            [0x00048044, 0x00000080, 0x00010000, 0x20100001],
        );
    }

    #[test]
    fn quarter_round_test_6() {
        assert_eq!(
            quarter_round(0xe7e8c006, 0xc4f9417d, 0x6479b4b2, 0x68c67137),
            [0xe876d72b, 0x9361dfd5, 0xf1460244, 0x948541a3],
        );
    }

    #[test]
    fn quarter_round_test_7() {
        assert_eq!(
            quarter_round(0xd3917c5b, 0x55f1c407, 0x52a58a7a, 0x8f887a3b),
            [0x3e2f308c, 0xd90a8f36, 0x6ab2a923, 0x2883524c],
        );
    }
}
