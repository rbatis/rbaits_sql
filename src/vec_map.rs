use std::ops::{Index, IndexMut};

#[derive(Clone, Eq, PartialEq)]
pub struct VecMap<K, V> {
    pub inner: Vec<(K, Option<V>)>,
}

impl<K, V> VecMap<K, V> {
    pub fn new() -> VecMap<K, V> {
        Self {
            inner: vec![],
        }
    }
    pub fn insert(&mut self, key: K, value: V)
        where
            K: Ord, {
        for (k, v) in &mut self.inner {
            if *k == key {
                *v = Some(value);
                return;
            }
        }
        self.inner.push((key, Some(value)));
    }

    pub fn remove(&mut self, key: K, value: V) -> (K, Option<V>)
        where
            K: Ord, {
        let mut index = 0;
        for (k, v) in &mut self.inner {
            if *k == key {
                return self.inner.remove(index);
            }
            index += 1;
        }
        return (key, None);
    }
}

impl<K, V> Index<K> for VecMap<K, V> where K: std::cmp::PartialEq {
    type Output = Option<V>;

    fn index(&self, index: K) -> &Self::Output {
        for (k, v) in &self.inner {
            if *k == index {
                return v;
            }
        }
        return &None;
    }
}

impl<K, V> Index<K> for &VecMap<K, V> where K: std::cmp::PartialEq {
    type Output = Option<V>;

    fn index(&self, index: K) -> &Self::Output {
        for (k, v) in &self.inner {
            if *k == index {
                return v;
            }
        }
        return &None;
    }
}

impl<K, V> IndexMut<K> for VecMap<K, V> where K: std::cmp::PartialEq {
    fn index_mut(&mut self, index: K) -> &mut Self::Output {
        for (k, v) in &mut self.inner {
            if *k == index {
                return v;
            }
        }
        panic!("no entry found for key")
    }
}