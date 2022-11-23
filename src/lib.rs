//! Rust macros which create enums with `TryFrom` trait implementation.
//!
//! ## Examples
//!
//! ```rust
//! use enum_try_from::impl_enum_try_from;
//!
//! impl_enum_try_from!(
//!     #[repr(u16)]
//!     #[derive(PartialEq, Eq, Debug)]
//!     enum MyEnum {
//!        Foo = 0,
//!        Bar = 1,
//!        Baz = 2,
//!     },
//!     u16,
//!     (),
//!     ()
//! );
//!
//! fn main() {
//!     assert_eq!(MyEnum::try_from(0), Ok(MyEnum::Foo));
//!     assert_eq!(MyEnum::try_from(1), Ok(MyEnum::Bar));
//!     assert_eq!(MyEnum::try_from(2), Ok(MyEnum::Baz));
//!     assert_eq!(MyEnum::try_from(3), Err(()));
//! }
//! ```
//!
//! ```rust
//! use enum_try_from::impl_enum_try_from;
//! use thiserror::Error;
//!
//! #[derive(Error, Debug, PartialEq, Eq)]
//! pub enum MyError {
//!    #[error("invalid value")]
//!    InvalidValue,
//! }
//!
//! impl_enum_try_from!(
//!     #[repr(u16)]
//!     #[derive(PartialEq, Eq, Debug)]
//!     enum MyEnum {
//!         Foo = 0,
//!         Bar = 1,
//!         Baz = 2,
//!     },
//!     u16,
//!     MyError,
//!     MyError::InvalidValue
//! );
//! ```
//!
//! If the value provided to `try_from` should be converted from big endian:
//!
//! ```rust
//! use enum_try_from::impl_enum_try_from_be;
//!
//! impl_enum_try_from_be!(
//!    #[repr(u16)]
//!    #[derive(PartialEq, Eq, Debug)]
//!    enum MyEnum {
//!       Foo = 0x1234,
//!       Bar = 0x5678,
//!       Baz = 0x9abc,
//!    },
//!    u16,
//!    (),
//!    ()
//! );
//!
//! fn main() {
//!     assert_eq!(MyEnum::try_from(0x3412), Ok(MyEnum::Foo));
//!     assert_eq!(MyEnum::try_from(0x7856), Ok(MyEnum::Bar));
//!     assert_eq!(MyEnum::try_from(0xbc9a), Ok(MyEnum::Baz));
//!     assert_eq!(MyEnum::try_from(0xdef0), Err(()));
//! }
//! ```
//!
//! ## Why does it exist?
//!
//! Rust projects very often consume values as regular integers and then try to
//! match them with enums. Doing so, requires implementing the `TryFrom` trait for
//! enums. Example:
//!
//! ```rust
//! #[repr(u16)]
//! #[derive(PartialEq, Eq, Debug)]
//! enum MyEnum {
//!     Foo = 0,
//!     Bar = 1,
//!     Baz = 2,
//! }
//!
//! impl TryFrom<u16> for MyEnum {
//!     type Error = ();
//!
//!     fn try_from(v: u16) -> Result<Self, Self::Error> {
//!         match v {
//!             0 => Ok(MyEnum::Foo),
//!             1 => Ok(MyEnum::Bar),
//!             2 => Ok(MyEnum::Baz),
//!             _ => Err(()),
//!         }
//!     }
//! }
//!
//! fn main() {
//!     assert_eq!(MyEnum::try_from(0), Ok(MyEnum::Foo));
//!     assert_eq!(MyEnum::try_from(1), Ok(MyEnum::Bar));
//!     assert_eq!(MyEnum::try_from(2), Ok(MyEnum::Baz));
//!     assert_eq!(MyEnum::try_from(3), Err(()));
//! }
//! ```
//!
//! It requires listing all enum variants multiple times and also requires writing
//! down new variants multiple times when adding them to the existing enum.
//!
//! The goal of this crate is to avoid that and define an enum with variants only
//! once.

#![no_std]

