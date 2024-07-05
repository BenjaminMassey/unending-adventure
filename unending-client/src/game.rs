#[derive(PartialEq)]
pub enum State {
    Normal,
    Dialogue,
}

pub struct StateData {
    pub quest: Option<unending_server::Quest>,
}