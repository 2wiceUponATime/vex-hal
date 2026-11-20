// SPDX-FileCopyrightText: 2026 Gabe Schrock
// SPDX-License-Identifier: MIT

pub use core::time::Duration;

pub use crate::{
    color::Color,
    competition::{Compete, CompeteExt},
    math::{Angle, AngleExt, Direction},
    peripherals::{
        Peripherals, PortError, SmartDevice, SmartDeviceType, SmartPort,
        adi::{
            AdiDevice, AdiDeviceType, AdiPort,
            accelerometer::{AdiAccelerometer, Sensitivity},
            addrled::AdiAddrLed,
            analog::AdiAnalogIn,
            digital::{AdiDigitalIn, AdiDigitalOut, LogicLevel},
            encoder::{AdiEncoder, AdiOpticalEncoder},
            gyroscope::{AdiGyroscope, YawError},
            light_sensor::AdiLightSensor,
            line_tracker::AdiLineTracker,
            motor::AdiMotor,
            potentiometer::{AdiPotentiometer, PotentiometerType},
            pwm::AdiPwmOut,
            range_finder::AdiRangeFinder,
            servo::AdiServo,
        },
        ai_vision::{
            AiVisionColor, AiVisionColorCode, AiVisionObject, AiVisionObjectError, AiVisionSensor,
            AprilTagFamily,
        },
        controller::{
            ButtonState, Controller, ControllerConnection, ControllerError, ControllerId,
            ControllerState, JoystickState,
        },
        display::{
            Alignment, Circle, Display, Fill, Font, FontFamily, FontSize, Line, Rect, RenderMode,
            Stroke, Text, TouchEvent, TouchState,
        },
        distance::{DistanceObject, DistanceObjectError, DistanceReadyError, DistanceSensor},
        electromagnet::Electromagnet,
        expander::AdiExpander,
        gps::GpsSensor,
        imu::{CalibrateError, InertialError, InertialOrientation, InertialSensor},
        link::{LinkError, LinkType, RadioLink},
        motor::{
            BrakeMode, Gearset, Motor, MotorControl, MotorFaults, MotorStatus, MotorType,
            SetGearsetError,
        },
        optical::{Gesture, GestureDirection, OpticalRaw, OpticalRgb, OpticalSensor},
        rotation::RotationSensor,
        serial::{SerialError, SerialPort},
        vision::{
            DetectionSource, LedMode, VisionCode, VisionMode, VisionObject, VisionObjectError,
            VisionSensor, VisionSignature, VisionSignatureError, WhiteBalance,
        },
    },
    print, println,
    runtime::{
        sleep::{sleep, sleep_until},
        task::{block_on, spawn},
    },
    time::{Instant, LowResolutionTime},
};
