use std::fmt::Display;
use twilight_model::id::Id;

pub trait QueryParameter: Display {}

impl QueryParameter for &str {}
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
