mod private {
    use twilight_model::id::Id;

    pub trait Sealed {}

    impl Sealed for str {}
    impl Sealed for i8 {}
    impl Sealed for i16 {}
    impl Sealed for i32 {}
    impl Sealed for i64 {}
    impl Sealed for isize {}
    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
    impl Sealed for u64 {}
    impl Sealed for usize {}
    impl Sealed for bool {}
    impl<T> Sealed for Id<T> {}
}

use std::fmt::Display;
use twilight_model::id::Id;

pub trait QueryParameter: Display + private::Sealed {}

impl QueryParameter for str {}
impl QueryParameter for i8 {}
impl QueryParameter for i16 {}
impl QueryParameter for i32 {}
impl QueryParameter for i64 {}
impl QueryParameter for isize {}
impl QueryParameter for u8 {}
impl QueryParameter for u16 {}
impl QueryParameter for u32 {}
impl QueryParameter for u64 {}
impl QueryParameter for usize {}
impl QueryParameter for bool {}
impl<T> QueryParameter for Id<T> {}
