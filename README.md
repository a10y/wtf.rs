# WTF Rust

This repo exists to document some behavior of the Rust compiler that I found very confusing.

The `inner` mod defines a type `TupleStruct`, which has a single `String` component. This should
be visible to the module. However, when you use a `crate::TupleStruct` import instead of `super::TupleStruct`
import, you lost access to the tuple component.

