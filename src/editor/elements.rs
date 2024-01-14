use eframe::epaint::Pos2;

pub trait Position {
    fn set_position(&mut self, position: Pos2);
    fn get_position(&self) -> Pos2;
}

pub struct EditorInput {
    position: Pos2,
}

pub struct EditorOutput {
    position: Pos2,
}

pub struct EditorComponent {
    position: Pos2,
}

pub struct EditorLine {
    start: Pos2,
    end: Pos2,
}

impl EditorInput {
    pub fn new(position: Pos2) -> Self {
        Self { position }
    }
}

impl EditorOutput {
    pub fn new(position: Pos2) -> Self {
        Self { position }
    }
}

impl EditorComponent {
    pub fn new(position: Pos2) -> Self {
        Self { position }
    }
}

impl EditorLine {
    pub fn new(start: Pos2, end: Pos2) -> Self {
        Self { start, end }
    }
}