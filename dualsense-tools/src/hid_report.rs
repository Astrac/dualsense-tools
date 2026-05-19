use crate::state::{DualsenseState, HatDirection};

/// Update a state struct parsing a HID report
pub fn read_input_report(report: &[u8; 64], state: &mut DualsenseState) {
    state.axes.x = report[1].into();
    state.axes.y = report[2].into();
    state.axes.rx = report[3].into();
    state.axes.ry = report[4].into();
    state.axes.z = report[5].into();
    state.axes.rz = report[6].into();
    state.hat = read_hat(report[8]);
    state.triangle = (report[8] & 0b10000000) != 0;
    state.circle = (report[8] & 0b01000000) != 0;
    state.cross = (report[8] & 0b00100000) != 0;
    state.square = (report[8] & 0b00010000) != 0;
    state.l1 = (report[9] & 0b00000001) != 0;
    state.r1 = (report[9] & 0b00000010) != 0;
    state.l2 = (report[9] & 0b00000100) != 0;
    state.r2 = (report[9] & 0b00001000) != 0;
    state.share = (report[9] & 0b00010000) != 0;
    state.option = (report[9] & 0b00100000) != 0;
    state.l3 = (report[9] & 0b01000000) != 0;
    state.r3 = (report[9] & 0b10000000) != 0;
    state.ps = (report[10] & 0b00000001) != 0;
    state.touch_click = (report[10] & 0b00000010) != 0;
    state.mic = (report[10] & 0b00000100) != 0;
    state.gyro.x = i16::from_le_bytes([report[16], report[17]]).into();
    state.gyro.y = i16::from_le_bytes([report[18], report[19]]).into();
    state.gyro.z = i16::from_le_bytes([report[20], report[21]]).into();
    state.accel.x = i16::from_le_bytes([report[22], report[23]]).into();
    state.accel.y = i16::from_le_bytes([report[24], report[25]]).into();
    state.accel.z = i16::from_le_bytes([report[26], report[27]]).into();
}

/// Parses the hat state from the dedicated byte in the HID report
fn read_hat(hat_byte: u8) -> HatDirection {
    match hat_byte & 0b00001111 {
        0 => HatDirection::Up,
        1 => HatDirection::UpRight,
        2 => HatDirection::Right,
        3 => HatDirection::DownRight,
        4 => HatDirection::Down,
        5 => HatDirection::DownLeft,
        6 => HatDirection::Left,
        7 => HatDirection::UpLeft,
        // We interpret anything that has the 4th digit set as Neutral
        _ => HatDirection::Neutral,
    }
}

#[test]
fn test_read_neutral_input_report() {
    let mut report: [u8; 64] = [0; 64];
    // Neutral hat has value 8, not 0
    report[8] = report[8] | 0b00001000;

    let mut state = DualsenseState::default();
    read_input_report(&report, &mut state);

    assert_eq!(state, DualsenseState::default());
}

#[test]
fn test_read_neutral_inverse_input_report() {
    use crate::state::{Accel, DualsenseAxes, DualsenseSensorValue, Gyro};

    let mut report: [u8; 64] = [u8::MAX; 64];
    // Neutral hat cannot have value 0b1111 (15), we test for up-left
    report[8] = report[8] & 0b11110111;

    // Two-complement value of a sequence of all 1 digits
    let sensor_value: DualsenseSensorValue = (-1).into();
    let expected_state = DualsenseState {
        axes: DualsenseAxes {
            x: u8::MAX.into(),
            y: u8::MAX.into(),
            z: u8::MAX.into(),
            rx: u8::MAX.into(),
            ry: u8::MAX.into(),
            rz: u8::MAX.into(),
        },
        hat: HatDirection::UpLeft,
        triangle: true,
        circle: true,
        cross: true,
        square: true,
        l1: true,
        l2: true,
        r1: true,
        r2: true,
        l3: true,
        r3: true,
        share: true,
        option: true,
        ps: true,
        touch_click: true,
        mic: true,
        gyro: Gyro::new(sensor_value, sensor_value, sensor_value),
        accel: Accel::new(sensor_value, sensor_value, sensor_value),
    };

    let mut state = DualsenseState::default();
    read_input_report(&report, &mut state);

    assert_eq!(state, expected_state);
}
