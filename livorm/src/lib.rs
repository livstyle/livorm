
pub trait HelloLiv {
    fn hello_liv();
    fn auto_hello() {
        println!("trait HelloLiv 中默认实现的方法auto_hello");
    }
}

// Re-export #[derive(LivDb)].
//
// The reason re-exporting is not enabled by default is that disabling it would
// be annoying for crates that provide handwritten impls or data formats. They
// would need to disable default features and then explicitly re-enable std.
// #[cfg(feature = "livorm_derive")]
// #[allow(unused_imports)]
// #[macro_use]
extern crate livorm_derive;
// #[cfg(feature = "livorm_derive")]
// #[doc(hidden)]
pub use livorm_derive::*;
