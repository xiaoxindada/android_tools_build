use std::convert::TryInto;
use std::hint::unreachable_unchecked;
use std::iter::{ExactSizeIterator, IntoIterator, Iterator};
use std::slice;
use std::str;

use thin_vec::ThinVec;

/// Store any string efficiently in an immutable way.
///
/// Can store at most `u32::MAX` strings, the accumulated length
/// of these strings can be at most `u32::MAX`.
#[derive(Debug, Default, Eq, PartialEq, Clone, Hash)]
pub struct Strings {
    strs: ThinVec<u8>,
    ends: ThinVec<u32>,
}

impl Strings {
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// * `len` - number of strings
    pub fn with_capacity(len: u32) -> Self {
        let mut strings = Self::default();
        strings.reserve(len);
        strings
    }

    /// **Strings can contain at most `u32::MAX` strings**
    pub fn push(&mut self, s: &str) {
        self.strs.extend_from_slice(s.as_bytes());
        self.ends.push(
            self.strs
                .len()
                .try_into()
                .expect("Strings cannot contain more than u32::MAX strings"),
        );
    }

    /// Accumulate length of all strings.
    #[inline(always)]
    pub fn strs_len(&self) -> u32 {
        match self.strs.len().try_into() {
            Ok(len) => len,
            Err(_err) => unsafe { unreachable_unchecked() },
        }
    }

    #[inline(always)]
    pub fn len(&self) -> u32 {
        match self.ends.len().try_into() {
            Ok(len) => len,
            Err(_err) => unsafe { unreachable_unchecked() },
        }
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline(always)]
    pub fn reserve(&mut self, strs_cnt: u32) {
        self.ends.reserve(strs_cnt as usize);
    }

    #[inline(always)]
    pub fn reserve_strs(&mut self, cnt: usize) {
        self.strs.reserve(cnt);
    }

    pub fn shrink_to_fit(&mut self) {
        self.strs.shrink_to_fit();
        self.ends.shrink_to_fit();
    }

    #[inline(always)]
    pub fn iter(&self) -> StringsIter<'_> {
        StringsIter {
            strings: self,
            ends_iter: self.ends.iter(),
            start: 0,
        }
    }

    pub fn get(&self, index: u32) -> Option<&str> {
        let end = *self.ends.get(index as usize)?;
        let start = if index == 0 {
            0
        } else {
            self.ends[(index - 1) as usize]
        };

        Some(self.get_str_impl(start, end))
    }

    #[inline(always)]
    fn get_str_impl(&self, start: u32, end: u32) -> &str {
        unsafe { str::from_utf8_unchecked(&self.strs[(start as usize)..(end as usize)]) }
    }

    pub fn as_str(&self) -> &str {
        self.get_str_impl(0, self.strs_len())
    }

    pub fn into_str(self) -> String {
        let mut vec = Vec::with_capacity(self.strs.len());
        vec.extend_from_slice(&self.strs);
        unsafe { String::from_utf8_unchecked(vec) }
    }
}
impl<'a> IntoIterator for &'a Strings {
    type Item = &'a str;
    type IntoIter = StringsIter<'a>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Clone, Debug)]
pub struct StringsIter<'a> {
    strings: &'a Strings,
    ends_iter: slice::Iter<'a, u32>,
    start: u32,
}

impl<'a> Iterator for StringsIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let start = self.start;
        let end = *self.ends_iter.next()?;

        self.start = end;

        Some(self.strings.get_str_impl(start, end))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.ends_iter.len();
        (len, Some(len))
    }
}

impl ExactSizeIterator for StringsIter<'_> {}

#[cfg(test)]
mod tests {
    use super::Strings;
    use std::convert::TryInto;

    fn assert_strs_in(strs: &Strings, input_strs: &Vec<String>) {
        for (string, input_str) in strs.iter().zip(input_strs) {
            assert_eq!(string, input_str);
        }
    }

    #[test]
    fn test() {
        let mut strs = Strings::new();
        let input_strs: Vec<String> = (0..256).map(|n| n.to_string()).collect();

        assert!(strs.is_empty());

        for (i, input_str) in input_strs.iter().enumerate() {
            strs.push(input_str);
            assert_eq!(strs.len() as usize, i + 1);

            assert_strs_in(&strs, &input_strs);
        }

        assert!(input_strs.iter().eq(strs.iter()));

        for (i, input_str) in input_strs.iter().enumerate() {
            assert_eq!(strs.get(i.try_into().unwrap()).unwrap(), input_str);
        }

        let input_str = input_strs.concat();

        assert_eq!(strs.as_str(), input_str);
        assert_eq!(strs.into_str(), input_str);
    }
}
