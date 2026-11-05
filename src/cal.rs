//! Dates without a library: the civil calendar from day numbers (Howard Hinnant's
//! `days_from_civil`), and the Russian names the calendar shows.

use windows_sys::Win32::Foundation::SYSTEMTIME;
use windows_sys::Win32::System::SystemInformation::GetLocalTime;

pub const MONTHS: [&str; 12] = ["Январь", "Февраль", "Март", "Апрель", "Май", "Июнь", "Июль", "Август", "Сентябрь", "Октябрь", "Ноябрь", "Декабрь"];
pub const MONTHS_GEN: [&str; 12] = ["января", "февраля", "марта", "апреля", "мая", "июня", "июля", "августа", "сентября", "октября", "ноября", "декабря"];
pub const WEEKDAYS_SHORT: [&str; 7] = ["Пн", "Вт", "Ср", "Чт", "Пт", "Сб", "Вс"];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Date {
    pub y: i32,
    pub m: u32,
    pub d: u32,
}

impl Date {
    pub fn new(y: i32, m: u32, d: u32) -> Date {
        Date { y, m, d }
    }

    pub fn today() -> Date {
        // SAFETY: fills a struct of ours.
        let t = unsafe {
            let mut t: SYSTEMTIME = std::mem::zeroed();
            GetLocalTime(&mut t);
            t
        };
        Date::new(t.wYear as i32, t.wMonth as u32, t.wDay as u32)
    }

    /// Days since 1970-01-01.
    pub fn days(self) -> i64 {
        let y = self.y as i64 - (self.m <= 2) as i64;
        let era = if y >= 0 { y } else { y - 399 } / 400;
        let yoe = y - era * 400;
        let mp = (self.m as i64 + 9) % 12; // March is 0
        let doy = (153 * mp + 2) / 5 + self.d as i64 - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        era * 146_097 + doe - 719_468
    }

    pub fn from_days(z: i64) -> Date {
        let z = z + 719_468;
        let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
        let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
        Date::new((yoe + era * 400) as i32 + (m <= 2) as i32, m, d)
    }

    /// 0 = Monday … 6 = Sunday. 1970-01-01 was a Thursday.
    pub fn weekday(self) -> usize {
        (self.days() + 3).rem_euclid(7) as usize
    }

    pub fn add_days(self, n: i64) -> Date {
        Date::from_days(self.days() + n)
    }

    pub fn first_of_month(self) -> Date {
        Date::new(self.y, self.m, 1)
    }

    /// The same day `n` months on, pulled back to the month's last day where it has fewer.
    pub fn add_months(self, n: i32) -> Date {
        let k = self.y * 12 + self.m as i32 - 1 + n;
        let (y, m) = (k.div_euclid(12), k.rem_euclid(12) as u32 + 1);
        Date::new(y, m, self.d.min(days_in_month(y, m)))
    }

    pub fn same_month(self, o: Date) -> bool {
        self.y == o.y && self.m == o.m
    }

    /// `YYYY-MM-DD`, as the notes file keeps it.
    pub fn parse(s: &str) -> Option<Date> {
        let mut it = s.trim().splitn(3, '-');
        let y = it.next()?.parse().ok()?;
        let m: u32 = it.next()?.parse().ok()?;
        let d: u32 = it.next()?.parse().ok()?;
        ((1..=12).contains(&m) && d >= 1 && d <= days_in_month(y, m)).then(|| Date::new(y, m, d))
    }

    pub fn iso(self) -> String {
        format!("{:04}-{:02}-{:02}", self.y, self.m, self.d)
    }

    /// "5 ноября", with the year when it is not this one.
    pub fn spoken(self, today: Date) -> String {
        let s = format!("{} {}", self.d, MONTHS_GEN[self.m as usize - 1]);
        if self.y == today.y { s } else { format!("{s} {}", self.y) }
    }
}

pub fn days_in_month(y: i32, m: u32) -> u32 {
    match m {
        2 if (y % 4 == 0 && y % 100 != 0) || y % 400 == 0 => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_and_weekdays() {
        assert_eq!(Date::new(1970, 1, 1).days(), 0);
        assert_eq!(Date::new(1970, 1, 1).weekday(), 3); // Thursday
        assert_eq!(Date::new(2026, 11, 5).weekday(), 3); // Thursday
        assert_eq!(Date::new(2000, 2, 29).add_days(1), Date::new(2000, 3, 1));
        for z in -800_000..800_000 {
            assert_eq!(Date::from_days(z).days(), z);
        }
    }

    #[test]
    fn months() {
        assert_eq!(Date::new(2026, 1, 31).add_months(1), Date::new(2026, 2, 28));
        assert_eq!(Date::new(2026, 1, 15).add_months(-1), Date::new(2025, 12, 15));
        assert_eq!(Date::parse("2026-11-05"), Some(Date::new(2026, 11, 5)));
        assert_eq!(Date::parse("2026-02-30"), None);
    }
}
