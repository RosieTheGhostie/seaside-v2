use crate::config::{
    bitflags_addons::{impl_deserialize, impl_serialize},
    presets::{
        impl_bitflags_has_basic_presets, maybe_using_preset, BasicPresets, HasBasicPresets,
        HasPresets,
    },
};
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Syscalls {
    #[serde(deserialize_with = "maybe_using_preset")]
    pub print: Print,
    #[serde(deserialize_with = "maybe_using_preset")]
    pub read: Read,
    #[serde(deserialize_with = "maybe_using_preset")]
    pub file: File,
    #[serde(deserialize_with = "maybe_using_preset")]
    pub system: System,
    #[serde(deserialize_with = "maybe_using_preset")]
    pub random: Random,
    #[serde(deserialize_with = "maybe_using_preset")]
    pub dialog: Dialog,
}

#[derive(Serialize, Deserialize)]
pub struct Dialog {
    #[serde(deserialize_with = "maybe_using_preset")]
    pub input: Input,
    #[serde(deserialize_with = "maybe_using_preset")]
    pub message: Message,
}

bitflags! {
    pub struct Print: u8 {
        const Int = 0x01;
        const Float = 0x02;
        const Double = 0x04;
        const String = 0x08;
        const Char = 0x10;
        const Hex = 0x20;
        const Bin = 0x40;
        const Uint = 0x80;
    }

    pub struct Read: u8 {
        const Int = 0x01;
        const Float = 0x02;
        const Double = 0x04;
        const String = 0x08;
        const Char = 0x10;
    }

    pub struct File: u8 {
        const Open = 0x01;
        const Read = 0x02;
        const Write = 0x04;
        const Close = 0x08;
    }

    pub struct System: u8 {
        const Sbrk = 0x01;
        const Exit = 0x02;
        const Exit2 = 0x04;
        const Time = 0x08;
        const Midi = 0x10;
        const Sleep = 0x20;
        const MidiSync = 0x40;
    }

    pub struct Random: u8 {
        const SetSeed = 0x01;
        const RandInt = 0x02;
        const RandIntRange = 0x04;
        const RandFloat = 0x08;
        const RandDouble = 0x10;
    }

    pub struct Input: u8 {
        const Confirm = 0x01;
        const Int = 0x02;
        const Float = 0x04;
        const Double = 0x08;
        const String = 0x10;
    }

    pub struct Message: u8 {
        const General = 0x01;
        const Int = 0x02;
        const Float = 0x04;
        const Double = 0x08;
        const String = 0x10;
    }
}

impl_serialize!(Print);
impl_deserialize!(Print);
impl_bitflags_has_basic_presets!(Print, Self::everything());
impl_serialize!(Read);
impl_deserialize!(Read);
impl_bitflags_has_basic_presets!(Read, Self::everything());
impl_serialize!(File);
impl_deserialize!(File);
impl_bitflags_has_basic_presets!(File, Self::nothing());
impl_serialize!(System);
impl_deserialize!(System);
impl_bitflags_has_basic_presets!(System, Self::Exit | Self::Exit2 | Self::Sleep);
impl_serialize!(Random);
impl_deserialize!(Random);
impl_bitflags_has_basic_presets!(Random, Self::everything());
impl_serialize!(Input);
impl_deserialize!(Input);
impl_bitflags_has_basic_presets!(Input, Self::nothing());
impl_serialize!(Message);
impl_deserialize!(Message);
impl_bitflags_has_basic_presets!(Message, Self::nothing());

impl HasPresets for Dialog {
    type Presets = BasicPresets;

    fn get_preset(preset: Self::Presets) -> Self {
        match preset {
            BasicPresets::Everything => Self {
                input: Input::everything(),
                message: Message::everything(),
            },
            BasicPresets::Nothing => Self {
                input: Input::nothing(),
                message: Message::nothing(),
            },
            BasicPresets::Recommended => Self::get_preset(BasicPresets::Nothing),
        }
    }
}
impl HasBasicPresets for Dialog {}