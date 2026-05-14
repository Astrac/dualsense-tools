use std::time::Duration;

use super::{AxisId, ButtonEventKind, ButtonId, Event};
use crate::{events::determinator_config::DeterminatorConfig, state::DualsenseState};

#[derive(Clone, Debug)]
pub struct EventsDeterminator<const TILT_SAMPLES: usize> {
    last_state: DualsenseState,
    config: DeterminatorConfig<TILT_SAMPLES>,
}

impl<const TILT_SAMPLES: usize> EventsDeterminator<TILT_SAMPLES> {
    pub fn next_events(&mut self, cur: &DualsenseState, dt: Duration) -> Vec<Event> {
        let mut events_vec = Vec::new();
        let events = &mut events_vec;
        let last = self.last_state;

        axis_event(AxisId::LX, last.axes.x, cur.axes.x, events);
        axis_event(AxisId::LY, last.axes.y, cur.axes.y, events);
        axis_event(AxisId::RX, last.axes.rx, cur.axes.rx, events);
        axis_event(AxisId::RY, last.axes.ry, cur.axes.ry, events);
        axis_event(AxisId::LZ, last.axes.z, cur.axes.z, events);
        axis_event(AxisId::RZ, last.axes.rz, cur.axes.rz, events);
        button_event(ButtonId::Cross, last.cross, cur.cross, events);
        button_event(ButtonId::Square, last.square, cur.square, events);
        button_event(ButtonId::Circle, last.circle, cur.circle, events);
        button_event(ButtonId::Triangle, last.triangle, cur.triangle, events);
        button_event(ButtonId::L1, last.l1, cur.l1, events);
        button_event(ButtonId::R1, last.r1, cur.r1, events);
        button_event(ButtonId::L2, last.l2, cur.l2, events);
        button_event(ButtonId::R2, last.r2, cur.r2, events);
        button_event(ButtonId::L3, last.l3, cur.l3, events);
        button_event(ButtonId::R3, last.r3, cur.r3, events);
        button_event(ButtonId::Opt, last.option, cur.option, events);
        button_event(ButtonId::Share, last.share, cur.share, events);
        button_event(ButtonId::Mic, last.mic, cur.mic, events);
        button_event(
            ButtonId::TouchClick,
            last.touch_click,
            cur.touch_click,
            events,
        );

        if last.hat != cur.hat {
            events.push(Event::HatDirectionChanged(cur.hat));
        }

        if self.config.accel_events_enabled && last.accel != cur.accel {
            events.push(Event::AccelUpdated(cur.accel));
        }

        if self.config.gyro_events_enabled && last.gyro != cur.gyro {
            events.push(Event::GyroUpdated(cur.gyro));
        }

        match &mut self.config.tilt_estimator {
            None => (),
            Some(e) => {
                let last_tilt = e.current().fused;
                let cur_tilt = e.next_estimate(&cur.accel, &cur.gyro, &dt).fused;
                if last_tilt != cur_tilt {
                    events.push(Event::TiltEstimateUpdted(cur_tilt));
                }
            }
        }

        self.last_state = *cur;

        events_vec
    }
}

fn button_event(id: ButtonId, old: bool, new: bool, events: &mut Vec<Event>) {
    if !old && new {
        events.push(Event::Button(id, ButtonEventKind::Pressed))
    } else if old && !new {
        events.push(Event::Button(id, ButtonEventKind::Released))
    }
}

fn axis_event(id: AxisId, old: u8, new: u8, events: &mut Vec<Event>) {
    if old != new {
        events.push(Event::AxisChanged(id, new))
    }
}
