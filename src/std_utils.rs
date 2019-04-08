pub type Error = failchain::BoxedError<ErrorKind>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind
{
    #[fail(display = "Option value was empty.")]
    EmptyOption
}

pub fn getOption<T>(optional: Option<T>) -> Result<T>
{
    optional.ok_or_else(|| Error::from(ErrorKind::EmptyOption))
}