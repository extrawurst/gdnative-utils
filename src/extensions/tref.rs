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

pub trait TRefExtension<'a, T: GodotObject> {
    fn safe(self) -> TRef<'a, T, Shared>;
}

impl<'a, T: GodotObject> TRefExtension<'a, T> for TRef<'a, T, Shared> {
    fn safe(self) -> TRef<'a, T, Shared> {
        self
    }
}
