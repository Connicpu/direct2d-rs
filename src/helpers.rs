use std::ops::Deref;

use com_wrapper::ComWrapper;

pub unsafe fn deref_com_wrapper<T, U>(wrapper: &T) -> &U
where
    T: ComWrapper,
    U: ComWrapper,
    T::Interface: Deref<Target = U::Interface>,
{
    assert_eq!(
        std::mem::size_of::<U>(),
        std::mem::size_of::<T>(),
        "Incompatible com wrappers derefed",
    );

    std::mem::transmute::<&T, &U>(wrapper)
}

pub unsafe fn deref_com_wrapper_mut<T, U>(wrapper: &mut T) -> &mut U
where
    T: ComWrapper,
    U: ComWrapper,
    T::Interface: Deref<Target = U::Interface>,
{
    assert_eq!(
        std::mem::size_of::<U>(),
        std::mem::size_of::<T>(),
        "Incompatible com wrappers derefed",
    );

    std::mem::transmute::<&mut T, &mut U>(wrapper)
}
