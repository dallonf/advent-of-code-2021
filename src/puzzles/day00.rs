// Day 00: Template

pub fn part_one() -> String {
    String::from("Hello world!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_answer() {
        let result = part_one();
        assert_eq!(result, "Hello world!");
    }
}
