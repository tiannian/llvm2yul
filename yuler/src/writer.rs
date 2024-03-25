use std::io::Write;

use anyhow::Result;

pub struct Writer<W> {
    pub(crate) w: W,
    step: usize,
    tab: String,
    begin_line: bool,
}

impl<W> Writer<W> {
    pub fn new(w: W, tab: impl Into<String>) -> Self {
        Self {
            w,
            step: 0,
            tab: tab.into(),
            begin_line: true,
        }
    }
}

impl<W> Writer<W>
where
    W: Write,
{
    pub fn enter_block(&mut self) {
        self.step += 1;
    }

    pub fn leave_block(&mut self) {
        self.step -= 1;
    }

    pub fn write_str(&mut self, s: &str) -> Result<()> {
        if self.begin_line {
            for _ in 0..self.step {
                self.w.write_all(self.tab.as_bytes())?;
            }
        }
        self.w.write_all(s.as_bytes())?;
        self.begin_line = false;
        Ok(())
    }

    pub fn write_end(&mut self) -> Result<()> {
        self.w.write_all(b"\n")?;
        self.begin_line = true;
        Ok(())
    }
}
