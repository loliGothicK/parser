#[allow(dead_code)]
struct Input {
    pos: u32,
    text: String,
}

#[allow(dead_code)]
struct Ast {}

#[allow(dead_code)]
struct Parser {}

// any_char, plus とかの parser を共通の型で縛りたい

#[allow(dead_code)]
fn any_char(input: &str) -> Option<(char, &str)> {
    input.chars().next().map(|first| (first, &input[1..]))
}

// 条件を渡すとパーサーを作ってくれる
#[allow(dead_code)]
fn sat<'a>(
    pred: impl Fn(char) -> bool,
) -> impl FnOnce(&'a str) -> Option<(char, &'a str)> {
    move |input| -> Option<(char, &'a str)> {
        any_char(input).and_then(|(parsed, rest)| pred(parsed).then(|| (parsed, rest)))
    }
}

#[allow(dead_code)]
fn is_num(input: char) -> bool {
    matches!(input, '0'..='9')
}

#[allow(dead_code)]
fn is_plus(input: char) -> bool {
    matches!(input, '+')
}

#[allow(dead_code)]
fn is_factor(input: char) -> bool {
    matches!(input, '*')
}

#[allow(dead_code)]
fn plus(input: &str) -> Option<(char, &str)> {
    let plus = sat(is_plus);
    plus(input)
}

#[allow(dead_code)]
fn factor(input: &str) -> Option<(char, &str)> {
    let plus = sat(is_factor);
    plus(input)
}

#[allow(dead_code)]
fn num(input: &str) -> Option<(char, &str)> {
    let plus = sat(is_num);
    plus(input)
}

// parse many digit "3333a"
// ((many digit) "3333a") -> Some(("3333","a"))
#[allow(dead_code)]
fn many<'a>(
    parser: impl Fn(&'a str) -> Option<(char, &'a str)>,
) -> impl FnOnce(&'a str) -> Option<(String, &'a str)> {
    move |input| {
        let mut result = String::new();
        result.reserve(input.len());
        let mut target = input;
        while let Some((accecpted, rest)) = parser(target) {
            result.push(accecpted);
            target = rest;
        }
        (!result.is_empty()).then(|| (result, target))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn any_char_test() {
        let actual = any_char("test");
        assert_eq!(actual, Some(('t', "est")));
    }

    #[test]
    fn any_char_test_empty() {
        let actual = any_char("");
        assert_eq!(actual, None);
    }

    #[test]
    fn any_char_test_single() {
        let actual = any_char("a");
        assert_eq!(actual, Some(('a', "")));
    }

    #[test]
    fn plus_test() {
        let actual = plus("+");
        assert_eq!(actual, Some(('+', "")));
    }

    #[test]
    fn plus_test_with_rest() {
        let actual = plus("+12");
        assert_eq!(actual, Some(('+', "12")));
    }

    #[test]
    fn not_included_plus() {
        let actual = plus("12");
        assert_eq!(actual, None);
    }

    #[test]
    fn many_parse() {
        let many_parser = many(num);
        let actual = many_parser("123a");
        assert_eq!(actual, Some(("123".to_string(), "a")));
    }
}
