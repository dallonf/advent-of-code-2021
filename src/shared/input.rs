macro_rules! include_lines {
    ($path:tt) => {
        include_str!($path).lines()
    };
}

pub(crate) use include_lines;
