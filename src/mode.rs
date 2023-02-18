pub enum Mode {
    Normal(NormalMode),
    Insert(InsertMode),
    Visual(VisualMode)
}

trait ModeHandler {
    fn handle_input(&mut self);
}


pub struct NormalMode {
}

impl ModeHandler for NormalMode {
    fn handle_input(&mut self) {}
}


pub struct InsertMode {

}

pub struct VisualMode {
}



