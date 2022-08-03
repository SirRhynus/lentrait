use std::{collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque}, ffi::{CStr, CString, OsStr, OsString}, hash::Hash};

pub trait Len {
    fn len(&self) -> usize;

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

// IMPLEMENTATIONS
impl Len for str {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }
}

impl Len for String {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> Len for Vec<T> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> Len for VecDeque<T> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T: ?Sized + Len> Len for Box<T> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.as_ref().len()
    }
}

impl<T> Len for [T] {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T, const N: usize> Len for [T;N] {
    fn len(&self) -> usize {
        N
    }
}

#[cfg(feature = "pyo3")]
impl Len for pyo3::PyAny {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len().expect("Failed to get length!")
    }
}

#[cfg(feature = "pyo3")]
impl Len for &pyo3::PyAny {
    #[inline(always)]
    fn len(&self) -> usize {
        (*self).len().expect("Failed to get length!")
    }
}

#[cfg(feature = "pyo3")]
impl Len for serde_json::Value {
    #[inline(always)]
    fn len(&self) -> usize {
        self.to_owned().len()
    }
}

#[cfg(feature = "serde_crates")]
impl Len for &serde_json::Value {
    #[inline(always)]
    fn len(&self) -> usize {
        (*self).len()
    }
}

impl<K: Eq + Hash, V> Len for HashMap<K, V> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<K: Eq + Hash> Len for HashSet<K> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> Len for LinkedList<T> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }
}

impl Len for CStr {
    #[inline(always)]
    fn len(&self) -> usize {
        self.as_ref().len()
    }
}

impl Len for CString {
    #[inline(always)]
    fn len(&self) -> usize {
        self.as_ref().len()
    }
}

impl Len for OsStr {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }
}

impl Len for OsString {
    #[inline(always)]
    fn len(&self) -> usize {
        self.as_os_str().len()
    }
}


impl<K: Ord, V> Len for BTreeMap<K, V> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T: Ord> Len for BTreeSet<T> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T: Ord> Len for BinaryHeap<T> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }
}
