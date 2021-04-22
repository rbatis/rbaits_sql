use std::ops::{Index, IndexMut};
use std::collections::hash_map::{Iter, IterMut};
use std::fmt::{Debug};
use serde::{Serializer, Serialize, Deserialize, Deserializer, de};
use serde::ser::SerializeMap;
use serde_json::ser::Formatter;
use std::fmt;
use crate::value::JsonValue;


#[derive(Clone, Eq, PartialEq)]
pub struct VecMap<K, V> {
    pub inner: Vec<(K, Option<V>)>,
}

impl<'a, K, V> VecMap<K, V> {
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

    pub fn remove(&mut self, key: K) -> (K, Option<V>)
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

impl<K, V> Iterator for VecMap<K, V> {
    type Item = (K, Option<V>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_empty() {
            return None;
        }
        let v = self.inner.remove(0);
        return Some(v);
    }
}


impl<K, V> Debug for VecMap<K, V> where   K: Serialize,
                                          V: Serialize, {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_json::json!(self).to_string());
        return Ok(());
    }
}

impl<K, V> fmt::Display for VecMap<K,V> where   K: Serialize,
                                          V: Serialize, {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_json::json!(self).to_string());
        return Ok(());
    }
}




impl<K, V> Serialize for VecMap<K, V>
    where
        K: Serialize,
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