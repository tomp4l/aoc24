#[derive(Debug, PartialEq, Eq)]
pub struct DayResult {
    pub part1: String,
    pub part2: Option<String>,
}

pub trait Day {
    fn run(&self, input: String) -> Result<DayResult, String>;
}
