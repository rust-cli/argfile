/// Parse a Microsoft response file
///
/// Quoting is subject to
/// [`CommandLineToArgvW`](https://learn.microsoft.com/en-us/cpp/c-language/parsing-c-command-line-arguments?view=msvc-170)
///
/// See [Microsoft response files](https://docs.microsoft.com/en-us/cpp/build/reference/at-specify-a-compiler-response-file?view=msvc-170).
#[cfg(feature = "response")]
pub fn parse_response(content: &str, _prefix: char) -> Vec<crate::Argument> {
    winsplit::split(content)
        .into_iter()
        .map(|s| crate::Argument::PassThrough(s.into()))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        let input = "";
        let expected: Vec<crate::Argument> = vec![];
        let actual = parse_response(input, crate::PREFIX);
        assert_eq!(expected, actual);
    }

    #[test]
    fn sample() {
        let input = r#"--hello world
@moon.txt
--goodbye "walker texas"
'single'
sun
c:\Windows
# comment
"#;
        let expected: Vec<crate::Argument> = vec![
            crate::Argument::PassThrough("--hello".into()),
            crate::Argument::PassThrough("world".into()),
            crate::Argument::PassThrough("@moon.txt".into()),
            crate::Argument::PassThrough("--goodbye".into()),
            crate::Argument::PassThrough("walker texas".into()),
            crate::Argument::PassThrough("'single'".into()),
            crate::Argument::PassThrough("sun".into()),
            crate::Argument::PassThrough("c:\\Windows".into()),
            crate::Argument::PassThrough("#".into()),
            crate::Argument::PassThrough("comment".into()),
        ];
        let actual = parse_response(input, crate::PREFIX);
        assert_eq!(expected, actual);
    }
}
