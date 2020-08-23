#[macro_export]
macro_rules! get_node {
    ($path:tt from root as $cls:ty) => {};
    ($path:tt from $node:ident as $cls:ty) => {
        unsafe {
            $node
                .get_node($path)
                .unwrap()
                .assume_safe()
                .cast::<$cls>()
                .unwrap()
        }
    };
}

#[macro_export]
macro_rules! cast {
    ($node:ident as $cls:ty) => {
        $node.cast::<$cls>().unwrap()
    };
}

#[macro_export]
macro_rules! load {
    ($path:tt as $cls:ty) => {
        unsafe {
            ResourceLoader::godot_singleton()
                .load($path, "", false)
                .unwrap()
                .assume_safe()
                .cast::<$cls>()
                .unwrap()
        }
    };
}

#[macro_export]
macro_rules! switch_scene {
    ($path:tt) => {};
}
