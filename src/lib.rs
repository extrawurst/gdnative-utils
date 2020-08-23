#[macro_use]
pub mod macros;
pub mod extensions;
pub mod gdu;

use crate::gdu::*;
use gdnative::prelude::*;
pub fn test() -> Option<()> {
    let res = load!("" as PackedScene)
        .instance(PackedScene::GEN_EDIT_STATE_DISABLED)?
        .safe();
    let sub = get_node!("" from res as Node);
    let sub_cast = cast!(sub as Sprite);

    Some(())
}
