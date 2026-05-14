use crate::{control::AxisId, state::DualsenseState};

pub struct AxesJoiner {
    negative: AxisId,
    positive: AxisId,
}

impl AxesJoiner {
    pub fn joiner(&self, state: DualsenseState) -> i16 {
        let negative = state.get_axis(self.negative);
        let positive = state.get_axis(self.positive);

        positive.as_u8() as i16 - negative.as_u8() as i16
    }
}
