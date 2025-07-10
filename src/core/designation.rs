//! Designation, tag, name, etc. related stuff …

pub trait IsNamed {
    fn designation(&self) -> String;
}
