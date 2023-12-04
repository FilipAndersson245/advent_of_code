#[doc(hidden)]
pub use anyhow;

#[doc(hidden)]
pub use once_cell::sync::OnceCell;

#[doc(hidden)]
pub use regex::Regex;

#[macro_export]
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: $crate::OnceCell<$crate::Regex> = $crate::OnceCell::new();
        RE.get_or_init(|| $crate::Regex::new($re).unwrap())
    }};
}

#[macro_export]
/// this macro generate a function that generate a tuple of captures
/// capture_regex!(test = r"(\d+),(\d+)", i32, i32);
macro_rules! capture_regex {
    ($fn_name:ident = $regex:literal, $($capture:ty),* $(,)?) => {
        fn $fn_name(input: &str) -> $crate::anyhow::Result<( $($capture),* )> {
            static RE: $crate::OnceCell<$crate::Regex> = $crate::OnceCell::new();
            let regex = RE.get_or_try_init(|| $crate::Regex::new($regex))?;
            let captures = regex
                .captures(input)
                .ok_or_else(|| $crate::anyhow::anyhow!("No captures was found"))?;
            let mut iter = captures.iter();
            $crate::anyhow::ensure!(iter.next().is_some(), "No captures was found");

            Ok(($(

                    iter.next()
                        .ok_or_else(|| $crate::anyhow::anyhow!("Insuffisint amount of"))?
                        .ok_or_else(|| $crate::anyhow::anyhow!("No match found"))?
                        .as_str()
                .parse::<$capture>()?
            ),*))
        }
    }
}

#[macro_export]
macro_rules! capture_regex_line {
    ($fn_name:ident = $regex:literal, $($capture:ty),* $(,)?) => {
        fn $fn_name(input: &str) -> $crate::anyhow::Result<Vec<( $($capture),* )>> {
            capture_regex!(inner = $regex, $($capture),*);
            input
                .lines()
                .map(|line| inner(line))
                .collect::<$crate::anyhow::Result<Vec<( $($capture),* )>>>()
        }


    }
}

#[cfg(test)]
mod tests {
    use super::*;

    capture_regex_line!(test = r"(\d+) (\d+) (\d+)", i32, i32, i32);

    #[test]
    fn test_name() {
        let a = "1 2 3\n4 5 6\n7 8 9";
        let res = test(a).unwrap();
        assert_eq!(res, vec![(1, 2, 3), (4, 5, 6), (7, 8, 9)]);
    }
}
