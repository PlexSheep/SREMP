// idk what to do, the async channel needs the channel type in
// its error
#![allow(clippy::result_large_err)]

pub mod chat;
pub mod domain;
pub mod error;
pub mod identity;
pub mod net;

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
