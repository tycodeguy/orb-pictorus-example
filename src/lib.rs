#![no_std]
#![no_main]

use nalgebra::base::SVector;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};


macro_rules! channel { // was previously $size:literal
    ($name:ident, $datatype:ty, $size:expr, $doc:expr) => {
        #[doc = $doc]
        pub static $name: Channel<CriticalSectionRawMutex, $datatype, $size> = Channel::new();
    };
}

macro_rules! channel_receiver {
    ($name:ident) => {
        $name.receiver()
    };
}

macro_rules! channel_sender {
    ($name:ident) => {
        $name.sender()
    };
}

macro_rules! mutex {
    ($name:ident, $datatype:ty, $initial_value:expr, $doc:expr) => {
        #[doc = $doc]
        pub static $name: embassy_sync::blocking_mutex::Mutex<embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex, $datatype> = 
            embassy_sync::blocking_mutex::Mutex::new($initial_value);
    };
}

// Global shiz -- would be many more in practice.
mutex!(ARMED, bool, false, "Current Armed State.");
channel!(VELOCITY_STATE, SVector<f32, 3>, 10, "V1 frame velocity. M/s");