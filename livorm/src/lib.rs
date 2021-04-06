#[macro_use]
extern crate lazy_static;
extern crate mongodb;

use std::sync::Arc;

use mongodb::Client;


lazy_static! {
    static ref LIV_DB_VEC: Arc<Vec<Client>> = Arc::new(vec![]);
}

pub struct DbConfig {
    pub _type: String,
}

async fn init_db_str(mut str: String) {
    // "mongodb://127.0.0.1"
    str = String::from("mongodb://127.0.0.1");
    let client = Client::with_uri_str(str.as_str()).await.expect("msg");
    // (*LIV_DB_VEC).push(client);
}

pub trait DbCreate {
}


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
