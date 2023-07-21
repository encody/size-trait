#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use size_trait::*;

#[test]
fn test() {
    struct Y<T: ZeroSize<true>>(T);

    let _ = Y(());

    struct X<T: SizeLessThan<10, true>>(T);

    let _ = X([0u8; 8]);
}
