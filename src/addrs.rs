pub mod game {
    use Addr;
    pub const CONSOLE: Addr = 0x804E1A60; // ArenaLow + 0x915C0
    pub const CONTROLLER_BUTTONS_DOWN: Addr = 0x803E0D2A;
    pub const CONTROLLER_BUTTONS_PRESSED: Addr = 0x803E0D2E;
    pub const EVENT_CANCEL: Addr = 0x803BD3A3;
    pub const FLAGS: Addr = 0x803B8700;
    pub const NEW_GAME_PLUS: Addr = 0x803B82A8;
    pub const PICTURE_COUNT: Addr = 0x803B8170;
    pub const TRIFORCE_SET: Addr = 0x803B82A9;
    pub const ENTRANCE: Addr = 0x803B8138;
    pub const WINDFALL_FLOWERS: Addr = 0x803B8814;
}

pub mod link {
    use Addr;

    pub use super::game::EVENT_CANCEL as STORAGE;

    pub const EQUIPS: Addr = 0x803B8111;
    pub const INVENTORY: Addr = 0x803B8144;
    pub const ITEM_SPAWN: Addr = 0x80026920;
    pub const LINK: Addr = 0x803B8108;
    pub const POSITION_OFFSET: Addr = 0x803d78fc;
    pub const VELOCITY_SIDE: Addr = 0x80398308;
    pub const VELOCITY_FRONT: Addr = 0x8039830C;
    pub const SPEED: Addr = 0x80398310;
    pub const ROOM: Addr = 0x803B9230;
    pub const HORIZONTAL_MOVEMENT_DIRECTION: Addr = 0x803EA3CA;
    pub const AIR_METER: Addr = 0x803BDC62;
    pub const NAME: Addr = 0x803B8264;
    pub const COLLISION_PTR: Addr = 0x803BDC40;
    pub const PEARL: Addr = 0x803B81C7;
    pub const QUEST_ITEMS: Addr = 0x803B81BC;
    pub const SONG: Addr = 0x803B81C5;
    pub const TRIFORCE: Addr = 0x803B81C6;
}

pub mod warping {
    use Addr;
    pub const ENTRANCE: Addr = 0x803BD23C;
    pub const WARP: Addr = 0x803BD248;
}

pub mod system {
    use Addr;
    pub const MEMALIGN: Addr = 0x8023ea88;
    pub const FREE: Addr = 0x8023eac0;
    pub const RANDOM_U32: Addr = 0x802a9500;
    pub const RANDOM: Addr = 0x80243b40;
    pub const CDYL_INIT_ASYNC: Addr = 0x80022A88;
    pub const DMETER_RUPY_INIT: Addr = 0x801F7868;
    pub const FRAME_COUNT: Addr = 0x80396218;
    pub const PAUSE_MENU_UP: Addr = 0x803EA537; // alternative: 0x80396228
    pub const WIND_DIRECTION: Addr = 0x803D894A;
    pub const FPCLY_LAYER: Addr = 0x8003b92c;
    pub const FPCLY_SET_CURRENT_LAYER: Addr = 0x8003b8cc;
    pub const FPCLY_CURRENT_LAYER: Addr = 0x8003b8d4;
    pub const ROOT_LAYER: Addr = 0x80365B7C;
    pub const DSTAGE_ACTOR_CREATE: Addr = 0x8003f484;
    pub const FOPACM_CREATE_APPEND: Addr = 0x80023f3c;
    pub const LAYER_LOADER: Addr = 0x80040f3c;
    pub const GROUND_CROSS: Addr = 0x80244074;
    pub const FOPMSGM_MESSAGE_SET: Addr = 0x8002b458;
    pub const DSTAGE_DT_C_STAGELOADER: Addr = 0x80040f98;
    pub const DSV_PLAYER_GET_ITEM_C_ONITEM: Addr = 0x800572bc;
    pub const DSV_PLAYER_RETURN_PLACE_C_SET: Addr = 0x800569c0;

    #[allow(non_snake_case)]
    pub mod JKRDvdFile {
        use Addr;
        pub const CONSTRUCTOR: Addr = 0x802b9d30;
        pub const DESTRUCTOR: Addr = 0x802b9ef4;
        pub const OPEN: Addr = 0x802b9ffc;
        pub const READ: Addr = 0x802ba15c;
        pub const CLOSE: Addr = 0x802ba0e4;
        pub const GET_FILE_SIZE: Addr = 0x802ba328;
    }

    #[allow(non_snake_case)]
    pub mod OS {
        use Addr;
        pub const GET_CURRENT_THREAD: Addr = 0x8030577c;
        pub const IS_THREAD_TERMINATED: Addr = 0x80305788;
        pub const CREATE_THREAD: Addr = 0x80305d84;
        pub const RESUME_THREAD: Addr = 0x803063ec;
        pub const SUSPEND_THREAD: Addr = 0x80306674;
        pub const JOIN_THREAD: Addr = 0x8030620c;
        pub const YIELD_THREAD: Addr = 0x80305d48;
        pub const INIT_MUTEX: Addr = 0x80303bb0;
        pub const LOCK_MUTEX: Addr = 0x80303be8;
        pub const UNLOCK_MUTEX: Addr = 0x80303cc4;
        pub const TRY_LOCK_MUTEX: Addr = 0x80303dfc;
        pub const INIT_COND: Addr = 0x80303eb8;
        pub const WAIT_COND: Addr = 0x80303ed8;
        pub const SIGNAL_COND: Addr = 0x80303fac;
        pub const GET_TIME: Addr = 0x80307334;
        pub const REPORT: Addr = 0x800068ec;
        pub const PANIC: Addr = 0x80006be8;
    }

    #[allow(non_snake_case)]
    pub mod JUTAssertion {
        use Addr;
        pub const GET_S_DEVICE: Addr = 0x802c4d0c;
        pub const SHOW_ASSERT: Addr = 0x802c4e04;
        pub const SET_VISIBLE: Addr = 0x802c5290;
    }
}
