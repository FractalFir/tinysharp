use std::{
    cmp::{Eq, PartialEq},
    collections::HashMap,
    hash::Hash,
    marker::PhantomData,
};
pub(crate) struct KeyedCollection<Key: Hash + PartialEq + Eq, Value> {
    map: HashMap<Key, usize>,
    values: Vec<Value>,
}
pub(crate) struct KCRef<Value> {
    index: usize,
    _pd: PhantomData<Value>,
}
impl<Key: Hash + PartialEq + Eq, Value> KeyedCollection<Key, Value> {
    pub(crate) fn new() -> Self {
        Self {
            map: HashMap::new(),
            values: Vec::new(),
        }
    }
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity(capacity),
            values: Vec::with_capacity(capacity),
        }
    }
    pub(crate) fn lookup(&self, key: &Key) -> Option<KCRef<Value>> {
        let index = *(self.map.get(key)?);
        Some(KCRef {
            index,
            _pd: PhantomData,
        })
    }
    pub(crate) fn insert(&mut self, key: Key, val: Value) -> KCRef<Value> {
        let index = self.values.len();
        self.values.push(val);
        self.map.insert(key, index);
        KCRef {
            index,
            _pd: PhantomData,
        }
    }
    pub(crate) fn get(&self, r: KCRef<Value>) -> &Value {
        &self.values[r.index]
    }
    pub(crate) fn values(&self) -> &[Value] {
        &self.values
    }
    pub(crate) fn values_mut(&mut self) -> &mut [Value] {
        &mut self.values
    }
}
#[test]
fn keyed_collection() {
    let mut kc = KeyedCollection::new();
    for i in 0..1000 {
        kc.insert(i, i ^ 0x345);
    }
    for i in 0..1000 {
        let kcref = kc.lookup(&i).unwrap();
        let res = kc.get(kcref);
        assert_eq!(*res, i ^ 0x345);
    }
}
