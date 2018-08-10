use super::flag::Flag;

macro_rules! flags {
    ($($name:ident: $addr:expr, $bit:expr)*) => {
        $(
            pub const $name: Flag = Flag($addr, 1 << $bit);
        )*
    };
}

flags! {
	HAS_OPENED_TREASURE_CHART_29: 0x803B81EC, 0
	HAS_OPENED_TREASURE_CHART_34: 0x803B81EC, 1
	HAS_OPENED_TREASURE_CHART_18: 0x803B81EC, 2
	HAS_OPENED_TREASURE_CHART_16: 0x803B81EC, 3
	HAS_OPENED_TREASURE_CHART_28: 0x803B81EC, 4
	HAS_OPENED_TREASURE_CHART_4: 0x803B81EC, 5
	HAS_OPENED_TREASURE_CHART_3: 0x803B81EC, 6
	HAS_OPENED_TREASURE_CHART_40: 0x803B81EC, 7

	HAS_OPENED_TREASURE_CHART_2: 0x803B81ED, 0
	HAS_OPENED_TREASURE_CHART_38: 0x803B81ED, 1
	HAS_OPENED_TREASURE_CHART_39: 0x803B81ED, 2
	HAS_OPENED_TREASURE_CHART_24: 0x803B81ED, 3
	HAS_OPENED_TREASURE_CHART_6: 0x803B81ED, 4
	HAS_OPENED_TREASURE_CHART_12: 0x803B81ED, 5
	HAS_OPENED_TREASURE_CHART_35: 0x803B81ED, 6
	HAS_OPENED_TREASURE_CHART_1: 0x803B81ED, 7

	HAS_OPENED_TREASURE_CHART_11: 0x803B81EE, 0
	HAS_OPENED_TREASURE_CHART_15: 0x803B81EE, 1
	HAS_OPENED_TREASURE_CHART_30: 0x803B81EE, 2
	HAS_OPENED_TREASURE_CHART_20: 0x803B81EE, 3
	HAS_OPENED_TREASURE_CHART_5: 0x803B81EE, 4
	HAS_OPENED_TREASURE_CHART_23: 0x803B81EE, 5
	HAS_OPENED_TREASURE_CHART_31: 0x803B81EE, 6
	HAS_OPENED_TREASURE_CHART_33: 0x803B81EE, 7

	HAS_OPENED_TRIFORCE_CHART_1: 0x803B81EF, 0
	HAS_OPENED_TRIFORCE_CHART_2: 0x803B81EF, 1
	HAS_OPENED_TRIFORCE_CHART_3: 0x803B81EF, 2
	HAS_OPENED_TRIFORCE_CHART_4: 0x803B81EF, 3
	HAS_OPENED_TRIFORCE_CHART_5: 0x803B81EF, 4
	HAS_OPENED_TRIFORCE_CHART_6: 0x803B81EF, 5
	HAS_OPENED_TRIFORCE_CHART_7: 0x803B81EF, 6
	HAS_OPENED_TRIFORCE_CHART_8: 0x803B81EF, 7

	HAS_OPENED_BEEDLE_MAP: 0x803B81F0, 3

	HAS_OPENED_TREASURE_CHART_21: 0x803B81F1, 0
	HAS_OPENED_TREASURE_CHART_27: 0x803B81F1, 1
	HAS_OPENED_TREASURE_CHART_7: 0x803B81F1, 2
	HAS_OPENED_FAIRY_MAP: 0x803B81F1, 5

	HAS_OPENED_TREASURE_CHART_25: 0x803B81F2, 0
	HAS_OPENED_TREASURE_CHART_37: 0x803B81F2, 1
	HAS_OPENED_TREASURE_CHART_8: 0x803B81F2, 2
	HAS_OPENED_TREASURE_CHART_26: 0x803B81F2, 3
	HAS_OPENED_TREASURE_CHART_41: 0x803B81F2, 4
	HAS_OPENED_TREASURE_CHART_19: 0x803B81F2, 5
	HAS_OPENED_TREASURE_CHART_32: 0x803B81F2, 6
	HAS_OPENED_TREASURE_CHART_13: 0x803B81F2, 7

	HAS_OPENED_TREASURE_CHART_10: 0x803B81F3, 0
	HAS_OPENED_TREASURE_CHART_14: 0x803B81F3, 1
	HAS_OPENED_TINGLE_MAP: 0x803B81F3, 2
	HAS_OPENED_GHOST_SHIP_CHART: 0x803B81F3, 3
	HAS_OPENED_TREASURE_CHART_9: 0x803B81F3, 4
	HAS_OPENED_TREASURE_CHART_22: 0x803B81F3, 5
	HAS_OPENED_TREASURE_CHART_36: 0x803B81F3, 6
	HAS_OPENED_TREASURE_CHART_17: 0x803B81F3, 7
}
