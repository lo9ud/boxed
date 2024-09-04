pub trait Error: std::error::Error {
    fn message(&self) -> String;

    fn position(&self) -> Option<Position>;

    fn source(&self) -> Option<&dyn Error>;

    fn warn(&self) {
        match self.source() {
            Some(source) => log::warn!("{}: {}", self.message(), source.message()),
            None => log::warn!("{}", self.message()),
        }
    }

    fn error(&self) {
        match self.source() {
            Some(source) => log::error!("{}: {}", self.message(), source.message()),
            None => log::error!("{}", self.message()),
        }
    }

    fn as_string(&self) -> String {
        match self.position() {
            Some(position) => format!(
                " {:0>3}:{:0>3} | {}",
                position.line(),
                position.column(),
                self.message()
            ),
            None => self.message(),
        }
    }
}

impl std::fmt::Display for dyn Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.position() {
            Some(position) => write!(
                f,
                " {:0>3}:{:0>3} | {}",
                position.line(),
                position.column(),
                self.message()
            ),
            None => write!(f, "{}", self.message()),
        }
    }
}

impl std::fmt::Debug for dyn Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.position() {
            Some(position) => write!(
                f,
                " {:0>3}:{:0>3} | {}",
                position.line(),
                position.column(),
                self.message()
            ),
            None => write!(f, "{}", self.message()),
        }
    }
}
