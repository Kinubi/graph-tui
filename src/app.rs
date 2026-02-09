use crossterm::event::{ self, Event, KeyCode, KeyEvent, KeyEventKind };

#[derive(Debug)]
pub struct App {
    pub counter: u64,
    pub graph: Vec<(u64, u64)>,
    pub exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            counter: 0,
            graph: Vec::new(),
            exit: false,
        }
    }

    pub fn update(&mut self) {
        self.counter += 1;
        self.graph.push((self.counter, self.counter * self.counter));
    }

    pub fn counter(&self) -> u64 {
        self.counter
    }

    pub fn on_key(&mut self, key: KeyEvent) {
        // Keep input handling at the app-intent level.
        match key.code {
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }

    pub fn increment_counter(&mut self) {
        self.counter += 1;
    }

    pub fn decrement_counter(&mut self) {
        if self.counter > 0 {
            self.counter -= 1;
        }
    }

    pub fn on_tick(&mut self) {
        self.update();
    }

    pub fn should_exit(&self) -> bool {
        self.exit
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }
}
