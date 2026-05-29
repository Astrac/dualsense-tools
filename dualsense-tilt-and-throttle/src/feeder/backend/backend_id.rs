#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum BackendId {
    VJoy,
    Unsupported,
}

impl std::fmt::Display for BackendId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BackendId::VJoy => f.write_str("VJoy"),
            BackendId::Unsupported => f.write_str("Unsupported - VJoy not found"),
        }
    }
}
