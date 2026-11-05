//! What the calendar remembers: red circles and notes, one line each in a plain text file
//! (`%APPDATA%\Glint Calendar\notes.txt`), so it can be read and edited by hand too:
//!
//! ```text
//! 2026-11-05	mark
//! 2026-11-07	note	Забрать посылку
//! ```
//!
//! Written whole on every change through a temporary file and a rename: a crash mid-write never
//! leaves half a file. Read again when the calendar opens if the file changed meanwhile.

use crate::cal::Date;
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Default, Clone)]
pub struct Day {
    pub mark: bool,
    pub note: Option<String>,
}

pub struct Notes {
    path: PathBuf,
    days: BTreeMap<Date, Day>,
    stamp: Option<SystemTime>,
}

const HEADER: &str = "# Glint Calendar: дата, затем mark (красный кружок) или note и текст заметки, через Tab.\n";

impl Notes {
    pub fn load() -> Notes {
        let base = std::env::var_os("APPDATA").map(PathBuf::from).unwrap_or_else(std::env::temp_dir);
        let mut n = Notes { path: base.join("Glint Calendar").join("notes.txt"), days: BTreeMap::new(), stamp: None };
        n.read();
        n
    }

    fn modified(&self) -> Option<SystemTime> {
        std::fs::metadata(&self.path).and_then(|m| m.modified()).ok()
    }

    fn read(&mut self) {
        self.stamp = self.modified();
        self.days.clear();
        let Ok(text) = std::fs::read_to_string(&self.path) else { return };
        for line in text.lines() {
            if line.starts_with('#') {
                continue;
            }
            let mut it = line.splitn(3, '\t');
            let (Some(date), Some(kind)) = (it.next().and_then(Date::parse), it.next()) else { continue };
            let day = self.days.entry(date).or_default();
            match kind.trim() {
                "mark" => day.mark = true,
                "note" => {
                    let t = it.next().unwrap_or("").trim();
                    if !t.is_empty() {
                        day.note = Some(t.to_string());
                    }
                }
                _ => {}
            }
        }
    }

    /// The file was edited by hand (or by another copy) since it was read.
    pub fn refresh(&mut self) {
        if self.modified() != self.stamp {
            self.read();
        }
    }

    pub fn day(&self, d: Date) -> Option<&Day> {
        self.days.get(&d)
    }

    pub fn toggle_mark(&mut self, d: Date) {
        let day = self.days.entry(d).or_default();
        day.mark = !day.mark;
        self.save();
    }

    pub fn set_note(&mut self, d: Date, text: Option<&str>) {
        let t = text.map(|t| t.replace(['\t', '\r', '\n'], " ").trim().to_string()).filter(|t| !t.is_empty());
        self.days.entry(d).or_default().note = t;
        self.save();
    }

    fn save(&mut self) {
        self.days.retain(|_, d| d.mark || d.note.is_some());
        let mut out = String::from(HEADER);
        for (date, day) in &self.days {
            if day.mark {
                out += &format!("{}\tmark\n", date.iso());
            }
            if let Some(n) = &day.note {
                out += &format!("{}\tnote\t{n}\n", date.iso());
            }
        }
        if let Some(dir) = self.path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let tmp = self.path.with_extension("tmp");
        if std::fs::write(&tmp, out).is_ok() {
            let _ = std::fs::rename(&tmp, &self.path);
        }
        self.stamp = self.modified();
    }
}
