use std::time::Instant;

pub struct PerfTimer<'a> {
    name: &'a str,
    start: Instant,
    end: Option<Instant>,
}

impl<'a> PerfTimer<'a> {
    pub fn new(name: &'a str) -> Self {
        let start = Instant::now();
        Self {
            name,
            start,
            end: None,
        }
    }

    pub fn stop(&mut self) {
        let now = Instant::now();
        assert!(self.end.is_none(), "Timer was already stopped");
        self.end = Some(now);
    }

    pub fn print(&self) {
        assert!(self.end.is_some(), "Timer is still running");
        eprintln!("{} took {:?}", self.name, self.duration());
    }

    pub fn is_running(&self) -> bool {
        self.end.is_none()
    }

    pub fn duration(&self) -> std::time::Duration {
        let end = self.end.unwrap_or_else(Instant::now);
        end - self.start
    }
}

impl<'a> Drop for PerfTimer<'a> {
    fn drop(&mut self) {
        let now = Instant::now();
        if self.is_running() {
            self.end = Some(now);
            self.print();
        }
    }
}

pub fn get_day_input(day: i32) -> String {
    std::fs::read_to_string(format!("input/day{day}.txt")).unwrap()
}
