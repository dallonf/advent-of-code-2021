pub fn split_lines(input: &str) -> Vec<&str> {
    input
        .split_terminator("\n")
        .map(|it| it.trim_end())
        .collect()
}

macro_rules! include_lines {
    ($path:tt) => {
        $crate::shared::input::split_lines(include_str!($path))
    };
}

pub(crate) use include_lines;
