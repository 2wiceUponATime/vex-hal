// SPDX-FileCopyrightText: 2023 Gavin Niederman
// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

use core::time::Duration;

use snafu::{Snafu, ensure};

use crate::{peripherals::adi::AdiPort, platform::*, time::LowResolutionTime};

pub mod adi;
pub mod ai_vision;
pub mod battery;
pub mod controller;
pub mod display;
pub mod distance;
pub mod electromagnet;
pub mod expander;
pub mod gps;
pub mod imu;
pub mod link;
pub mod motor;
pub mod optical;
pub mod rotation;
pub mod serial;
pub mod vision;

/// Defines common functionality shared by all Smart Port devices.
pub trait SmartDevice {
    /// The interval at which the V5 and EXP brains read packets from Smart devices.
    const UPDATE_INTERVAL: Duration = Duration::from_millis(10);

    /// Returns the port number of the [`SmartPort`] this device is registered on.
    ///
    /// Ports are numbered starting from 1.
    #[must_use]
    fn port_number(&self) -> u8;

    /// Returns the variant of [`SmartDeviceType`] that this device is associated with.
    #[must_use]
    fn device_type(&self) -> SmartDeviceType;

    /// Determines if this device type is currently plugged into its [`SmartPort`].
    #[must_use]
    fn is_connected(&self) -> bool {
        let mut device_types: [V5_DeviceType; V5_MAX_DEVICE_PORTS] = unsafe { core::mem::zeroed() };
        unsafe {
            vexDeviceGetStatus(device_types.as_mut_ptr());
        }

        SmartDeviceType::from(device_types[(self.port_number() - 1) as usize]) == self.device_type()
    }

    /// Returns a timestamp recorded when the last packet sent by this device was processed by
    /// VEXos.
    ///
    /// # Precision
    ///
    /// This is a timestamp from the brain's low-resolution timer, meaning it has a precision of 1
    /// millisecond. See the [`LowResolutionTime`] API for more information.
    ///
    /// # Errors
    ///
    /// Currently, this function never returns an error. This behavior should be considered
    /// unstable.
    fn timestamp(&self) -> Result<LowResolutionTime, PortError> {
        Ok(LowResolutionTime::from_millis_since_epoch(unsafe {
            vexDeviceGetTimestamp(vexDeviceGetByIndex(u32::from(self.port_number() - 1))) as u32
        }))
    }

    /// Verify that the device type is currently plugged into this port, returning an appropriate
    /// [`PortError`] if not available.
    ///
    /// # Errors
    ///
    /// Returns a [`PortError`] if there is not a physical device of type
    /// [`SmartDevice::device_type`] in this [`SmartDevice`]'s port.
    fn validate_port(&self) -> Result<(), PortError> {
        validate_port(self.port_number(), self.device_type())
    }
}

/// Verify that the device type is currently plugged into this port.
///
/// This function provides the internal implementations of [`SmartDevice::validate_port`],
/// [`SmartPort::validate_type`], and [`AdiPort::validate_expander`].
pub(crate) fn validate_port(number: u8, device_type: SmartDeviceType) -> Result<(), PortError> {
    let mut device_types: [V5_DeviceType; V5_MAX_DEVICE_PORTS] = unsafe { core::mem::zeroed() };
    unsafe {
        vexDeviceGetStatus(device_types.as_mut_ptr());
    }

    let connected_type: Option<SmartDeviceType> = match device_types[(number - 1) as usize] {
        V5_DeviceType::kDeviceTypeNoSensor => None,
        raw_type => Some(raw_type.into()),
    };

    if let Some(connected_type) = connected_type {
        // The connected device must match the requested type.
        ensure!(
            connected_type == device_type,
            IncorrectDeviceSnafu {
                expected: device_type,
                actual: connected_type,
                port: number,
            }
        );
    } else {
        // No device is plugged into the port.
        return DisconnectedSnafu { port: number }.fail();
    }

    Ok(())
}

/// A Smart Port on a Brain.
#[derive(Debug, Eq, PartialEq)]
pub struct SmartPort {
    /// The number of the port (port number).
    ///
    /// Ports are numbered starting from 1.
    number: u8,
}

impl SmartPort {
    /// Creates a new Smart Port on a specified port number.
    ///
    /// # Safety
    ///
    /// Creating new `SmartPort`s is inherently unsafe due to the possibility of constructing more
    /// than one device on the same port index allowing multiple mutable references to the same
    /// hardware device. This violates Rust's borrow checker guarantees. Prefer using
    /// [`Peripherals`] to register devices if possible.
    #[must_use]
    pub const unsafe fn new(number: u8) -> Self {
        Self { number }
    }

