#[cfg(not(target_arch = "x86"))]
compile_error!("Only x86 arch is supported.");

#[path = "../generated"]
#[allow(bad_style)]
mod generated {
    #[cfg(feature = "server")]
    pub mod server {
        fo_bindgen_exports::exports!(server);
    }

    #[cfg(feature = "client")]
    pub mod client {
        fo_bindgen_exports::exports!(client);
    }

    pub mod angelscript;
    pub mod num;
    pub mod opaque;
    pub mod stl {
        use std::cell::UnsafeCell;
        use std::marker::PhantomData;
        pub struct std_vector<V>([u32; 4], PhantomData<UnsafeCell<V>>);
        pub struct std_map<K, V>(
            [u32; 4],
            PhantomData<UnsafeCell<K>>,
            PhantomData<UnsafeCell<V>>,
        );
        pub struct std_set<V>([u32; 4], PhantomData<UnsafeCell<V>>);
        pub struct std_pair<A, B>(A, B);
        pub struct std_string([u32; 7]);

        use super::num::*;
        fo_bindgen_exports::stl!(num);
    }
    use stl::*;
}
pub use generated::*;
