/// Parse a Python fromfile
pub fn parse_fromfile(content: &str, prefix: char) -> Vec<crate::Argument> {
    content
        .lines()
        .map(move |l| crate::Argument::parse(l, prefix))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn empty() {
        let input = "";
        let expected: Vec<crate::Argument> = vec![];
        let actual = parse_fromfile(input, crate::PREFIX);
        assert_eq!(expected, actual);
    }

    #[test]
    fn sample() {
        let input = "--hello
world
@moon.txt
--goodbye
sun";
        let expected: Vec<crate::Argument> = vec![
            crate::Argument::PassThrough("--hello".into()),
            crate::Argument::PassThrough("world".into()),
            crate::Argument::Path("moon.txt".into()),
            crate::Argument::PassThrough("--goodbye".into()),
            crate::Argument::PassThrough("sun".into()),
        ];
        let actual = parse_fromfile(input, crate::PREFIX);
        assert_eq!(expected, actual);
    }
}
