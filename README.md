# Dualsense Tools

This crate contains a set of tools that interface with the Sony Dualsense PS5 controller:

- A low-level interface to read and decode HID reports from the controller
- An implementation of accelerometer-corrected-gyro tilt estimation (based on [this document](https://stanford.edu/class/ee267/notes/ee267_notes_imu.pdf))
- A bevy plugin that exposes tilt estimates as a resource, including an example to quickly visualize the estimates
- A virtual device application that creates a custom controller tailored for 6-axis simulations (e.g. space games)

## Dualsense tools library

The `dualsense-tools` crate implements an interface that facilitates reading the state of a Dualsense controller consisting of the functions:

```rust
pub fn read(&mut self) -> HidResult<DualsenseState>
pub fn read_into(&mut self, state: &mut DualsenseState) -> HidResult<()>
```

The `DualsenseState` struct contains information about axes, hat and buttons states.

It also provide a `TiltEstimator` component that implements basic handling of the gyro and accelerometer readings as well as a sensor-fusion approach similar to the one described in [this document](https://stanford.edu/class/ee267/notes/ee267_notes_imu.pdf) to determine the tilt (roll/pitch) of the gamepad. This is a stateful component and provides as output some quaternions representing different aspects of the gamepad tilt estimation process.

This library doesn't provide an event-loop or an event-based interface in general; it is being considered as a possibility for further development.

## Bevy plugin

See dedicated [README](./dualsense-tools-bevy/).

## Tilt and throttle emulator

See dedicated [README](./dualsense-tilt-and-throttle/).
