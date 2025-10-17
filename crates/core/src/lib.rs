// idk what to do, the async channel needs the channel type in
// its error
#![allow(clippy::result_large_err)]

pub mod chat;
pub mod domain;
pub mod error;
pub mod identity;
pub mod net;

pub const PROTOCOL_DIRECT_NAME: &[u8; 12] = b"SREMP_DIRECT";

pub fn version() -> String {
    format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
        .trim()
        .to_string()
}

#[macro_export]
macro_rules! trace_current_function {
    () => {{
        log::trace!("{}", current_function!());
    }};
}

#[macro_export]
macro_rules! current_function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }};
}

pub mod ser_helper {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    #[inline(always)]
    pub fn ser_arc<T: Serialize, S: Serializer>(t: &Arc<T>, s: S) -> Result<S::Ok, S::Error> {
        (*t).serialize(s)
    }

    #[inline(always)]
    pub fn deser_arc<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
        d: D,
    ) -> Result<Arc<T>, D::Error> {
        let t = T::deserialize(d)?;
        Ok(Arc::new(t))
    }

    #[inline(always)]
    pub fn ser_arc_opt<T, S>(t: &Option<Arc<T>>, s: S) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
        S: Serializer,
        T: Clone,
    {
        t.as_ref().map(|i| (**i).clone()).serialize(s)
    }

    #[inline]
    pub fn deser_arc_opt<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
        d: D,
    ) -> Result<Option<Arc<T>>, D::Error> {
        let t: Option<_> = Option::<T>::deserialize(d)?;
        Ok(t.map(|i| Arc::new(i)))
    }

    #[inline]
    pub fn ser_arc_hm<K, V, S>(hm: &HashMap<K, Arc<V>>, s: S) -> Result<S::Ok, S::Error>
    where
        K: Serialize + Eq + std::hash::Hash,
        V: Serialize + Clone,
        S: Serializer,
    {
        let hm_clone: HashMap<&K, V> = hm.iter().map(|(k, v)| (k, (**v).clone())).collect();
        hm_clone.serialize(s)
    }

    #[inline]
    pub fn deser_arc_hm<'de, D, K, V>(d: D) -> Result<HashMap<K, Arc<V>>, D::Error>
    where
        K: Serialize + Eq + std::hash::Hash,
        V: Serialize + Clone,
        D: Deserializer<'de>,
        K: Deserialize<'de>,
        V: Deserialize<'de>,
    {
        let hm_clone: HashMap<K, V> = HashMap::deserialize(d)?;
        let hm = hm_clone
            .into_iter()
            .map(|(k, v)| (k, Arc::new(v)))
            .collect();
        Ok(hm)
    }

    #[inline(always)]
    pub fn ser_arcmut<T: Serialize, S: Serializer>(
        t: &Arc<Mutex<T>>,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        t.lock()
            .expect("could not lock mutex for serialization")
            .serialize(s)
    }

    #[inline(always)]
    pub fn deser_arcmut<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
        d: D,
    ) -> Result<Arc<Mutex<T>>, D::Error> {
        let t = T::deserialize(d)?;
        Ok(Arc::new(Mutex::new(t)))
    }
}
