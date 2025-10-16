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
    use std::sync::{Arc, Mutex};

    pub fn ser_arc<T: Serialize, S: Serializer>(t: &Arc<T>, s: S) -> Result<S::Ok, S::Error> {
        (*t).serialize(s)
    }

    pub fn deser_arc<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
        d: D,
    ) -> Result<Arc<T>, D::Error> {
        let t = T::deserialize(d)?;
        Ok(Arc::new(t))
    }

    pub fn ser_arcmut<T: Serialize, S: Serializer>(
        t: &Arc<Mutex<T>>,
        s: S,
    ) -> Result<S::Ok, S::Error> {
        t.lock()
            .expect("could not lock mutex for serialization")
            .serialize(s)
    }

    pub fn deser_arcmut<'de, D: Deserializer<'de>, T: Deserialize<'de>>(
        d: D,
    ) -> Result<Arc<Mutex<T>>, D::Error> {
        let t = T::deserialize(d)?;
        Ok(Arc::new(Mutex::new(t)))
    }
}
