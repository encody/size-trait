#![doc = include_str!("../README.md")]
#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![no_std]

mod sealed {
    pub trait SizeLessThan<const SIZE: usize, const CHECK: bool> {}

    impl<const SIZE: usize, T> SizeLessThan<SIZE, { core::mem::size_of::<T>() < SIZE }> for T {}

    pub trait SizeGreaterThan<const SIZE: usize, const CHECK: bool> {}

    impl<const SIZE: usize, T> SizeGreaterThan<SIZE, { core::mem::size_of::<T>() > SIZE }> for T {}

    pub trait Size<const SIZE: usize> {}

    impl<T> Size<{ core::mem::size_of::<T>() }> for T {}

    pub trait ZeroSize<const ZERO: bool> {}

    impl<T> ZeroSize<{ core::mem::size_of::<T>() == 0 }> for T {}
}

/// Describes a type whose size is less than `SIZE` bytes.
///
/// # Examples
///
/// ```
/// #![feature(generic_const_exprs)]
/// struct LessThan10Bytes<T: size_trait::SizeLessThan<10, true>>(T);
/// let _ = LessThan10Bytes([0u8; 5]);
/// ```
///
/// ```
/// #![feature(generic_const_exprs)]
/// struct NotLessThan2Bytes<T: size_trait::SizeLessThan<2, false>>(T);
/// let _ = NotLessThan2Bytes([0u8; 20]);
/// ```
///
/// # Compilation Errors
///
/// ```compile_fail,E0308
/// #![feature(generic_const_exprs)]
/// struct LessThan10Bytes<T: size_trait::SizeLessThan<10, true>>(T);
/// let _ = LessThan10Bytes([0u8; 11]);
/// ```
///
/// ```compile_fail,E0308
/// #![feature(generic_const_exprs)]
/// struct NotLessThan2Bytes<T: size_trait::SizeLessThan<2, false>>(T);
/// let _ = NotLessThan2Bytes(());
/// ```
///
/// ```compile_fail,E0308
/// #![feature(generic_const_exprs)]
/// struct ElevenBytes([u8; 11]);
/// impl size_trait::SizeLessThan<10, true> for ElevenBytes {}
/// ```
///
/// ```compile_fail,E0308
/// #![feature(generic_const_exprs)]
/// struct OneByte(u8);
/// impl size_trait::SizeLessThan<10, false> for OneByte {}
/// ```
pub trait SizeLessThan<const SIZE: usize, const CHECK: bool>:
    sealed::SizeLessThan<SIZE, CHECK>
{
}

impl<const SIZE: usize, const CHECK: bool, T: sealed::SizeLessThan<SIZE, CHECK>>
    SizeLessThan<SIZE, CHECK> for T
{
}

/// Describes a type whose size is greater than `SIZE` bytes.
///
/// # Examples
///
/// ```
/// #![feature(generic_const_exprs)]
/// struct GreaterThan10Bytes<T: size_trait::SizeGreaterThan<10, true>>(T);
/// let _ = GreaterThan10Bytes([0u8; 11]);
/// ```
///
/// ```
/// #![feature(generic_const_exprs)]
/// struct NotGreaterThan1Byte<T: size_trait::SizeGreaterThan<1, false>>(T);
/// let _ = NotGreaterThan1Byte([0u8; 1]);
/// let _ = NotGreaterThan1Byte(());
/// ```
///
/// # Compilation Errors
///
/// ```compile_fail,E0308
/// #![feature(generic_const_exprs)]
/// struct GreaterThan10Bytes<T: size_trait::SizeGreaterThan<10, true>>(T);
/// let _ = GreaterThan10Bytes(());
/// ```
///
/// ```compile_fail,E0308
/// #![feature(generic_const_exprs)]
/// struct NotGreaterThan1Byte<T: size_trait::SizeGreaterThan<1, false>>(T);
/// let _ = NotGreaterThan1Byte([0u8; 2]);
/// ```
///
/// ```compile_fail,E0308
/// #![feature(generic_const_exprs)]
/// struct NineBytes([u8; 9]);
/// impl size_trait::SizeGreaterThan<10, true> for NineBytes {}
/// ```
///
/// ```compile_fail,E0308
/// #![feature(generic_const_exprs)]
/// struct OneByte(u8);
/// impl size_trait::SizeGreaterThan<0, false> for OneByte {}
/// ```
pub trait SizeGreaterThan<const SIZE: usize, const CHECK: bool>:
    sealed::SizeGreaterThan<SIZE, CHECK>
{
}

impl<const SIZE: usize, const CHECK: bool, T: sealed::SizeGreaterThan<SIZE, CHECK>>
    SizeGreaterThan<SIZE, CHECK> for T
{
}

