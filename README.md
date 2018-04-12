# volatile_getset

A procedural macro for generating volatile getters and setters on fields.

Getters are generated as `fn volatile_get_field(&self) -> type`, while setters are generated as `fn volatile_set_field(&mut self, val: type)`.

These macros are not intended to be used on fields which require custom logic inside of their setters and getters. Just write your own in that case!

> Yes! It supports nightly with `pub(crate)` etc!

```rust
#[macro_use]
extern crate volatile_getset;

#[derive(VolatileGetters, VolatileSetters, Default)]
pub struct Foo<T> where T: Copy + Clone + Default {
    /// Doc comments are supported!
    /// Multiline, even.
    #[volatile_get] #[volatile_set]
    private: T,

    /// Doc comments are supported!
    /// Multiline, even.
    #[volatile_get = "pub"] #[volatile_set = "pub"]
    public: T,
}

fn main() {
    let mut foo = Foo::default();
    foo.volatile_set_private(2);
    assert_eq!(foo.volatile_get_private(), 2);
}
```