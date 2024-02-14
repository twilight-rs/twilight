pub trait Format {
    fn bold(self) -> Self;

    fn inline_code(self) -> Self;

    fn italic(self) -> Self;

    fn relative_timestamp(self) -> Self;

    fn underline(self) -> Self;

    fn strikethrough(self) -> Self;
}

impl Format for &str {
    fn bold(self) -> Self {
        concat!("**", self, "**")
    }

    fn inline_code(self) -> Self {
        concat!("`", self, "`")
    }

    fn italic(self) -> Self {
        concat!("*", self, "*")
    }

    fn relative_timestamp(self) -> Self {
        concat!("<t:", self, ":R>")
    }

    fn underline(self) -> Self {
        concat!("__", self, "__")
    }

    fn strikethrough(self) -> Self {
        concat!("~~", self, "~~")
    }
}
