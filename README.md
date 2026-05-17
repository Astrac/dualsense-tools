# Dualsense Tools

This crate contains a set of tools that interface with the Sony Dualsense PS5 controller:

- A low-level interface to read and decode HID reports from the controller
- An implementation of accelerometer-corrected-gyro tilt estimation (based on [this document](https://stanford.edu/class/ee267/notes/ee267_notes_imu.pdf))
- A bevy plugin that exposes tilt estimates as a resource, including an example to quickly visualize the estimates
- [WIP] A virtual device application that creates a custom controller tailored for 6-axis simulations (e.g. space games) featuring:
  * Additional axes representing roll and pitch
  * Unmapped L2/R2 buttons replaced with a single "throttle" axis that fuses both
