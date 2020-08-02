mod bytes;
mod input_file;
mod string;

pub(crate) use input_file::InputFileUnserializer;

use std::fmt::{self, Display};

use serde::ser;

#[derive(Debug, PartialEq, Eq)]
pub enum UnserializerError {
    Custom(String),
    UnsupportedType {
        ty: &'static str,
        supported: &'static str,
    },
    UnexpectedField {
        name: &'static str,
        expected: &'static [&'static str],
    },
    UnexpectedVariant {
        name: &'static str,
        expected: &'static [&'static str],
    },
    WrongLen {
        len: usize,
        expected: usize,
    },
}

impl ser::Error for UnserializerError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self::Custom(msg.to_string())
    }
}

impl Display for UnserializerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for UnserializerError {}

#[test]
fn test() {
    use serde::Serialize;

    use crate::unserializers::input_file::InputFileUnserializer;
    use crate::InputFile;
    use std::borrow::Cow;

    assert_eq!(
        String::from("test").serialize(StringUnserializer),
        Ok(String::from("test"))
    );

    let value = InputFile::Url(String::from("url"));
    assert_eq!(
        value.clone().serialize(InputFileUnserializer::NotMem),
        Ok(value)
    );

    let value = InputFile::FileId(String::from("file_id"));
    assert_eq!(
        value.clone().serialize(InputFileUnserializer::NotMem),
        Ok(value)
    );

    let value = InputFile::Memory {
        file_name: String::from("name"),
        data: Cow::Owned(vec![1, 2, 3]),
    };
    assert_eq!(
        value.clone().serialize(InputFileUnserializer::memory()),
        Ok(value)
    );

    let value = InputFile::File("a/b/c".into());
    assert_eq!(
        value.clone().serialize(InputFileUnserializer::NotMem),
        Ok(value)
    );
}
