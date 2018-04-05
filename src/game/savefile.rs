use addrs::game::{ENTRANCE, NEW_GAME_PLUS, PICTURE_COUNT, TRIFORCE_SET};
use system::memory::{read, reference};
use warping::Entrance;

pub fn is_new_game_plus() -> bool {
    read(NEW_GAME_PLUS)
}

pub fn get_picture_count() -> u8 {
    read(PICTURE_COUNT)
}

pub fn get_triforce_set() -> u8 {
    read(TRIFORCE_SET)
}

pub fn get_entrance() -> &'static mut Entrance {
    reference(ENTRANCE)
}
