//! This module defines the room layout and the logic of each of the house's rooms.

use bevy::prelude::{Component, States};
use std::fmt;

#[derive(Component, Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, States)]
/// All rooms of the house with a boolean signifying if whether the player is inside this room or
/// not.
///
/// # Examples
///
/// ```
/// use home_invasion::components::rooms::Rooms;
///
/// let room = Rooms::Basement(true);
/// println!("{room}");
/// ```
pub enum Rooms {
    /// The house's basement.
    Basement(bool),
    /// The house's bathroom.
    Bathroom(bool),
    /// The house's bedroom.
    Bedroom(bool),
    /// The house's hallway.
    Hallway(bool),
    /// The house's home office.
    HomeOffice(bool),
    /// The house's kid's room.
    KidsRoom(bool),
    /// The house's kitchen.
    Kitchen(bool),
    /// The house's living room.
    LivingRoom(bool),
    /// The house's office.
    Office(bool),
    /// The house's shower.
    Shower(bool),
    /// The house's first storage.
    Storage1(bool),
    /// The house's second storage.
    Storage2(bool),
    /// The house's toilet.
    Toilet(bool),
}

impl fmt::Display for Rooms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Rooms::Basement(_) => "Basement",
            Rooms::Bathroom(_) => "Bathroom",
            Rooms::Bedroom(_) => "Bedroom",
            Rooms::Hallway(_) => "Hallway",
            Rooms::HomeOffice(_) => "Home Office",
            Rooms::KidsRoom(_) => "Kid's Room",
            Rooms::Kitchen(_) => "Kitchen",
            Rooms::LivingRoom(_) => "Living Room",
            Rooms::Office(_) => "Office",
            Rooms::Shower(_) => "Shower",
            Rooms::Storage1(_) => "Storage 1",
            Rooms::Storage2(_) => "Storage 2",
            Rooms::Toilet(_) => "Toilet",
        };
        write!(f, "{name}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rooms_fmt() {
        assert_eq!(format!("{}", Rooms::Basement(true)), "Basement");
        assert_eq!(format!("{}", Rooms::Bathroom(true)), "Bathroom");
        assert_eq!(format!("{}", Rooms::Bedroom(true)), "Bedroom");
        assert_eq!(format!("{}", Rooms::Hallway(true)), "Hallway");
        assert_eq!(format!("{}", Rooms::HomeOffice(true)), "Home Office");
        assert_eq!(format!("{}", Rooms::KidsRoom(true)), "Kid's Room");
        assert_eq!(format!("{}", Rooms::Kitchen(true)), "Kitchen");
        assert_eq!(format!("{}", Rooms::LivingRoom(true)), "Living Room");
        assert_eq!(format!("{}", Rooms::Office(true)), "Office");
        assert_eq!(format!("{}", Rooms::Shower(true)), "Shower");
        assert_eq!(format!("{}", Rooms::Storage1(true)), "Storage 1");
        assert_eq!(format!("{}", Rooms::Storage2(true)), "Storage 2");
        assert_eq!(format!("{}", Rooms::Toilet(true)), "Toilet");
    }
}