/// Macro which implements the `TryFrom` trait for the given enum and type.
///
/// The first argument is the enum to implement the trait for.
///
/// The second argument is the type to implement the trait for. Usually `i32` or
/// `u32` would be the best choice. However, if you are providing any concrete
/// primitive type in `repr` (i.e. `#[repr(u8)]`), then you should use the same
/// type.
///
/// The third argument is the type of the error which should be returned if the
/// value provided to `try_from` is not a valid variant of the enum.
///
/// The fourth argument is the concrete error value which should be returned if
/// the value provided to `try_from` is not a valid variant of the enum.
///
/// # Examples
///
/// ```
/// # use enum_try_from::impl_enum_try_from;
/// impl_enum_try_from!(
///     #[repr(u16)]
///     #[derive(PartialEq, Eq, Debug)]
///     enum MyEnum {
///        Foo = 0,
///        Bar = 1,
///        Baz = 2,
///     },
///     u16,
///     (),
///     ()
/// );
///
/// # fn main() {
/// assert_eq!(MyEnum::try_from(0), Ok(MyEnum::Foo));
/// assert_eq!(MyEnum::try_from(1), Ok(MyEnum::Bar));
/// assert_eq!(MyEnum::try_from(2), Ok(MyEnum::Baz));
/// assert_eq!(MyEnum::try_from(3), Err(()));
/// # }
/// ```
///
/// ```
/// use thiserror::Error;
/// # use enum_try_from::impl_enum_try_from;
///
/// #[derive(Error, Debug, PartialEq, Eq)]
/// pub enum MyError {
///    #[error("invalid value")]
///    InvalidValue,
/// }
///
/// impl_enum_try_from!(
///     #[repr(u16)]
///     #[derive(PartialEq, Eq, Debug)]
///     enum MyEnum {
///         Foo = 0,
///         Bar = 1,
///         Baz = 2,
///     },
///     u16,
///     MyError,
///     MyError::InvalidValue,
/// );
/// ```
#[macro_export]
macro_rules! impl_enum_try_from {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }, $type:ty, $err_ty:ty, $err:expr $(,)?) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl TryFrom<$type> for $name {
            type Error = $err_ty;

            fn try_from(v: $type) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as $type => Ok($name::$vname),)*
                    _ => Err($err),
                }
            }
        }
    }
}

/// Macro which implements the `TryFrom` trait for the given enum and type, with
/// conversion of the input value from big endian.
///
/// The first argument is the enum to implement the trait for.
///
/// The second argument is the type to implement the trait for. Usually `i32` or
/// `u32` would be the best choice. However, if you are providing any concrete
/// primitive type in `repr` (i.e. `#[repr(u8)]`), then you should use the same
/// type.
///
/// The third argument is the type of the error which should be returned if the
/// value provided to `try_from` is not a valid variant of the enum.
///
/// The fourth argument is the concrete error value which should be returned if
/// the value provided to `try_from` is not a valid variant of the enum.
///
/// # Examples
///
/// ```
/// # use enum_try_from::impl_enum_try_from_be;
/// impl_enum_try_from_be!(
///    #[repr(u16)]
///    #[derive(PartialEq, Eq, Debug)]
///    enum MyEnum {
///       Foo = 0x1234,
///       Bar = 0x5678,
///       Baz = 0x9abc,
///    },
///    u16,
///    (),
///    ()
/// );
///
/// # fn main() {
/// assert_eq!(MyEnum::try_from(0x3412), Ok(MyEnum::Foo));
/// assert_eq!(MyEnum::try_from(0x7856), Ok(MyEnum::Bar));
/// assert_eq!(MyEnum::try_from(0xbc9a), Ok(MyEnum::Baz));
/// assert_eq!(MyEnum::try_from(0xdef0), Err(()));
/// # }
/// ```
///
/// ```
/// use thiserror::Error;
/// # use enum_try_from::impl_enum_try_from_be;
///
/// #[derive(Error, Debug, PartialEq, Eq)]
/// pub enum MyError {
///     #[error("invalid value")]
///     InvalidValue,
/// }
///
/// impl_enum_try_from_be!(
///     #[repr(u16)]
///     #[derive(PartialEq, Eq, Debug)]
///     enum MyEnum {
///         Foo = 0x1234,
///         Bar = 0x5678,
///         Baz = 0x9abc,
///     },
///     u16,
///     MyError,
///     MyError::InvalidValue,
/// );
/// ```
#[macro_export]
macro_rules! impl_enum_try_from_be {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }, $type:ty, $err_ty:ty, $err:expr $(,)?) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl TryFrom<$type> for $name {
            type Error = $err_ty;

            fn try_from(v: $type) -> Result<Self, Self::Error> {
                let v = <$type>::from_be(v);
                match v {
                    $(x if x == $name::$vname as $type => Ok($name::$vname),)*
                    _ => Err($err),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_impl_enum_try_from() {
        impl_enum_try_from!(
            #[repr(u16)]
            #[derive(PartialEq, Eq, Debug)]
            enum Test {
                Test = 0x1234,
                Test2 = 0x5678,
            },
            u16,
            (),
            ()
        );

        assert_eq!(Test::try_from(0x1234), Ok(Test::Test));
        assert_eq!(Test::try_from(0x5678), Ok(Test::Test2));
    }

    #[test]
    fn test_impl_enum_try_from_be() {
        impl_enum_try_from_be!(
            #[repr(u16)]
            #[derive(PartialEq, Eq, Debug)]
            enum Test {
                Test = 0x1234,
                Test2 = 0x5678,
            },
            u16,
            (),
            ()
        );

        assert_eq!(Test::try_from(0x3412), Ok(Test::Test));
        assert_eq!(Test::try_from(0x7856), Ok(Test::Test2));
    }
}
