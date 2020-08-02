use serde::{ser, Serialize};
use std::fmt::Display;
use std::fmt;
use crate::unserializers::string::StringUnserializer;

mod input_file;
mod string;
mod bytes;

pub(crate) use {
    input_file::*,string::*,self::bytes::*,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct UnimplementedError;

#[test]
fn test() {
    use serde::Serialize;

    use std::borrow::Cow;
    use crate::InputFile;
    use crate::unserializers::input_file::InputFileUnserializer;

    assert_eq!(String::from("test").serialize(StringUnserializer), Ok(String::from("test")));

    let value = InputFile::Url(String::from("url"));
    assert_eq!(
        value.clone().serialize(InputFileUnserializer::url()),
        Ok(value)
    );

    let value = InputFile::FileId(String::from("file_id"));
    assert_eq!(
        value.clone().serialize(InputFileUnserializer::file_id()),
        Ok(value)
    );

    let value = InputFile::Memory { file_name: String::from("name"), data: Cow::Owned(vec![1, 2, 3]) };
    assert_eq!(
        value.clone().serialize(InputFileUnserializer::memory()),
        Ok(value)
    );

    let value = InputFile::File("a/b/c".into());
    assert_eq!(
        value.clone().serialize(InputFileUnserializer::file()),
        Ok(value)
    );
}

impl ser::Error for UnimplementedError {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        unimplemented!()
    }
}

impl Display for UnimplementedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl std::error::Error for UnimplementedError {}
