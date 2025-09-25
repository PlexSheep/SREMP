use std::ops::Deref;
use std::{fmt::Display, sync::Arc};

// WARN: we use a standard mutex in this module, even though we are using it
// in async contexts. This is because we need to access the fields in functions for the PartialEq,
// Serialize traits and so on
use std::sync::Mutex;

use chrono::{DateTime, Utc};
use ed25519_dalek::VerifyingKey;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::error::CoreResult;

pub type MessageID = u32;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SharedMessage {
    #[serde(serialize_with = "ser_arc", deserialize_with = "deser_arc")]
    inner: Arc<Message>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Message {
    pub text: String,
    pub attachments: MessageAttachments,
    pub meta: MessageMeta,
    pub flags: MessageFlags,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MessageMeta {
    pub author_key: VerifyingKey,
    pub time_received: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MessageAttachments {
    pub files: Vec<Vec<u8>>,
}

// this macro makes it a bit easier to declare our flags, even if imperfect because we cant create
// concatenated IDs with declarative macros.
// Ideally, this would be a proc macro, but those are kinda hard to code.
macro_rules! declare_flags {
    ($sname:ident ,$($flag:ident $(,)?)+) => {
        #[derive(Debug, Clone, Serialize, Deserialize, Default)]
        pub struct $sname {
            $(
                #[serde(serialize_with = "ser_arcmut", deserialize_with = "deser_arcmut")]
                pub $flag: Arc<Mutex<bool>>,
            )+
        }

        impl $sname {
            $(
                /// Read the value of the flag
                pub fn $flag(&self) -> bool {
                    self.$flag.lock().unwrap().clone()
                }
            )+
        }

        impl PartialEq for $sname {
            fn eq(&self, other: &Self) -> bool {
                $( *self.$flag.lock().unwrap() == *other.$flag.lock().unwrap() &&)+ true
            }
        }

        impl Eq for MessageFlags {}
    }
}
declare_flags!(MessageFlags, received, sent, read);

impl MessageFlags {
    pub(crate) fn set_sent(&self, value: bool) {
        *self.sent.lock().unwrap() = value;
    }
    pub(crate) fn set_received(&self, value: bool) {
        *self.received.lock().unwrap() = value;
    }
    pub(crate) fn set_read(&self, value: bool) {
        *self.read.lock().unwrap() = value;
    }
}

impl Message {
    pub fn new(text: impl Display, time_received: DateTime<Utc>, author_key: VerifyingKey) -> Self {
        Self {
            text: text.to_string(),
            attachments: Default::default(),
            meta: MessageMeta::new(time_received, author_key),
            flags: Default::default(),
        }
    }

    #[inline]
    pub fn meta(&self) -> &MessageMeta {
        &self.meta
    }

    #[inline]
    pub fn from_wire(&self, raw: &[u8]) -> CoreResult<Self> {
        Ok(rmp_serde::from_slice(raw)?)
    }

    #[inline]
    pub fn to_wire(&self) -> Vec<u8> {
        rmp_serde::to_vec(self).expect("could not serialize Message")
    }
}

impl MessageMeta {
    pub fn new(time_received: DateTime<Utc>, author_key: VerifyingKey) -> Self {
        Self {
            time_received,
            author_key,
        }
    }
}

impl Deref for SharedMessage {
    type Target = Message;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<Message> for SharedMessage {
    fn from(value: Message) -> Self {
        SharedMessage {
            inner: Arc::new(value),
        }
    }
}

pub fn ser_arc<T: Serialize, S: Serializer>(t: &Arc<T>, s: S) -> Result<S::Ok, S::Error> {
    (*t).serialize(s)
}

pub fn deser_arc<'de, D: Deserializer<'de>, T: Deserialize<'de>>(d: D) -> Result<Arc<T>, D::Error> {
    let t = T::deserialize(d)?;
    Ok(Arc::new(t))
}

pub fn ser_arcmut<T: Serialize, S: Serializer>(t: &Arc<Mutex<T>>, s: S) -> Result<S::Ok, S::Error> {
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
