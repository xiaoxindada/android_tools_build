use super::small_array_box::*;
use super::{Strings, StringsIter, StringsNoIndex, StringsNoIndexIter, TwoStrs};

use std::convert::TryInto;
use std::fmt;
use std::iter::Iterator;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::ops::DerefMut;

use serde::de::{Deserialize, Deserializer, Error, SeqAccess, Visitor};
use serde::ser::{Serialize, SerializeTuple, Serializer};

macro_rules! impl_ser_de_for_strings {
    ($Strings:ident) => {
        impl Serialize for $Strings {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.collect_seq(self)
            }
        }

        impl<'de> Deserialize<'de> for $Strings {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                struct StringsVisitor;

                impl<'de> Visitor<'de> for StringsVisitor {
                    type Value = $Strings;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        write!(formatter, "A u32 length and &[str]")
                    }

                    fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
                    where
                        V: SeqAccess<'de>,
                    {
                        let len = seq.size_hint().unwrap_or(10);

                        let mut values =
                            $Strings::with_capacity(len.try_into().map_err(|_| {
                                V::Error::invalid_length(len, &"Expect u32 length")
                            })?);

                        while let Some(value) = seq.next_element()? {
                            values.push(value);
                        }

                        Ok(values)
                    }
                }

                deserializer.deserialize_seq(StringsVisitor)
            }
        }
    };
}

impl_ser_de_for_strings!(Strings);
impl_ser_de_for_strings!(StringsNoIndex);

macro_rules! impl_Serialize_for_iter {
    ($Iter:ident) => {
        /// The iterator is formatted as (&str, ...)
        impl Serialize for $Iter<'_> {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                let mut tuple_serializer = serializer.serialize_tuple(self.size_hint().0)?;

                for string in self.clone() {
                    tuple_serializer.serialize_element(string)?;
                }

                tuple_serializer.end()
            }
        }
    };
}

impl_Serialize_for_iter!(StringsIter);
impl_Serialize_for_iter!(StringsNoIndexIter);

/// Format: (&str, &str)
impl Serialize for TwoStrs {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.get().serialize(serializer)
    }
}

/// Format: (&str, &str)
impl<'de> Deserialize<'de> for TwoStrs {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let (s1, s2) = <(&'de str, &'de str)>::deserialize(deserializer)?;
        Ok(Self::new(s1, s2))
    }
}

impl<T: Serialize, const INLINE_LEN: usize> Serialize for SmallArrayBox<T, INLINE_LEN> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.deref().serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>, const INLINE_LEN: usize> Deserialize<'de>
    for SmallArrayBox<T, INLINE_LEN>
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct SmallArrayBoxVisitor<T, const INLINE_LEN: usize>(PhantomData<T>);

        impl<'de, T: Deserialize<'de>, const INLINE_LEN: usize> Visitor<'de>
            for SmallArrayBoxVisitor<T, INLINE_LEN>
        {
            type Value = SmallArrayBox<T, INLINE_LEN>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "Expected slice")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let size_hint = seq.size_hint();

                if let Some(len) = size_hint {
                    if len <= INLINE_LEN {
                        let mut this = SmallArrayBox::uninit_inline_storage();

                        let inline_storage = unsafe { this.storage.inline_storage.deref_mut() };

                        while let Some(value) = seq.next_element()? {
                            inline_storage[this.len] = MaybeUninit::new(value);
                            this.len += 1;
                        }

                        return Ok(this);
                    }
                }

                let mut values = Vec::with_capacity(size_hint.unwrap_or(10));

                while let Some(value) = seq.next_element()? {
                    values.push(value);
                }

                Ok(values.into())
            }
        }

        deserializer.deserialize_seq(SmallArrayBoxVisitor(PhantomData))
    }
}

#[cfg(test)]
mod tests {
    const INLINE_LEN: usize = 8;

    use super::{Strings, StringsNoIndex, TwoStrs};
    type SmallArrayBox = super::SmallArrayBox<u8, INLINE_LEN>;

    use std::error::Error;
    use std::fmt::{self, Display};
    use std::mem::MaybeUninit;

    use once_cell::sync::OnceCell;
    use serde_test::{assert_ser_tokens, assert_tokens, Token};

    use serde::de::{self, value::SeqAccessDeserializer, Deserialize, DeserializeSeed, SeqAccess};

    // Test using serde_test

    #[test]
    fn test_ser_de_empty_serde_strings() {
        assert_tokens(
            &Strings::new(),
            &[Token::Seq { len: Some(0) }, Token::SeqEnd],
        );
    }

    #[test]
    fn test_ser_de_empty_serde_strings_no_index() {
        assert_tokens(
            &StringsNoIndex::new(),
            &[Token::Seq { len: Some(0) }, Token::SeqEnd],
        );
    }

