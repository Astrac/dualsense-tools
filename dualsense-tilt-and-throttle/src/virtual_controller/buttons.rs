#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct Buttons {
    pub square: bool,
    pub triangle: bool,
    pub circle: bool,
    pub cross: bool,
    pub l1: bool,
    pub r1: bool,
    pub l2: bool,
    pub r2: bool,
    pub l3: bool,
    pub r3: bool,
    pub mic: bool,
    pub option: bool,
    pub ps: bool,
    pub share: bool,
    pub touch_click: bool,
}
