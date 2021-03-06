pub fn problem_002() -> u32 {
    let mut previous = 0;
    let mut next = 1;
    let mut sum = 0;

    while next < 4_000_000 {
        if next % 2 == 0 {
            sum += next;
        }
        let tmp = next;
        next = previous + next;
        previous = tmp;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn problem_002_solution() {
        let result = problem_002();
        assert_eq!(result, 4613732);
    }
}
