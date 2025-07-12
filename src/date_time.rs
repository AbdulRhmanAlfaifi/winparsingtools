//! Contains structs to parse common windows date and time structs.
use chrono::{DateTime, Duration, NaiveDate, Utc};
use serde::{Serialize, Serializer};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::io::Result;

/// [DosDateTime](https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-filetimetodosdatetime#parameters) struct parser.
pub struct DosDateTime {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minutes: u8,
    seconds: u8,
}

impl DosDateTime {
    /// Create `DosDateTime` instence from components.
    pub fn new(year: u16, month: u8, day: u8, hour: u8, minutes: u8, seconds: u8) -> DosDateTime {
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
    pub fn from_u32(num: u32) -> Result<Self> {
        let day = (num & 31) as u8;
        let month = ((num >> 5) & 15) as u8;
        let year = ((num >> 9) & 127) as u16 + 1980;
        let mut seconds = (((num >> 16) & 31) * 2) as u8;
        let minutes = ((num >> 21) & 63) as u8;
        let hour = ((num >> 27) & 31) as u8;

        if seconds > 59 {
            seconds = 59;
        }

        Ok(DosDateTime {
            day,
            month,
            year,
            hour,
            minutes,
            seconds,
        })
    }
}

impl Default for DosDateTime {
    fn default() -> Self {
        DosDateTime::new(1980, 1, 1, 0, 0, 0)
    }
}

impl Display for DosDateTime {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let ntime =
            match NaiveDate::from_ymd_opt(self.year as i32, self.month as u32, self.day as u32) {
                Some(date) => {
                    match date.and_hms_opt(self.hour as u32, self.minutes as u32, self.seconds as u32) {
                        Some (time) => time,
                        // Unwraping here is safe because the timestamp is always valid.
                        None => NaiveDate::from_ymd_opt(1980, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                    }
                }
                // Unwraping here is safe because the timestamp is always valid.
                None => NaiveDate::from_ymd_opt(1980, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
            };
        let dos_date_time: DateTime<Utc> = DateTime::from_naive_utc_and_offset(ntime, Utc);
        write!(f, "{}", dos_date_time.format("%Y-%m-%dT%H:%M:%SZ"))
    }
}

impl Debug for DosDateTime {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.to_string())
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
        // Unwraping here is safe because the timestamp is always valid.
        FileTime(DateTime::from_naive_utc_and_offset(
            NaiveDate::from_ymd_opt(1601, 1, 1).unwrap().and_hms_nano_opt(0, 0, 0, 0).unwrap()
                + Duration::microseconds((timestamp / 10) as i64),
            Utc,
        ))
    }
}

impl Display for FileTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0.format("%Y-%m-%dT%H:%M:%SZ"))
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
