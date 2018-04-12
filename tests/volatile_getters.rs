#[macro_use]
extern crate volatile_getset;

use submodule::other::{Plain, Generic, Where};

// For testing `pub(super)`
mod submodule {
    // For testing `pub(in super::other)`
    pub mod other {
        #[derive(VolatileGetters, Default)]
        pub struct Plain {
            /// A doc comment.
            /// Multiple lines, even.
            #[volatile_get]
            private_accessible: usize,
            
            /// A doc comment.
            #[volatile_get = "pub"]
            public_accessible: usize,

            // /// A doc comment.
            // #[get = "pub(crate)"]
            // crate_accessible: usize,

            // /// A doc comment.
            // #[get = "pub(super)"]
            // super_accessible: usize,

            // /// A doc comment.
            // #[get = "pub(in super::other)"]
            // scope_accessible: usize,
        }

        #[derive(VolatileGetters, Default)]
        pub struct Generic<T: Copy + Clone + Default> {
            /// A doc comment.
            /// Multiple lines, even.
            #[volatile_get]
            private_accessible: T,
            
            /// A doc comment.
            #[volatile_get = "pub"]
            public_accessible: T,

            // /// A doc comment.
            // #[get = "pub(crate)"]
            // crate_accessible: usize,

            // /// A doc comment.
            // #[get = "pub(super)"]
            // super_accessible: usize,

            // /// A doc comment.
            // #[get = "pub(in super::other)"]
            // scope_accessible: usize,
        }

        #[derive(VolatileGetters, Default)]
        pub struct Where<T> where T: Copy + Clone + Default {
            /// A doc comment.
            /// Multiple lines, even.
            #[volatile_get]
            private_accessible: T,
            
            /// A doc comment.
            #[volatile_get = "pub"]
            public_accessible: T,

            // /// A doc comment.
            // #[get = "pub(crate)"]
            // crate_accessible: usize,

            // /// A doc comment.
            // #[get = "pub(super)"]
            // super_accessible: usize,

            // /// A doc comment.
            // #[get = "pub(in super::other)"]
            // scope_accessible: usize,
        }

        #[test]
        fn test_plain() {
            let val = Plain::default();
            val.volatile_get_private_accessible();
        }

        #[test]
        fn test_generic() {
            let val = Generic::<usize>::default();
            val.volatile_get_private_accessible();
        }

        #[test]
        fn test_where() {
            let val = Where::<usize>::default();
            val.volatile_get_private_accessible();
        }
    }
}

#[test]
fn test_plain() {
    let val = Plain::default();
    val.volatile_get_public_accessible();
}

#[test]
fn test_generic() {
    let val = Generic::<usize>::default();
    val.volatile_get_public_accessible();
}

#[test]
fn test_where() {
    let val = Where::<usize>::default();
    val.volatile_get_public_accessible();
}