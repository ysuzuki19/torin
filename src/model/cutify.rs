enum Color {
    Default,
    Red,
}

impl Color {
    fn code(&self) -> &'static str {
        match self {
            Color::Default => "39",
            Color::Red => "31",
        }
    }
}

pub struct Cutify<'a> {
    content: &'a str,
    color: Color,
    bold: bool,
}

// Implementation for building
impl Cutify<'_> {
    pub fn red(mut self) -> Self {
        self.color = Color::Red;
        self
    }

    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }
}

impl Cutify<'_> {
    fn mid_bold(&self) -> &'_ str {
        if self.bold {
            "1;"
        } else {
            ""
        }
    }
}

impl std::fmt::Display for Cutify<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\x1b[{}{}m{}\x1b[0m",
            self.mid_bold(),
            self.color.code(),
            self.content
        )
    }
}

pub trait CutifyOps<'a> {
    fn cutify(&'a self) -> Cutify<'a>;
}

impl<'a, S> CutifyOps<'a> for S
where
    S: AsRef<str> + 'a,
{
    fn cutify(&'a self) -> Cutify<'a>
    where
        S: AsRef<str> + 'a,
    {
        Cutify {
            content: self.as_ref(),
            color: Color::Default,
            bold: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cutify() {
        let text = "Hello, World!";
        assert_eq!(format!("{}", text.cutify()), "\x1b[39mHello, World!\x1b[0m");
    }

    #[test]
    fn test_cutify_default() {
        assert_eq!(
            format!("{}", "TEST".cutify().bold()),
            "\x1b[1;39mTEST\x1b[0m"
        );
    }

    #[test]
    fn test_cutify_red() {
        assert_eq!(format!("{}", "TEST".cutify().red()), "\x1b[31mTEST\x1b[0m");
    }

    #[test]
    fn test_cutify_bold() {
        assert_eq!(
            format!("{}", "TEST".cutify().bold()),
            "\x1b[1;39mTEST\x1b[0m"
        );
    }

    #[test]
    fn test_cutify_red_bold() {
        assert_eq!(
            format!("{}", "TEST".cutify().red().bold()),
            "\x1b[1;31mTEST\x1b[0m"
        );
    }
}
