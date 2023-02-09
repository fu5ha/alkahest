use crate::{
    cold::err,
    deserialize::{Deserialize, Deserializer, Error},
    formula::{Formula, NonRefFormula},
    serialize::{Serialize, Serializer},
};

impl Formula for str {
    const MAX_STACK_SIZE: Option<usize> = <[u8] as Formula>::MAX_STACK_SIZE;
    const EXACT_SIZE: bool = true;
    const HEAPLESS: bool = true;
}

impl NonRefFormula for str {}

impl Serialize<str> for &str {
    #[inline(always)]
    fn serialize<S>(self, ser: impl Into<S>) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut ser = ser.into();
        ser.write_bytes(self.as_bytes())?;
        ser.finish()
    }

    #[inline(always)]
    fn fast_sizes(&self) -> Option<usize> {
        Some(self.len())
    }
}

impl<'de, 'fe: 'de> Deserialize<'fe, str> for &'de str {
    #[inline(always)]
    fn deserialize(deserializer: Deserializer<'fe>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let bytes = deserializer.read_all_bytes();
        match core::str::from_utf8(bytes) {
            Ok(s) => Ok(s),
            Err(error) => err(Error::NonUtf8(error)),
        }
    }

    #[inline(always)]
    fn deserialize_in_place(&mut self, deserializer: Deserializer<'fe>) -> Result<(), Error> {
        let bytes = deserializer.read_all_bytes();
        match core::str::from_utf8(bytes) {
            Ok(s) => {
                *self = s;
                Ok(())
            }
            Err(error) => err(Error::NonUtf8(error)),
        }
    }
}
