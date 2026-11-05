//! `glint-calendar.ini` next to the exe: `key = value`, `#` comments. Written with the defaults
//! on the first run, so the knobs are there to edit.

use std::path::PathBuf;

pub struct Config {
    /// The colour laid over the blur, RGB, and how much of it (0–255). Matched by eye and by
    /// sampled pixels to Explorer under ExplorerBlurMica (neutral grey ≈ 38): its own `a=50`
    /// black lands on top of Explorer's dark background, which the calendar does not have.
    pub tint: (u8, u8, u8),
    pub tint_alpha: u8,
    /// Saturday and Sunday in a colour of their own.
    pub weekends: bool,
}

const DEFAULT: &str = "\
# Glint Calendar. Удали файл, чтобы вернуть настройки по умолчанию.

# подложка поверх размытия: цвет RRGGBB и плотность 0..255 (подобрано под проводник с ExplorerBlurMica)
tint = 2e2e2e
tint_alpha = 120
# выходные другим цветом
weekends = true
";

impl Config {
    pub fn load() -> Config {
        let path = path();
        let text = std::fs::read_to_string(&path).unwrap_or_else(|_| {
            let _ = std::fs::write(&path, DEFAULT);
            DEFAULT.to_string()
        });
        let mut c = Config { tint: (0x2e, 0x2e, 0x2e), tint_alpha: 120, weekends: true };
        for line in text.lines() {
            let line = line.split('#').next().unwrap_or("");
            let Some((k, v)) = line.split_once('=') else { continue };
            let (k, v) = (k.trim(), v.trim());
            match k {
                "tint" => {
                    if let Ok(x) = u32::from_str_radix(v.trim_start_matches('#'), 16) {
                        c.tint = ((x >> 16) as u8, (x >> 8) as u8, x as u8);
                    }
                }
                "tint_alpha" => c.tint_alpha = v.parse().unwrap_or(120),
                "weekends" => c.weekends = matches!(v, "true" | "1" | "yes" | "on"),
                _ => {}
            }
        }
        c
    }
}

fn path() -> PathBuf {
    std::env::current_exe().unwrap_or_default().with_file_name("glint-calendar.ini")
}
