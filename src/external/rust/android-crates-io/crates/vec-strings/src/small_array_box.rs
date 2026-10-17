use std::mem::{ManuallyDrop, MaybeUninit};
use std::ptr::NonNull;
use std::slice::{from_raw_parts, from_raw_parts_mut};

use std::iter::IntoIterator;
use std::iter::{ExactSizeIterator, Iterator};

use std::fmt::{self, Debug};
use std::ops::{Deref, DerefMut};

use std::cmp::{Eq, PartialEq};

pub(crate) union SmallArrayBoxInner<T, const INLINE_LEN: usize> {
    ptr: NonNull<T>,
    pub(crate) inline_storage: ManuallyDrop<[MaybeUninit<T>; INLINE_LEN]>,
}

/// * `INLINE_LEN` - Number of elements that can be stored inline.
pub struct SmallArrayBox<T, const INLINE_LEN: usize> {
    pub(crate) storage: SmallArrayBoxInner<T, INLINE_LEN>,
    pub(crate) len: usize,
}

unsafe impl<T: Send, const INLINE_LEN: usize> Send for SmallArrayBox<T, INLINE_LEN> {}
unsafe impl<T: Sync, const INLINE_LEN: usize> Sync for SmallArrayBox<T, INLINE_LEN> {}

impl<T, const INLINE_LEN: usize> Default for SmallArrayBox<T, INLINE_LEN> {
    fn default() -> Self {
        Self::new_empty()
    }
}

impl<T, const INLINE_LEN: usize> From<Box<[T]>> for SmallArrayBox<T, INLINE_LEN> {
    fn from(boxed: Box<[T]>) -> Self {
        Self::from_box(boxed)
    }
}

impl<T, const INLINE_LEN: usize> From<Vec<T>> for SmallArrayBox<T, INLINE_LEN> {
    fn from(vec: Vec<T>) -> Self {
        if vec.len() <= INLINE_LEN {
            Self::new(vec)
        } else {
            vec.into_boxed_slice().into()
        }
    }
}

impl<T: Clone, const INLINE_LEN: usize> From<&[T]> for SmallArrayBox<T, INLINE_LEN> {
    fn from(slice: &[T]) -> Self {
        Self::new(slice.iter().cloned())
    }
}

impl<T: Clone, const INLINE_LEN: usize> Clone for SmallArrayBox<T, INLINE_LEN> {
    fn clone(&self) -> Self {
        Self::new(self.iter().cloned())
    }
}

impl<T, const INLINE_LEN: usize> SmallArrayBox<T, INLINE_LEN> {
    pub(crate) fn uninit_inline_storage() -> Self {
        Self {
            storage: SmallArrayBoxInner {
                // Safety:
                //
                // It is safe because the array contains `MaybeUninit<T>`.
                inline_storage: ManuallyDrop::new(unsafe { MaybeUninit::uninit().assume_init() }),
            },
            len: 0,
        }
    }

    pub const fn new_empty() -> Self {
        Self {
            storage: SmallArrayBoxInner {
                ptr: NonNull::dangling(),
            },
            len: 0,
        }
    }

    pub fn new<I>(iter: impl IntoIterator<IntoIter = I>) -> Self
    where
        I: Iterator<Item = T> + ExactSizeIterator,
    {
        let iter = iter.into_iter();

        let len = iter.len();

        if len <= INLINE_LEN {
            let mut this = Self::uninit_inline_storage();

            let inline_storage = unsafe { this.storage.inline_storage.deref_mut() };

            iter.zip(inline_storage).for_each(|(src, dst)| {
                *dst = MaybeUninit::new(src);
            });

            this.len = len;

            this
        } else {
            let vec: Vec<T> = iter.collect();
            let array_ptr = Box::into_raw(vec.into_boxed_slice());
            let slice = unsafe { &mut *array_ptr };
            let ptr = unsafe { NonNull::new_unchecked(slice.as_mut_ptr()) };

            Self {
                storage: SmallArrayBoxInner { ptr },
                len,
            }
        }
    }

    pub fn from_box(boxed: Box<[T]>) -> Self {
        let len = boxed.len();

        if len <= INLINE_LEN {
            let vec: Vec<T> = boxed.into();
            Self::new(vec)
        } else {
            let array_ptr = Box::into_raw(boxed);
            let slice = unsafe { &mut *array_ptr };
            let ptr = unsafe { NonNull::new_unchecked(slice.as_mut_ptr()) };

            debug_assert_eq!(slice.len(), len);

            Self {
                storage: SmallArrayBoxInner { ptr },
                len,
            }
        }
    }

