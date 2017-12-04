
#[derive(Copy, Clone, Debug)]
/// The `RocketErr` Type. This is the main error type.
pub enum RocketErr {
    ConnectionError,
    Other(&'static str)
}

impl RocketErr {
    fn desc(&self) -> &str {
        match *self {
            RocketErr::ConnectionError => "Connection error.",
            RocketErr::Other(s) => s
        }
    }
}

impl ::std::error::Error for RocketErr {

    fn description(&self) -> &str {
        self.desc()
    }
}

impl ::std::fmt::Display for RocketErr {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{}", self.desc())
    }
}