/// Describes a type whose size is at most `SIZE` bytes.
///
/// # Examples
///
/// ```
/// #![feature(generic_const_exprs)]
/// struct MaxSize10Bytes<T: size_trait::MaxSize<10>>(T);
/// let _ = MaxSize10Bytes(());
/// let _ = MaxSize10Bytes([0u8; 9]);
/// ```
///
/// # Compilation Errors
///
/// ```compile_fail,E0308
/// #![feature(generic_const_exprs)]
/// struct MaxSize10Bytes<T: size_trait::MaxSize<10>>(T);
/// let _ = MaxSize10Bytes([0u8; 11]);
/// ```
///
/// ```compile_fail,E0308
/// #![feature(generic_const_exprs)]
/// struct TenBytes([u8; 10]);
/// impl size_trait::MaxSize<5> for TenBytes {}
/// ```
pub trait MaxSize<const SIZE: usize>: SizeGreaterThan<SIZE, false> {}

impl<const SIZE: usize, T: SizeGreaterThan<SIZE, false>> MaxSize<SIZE> for T {}

/// Describes a type whose size is at least `SIZE` bytes.
///
/// # Examples
///
/// ```
/// #![feature(generic_const_exprs)]
/// struct MinSize10Bytes<T: size_trait::MinSize<10>>(T);
/// let _ = MinSize10Bytes([0u8; 10]);
/// let _ = MinSize10Bytes(0u128);
/// ```
///
/// # Compilation Errors
///
/// ```compile_fail,E0308
/// #![feature(generic_const_exprs)]
/// struct MinSize10Bytes<T: size_trait::MinSize<10>>(T);
/// let _ = MinSize10Bytes(());
/// ```
///
/// ```compile_fail,E0308
/// #![feature(generic_const_exprs)]
/// struct TenBytes([u8; 10]);
/// impl size_trait::MinSize<15> for TenBytes {}
/// ```
pub trait MinSize<const SIZE: usize>: SizeLessThan<SIZE, false> {}

impl<const SIZE: usize, T: SizeLessThan<SIZE, false>> MinSize<SIZE> for T {}

/// Describes a type whose size is between `MIN` and `MAX` bytes (inclusive).
///
/// # Examples
///
/// ```
/// #![feature(generic_const_exprs)]
/// struct Bounded10Bytes<T: size_trait::BoundedSize<1, 10>>(T);
/// let _ = Bounded10Bytes(0u8);
/// let _ = Bounded10Bytes([0u8; 9]);
/// ```
///
/// # Compilation Errors
///
/// ```compile_fail,E0308
/// #![feature(generic_const_exprs)]
/// struct Bounded10Bytes<T: size_trait::BoundedSize<1, 10>>(T);
/// let _ = Bounded10Bytes(());
/// ```
///
/// ```compile_fail,E0308
/// #![feature(generic_const_exprs)]
/// struct Bounded10Bytes<T: size_trait::BoundedSize<1, 10>>(T);
/// let _ = Bounded10Bytes([0u8; 11]);
/// ```
///
/// ```compile_fail,E0308
/// #![feature(generic_const_exprs)]
/// struct TwoBytes([u8; 2]);
/// impl size_trait::BoundedSize<15, 100> for TwoBytes {}
/// ```
pub trait BoundedSize<const MIN: usize, const MAX: usize>: MinSize<MIN> + MaxSize<MAX> {}

impl<const MIN: usize, const MAX: usize, T: MinSize<MIN> + MaxSize<MAX>> BoundedSize<MIN, MAX>
    for T
{
}

/// Describes a type whose size is exactly `SIZE` bytes.
///
/// # Examples
///
/// ```
/// #![feature(generic_const_exprs)]
/// struct OneByte<T: size_trait::Size<1>>(T);
/// let _ = OneByte(0u8);
///
/// struct FourBytes<T: size_trait::Size<4>>(T);
/// let _ = FourBytes([0u8; 4]);
/// let _ = FourBytes(0u32);
/// ```
///
/// # Compilation Errors
///
/// ```compile_fail,E0308
/// #![feature(generic_const_exprs)]
/// struct OneByte(u8);
/// impl size_trait::Size<0> for OneByte {}
/// ```
///
/// ```compile_fail,E0308
/// #![feature(generic_const_exprs)]
/// struct ZeroBytes;
/// impl size_trait::Size<1> for ZeroBytes {}
/// ```
///
/// ```compile_fail,E0308
/// #![feature(generic_const_exprs)]
/// struct ZeroBytes<T: size_trait::Size<0>>(T);
/// let _ = ZeroBytes(0u8);
/// ```
///
/// ```compile_fail,E0308
/// #![feature(generic_const_exprs)]
/// struct TwoBytes<T: size_trait::Size<2>>(T);
/// let _ = TwoBytes(0u8);
/// ```
pub trait Size<const SIZE: usize>: sealed::Size<SIZE> {}

impl<const SIZE: usize, T: sealed::Size<SIZE>> Size<SIZE> for T {}

