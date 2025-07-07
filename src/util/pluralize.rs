use std::borrow::Cow;

pub trait Pluralizer {
    /// Pluralizes a word by appending "s", avoiding allocation for the singular case.
    ///
    /// Returns a `Cow<str>`, which will be a borrowed slice for the singular form
    /// and an owned String for the plural form.
    fn pluralize_regular<'a>(&self, base: &'a str) -> Cow<'a, str>;
    fn pluralize<'a>(&self, singular: &'a str, plural: &'a str) -> &'a str;
}

#[macro_export]
macro_rules! implement_pluralizer {
    ( for $($t:ty),+) => {
        $(
            impl Pluralizer for $t {
                fn pluralize_regular<'a>(&self, base: &'a str) -> Cow<'a, str> {
                    match *self {
                        1 => Cow::Borrowed(base),
                        _ => Cow::Owned(format!("{}s", base))
                    }
                }

                fn pluralize<'a>(&self, singular: &'a str, plural: &'a str) -> &'a str {
                    match *self {
                        1 => singular,
                        _ => plural
                    }
                }
            }
        )+
    };
}

implement_pluralizer!(for usize, u64, i32);
