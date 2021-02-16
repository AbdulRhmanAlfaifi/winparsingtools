use super::Name;
use byteorder::ReadBytesExt;
use serde::Serialize;
use std::{
    convert::From,
    io::{Cursor, Read, Result, Seek, SeekFrom},
    fmt::{Display, Formatter, Result as FmtResult}
};

#[derive(Debug, Serialize)]
pub enum ControlPanelCategory {
    AllControlPanelItems,
    AppearanceAndPersonalization,
    HardwareAndSound,
    NetworkAndInternet,
    SoundSpeechAndAudioDevices,
    SystemAndSecurity,
    ClockLanguageRegion,
    EaseOfAccess,
    Programs,
    UserAccounts,
    SecurityCenter,
    MobilePC,
    UNKNWON(u8),
}

impl From<u8> for ControlPanelCategory {
    fn from(num: u8) -> Self {
        match num {
            0x0 => ControlPanelCategory::AllControlPanelItems,
            0x1 => ControlPanelCategory::AppearanceAndPersonalization,
            0x2 => ControlPanelCategory::HardwareAndSound,
            0x3 => ControlPanelCategory::NetworkAndInternet,
            0x4 => ControlPanelCategory::SoundSpeechAndAudioDevices,
            0x5 => ControlPanelCategory::SystemAndSecurity,
            0x6 => ControlPanelCategory::ClockLanguageRegion,
            0x7 => ControlPanelCategory::EaseOfAccess,
            0x8 => ControlPanelCategory::Programs,
            0x9 => ControlPanelCategory::UserAccounts,
            0x10 => ControlPanelCategory::SecurityCenter,
            0x11 => ControlPanelCategory::MobilePC,
            other => ControlPanelCategory::UNKNWON(other),
        }
    }
}

impl Display for ControlPanelCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let frindly_name = match &self {
            ControlPanelCategory::AllControlPanelItems => String::from("All Control Panel Items"),
            ControlPanelCategory::AppearanceAndPersonalization => String::from("Appearance and Personalization"),
            ControlPanelCategory::HardwareAndSound => String::from("Hardware and Sound"),
            ControlPanelCategory::NetworkAndInternet => String::from("Network and Internet"),
            ControlPanelCategory::SoundSpeechAndAudioDevices => String::from("Sound, Speech and Audio Devices"),
            ControlPanelCategory::SystemAndSecurity => String::from("System and Security"),
            ControlPanelCategory::ClockLanguageRegion => String::from("Clock, Language, and Region"),
            ControlPanelCategory::EaseOfAccess => String::from("Ease of Access"),
            ControlPanelCategory::Programs => String::from("Programs"),
            ControlPanelCategory::UserAccounts => String::from("User Accounts"),
            ControlPanelCategory::SecurityCenter => String::from("Security Center"),
            ControlPanelCategory::MobilePC => String::from("Mobile PC"),
            ControlPanelCategory::UNKNWON(other) => format!("Unknwon Category : '{:#02X}'", other.to_owned())
        };
        write!(f,"{}",frindly_name)
    }
}

/// [ControlPanelCategory](https://github.com/EricZimmerman/Lnk/tree/master/Lnk/ShellItems) struct parser.
#[derive(Debug, Serialize)]
pub struct ControlPanelCategoryShellItem {
    pub cat_type: ControlPanelCategory,
}

impl ControlPanelCategoryShellItem {
    pub fn from_buffer(buf: &[u8]) -> Result<Self> {
        Self::from_reader(&mut Cursor::new(buf))
    }

    pub fn from_reader<R: Read + Seek>(r: &mut R) -> Result<Self> {
        r.seek(SeekFrom::Current(8))?; // Seek to the category type byte
        let cat_type = ControlPanelCategory::from(r.read_u8()?);
        Ok(
            Self{
                cat_type
            }
        )
    }
}

impl Name for ControlPanelCategoryShellItem {
    fn name(&self) -> String {
        self.cat_type.to_string()
    }
}