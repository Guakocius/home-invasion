use bevy::prelude::{Component, States};

#[derive(Component, Debug)]
pub enum Rooms {
    Bedroom,
    Hallway,
    Kitchen,
    Livingroom,
    Storage,
}

#[derive(Component, Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord,Default, States)]
pub enum InsideRoom {
    InHallway,
    InKitchen,
    InLivingroom,
    InStorage,
    #[default]
    InBedroom,
}
