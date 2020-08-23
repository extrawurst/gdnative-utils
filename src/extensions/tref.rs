use gdnative::{
    object::{AssumeSafeLifetime, LifetimeConstraint},
    prelude::*,
};

pub trait RefExtension<T: GodotObject> {
    fn safe<'a, 'r>(&'r self) -> TRef<'a, T, Shared>
    where
        AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<T::RefKind>;
}

impl<T: GodotObject> RefExtension<T> for Ref<T> {
    fn safe<'a, 'r>(&'r self) -> TRef<'a, T, Shared>
    where
        AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<T::RefKind>,
    {
        unsafe { self.assume_safe() }
    }
}