    /// Returns the number of the port.
    ///
    /// Ports are numbered starting from 1.
    #[must_use]
    pub const fn number(&self) -> u8 {
        self.number
    }

    pub(crate) const fn index(&self) -> u32 {
        (self.number - 1) as u32
    }

    /// Returns the type of device currently connected to this port, or `None` if no device is
    /// connected.
    #[must_use]
    pub fn device_type(&self) -> Option<SmartDeviceType> {
        let mut device_types: [V5_DeviceType; V5_MAX_DEVICE_PORTS] = unsafe { core::mem::zeroed() };
        unsafe {
            vexDeviceGetStatus(device_types.as_mut_ptr());
        }

        match device_types[self.index() as usize] {
            V5_DeviceType::kDeviceTypeNoSensor => None,
            raw_type => Some(raw_type.into()),
        }
    }

    /// Verify that a device type is currently plugged into this port, returning an appropriate
    /// [`PortError`] if not available.
    ///
    /// # Errors
    ///
    /// Returns a [`PortError`] if there is not a device of the specified type in this port.
    pub fn validate_type(&self, device_type: SmartDeviceType) -> Result<(), PortError> {
        if let Some(connected_type) = self.device_type() {
            // The connected device must match the requested type.
            ensure!(
                connected_type == device_type,
                IncorrectDeviceSnafu {
                    expected: device_type,
                    actual: connected_type,
                    port: self.number,
                }
            );
        } else {
            // No device is plugged into the port.
            return DisconnectedSnafu { port: self.number }.fail();
        }

        Ok(())
    }

    /// Returns the timestamp of the last device packet processed by this port, or `None` if no
    /// device is connected.
    ///
    /// # Precision
    ///
    /// This is a timestamp from the brain's low-resolution timer, meaning it has a precision of 1
    /// millisecond. See the [`LowResolutionTime`] API for more information.
    #[must_use]
    pub fn timestamp(&self) -> Option<LowResolutionTime> {
        if self.device_type().is_some() {
            Some(LowResolutionTime::from_millis_since_epoch(unsafe {
                vexDeviceGetTimestamp(vexDeviceGetByIndex(self.index())) as u32
            }))
        } else {
            None
        }
    }

    /// Returns the raw handle of the underlying Smart device connected to this port.
    pub(crate) unsafe fn device_handle(&self) -> V5_DeviceT {
        unsafe { vexDeviceGetByIndex(self.index()) }
    }
}

/// A possible type of device that can be plugged into a [`SmartPort`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[non_exhaustive]
pub enum SmartDeviceType {
    /// Smart Motor
    ///
    /// This corresponds to the [`Motor`](motor::Motor) device.
    Motor,

    /// Rotation Sensor
    ///
    /// This corresponds to the [`RotationSensor`](rotation::RotationSensor) device.
    Rotation,

    /// Inertial Sensor
    ///
    /// This corresponds to the [`InertialSensor`](imu::InertialSensor) device.
    Imu,

    /// Distance Sensor
    ///
    /// This corresponds to the [`DistanceSensor`](distance::DistanceSensor) device.
    Distance,

    /// Vision Sensor
    ///
    /// This corresponds to the [`VisionSensor`](vision::VisionSensor) device.
    Vision,

    /// AI Vision Sensor
    ///
    /// This corresponds to the [`AiVisionSensor`](ai_vision::AiVisionSensor) device.
    AiVision,

    /// Workcell Electromagnet
    ///
    /// This corresponds to the [`Electromagnet`](electromagnet::Electromagnet) device.
    Electromagnet,

    /// CTE Workcell Light Tower
    LightTower,

    /// CTE Workcell Arm
    Arm,

    /// Optical Sensor
    ///
    /// This corresponds to the [`OpticalSensor`](optical::OpticalSensor) device.
    Optical,

    /// GPS Sensor
    ///
    /// This corresponds to the [`GpsSensor`](gps::GpsSensor) device.
    Gps,

    /// Smart Radio
    Radio,

    /// ADI Expander
    ///
    /// This corresponds to the [`AdiExpander`](expander::AdiExpander) device.
    Adi,

    /// Generic Serial Port
    ///
    /// This corresponds to the [`SerialPort`](serial::SerialPort) device.
    GenericSerial,

