use crate::lexer::token::Token;

pub fn print_difference(input: Vec<Token>, expected: Vec<Token>) -> String {
    const RED: &str = "\x1b[31m";
    const GREEN: &str = "\x1b[32m";
    const CLEAR: &str = "\x1b[0m";
    let (mut input_repr, mut expect_repr): (String, String) =
        (String::from("Output:\t  ["), String::from("Expected: ["));
    let mut i = 0;
    while i < expected.len() || i < input.len() {
        if i == expected.len() - 1 && i == input.len() - 1 {
            if input[i] != expected[i] {
                input_repr += RED;
                expect_repr += GREEN;
            }
            input_repr += &format!("{:?}", input[i]);
            expect_repr += &format!("{:?}", expected[i]);

            input_repr += CLEAR;
            expect_repr += CLEAR;
            break;
        }
        if i == expected.len() {
            input_repr += &format!("{RED}{:?}{CLEAR}", input[i]);
            for j in i + 1..input.len() {
                input_repr += &format!(", {:?}", input[j]);
            }

            expect_repr += &format!("{GREEN}___{CLEAR}");
            break;
        } else if i == input.len() {
            expect_repr += &format!("{GREEN}{:?}{CLEAR}", expected[i]);
            for j in i + 1..expected.len() {
                expect_repr += &format!(", {:?}", input[j]);
            }

            input_repr += &format!("{RED}___{CLEAR}");
            break;
        } else {
            if input[i] != expected[i] {
                input_repr += RED;
                expect_repr += GREEN;
            }
            input_repr += &format!("{:?}, ", input[i]);
            expect_repr += &format!("{:?}, ", expected[i]);

            input_repr += CLEAR;
            expect_repr += CLEAR;
        }
        i += 1;
    }
    input_repr += "]";
    expect_repr += "]";
    return format!(
        "\n{}\n{input_repr}\n\n{expect_repr}\n{}",
        "-".repeat(100),
        "-".repeat(100)
    );
}
