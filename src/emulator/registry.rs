use osui::state::State;

#[derive(Debug, Clone)]
pub struct Registry(pub State<[usize; 8]>);