    /// Other device type code returned by the SDK that is currently unsupported, undocumented, or
    /// unknown.
    Unknown(V5_DeviceType),
}

impl From<V5_DeviceType> for SmartDeviceType {
    fn from(value: V5_DeviceType) -> Self {
        match value {
            V5_DeviceType::kDeviceTypeMotorSensor => Self::Motor,
            V5_DeviceType::kDeviceTypeAbsEncSensor => Self::Rotation,
            V5_DeviceType::kDeviceTypeImuSensor => Self::Imu,
            V5_DeviceType::kDeviceTypeDistanceSensor => Self::Distance,
            V5_DeviceType::kDeviceTypeRadioSensor => Self::Radio,
            V5_DeviceType::kDeviceTypeVisionSensor => Self::Vision,
            V5_DeviceType::kDeviceTypeAdiSensor => Self::Adi,
            V5_DeviceType::kDeviceTypeOpticalSensor => Self::Optical,
            V5_DeviceType::kDeviceTypeMagnetSensor => Self::Electromagnet,
            V5_DeviceType::kDeviceTypeGpsSensor => Self::Gps,
            V5_DeviceType::kDeviceTypeLightTowerSensor => Self::LightTower,
            V5_DeviceType::kDeviceTypeArmDevice => Self::Arm,
            V5_DeviceType::kDeviceTypeAiVisionSensor => Self::AiVision,
            V5_DeviceType::kDeviceTypeGenericSerial => Self::GenericSerial,
            other => Self::Unknown(other),
        }
    }
}

impl From<SmartDeviceType> for V5_DeviceType {
    fn from(value: SmartDeviceType) -> Self {
        match value {
            SmartDeviceType::Motor => V5_DeviceType::kDeviceTypeMotorSensor,
            SmartDeviceType::Rotation => V5_DeviceType::kDeviceTypeAbsEncSensor,
            SmartDeviceType::Imu => V5_DeviceType::kDeviceTypeImuSensor,
            SmartDeviceType::Distance => V5_DeviceType::kDeviceTypeDistanceSensor,
            SmartDeviceType::Vision => V5_DeviceType::kDeviceTypeVisionSensor,
            SmartDeviceType::AiVision => V5_DeviceType::kDeviceTypeAiVisionSensor,
            SmartDeviceType::Electromagnet => V5_DeviceType::kDeviceTypeMagnetSensor,
            SmartDeviceType::LightTower => V5_DeviceType::kDeviceTypeLightTowerSensor,
            SmartDeviceType::Arm => V5_DeviceType::kDeviceTypeArmDevice,
            SmartDeviceType::Optical => V5_DeviceType::kDeviceTypeOpticalSensor,
            SmartDeviceType::Gps => V5_DeviceType::kDeviceTypeGpsSensor,
            SmartDeviceType::Radio => V5_DeviceType::kDeviceTypeRadioSensor,
            SmartDeviceType::Adi => V5_DeviceType::kDeviceTypeAdiSensor,
            SmartDeviceType::GenericSerial => V5_DeviceType::kDeviceTypeGenericSerial,
            SmartDeviceType::Unknown(raw_type) => raw_type,
        }
    }
}

/// Errors that can occur when performing operations on [`SmartPort`]-connected devices.
///
/// Most smart devices will return this type or something wrapping this type when an error occurs.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Snafu)]
pub enum PortError {
    /// No device was plugged into the port, when one was expected.
    #[snafu(display("Expected a device to be connected to port {port}"))]
    Disconnected {
        /// The port that was expected to have a device
        port: u8,
    },

    /// The wrong type of device is plugged into the port.
    #[snafu(display(
        "Expected a {expected:?} device on port {port}, but found a {actual:?} device"
    ))]
    IncorrectDevice {
        /// The device type that was expected
        expected: SmartDeviceType,
        /// The device type that was found
        actual: SmartDeviceType,
        /// The port that was expected to have a device
        port: u8,
    },
}

