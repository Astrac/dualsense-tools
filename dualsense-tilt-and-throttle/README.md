# Dualsense tilt-and-throttle emulator

This tool will use a virtual controller emulator (such as vjoy) to create a virtual device tailored for games that benefit from 6-axis controls. In particular the emulated controller:

- Forwards the standard X/Y/RX/RY axes to emulated axes
- Forwards all standard buttons to emulated buttons except for L2 and R2
- Adds dedicated roll and pitch axes determined using the tilt estimator
- Adds a throttle axis that fuses L2 and R2 so that L2 becomes the reverse value and R2 the forward one. L2 and R2 are not mapped to buttons

This project is a work in progress and at the moment it only shows the would-be emulated controls as a ratatui UI.
