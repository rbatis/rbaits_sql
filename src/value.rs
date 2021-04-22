use std::ops::{Index, IndexMut};
use serde_json::Number;

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

#[derive(Clone, Eq, PartialEq)]
pub enum JsonValue {
    /// Represents a JSON null value.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(null);
    /// ```
    Null,

    /// Represents a JSON boolean.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(true);
    /// ```
    Bool(bool),

    /// Represents a JSON number, whether integer or floating point.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(12.5);
    /// ```
    Number(Number),

    /// Represents a JSON string.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!("a string");
    /// ```
    String(String),

    /// Represents a JSON array.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!(["an", "array"]);
    /// ```
    Array(Vec<JsonValue>),

    /// Represents a JSON object.
    ///
    /// By default the map is backed by a BTreeMap. Enable the `preserve_order`
    /// feature of serde_json to use IndexMap instead, which preserves
    /// entries in the order they are inserted into the map. In particular, this
    /// allows JSON data to be deserialized into a Value and serialized to a
    /// string while retaining the order of map keys in the input.
    ///
    /// ```
    /// # use serde_json::json;
    /// #
    /// let v = json!({ "an": "object" });
    /// ```
    Object(VecMap<String, JsonValue>),
}