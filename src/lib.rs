#[cfg(test)]
mod tests {
    fn quarter_round_test_1() {
        assert_eq!(
            quarter_round(0, 0, 0, 0),
            [4; 0],
        );
    }
}
