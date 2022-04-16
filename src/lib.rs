fn quarter_round(mut a: u32, mut b: u32, mut c: u32, mut d: u32) -> [u32; 4] {
    b ^= a.wrapping_add(d).rotate_left(7);
    c ^= b.wrapping_add(a).rotate_left(9);
    d ^= c.wrapping_add(b).rotate_left(13);
    a ^= d.wrapping_add(c).rotate_left(18);

    [a, b, c, d]
}

fn row_round(mut state: [u32; 16]) -> [u32; 16] {
    [state[0], state[1], state[2], state[3]] = quarter_round(state[0], state[1], state[2], state[3]);
    [state[5], state[6], state[7], state[4]] = quarter_round(state[5], state[6], state[7], state[4]);
    [state[10], state[11], state[8], state[9]] = quarter_round(state[10], state[11], state[8], state[9]);
    [state[15], state[12], state[13], state[14]] = quarter_round(state[15], state[12], state[13], state[14]);

    state
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

    #[test]
    fn row_round_test_1() {
        assert_eq!(
            row_round(
                [
                    0x00000001, 0x00000000, 0x00000000, 0x00000000,
                    0x00000001, 0x00000000, 0x00000000, 0x00000000,
                    0x00000001, 0x00000000, 0x00000000, 0x00000000,
                    0x00000001, 0x00000000, 0x00000000, 0x00000000,
                ]
            ),
            [
                0x08008145, 0x00000080, 0x00010200, 0x20500000,
                0x20100001, 0x00048044, 0x00000080, 0x00010000,
                0x00000001, 0x00002000, 0x80040000, 0x00000000,
                0x00000001, 0x00000200, 0x00402000, 0x88000100,
            ]
        );
    }

    #[test]
    fn row_round_test_2() {
        assert_eq!(
            row_round(
                [
                    0x08521bd6, 0x1fe88837, 0xbb2aa576, 0x3aa26365,
                    0xc54c6a5b, 0x2fc74c2f, 0x6dd39cc3, 0xda0a64f6,
                    0x90a2f23d, 0x067f95a6, 0x06b35f61, 0x41e4732e,
                    0xe859c100, 0xea4d84b7, 0x0f619bff, 0xbc6e965a,
                ]
            ),
            [
                0xa890d39d, 0x65d71596, 0xe9487daa, 0xc8ca6a86,
                0x949d2192, 0x764b7754, 0xe408d9b9, 0x7a41b4d1,
                0x3402e183, 0x3c3af432, 0x50669f96, 0xd89ef0a8,
                0x0040ede5, 0xb545fbce, 0xd257ed4f, 0x1818882d,
            ]
        );
    }
}
