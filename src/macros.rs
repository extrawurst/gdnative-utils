#[macro_export]
macro_rules! get_node {
    ($path:tt from !root as $cls:ty) => {};
    ($path:tt from $node:ident as $cls:ty) => {
        $node
            .safe()
            .get_node($path)
            .unwrap()
            .safe()
            .cast::<$cls>()
            .unwrap()
    };
    ($path:tt from $node:ident) => {
        $node.safe().get_node($path).unwrap().safe()
    };
}

#[macro_export]
macro_rules! cast {
    ($node:ident as $cls:ty) => {
        $node.safe().cast::<$cls>().unwrap()
    };
}

#[macro_export]
macro_rules! free {
    ($node:expr) => {
        $node.safe().queue_free()
    };
}

#[macro_export]
macro_rules! load {
    ($path:tt as $cls:ty) => {
        ResourceLoader::godot_singleton()
            .load($path, "", false)
            .unwrap()
            .safe()
            .cast::<$cls>()
            .unwrap()
    };
    ($path:tt) => {
        ResourceLoader::godot_singleton()
            .load($path, "", false)
            .unwrap()
            .safe()
    };
}

#[macro_export]
macro_rules! switch_scene {
    ($path:tt) => {};
}
