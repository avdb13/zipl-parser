use crate::Section;

use chumsky::{
    error::Simple,
    extra,
    prelude::just,
    primitive::choice,
    text::{ascii::ident, int},
    Parser,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Auto<'a> {
    target: &'a str,
    default: &'a str,
    prompt: &'a str,
    timeout: &'a str,
    secure: &'a str,
}

#[derive(Clone, Debug, PartialEq)]
enum AutoOptions<'a> {
    Target(&'a str),
    Default(Option<&'a str>),
    Prompt(Option<&'a str>),
    Timeout(Option<&'a str>),
    Secure(Option<&'a str>),
}

fn parser<'a>() -> impl Parser<'a, &'a str, Section<'a>, extra::Err<Simple<'a, char>>> {
    let eq = just('=').padded();
    let path = ident().separated_by(just('/')).allow_leading().to_slice();

    let target = just("target")
        .ignore_then(eq.ignore_then(path))
        .map(AutoOptions::Target);

    let default = just("default")
        .ignore_then(eq.ignore_then(int(10).or_not()))
        .map(AutoOptions::Default);
    let prompt = just("prompt")
        .ignore_then(eq.ignore_then(int(2)).or_not())
        .map(AutoOptions::Prompt);
    let timeout = just("timeout")
        .ignore_then(eq.ignore_then(int(10).or_not()))
        .map(AutoOptions::Timeout);
    let secure = just("secure")
        .ignore_then(eq.ignore_then(int(2).or(just("auto")).or_not()))
        .map(AutoOptions::Secure);

    just("[defaultboot]").padded().ignore_then(
        choice((
            just("default")
                .padded()
                .ignore_then(eq.ignore_then(ident()))
                .map(Section::Default),
            just("defaultmenu")
                .padded()
                .ignore_then(eq.ignore_then(ident()))
                .map(Section::Menu),
            just("defaultauto")
                .padded()
                .map(|_| Auto::default())
                .foldl(
                    choice((target, default, prompt, timeout, secure))
                        .padded()
                        .repeated(),
                    |options, b| match b {
                        AutoOptions::Target(target) => Auto { target, ..options },
                        AutoOptions::Default(Some(default)) => Auto { default, ..options },
                        AutoOptions::Prompt(Some(prompt)) => Auto { prompt, ..options },
                        AutoOptions::Timeout(Some(timeout)) => Auto { timeout, ..options },
                        AutoOptions::Secure(Some(secure)) => Auto { secure, ..options },
                        _ => options,
                    },
                )
                .map(Section::Auto),
        ))
        .padded(),
    )
}

#[cfg(test)]
mod tests {
    use chumsky::Parser;

    use crate::{
        sections::default::{parser, Auto},
        Section,
    };

    #[test]
    fn parser_works() {
        let data = [
            (
                r#"
                    [defaultboot]
                    default = boot1
                "#,
                Ok(Section::Default("boot1")),
            ),
            (
                r#"
                    [defaultboot]
                    defaultmenu=menu1
                "#,
                Ok(Section::Menu("menu1")),
            ),
            (
                r#"
                    [defaultboot]
                    defaultauto
                    target=/boot
                    default=1
                "#,
                Ok(Section::Auto(Auto {
                    target: "/boot",
                    default: "1",
                    ..Auto::default()
                })),
            ),
        ];

        for (input, result) in data {
            assert_eq!(result, parser().parse(input).into_result());
        }
    }
}
