use std::borrow::Cow;

pub fn borrow_str<'a, 'b: 'a>(cow: &'b Cow<'a, str>) -> &'a str {
    &*cow
}

pub trait ToCow<'a, T>
where
    T: ?Sized + ToOwned,
{
    fn to_cow(self) -> Cow<'a, T>;
}

impl<'a> ToCow<'a, str> for &'a str {
    fn to_cow(self) -> Cow<'a, str> {
        Cow::Borrowed(self)
    }
}

impl<'a> ToCow<'a, str> for String {
    fn to_cow(self) -> Cow<'a, str> {
        Cow::Owned(self)
    }
}
