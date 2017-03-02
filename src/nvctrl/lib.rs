extern crate libc;

#[cfg(target_os="windows")]
#[macro_use] extern crate lazy_static;

#[cfg(target_os="windows")]
extern crate libloading;

use std::collections::HashMap;

#[cfg(any(target_os="linux", target_os="freebsd"))]
pub use self::os::unix::*;

#[cfg(target_os="windows")]
pub use self::os::windows::*;

#[macro_use] extern crate serde_derive;

pub mod os;

/// All Fan Controller implementations should implement the
/// NvFanController trait whcih provides basic functions to monitor
/// and manipulate the GPU fan.
pub trait NvFanController {
    /// Returns the temperature of the GPU in degrees Celsius
    fn get_temp(&self) -> Result<i32, String>;

    /// Returns the control status of the cooler
    fn get_ctrl_status(&self) -> Result<NVCtrlFanControlState, String>;

    /// Sets the control status of the cooler
    ///
    /// **Arguments**
    ///
    /// * `state` - Set the mode of fan control to either `Auto` or `Manual`
    fn set_ctrl_type(&self, state: NVCtrlFanControlState) -> Result<(), String>;

    /// Returns the speed of the fan in %
    fn get_fanspeed(&self) -> Result<i32, String>;

    /// Returns the speed of the fan in RPM
    fn get_fanspeed_rpm(&self) -> Result<i32, String>;

    /// Sets the fan speed (in %)
    ///
    /// **Arguments**
    ///
    /// * `speed` - The target speed (%)
    fn set_fanspeed(&self, speed: i32) -> Result<(), String>;

    /// Returns version of the NVidia driver in use
    fn get_version(&self) -> Result<String, String>;

    /// Returns the name of the graphics adapter in use
    fn get_adapter(&self) -> Result<String, String>;

    /// Returns a `HashMap` containing all values of the utilization.
    /// On both Unix and Windows the following keys are available
    ///
    /// * `graphics` - GPU core utilization (in %)
    /// * `memory` - Memory bus utilization (in %)
    /// * `video` - Video decoder bus utilization (in %)
    ///
    /// On Unix there is an additional key available
    ///
    /// * `PCIe` - PCI express bus utilization (in %)
    fn get_utilization(&self) -> Result<HashMap<&str, i32>, String>;
}

/// `NVCtrlFanControlState` represents the control state of a
/// GPU fan. This can be either auto or manual.
#[derive(Serialize, Deserialize, Debug)]
pub enum NVCtrlFanControlState {
    Auto = 0,
    Manual
}

/// Common implementation of `NvidiaControl` which is the only `NvFanController`
/// implementation so far. The system dependent bits are implemented in the
/// platform specific subcrates
impl NvidiaControl {

    /// Creates a new `NvidiaControl` using the provided low and high limits
    /// for the fan. If the requested fan speed is lower (or higher) than the
    /// specified limits the fan speed is clipped to lowest (or highest) limit
    /// provided. If this is `None` no limits are applied (in effect (0,100)).
    ///
    /// **Arguments**
    ///
    /// * `lim`: An optional lower and upper limit set
    pub fn new(lim: Option<(u16, u16)>) -> Result<NvidiaControl, String> {
        let limits = match lim {
            Some((low, high)) => {
                if high > 100 {
                    (low, 100)
                } else {
                    (low, high)
                }
            },
            None => (0, 100)
        };

        // This is implemented in the platform specific subcrate
        NvidiaControl::init(limits)
    }

    /// Returns the clipped fan speed for the requested fan speed
    ///
    /// **Arguments**
    ///
    /// * `speed` - The target fan speed
    fn true_speed(&self, speed: i32) -> u16 {
        let true_speed: u16;
        let (low, high) = self.limits;
        if speed < low as i32 {
            true_speed = low;
        } else if speed > high as i32 {
            true_speed = high;
        } else {
            true_speed = speed as u16;
        }

        true_speed
    }

}
