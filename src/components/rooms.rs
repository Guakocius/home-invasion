use bevy::prelude::{Component, States};

#[derive(Component, Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, States)]
pub enum Rooms {
    Basement(bool),
    Bathroom(bool),
    Bedroom(bool),
    Hallway(bool),
    HomeOffice(bool),
    KidsRoom(bool),
    Kitchen(bool),
    LivingRoom(bool),
    Office(bool),
    Shower(bool),
    Storage1(bool),
    Storage2(bool),
    Toilet(bool),
}

#[cfg(test)]
mod tests {}