    macro_rules! assert_ser_de_serde {
        ($strings:expr) => {
            let strings = $strings;

            // Test Strings
            let mut tokens = vec![Token::Seq {
                len: Some(strings.len() as usize),
            }];

            for string in strings {
                tokens.push(Token::BorrowedStr(string));
            }

            tokens.push(Token::SeqEnd);

            assert_tokens(strings, &tokens);

            // Test StringsIter
            tokens[0] = Token::Tuple {
                len: strings.len() as usize,
            };
            *tokens.last_mut().unwrap() = Token::TupleEnd;

            assert_ser_tokens(&strings.iter(), &tokens);
        };
    }

    fn get_strings() -> &'static Strings {
        static STRINGS: OnceCell<Strings> = OnceCell::new();

        STRINGS.get_or_init(|| {
            let mut strings = Strings::new();
            for i in 0..1024 {
                strings.push(&i.to_string());
            }
            strings
        })
    }

    #[test]
    fn test_ser_de_serde_strings() {
        assert_ser_de_serde!(get_strings());
    }

    fn get_strings_no_index() -> &'static StringsNoIndex {
        static STRINGS: OnceCell<StringsNoIndex> = OnceCell::new();

        STRINGS.get_or_init(|| {
            let mut strings = StringsNoIndex::new();
            for i in 0..1024 {
                strings.push(&i.to_string());
            }
            strings
        })
    }

    #[test]
    fn test_ser_de_serde_strings_no_index() {
        assert_ser_de_serde!(get_strings_no_index());
    }

    // Test using serde_json

    macro_rules! assert_ser_de_json {
        ($strings:expr, $strings_type:ident) => {
            let strings = $strings;
            let json = serde_json::to_string(strings).unwrap();
            assert_eq!(
                serde_json::from_str::<'_, $strings_type>(&json).unwrap(),
                *strings
            );
        };
    }

    #[test]
    fn test_ser_de_serde_json_strings() {
        assert_ser_de_json!(get_strings(), Strings);
    }

    #[test]
    fn test_ser_de_serde_json_strings_no_index() {
        assert_ser_de_json!(get_strings_no_index(), StringsNoIndex);
    }

    #[test]
    fn test_ser_de_two_strs() {
        let s1 = "1234<<";
        let s2 = "234a";

        let two_strs = TwoStrs::new(s1, s2);

        assert_tokens(
            &two_strs,
            &[
                Token::Tuple { len: 2 },
                Token::BorrowedStr(s1),
                Token::BorrowedStr(s2),
                Token::TupleEnd,
            ],
        );
    }

    #[test]
    fn test_ser_de_serde_json_two_strs() {
        let s1 = "1234<<";
        let s2 = "234a";

        let two_strs = TwoStrs::new(s1, s2);

        assert_ser_de_json!(&two_strs, TwoStrs);
    }

    #[test]
    fn test_ser_de_small_array_box_empty() {
        let tokens = [Token::Seq { len: Some(0) }, Token::SeqEnd];

        let array = SmallArrayBox::new([]);
        assert_tokens(&array, &tokens);

        let array = SmallArrayBox::new_empty();
        assert_tokens(&array, &tokens);
    }

    #[test]
    fn test_ser_de_small_array_box() {
        let vec: Vec<u8> = (0..100).collect();

        let mut tokens = Vec::new();

        for len in 0..vec.len() {
            let slice = &vec[..len];

            let array = SmallArrayBox::new(slice.iter().copied());

            tokens.reserve_exact(len + 2);

            tokens.push(Token::Seq { len: Some(len) });

            for i in 0..(len as u8) {
                tokens.push(Token::U8(i));
            }

            tokens.push(Token::SeqEnd);
            assert_tokens(&array, &tokens);

            tokens.clear();
        }
    }

    #[test]
    fn test_small_array_box_de_error() {
        #[derive(Debug)]
        struct DummyError;

        impl Display for DummyError {
            fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
                Ok(())
            }
        }

        impl de::Error for DummyError {
            fn custom<T: Display>(_msg: T) -> Self {
                Self
            }
        }

        impl Error for DummyError {}

        struct ErrSeqAccess(usize);

        impl<'de> SeqAccess<'de> for ErrSeqAccess {
            type Error = DummyError;

            fn next_element_seed<T>(&mut self, _seed: T) -> Result<Option<T::Value>, Self::Error>
            where
                T: DeserializeSeed<'de>,
            {
                if self.0 > 0 {
                    self.0 -= 1;
                    Ok(Some(unsafe { MaybeUninit::zeroed().assume_init() }))
                } else {
                    Err(DummyError)
                }
            }

            fn size_hint(&self) -> Option<usize> {
                Some(self.0)
            }
        }

        for len in 0..INLINE_LEN {
            let deserializer = SeqAccessDeserializer::new(ErrSeqAccess(len));

            assert!(SmallArrayBox::deserialize(deserializer).is_err());
        }
    }
}
