pub fn is_valid(code: &str) -> bool {
    let it = code.chars().filter(|&c| c != ' ').rev();
    if it.clone().any(|c| !c.is_ascii_digit()) {
        return false;
    }
    let it = it.filter_map(|c| c.to_digit(10));
    if it.clone().count() <= 1 {
        return false;
    }
    let a: u32 = it.clone().step_by(2).sum();
    let b: u32 = it
        .skip(1)
        .step_by(2)
        .map(|n| n * 2)
        .map(|x| if x > 9 { x - 9 } else { x })
        .sum();
    (a + b) % 10 == 0
}