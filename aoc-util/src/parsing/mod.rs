pub use {
    anyhow::{anyhow, ensure, Context, Result},
    once_cell::sync::OnceCell,
    regex,
};

#[macro_export]
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

#[macro_export]
macro_rules! capture_regex {
    ($fn_name:ident = $regex:literal, $($capture:ty),* $(,)?) => {
        fn $fn_name(input: &[u8]) -> $crate::parsing::Result<( $($capture),* )> {
            static RE: $crate::parsing::OnceCell<$crate::parsing::regex::bytes::Regex> = $crate::parsing::OnceCell::new();
            let regex = RE.get_or_try_init(|| $crate::parsing::regex::bytes::Regex::new($regex))?;
            let captures = regex
                .captures(input)
                .ok_or_else(|| $crate::parsing::anyhow!("No captures was found"))?;
            let mut iter = captures.iter();
            $crate::parsing::ensure!(iter.next().is_some(), "No captures was found");

            Ok(($(
                std::str::from_utf8(
                    iter.next()
                        .ok_or_else(|| $crate::parsing::anyhow!("Insuffisint amount of"))?
                        .ok_or_else(|| $crate::parsing::anyhow!("No match found"))?
                        .as_bytes(),
                )?
                .parse::<$capture>()?
            ),*))
        }
    }
}
