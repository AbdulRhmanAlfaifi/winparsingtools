//! Contains structs to parse common windows date and time structs.
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::io::Result;
use chrono::{DateTime, Utc, NaiveDate, Duration};
use serde::{Serialize, Serializer};


/// [DosDateTime](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-filetimetodosdatetime#parameters) struct parser.
pub struct DosDateTime{
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minutes: u8,
    seconds: u8
}

impl DosDateTime{
    /// Create `DosDateTime` instence from components.
    pub fn new(year: u16, month: u8, day: u8, hour: u8, minutes: u8, seconds: u8) -> DosDateTime{
        DosDateTime {
            year,
            month,
            day,
            hour,
            minutes,
            seconds,
        }
    }
    /// Create DosDateTime struct from `u32`.
    pub fn from_u32(num: u32) -> Result<Self>{
        Ok(
            DosDateTime {
                day: (num & 31) as u8,
                month: ((num >> 5) & 15) as u8,
                year: ((num >> 9) & 127) as u16 + 1980,
                seconds: (((num >> 16) & 31) * 2 ) as u8,
                minutes: ((num >> 21) & 63) as u8,
                hour: ((num >> 27) & 31) as u8,
            }
        )
    }
}

impl Display for DosDateTime {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let ntime = match NaiveDate::from_ymd_opt(self.year as i32, self.month as u32, self.day as u32) {
            Some(date) => date.and_hms(self.hour as u32, self.minutes as u32, self.seconds as u32),
            None => NaiveDate::from_ymd(1980, 1, 1).and_hms(0, 0, 0)
        };
        let dos_date_time: DateTime<Utc> = DateTime::from_utc(
            ntime , Utc);
        write!(
            f,
            "{}",
            dos_date_time.format("%Y-%m-%dT%H:%M:%SZ")
        )
    }
}

impl Debug for DosDateTime {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{}",
            self.to_string()
        )
    }
}

impl Serialize for DosDateTime {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

/// [FILETIME](https://docs.microsoft.com/en-us/windows/win32/api/minwinbase/ns-minwinbase-filetime) struct parser.
/// From [forensicmatt](https://github.com/forensicmatt/RustyUsn/blob/master/src/utils.rs)
#[derive(Debug)]
pub struct FileTime(DateTime<Utc>);

impl FileTime {
    /// Create FileTime struct from `u64`.
    pub fn new(timestamp: u64) -> Self {
        FileTime (DateTime::from_utc(
            NaiveDate::from_ymd(1601, 1, 1)
                .and_hms_nano(0, 0, 0, 0)
                + Duration::microseconds(
                    (timestamp / 10) as i64
                ),
            Utc,
        ))
    }
}

impl Display for FileTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f,"{}",self.0.format("%Y-%m-%dT%H:%M:%SZ"))
    }
}

impl Serialize for FileTime {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<DateTime<Utc>> for FileTime {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

impl From<FileTime> for DateTime<Utc> {
    fn from(value: FileTime) -> Self {
        value.0
    }
}

impl From<&FileTime> for DateTime<Utc> {
    fn from(value: &FileTime) -> Self {
        value.0
    }
}