use std::ops::{Index, IndexMut, Deref};
use std::collections::hash_map::{Iter, IterMut, RandomState};
use std::fmt::{Debug};
use serde::{Serializer, Serialize, Deserialize, Deserializer, de};
use serde::ser::SerializeMap;
use serde_json::ser::Formatter;
use std::{fmt, ops};
use crate::value::JsonValue;
use std::hash::{Hash, BuildHasher};
use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;


#[derive(Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum VecMapMode {
    Vec = 0,
    Index = 1,
}

#[derive(Clone,Eq)]
pub struct VecMap<K, V> where K: Ord+Hash {
    pub inner: Vec<(K, Option<V>)>,
    pub index: HashMap<K, usize>,
    pub change_factor: usize,
    pub mode: VecMapMode,
}

impl<K, V> PartialEq for VecMap<K, V>
    where K: Ord+Eq+Hash,
          V:PartialEq{
    fn eq(&self, other: &Self) -> bool {
        return self.inner.eq(&other.inner);
    }
}

impl<'a, K, V> VecMap<K, V> where K: Ord+Hash {
    pub fn new() -> VecMap<K, V> {
        Self {
            inner: vec![],
            index: HashMap::new(),
            change_factor: 20,
            mode: VecMapMode::Vec,
        }
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: Vec::with_capacity(capacity),
            index: Default::default(),
            change_factor: 20,
            mode: VecMapMode::Vec,
        }
    }

    pub fn insert(&mut self, key: K, value: V)
        where
            K: Ord+Clone, {
        for (k, v) in &mut self.inner {
            if *k == key {
                *v = Some(value);
                return;
            }
        }
        self.inner.push((key.clone(), Some(value)));
        self.index.insert(key, self.inner.len() - 1);
        if self.len() > self.change_factor {
            self.mode = VecMapMode::Index;
        }
    }

    pub fn remove(&mut self, key: K) -> Option<V>
        where
            K: Ord, {
        let mut index = 0;
        for (k, v) in &mut self.inner {
            if *k == key {
                let removed = self.inner.remove(index).1;
                self.index.remove(key.borrow());
                if self.len() <= self.change_factor {
                    self.mode = VecMapMode::Vec;
                }
                return removed;
            }
            index += 1;
        }
        return None;
    }

    /// Returns the number of elements in the map.
    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns true if the map contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline]
    pub fn iter(&'a self) -> std::slice::Iter<'_, (K, Option<V>)> {
        self.inner.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, (K, Option<V>)> {
        self.inner.iter_mut()
    }

    #[inline]
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
        where
            K: Borrow<Q>,
            Q: Hash + Eq,
    {
        match self.mode {
            VecMapMode::Vec => {
                for (k, v) in &self.inner {
                    if k.borrow().eq(key) {
                        return v.into();
                    }
                }
            }
            VecMapMode::Index => {
                let idx = self.index.get(key);
                match idx {
                    Some(idx) => {
                        return Option::<&V>::from(&self.inner[*idx].1);
                    }
                    _ => {}
                }
            }
        }
        return None;
    }

    #[inline]
    pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
        where
            K: Borrow<Q>,
            Q: Hash + Eq,
    {
        match self.mode {
            VecMapMode::Vec => {
                let mut index = 0;
                for (k, v) in &self.inner {
                    if k.borrow().eq(key) {
                        return match self.inner.get_mut(index) {
                            None => { None }
                            Some((_, result_v)) => {
                                match result_v {
                                    None => {
                                        None
                                    }
                                    Some(v) => {
                                        Some(v)
                                    }
                                }
                            }
                        };
                    }
                    index += 1;
                }
            }
            VecMapMode::Index => {
                let idx = self.index.get(key);
                match idx {
                    Some(idx) => {
                        return Option::<&mut V>::from(&mut self.inner[*idx].1);
                    }
                    _ => {}
                }
            }
        }
        return None;
    }
}

impl<K, Q: ?Sized, V> Index<&Q> for VecMap<K, V>
    where
        K: Eq + Hash + Borrow<Q> + Ord,
        Q: Eq + Hash,
{
    type Output = V;

    /// Returns a reference to the value corresponding to the supplied key.
    ///
    /// # Panics
    ///
    /// Panics if the key is not present in the `HashMap`.
    #[inline]
    fn index(&self, key: &Q) -> &V {
        self.get(key).expect("no entry found for key")
    }
}

impl<K, Q: ?Sized, V> ops::IndexMut<&Q> for VecMap<K, V>
    where
        K: Eq + Hash + Borrow<Q> + Ord,
        Q: Eq + Hash,
{
    fn index_mut(&mut self, index: &Q) -> &mut Self::Output {
        self.get_mut(index).expect("no entry found for key")
    }
}


impl<K, V> Iterator for VecMap<K, V> where K: Hash + Ord {
    type Item = (K, Option<V>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_empty() {
            return None;
        }
        let v = self.inner.remove(0);
        return Some(v);
    }
}


impl<K, V> Debug for VecMap<K, V> where K: Serialize + Hash + Ord,
                                        V: Serialize, {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_json::json!(self).to_string());
        return Ok(());
    }
}

impl<K, V> fmt::Display for VecMap<K, V> where K: Serialize + Hash + Ord,
                                               V: Serialize, {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_json::json!(self).to_string());
        return Ok(());
    }
}


impl<K, V> Serialize for VecMap<K, V>
    where
        K: Serialize + Hash + Ord,
        V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer, {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in &self.inner {
            map.serialize_key(k);
            map.serialize_value(v);
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for VecMap<String, JsonValue> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = VecMap<String, JsonValue>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map")
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<Self::Value, E>
                where
                    E: de::Error,
            {
                Ok(VecMap::new())
            }

            #[inline]
            fn visit_map<V>(self, mut visitor: V) -> Result<Self::Value, V::Error>
                where
                    V: de::MapAccess<'de>,
            {
                let mut values = VecMap::new();

                while let Some((key, value)) = visitor.next_entry()? {
                    values.insert(key, value);
                }

                Ok(values)
            }
        }
        deserializer.deserialize_map(Visitor)
    }
}