pub struct Peripherals {
    /// Smart Port 1 on the Brain.
    pub port_1: SmartPort,
    /// Smart Port 2 on the Brain.
    pub port_2: SmartPort,
    /// Smart Port 3 on the Brain.
    pub port_3: SmartPort,
    /// Smart Port 4 on the Brain.
    pub port_4: SmartPort,
    /// Smart Port 5 on the Brain.
    pub port_5: SmartPort,
    /// Smart Port 6 on the Brain.
    pub port_6: SmartPort,
    /// Smart Port 7 on the Brain.
    pub port_7: SmartPort,
    /// Smart Port 8 on the Brain.
    pub port_8: SmartPort,
    /// Smart Port 9 on the Brain.
    pub port_9: SmartPort,
    /// Smart Port 10 on the Brain.
    pub port_10: SmartPort,
    /// Smart Port 11 on the Brain.
    #[cfg(feature = "platform-v5")]
    pub port_11: SmartPort,
    /// Smart Port 12 on the Brain.
    #[cfg(feature = "platform-v5")]
    pub port_12: SmartPort,
    /// Smart Port 13 on the Brain.
    #[cfg(feature = "platform-v5")]
    pub port_13: SmartPort,
    /// Smart Port 14 on the Brain.
    #[cfg(feature = "platform-v5")]
    pub port_14: SmartPort,
    /// Smart Port 15 on the Brain.
    #[cfg(feature = "platform-v5")]
    pub port_15: SmartPort,
    /// Smart Port 16 on the Brain.
    #[cfg(feature = "platform-v5")]
    pub port_16: SmartPort,
    /// Smart Port 17 on the Brain.
    #[cfg(feature = "platform-v5")]
    pub port_17: SmartPort,
    /// Smart Port 18 on the Brain.
    #[cfg(feature = "platform-v5")]
    pub port_18: SmartPort,
    /// Smart Port 19 on the Brain.
    #[cfg(feature = "platform-v5")]
    pub port_19: SmartPort,
    /// Smart Port 20 on the Brain.
    #[cfg(feature = "platform-v5")]
    pub port_20: SmartPort,
    /// Smart Port 21 on the Brain.
    #[cfg(feature = "platform-v5")]
    pub port_21: SmartPort,

    /// ADI port A on the Brain.
    pub adi_a: AdiPort,
    /// ADI port B on the Brain.
    pub adi_b: AdiPort,
    /// ADI port C on the Brain.
    pub adi_c: AdiPort,
    /// ADI port D on the Brain.
    pub adi_d: AdiPort,
    /// ADI port E on the Brain.
    pub adi_e: AdiPort,
    /// ADI port F on the Brain.
    pub adi_f: AdiPort,
    /// ADI port G on the Brain.
    pub adi_g: AdiPort,
    /// ADI port H on the Brain.
    pub adi_h: AdiPort,
}

impl Peripherals {
    /// # Safety
    /// Must be called at most once for the lifetime of the program. Each call
    /// constructs a `SmartPort` for every port number, and `SmartPort::new`
    /// requires each number be used only once (a duplicate would allow aliased
    /// mutable access to the same device). Calling this twice violates that.
    pub unsafe fn new() -> Self {
        unsafe {
            Self {
                port_1: SmartPort::new(1),
                port_2: SmartPort::new(2),
                port_3: SmartPort::new(3),
                port_4: SmartPort::new(4),
                port_5: SmartPort::new(5),
                port_6: SmartPort::new(6),
                port_7: SmartPort::new(7),
                port_8: SmartPort::new(8),
                port_9: SmartPort::new(9),
                port_10: SmartPort::new(10),
                #[cfg(feature = "platform-v5")]
                port_11: SmartPort::new(11),
                #[cfg(feature = "platform-v5")]
                port_12: SmartPort::new(12),
                #[cfg(feature = "platform-v5")]
                port_13: SmartPort::new(13),
                #[cfg(feature = "platform-v5")]
                port_14: SmartPort::new(14),
                #[cfg(feature = "platform-v5")]
                port_15: SmartPort::new(15),
                #[cfg(feature = "platform-v5")]
                port_16: SmartPort::new(16),
                #[cfg(feature = "platform-v5")]
                port_17: SmartPort::new(17),
                #[cfg(feature = "platform-v5")]
                port_18: SmartPort::new(18),
                #[cfg(feature = "platform-v5")]
                port_19: SmartPort::new(19),
                #[cfg(feature = "platform-v5")]
                port_20: SmartPort::new(20),
                #[cfg(feature = "platform-v5")]
                port_21: SmartPort::new(21),

                adi_a: AdiPort::new(1, None),
                adi_b: AdiPort::new(2, None),
                adi_c: AdiPort::new(3, None),
                adi_d: AdiPort::new(4, None),
                adi_e: AdiPort::new(5, None),
                adi_f: AdiPort::new(6, None),
                adi_g: AdiPort::new(7, None),
                adi_h: AdiPort::new(8, None),
            }
        }
    }
}
