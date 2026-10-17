use std::convert::TryInto;
use std::iter::{ExactSizeIterator, IntoIterator, Iterator};
use std::str;

use thin_vec::ThinVec;

/// Store any string efficiently in an immutable way.
///
/// Can store at most `u32::MAX` strings and only provides
/// `StringsNoIndexIter` and does not provide arbitary indexing.
#[derive(Debug, Default, Eq, PartialEq, Clone, Hash)]
pub struct StringsNoIndex {
    strs: ThinVec<u8>,
}

impl StringsNoIndex {
    pub fn new() -> Self {
        Self::default()
    }

    /// * `len` - number of strings
    ///
    /// NOTE that this function does nothing and is defined just to be compatible
    /// with `Strings`.
    pub fn with_capacity(_len: u32) -> Self {
        Self::new()
    }

    fn set_len(&mut self, new_len: u32) {
        self.strs[..4].copy_from_slice(&new_len.to_ne_bytes());
    }

    pub fn len(&self) -> u32 {
        if self.is_empty() {
            0
        } else {
            u32::from_ne_bytes(self.strs[..4].try_into().unwrap())
        }
    }

    pub fn is_empty(&self) -> bool {
        self.strs.is_empty()
    }

    /// * `s` - must not contain null byte.
    pub fn push(&mut self, s: &str) {
        if self.is_empty() {
            let len: u32 = 1;
            self.strs.extend_from_slice(&len.to_ne_bytes());
        } else {
            let len = self.len();

            if len == u32::MAX {
                panic!(
                    "StringsNoIndex cannot contain more than u32::MAX {} elements",
                    u32::MAX
                );
            }

            self.set_len(len + 1);
        }

        self.strs
            .extend(s.as_bytes().iter().copied().filter(|byte| *byte != b'\0'));
        self.strs.push(0);
    }

    /// Accumulate length of all strings.
    #[inline(always)]
    pub fn strs_len(&self) -> usize {
        self.strs.len()
    }

    #[inline(always)]
    pub fn reserve_strs(&mut self, cnt: usize) {
        self.strs.reserve(cnt);
    }

    pub fn shrink_to_fit(&mut self) {
        self.strs.shrink_to_fit();
    }

    #[inline(always)]
    pub fn iter(&self) -> StringsNoIndexIter<'_> {
        let slice = if self.is_empty() {
            &[]
        } else {
            &self.strs[4..]
        };
        StringsNoIndexIter::new(slice, self.len())
    }
}
impl<'a> IntoIterator for &'a StringsNoIndex {
    type Item = &'a str;
    type IntoIter = StringsNoIndexIter<'a>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Clone, Debug)]
pub struct StringsNoIndexIter<'a>(&'a [u8], u32);

impl<'a> StringsNoIndexIter<'a> {
    fn new(strs: &'a [u8], len: u32) -> Self {
        Self(strs, len)
    }
}

impl<'a> Iterator for StringsNoIndexIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            return None;
        }

        self.1 -= 1;

        let pos = self.0.iter().position(|byte| *byte == 0).unwrap();
        let slice = &self.0[..pos];
        self.0 = &self.0[(pos + 1)..];
        Some(unsafe { str::from_utf8_unchecked(slice) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.1 as usize;
        (len, Some(len))
    }
}

impl ExactSizeIterator for StringsNoIndexIter<'_> {}

#[cfg(test)]
mod tests {
    use super::StringsNoIndex;

    fn assert_strs_in(strs: &StringsNoIndex, input_strs: &Vec<String>) {
        for (string, input_str) in strs.iter().zip(input_strs) {
            assert_eq!(string, input_str);
        }
    }

    #[test]
    fn test() {
        let mut strs = StringsNoIndex::new();
        let input_strs: Vec<String> = (0..256).map(|n| n.to_string()).collect();

        assert!(strs.is_empty());

        for (i, input_str) in input_strs.iter().enumerate() {
            strs.push(input_str);
            assert_eq!(strs.len() as usize, i + 1);

            assert_strs_in(&strs, &input_strs);
        }

        assert!(!strs.is_empty());

        assert!(input_strs.iter().eq(strs.iter()));
    }

    #[test]
    fn test_adding_empty_strs() {
        let mut strs = StringsNoIndex::new();

        assert!(strs.is_empty());

        for i in 0..10 {
            strs.push("");
            assert_eq!(strs.len() as usize, i + 1);
        }

        assert!(!strs.is_empty());

        strs.push("12345");

        for (i, string) in strs.iter().enumerate() {
            if i < 10 {
                assert_eq!(string, "");
            } else {
                assert_eq!(string, "12345");
            }
        }
    }
}