    pub fn into_boxed_slice(self) -> Box<[T]> {
        let len = self.len;

        let mut this = ManuallyDrop::new(self);

        if len <= INLINE_LEN {
            let inline_storage = unsafe { &mut this.storage.inline_storage };
            let mut vec = Vec::with_capacity(len);

            for elem in inline_storage[..len].iter_mut() {
                let ptr = elem.as_mut_ptr();
                vec.push(unsafe { ptr.read() });
            }

            debug_assert_eq!(vec.len(), len);

            vec.into_boxed_slice()
        } else {
            let ptr = unsafe { this.storage.ptr }.as_ptr();
            let slice = unsafe { from_raw_parts_mut(ptr, len) };
            unsafe { Box::from_raw(slice) }
        }
    }
}

impl<T, const INLINE_LEN: usize> Deref for SmallArrayBox<T, INLINE_LEN> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        let len = self.len;

        if len <= INLINE_LEN {
            let inline_storage = unsafe { self.storage.inline_storage.deref() };
            unsafe { &*(&inline_storage[..len] as *const _ as *const [T]) }
        } else {
            let ptr = unsafe { self.storage.ptr }.as_ptr();
            unsafe { from_raw_parts(ptr, len) }
        }
    }
}

impl<T, const INLINE_LEN: usize> DerefMut for SmallArrayBox<T, INLINE_LEN> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let len = self.len;

        if len <= INLINE_LEN {
            let inline_storage = unsafe { self.storage.inline_storage.deref_mut() };
            unsafe { &mut *(&mut inline_storage[..len] as *mut _ as *mut [T]) }
        } else {
            let ptr = unsafe { self.storage.ptr }.as_ptr();
            unsafe { from_raw_parts_mut(ptr, len) }
        }
    }
}

impl<T, const INLINE_LEN: usize> Drop for SmallArrayBox<T, INLINE_LEN> {
    fn drop(&mut self) {
        let len = self.len;

        if len <= INLINE_LEN {
            let inline_storage = unsafe { self.storage.inline_storage.deref_mut() };

            inline_storage[..len].iter_mut().for_each(|elem| {
                let ptr = elem.as_mut_ptr();
                unsafe {
                    ptr.drop_in_place();
                }
            });
        } else {
            let ptr = unsafe { self.storage.ptr }.as_ptr();
            let slice = unsafe { from_raw_parts_mut(ptr, len) };
            drop(unsafe { Box::from_raw(slice) });
        }
    }
}

impl<T: Debug, const INLINE_LEN: usize> Debug for SmallArrayBox<T, INLINE_LEN> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self.deref())
    }
}

impl<T: PartialEq, const INLINE_LEN: usize> PartialEq for SmallArrayBox<T, INLINE_LEN> {
    fn eq(&self, other: &Self) -> bool {
        self.deref().eq(other.deref())
    }
}

impl<T: Eq, const INLINE_LEN: usize> Eq for SmallArrayBox<T, INLINE_LEN> {}

#[cfg(test)]
mod tests {
    type SmallArrayBox = super::SmallArrayBox<u8, 8>;

    use std::ops::{Deref, DerefMut};
    use std::ptr;

    fn assert_ptr_eq(x: *const [u8], y: *const [u8]) {
        assert!(ptr::eq(x, y));
    }

    #[test]
    fn test_new_empty() {
        let mut empty_array = SmallArrayBox::new_empty();

        let empty: &[u8] = &[];

        assert_eq!(empty_array.deref(), empty);
        assert_eq!(empty_array.deref_mut(), empty);
        assert_ptr_eq(empty_array.deref(), empty_array.deref_mut());

        let boxed = empty_array.into_boxed_slice();

        assert_eq!(&*boxed, empty);
    }

    #[test]
    fn test_new() {
        let vec: Vec<u8> = (0..100).collect();

        for len in 0..vec.len() {
            let slice = &vec[..len];

            let mut array = SmallArrayBox::new(slice.iter().copied());

            assert_eq!(array.deref(), slice);
            assert_eq!(array.deref_mut(), slice);
            assert_ptr_eq(array.deref(), array.deref_mut());

            let boxed = array.into_boxed_slice();

            assert_eq!(&*boxed, slice);
        }
    }

    #[test]
    fn test_from_box() {
        let vec: Vec<u8> = (0..100).collect();

        for len in 0..vec.len() {
            let slice = &vec[..len];

            let vec: Vec<u8> = slice.to_vec();

            let mut array = SmallArrayBox::from_box(vec.into_boxed_slice());

            assert_eq!(array.deref(), slice);
            assert_eq!(array.deref_mut(), slice);
            assert_ptr_eq(array.deref(), array.deref_mut());

            let boxed = array.into_boxed_slice();

            assert_eq!(&*boxed, slice);
        }
    }
}
