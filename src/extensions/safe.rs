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

pub trait TRefNodeExtension<T: GodotObject> {
    fn safe<'a>(&'a self) -> &'a T;
}

impl<T: GodotObject> TRefNodeExtension<T> for T {
    fn safe<'a>(&'a self) -> &'a T {
        self
    }
}

pub trait InstanceExtension<T: NativeClass> {
    fn safe<'a, 'r>(&'r self) -> RefInstance<'a, T, Shared>
    where
        AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<<T::Base as GodotObject>::RefKind>;
}

impl<T: NativeClass> InstanceExtension<T> for Instance<T, Shared> {
    fn safe<'a, 'r>(&'r self) -> RefInstance<'a, T, Shared>
    where
        AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<<T::Base as GodotObject>::RefKind>,
    {
        unsafe { self.assume_safe() }
    }
}