/// Describes a type whose size is zero.
///
/// # Examples
///
/// ```
/// #![feature(generic_const_exprs)]
/// struct Zst<T: size_trait::ZeroSize<true>>(T);
/// let _ = Zst(());
/// let _ = Zst(core::marker::PhantomData::<u8>);
/// let _ = Zst([] as [u64; 0]);
/// let _ = Zst(Zst(Zst(Zst((Zst(()), Zst(()))))));
///
/// struct NonZst<T: size_trait::ZeroSize<false>>(T);
/// let _ = NonZst(0);
/// let _ = NonZst(true);
/// let _ = NonZst(&[] as &[u64]);
/// let _ = NonZst([0u8; 100]);
/// let _ = NonZst(NonZst(NonZst(NonZst((NonZst(0u8), NonZst(0u8))))));
/// ```
///
/// # Compilation Errors
///
/// ```compile_fail,E0308
/// #![feature(generic_const_exprs)]
/// struct OneByte(u8);
/// impl size_trait::ZeroSize<true> for OneByte {}
/// ```
///
/// ```compile_fail,E0308
/// #![feature(generic_const_exprs)]
/// struct ZeroBytes;
/// impl size_trait::ZeroSize<false> for ZeroBytes {}
/// ```
///
/// ```compile_fail,E0308
/// #![feature(generic_const_exprs)]
/// struct Nothing<T: size_trait::ZeroSize<true>>(T);
/// let _ = Nothing(0u8);
/// ```
///
/// ```compile_fail,E0308
/// #![feature(generic_const_exprs)]
/// struct Something<T: size_trait::ZeroSize<false>>(T);
/// let _ = Something(());
/// ```
pub trait ZeroSize<const ZERO: bool>: sealed::ZeroSize<ZERO> {}

impl<const ZERO: bool, T: sealed::ZeroSize<ZERO>> ZeroSize<ZERO> for T {}

#[cfg(test)]
mod tests {
    use crate::{BoundedSize, MaxSize, MinSize, Size, SizeGreaterThan, SizeLessThan, ZeroSize};

    #[test]
    fn zero_size() {
        struct Nothing<T: ZeroSize<true>>(T);
        struct Something<T: ZeroSize<false>>(T);

        let _ = Nothing(());
        let _ = Nothing(core::marker::PhantomData::<u8>);
        let _ = Nothing([] as [u64; 0]);
        let _ = Nothing(Nothing(Nothing(Nothing((Nothing(()), Nothing(()))))));
        // let _ = Nothing(0u8); // error!

        let _ = Something(0);
        let _ = Something(true);
        let _ = Something(&[] as &[u64]);
        let _ = Something([0u8; 100]);
        // let _ = Something(()); // error!
    }

    #[test]
    fn size() {
        struct ZeroBytes<T: Size<0>>(T);
        let _ = ZeroBytes(());
        let _ = ZeroBytes(core::marker::PhantomData::<u8>);
        let _ = ZeroBytes([] as [u64; 0]);

        struct TwoBytes<T: Size<2>>(T);
        let _ = TwoBytes([0u8; 2]);
        let _ = TwoBytes(0u16);
        let _ = TwoBytes((0u8, 0u8));
    }

    #[test]
    fn size_less_than() {
        struct LessThan10Bytes<T: SizeLessThan<10, true>>(T);
        let _ = LessThan10Bytes(());
        let _ = LessThan10Bytes([0u8; 9]);

        struct LessThan1Byte<T: SizeLessThan<1, true>>(T);
        let _ = LessThan1Byte(());

        struct NotLessThan2Bytes<T: SizeLessThan<2, false>>(T);
        let _ = NotLessThan2Bytes((0u8, 0u8));
        let _ = NotLessThan2Bytes([0u8; 20]);
    }

    #[test]
    fn size_greater_than() {
        struct GreaterThan10Bytes<T: SizeGreaterThan<10, true>>(T);
        let _ = GreaterThan10Bytes([0u8; 11]);
        let _ = GreaterThan10Bytes(0u128);

        struct NotGreaterThan1Byte<T: SizeGreaterThan<1, false>>(T);
        let _ = NotGreaterThan1Byte(());
        let _ = NotGreaterThan1Byte(0u8);
    }

    #[test]
    fn min_size() {
        struct MinSize10Bytes<T: MinSize<10>>(T);
        let _ = MinSize10Bytes([0u8; 10]);
        let _ = MinSize10Bytes(0u128);

        struct MinSize1Byte<T: MinSize<1>>(T);
        let _ = MinSize1Byte([0u8; 10]);
        let _ = MinSize1Byte(0u8);
    }

    #[test]
    fn max_size() {
        struct MaxSize10Bytes<T: MaxSize<10>>(T);
        let _ = MaxSize10Bytes(());
        let _ = MaxSize10Bytes([0u8; 9]);

        struct MaxSize1Byte<T: MaxSize<1>>(T);
        let _ = MaxSize1Byte(());
        let _ = MaxSize1Byte(0u8);
    }

    #[test]
    fn bounded() {
        struct Bounded10Bytes<T: BoundedSize<1, 10>>(T);
        let _ = Bounded10Bytes(0u8);
        let _ = Bounded10Bytes([0u8; 9]);

        struct Bounded1Byte<T: BoundedSize<1, 1>>(T);
        let _ = Bounded1Byte(0u8);
    }
}
