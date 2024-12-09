//! In this file extension traits are defined to be used within the `plc_xml` crate.

use std::borrow::Cow;

use quick_xml::name::QName;
use rustc_hash::FxHashMap;

use crate::error::Error;

/// Trait for [`quick_xml`]s tags defined in [`quick_xml::events`]
pub(crate) trait TryToString {
    fn try_to_string(self) -> Result<String, Error>;
}

/// Trait to extract attribute values from XML elements wrapped inside HashMaps
pub(crate) trait GetOrErr {
    fn get_or_err(&self, key: &str) -> Result<String, Error>;
}

impl TryToString for &[u8] {
    fn try_to_string(self) -> Result<String, Error> {
        String::from_utf8(self.as_ref().to_vec()).map_err(|err| Error::Encoding(err.utf8_error()))
    }
}

impl TryToString for QName<'_> {
    fn try_to_string(self) -> Result<String, Error> {
        String::from_utf8(self.into_inner().to_vec()).map_err(|err| Error::Encoding(err.utf8_error()))
    }
}

impl TryToString for Cow<'_, [u8]> {
    fn try_to_string(self) -> Result<String, Error> {
        String::from_utf8(self.to_vec()).map_err(|err| Error::Encoding(err.utf8_error()))
    }
}

impl GetOrErr for FxHashMap<String, String> {
    fn get_or_err(&self, key: &str) -> Result<String, Error> {
        self.get(key).map(|it| it.to_owned()).ok_or(Error::MissingAttribute(key.to_string()))
    }
}
