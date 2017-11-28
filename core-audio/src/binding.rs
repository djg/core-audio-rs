#[doc(hidden)]
pub trait Binding: Sized {
    type Raw;

    unsafe fn from_raw(raw: Self::Raw) -> Self;
    fn raw(&self) -> Self::Raw;
}
