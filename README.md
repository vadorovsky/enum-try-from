# enum-try-from
Rust macros which create enums with `TryFrom` trait implementation.

## Examples

```rust
use enum_try_into::impl_enum_try_from;

impl_enum_try_from!(
    #[repr(u16)]
    #[derive(PartialEq, Eq, Debug)]
    enum MyEnum {
       Foo = 0,
       Bar = 1,
       Baz = 2,
    },
    u16,
    (),
    ()
);

fn main() {
    assert_eq!(MyEnum::try_from(0), Ok(MyEnum::Foo));
    assert_eq!(MyEnum::try_from(1), Ok(MyEnum::Bar));
    assert_eq!(MyEnum::try_from(2), Ok(MyEnum::Baz));
    assert_eq!(MyEnum::try_from(3), Err(()));
}
```

```rust
use enum_try_into::impl_enum_try_from;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum MyError {
    #[error("invalid value")]
    InvalidValue,
}

impl_enum_try_from!(
    #[repr(u16)]
    #[derive(PartialEq, Eq, Debug)]
    enum MyEnum {
        Foo = 0,
        Bar = 1,
        Baz = 2,
    },
    u16,
    MyError,
    MyError::InvalidValue
);
```

```rust
use enum_try_into::impl_enum_try_from;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum MyError {
   #[error("invalid value: {0}")]
   InvalidValue(u16),
}

impl_enum_try_from!(
    #[repr(u16)]
    #[derive(PartialEq, Eq, Debug)]
    enum MyEnum {
        Foo = 0,
        Bar = 1,
        Baz = 2,
    },
    u16,
    MyError,
    MyError::InvalidValue
);
```

If the value provided to `try_from` should be converted from big endian:

```rust
use enum_try_into::impl_enum_try_from_be;

impl_enum_try_from_be!(
   #[repr(u16)]
   #[derive(PartialEq, Eq, Debug)]
   enum MyEnum {
      Foo = 0x1234,
      Bar = 0x5678,
      Baz = 0x9abc,
   },
   u16,
   (),
   ()
);

fn main() {
    assert_eq!(MyEnum::try_from(0x3412), Ok(MyEnum::Foo));
    assert_eq!(MyEnum::try_from(0x7856), Ok(MyEnum::Bar));
    assert_eq!(MyEnum::try_from(0xbc9a), Ok(MyEnum::Baz));
    assert_eq!(MyEnum::try_from(0xdef0), Err(()));
}
```

## Why does it exist?

Rust projects very often consume values as regular integers and then try to
match them with enums. Doing so, requires implementing the `TryFrom` trait for
enums. Example:

```rust
#[repr(u16)]
#[derive(PartialEq, Eq, Debug)]
enum MyEnum {
    Foo = 0,
    Bar = 1,
    Baz = 2,
}

impl TryFrom<u16> for MyEnum {
    type Error = ();

    fn try_from(v: u16) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(MyEnum::Foo),
            1 => Ok(MyEnum::Bar),
            2 => Ok(MyEnum::Baz),
            _ => Err(()),
        }
    }
}

fn main() {
    assert_eq!(MyEnum::try_from(0), Ok(MyEnum::Foo));
    assert_eq!(MyEnum::try_from(1), Ok(MyEnum::Bar));
    assert_eq!(MyEnum::try_from(2), Ok(MyEnum::Baz));
    assert_eq!(MyEnum::try_from(3), Err(()));
}
```

It requires listing all enum variants multiple times and also requires writing
down new variants multiple times when adding them to the existing enum.

The goal of this crate is to avoid that and define an enum with variants only
once.
