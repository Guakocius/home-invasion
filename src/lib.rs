#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
pub mod components {
    pub mod cam;
    pub mod house;
    pub mod player;
    pub mod rooms;
}

use components::{cam::*, house::*, player::*, rooms::*};
