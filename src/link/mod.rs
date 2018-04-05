use self::quest_items::{QuestItems, Shield, Sword};
use addrs::link::*;
use core::fmt::{self, Display};
use system::memory::{ptr, read, read_str, reference, write};
use {Addr, Coord};

pub mod equips;
pub mod inventory;
pub mod item;
pub mod pearl;
pub mod quest_items;
pub mod song;
pub mod triforce;

#[repr(C, packed)]
pub struct Link {
    _p0: [u8; 1],
    pub heart_pieces: u8, // 8109
    _p1: [u8; 1],
    pub heart_quarters: u8, // 810B
    pub rupees: u16,        // 810C
    _p2: [u8; 8],
    pub sword_id: u8,  // 8116
    pub shield_id: u8, // 8117
    _p3: [u8; 3],
    pub max_magic: u8, // 811B
    pub magic: u8,     // 811C
}

impl Link {
    pub fn get() -> &'static mut Link {
        reference(LINK)
    }

    pub fn position() -> &'static mut Coord {
        reference(POSITION_OFFSET)
    }

    pub fn velocity_side() -> &'static mut f32 {
        reference(VELOCITY_SIDE)
    }

    pub fn velocity_front() -> &'static mut f32 {
        reference(VELOCITY_FRONT)
    }

    pub fn speed() -> &'static mut f32 {
        reference(SPEED)
    }

    pub fn room() -> u8 {
        read(ROOM)
    }

    pub fn horizontal_movement_direction() -> u16 {
        read(HORIZONTAL_MOVEMENT_DIRECTION)
    }

    pub fn air_meter() -> u16 {
        read(AIR_METER)
    }

    pub fn set_air_meter(frames: u16) {
        write(AIR_METER, frames);
    }

    pub fn name() -> &'static str {
        read_str(ptr(NAME))
    }

    pub fn activate_storage() {
        write(STORAGE, true);
    }

    pub fn set_sword(&mut self, sword: Sword) {
        let quest_items = QuestItems::get();
        quest_items.sword = sword;
        self.sword_id = sword.item_id();
    }

    pub fn set_shield(&mut self, shield: Shield) {
        let quest_items = QuestItems::get();
        quest_items.shield = shield;
        self.shield_id = shield.item_id();
    }

    pub fn set_collision(collision: CollisionType) {
        let ptr = ptr::<u16>(read::<Addr>(COLLISION_PTR) + (0x24B << 1));
        match collision {
            CollisionType::Default => unsafe {
                *ptr &= 0xFFFF ^ 0x4004;
            },
            CollisionType::ChestStorage => unsafe {
                *ptr = (*ptr & (0xFFFF ^ 0x4000)) | 0x4;
            },
            CollisionType::DoorCancel => unsafe {
                *ptr |= 0x4004;
            },
        }
    }

    pub fn collision() -> CollisionType {
        // I read the address stored at 0x803BDC40 add 0x24B << 1 to it
        // and that's the address of the collision flags
        let data = read::<u16>(read::<Addr>(COLLISION_PTR) + (0x24B << 1));
        let door_cancel_bit = data & 0x4000 != 0;
        let chest_storage_bit = data & 0x4 != 0;
        match (door_cancel_bit, chest_storage_bit) {
            (true, true) => CollisionType::DoorCancel,
            (_, true) => CollisionType::ChestStorage,
            _ => CollisionType::Default,
        }
    }
}

#[derive(Copy, Clone)]
pub enum CollisionType {
    Default,
    ChestStorage,
    DoorCancel,
}

impl Display for CollisionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            CollisionType::ChestStorage => "Chest Storage",
            CollisionType::DoorCancel => "Door Cancel",
            CollisionType::Default => "Default",
        };
        write!(f, "{}", s)
    }
}
