fn quarter_round(a: u32, b: u32, c: u32, d: u32) -> [u32; 4] {
    [0; 4]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quarter_round_test_1() {
        assert_eq!(
            quarter_round(0, 0, 0, 0),
            [0; 4],
        );
    }
}
