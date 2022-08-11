use std::borrow::Cow;


#[derive(Debug)]
pub(crate) enum LineDiff<'a>
{
    Equal(Cow<'a, str>),
    Delete(Cow<'a, str>),
    Insert(Cow<'a, str>)
}
