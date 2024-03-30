#[macro_export]
/// This macro will `read a line` from stdin and return it as a `String`
/// There are two ways of using it.
/// parsing a `&str` as a message or simply prompting without notifying.
macro_rules! read_line {
    () => {{
        pub use std::io;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input.trim().to_string()
    }};
    ($prompt:expr) => {{
        println!("{}", $prompt);
        read_line!()
    }};
}
