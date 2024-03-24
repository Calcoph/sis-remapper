#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use std::{ffi::CString, os::raw::{c_char, c_uint, c_void}};

use sys::{CorsairDeviceType_CDT_Cooler, CorsairDeviceType_CDT_FanLedController, CorsairDeviceType_CDT_GraphicsCard, CorsairDeviceType_CDT_Headset, CorsairDeviceType_CDT_HeadsetStand, CorsairDeviceType_CDT_Keyboard, CorsairDeviceType_CDT_LedController, CorsairDeviceType_CDT_MemoryModule, CorsairDeviceType_CDT_Motherboard, CorsairDeviceType_CDT_Mouse, CorsairDeviceType_CDT_Mousemat, CorsairDeviceType_CDT_Touchbar, CorsairDeviceType_CDT_Unknown, CorsairSessionState_CSS_Closed, CorsairSessionState_CSS_Connected, CorsairSessionState_CSS_Connecting, CorsairSessionState_CSS_ConnectionLost, CorsairSessionState_CSS_ConnectionRefused, CorsairSessionState_CSS_Invalid, CorsairSessionState_CSS_Timeout};

pub mod sys;

pub const CORSAIR_STRING_SIZE_S: c_uint = sys::CORSAIR_STRING_SIZE_S;
pub const CORSAIR_STRING_SIZE_M: c_uint = sys::CORSAIR_STRING_SIZE_M;
pub const CORSAIR_LAYER_PRIORITY_MAX: c_uint = sys::CORSAIR_LAYER_PRIORITY_MAX;
pub const CORSAIR_DEVICE_COUNT_MAX: c_uint = sys::CORSAIR_DEVICE_COUNT_MAX;
pub const CORSAIR_DEVICE_LEDCOUNT_MAX: c_uint = sys::CORSAIR_DEVICE_LEDCOUNT_MAX;

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum CorsairError {
    NotConnected = sys::CorsairError_CE_NotConnected,
    NoControl = sys::CorsairError_CE_NoControl,
    IncompatibleProtocol = sys::CorsairError_CE_IncompatibleProtocol,
    InvalidArguments = sys::CorsairError_CE_InvalidArguments,
    InvalidOperation = sys::CorsairError_CE_InvalidOperation,
    DeviceNotFound = sys::CorsairError_CE_DeviceNotFound,
    NotAllowed = sys::CorsairError_CE_NotAllowed,
    UnknownError = u32::MAX
}

pub trait IntoCE {
    fn into_ce(self) -> Result<(), CorsairError>;
}

impl IntoCE for sys::CorsairError {
    fn into_ce(self) -> Result<(), CorsairError> {
        CorsairError::try_from(self)
    }
}

impl CorsairError {
    fn try_from(value: u32) -> Result<(), CorsairError> {
        use CorsairError as CE;
        match value {
            sys::CorsairError_CE_Success => Ok(()),
            sys::CorsairError_CE_NotConnected => Err(CE::NotConnected),
            sys::CorsairError_CE_NoControl => Err(CE::NoControl),
            sys::CorsairError_CE_IncompatibleProtocol => Err(CE::IncompatibleProtocol),
            sys::CorsairError_CE_InvalidArguments => Err(CE::InvalidArguments),
            sys::CorsairError_CE_InvalidOperation => Err(CE::InvalidOperation),
            sys::CorsairError_CE_DeviceNotFound => Err(CE::DeviceNotFound),
            sys::CorsairError_CE_NotAllowed => Err(CE::NotAllowed),
            _ => Err(CE::UnknownError)
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorsairSessionState {
    Closed = CorsairSessionState_CSS_Closed,
    Connected = CorsairSessionState_CSS_Connected,
    Connecting = CorsairSessionState_CSS_Connecting,
    ConnectionLost = CorsairSessionState_CSS_ConnectionLost,
    ConnectionRefused = CorsairSessionState_CSS_ConnectionRefused,
    Invalid = CorsairSessionState_CSS_Invalid,
    Timeout = CorsairSessionState_CSS_Timeout
}

#[derive(Debug)]
pub struct UnknownError;

impl TryFrom<sys::CorsairSessionState> for CorsairSessionState {
    type Error = UnknownError;

    fn try_from(value: sys::CorsairSessionState) -> Result<Self, Self::Error> {
        use CorsairSessionState as CSS;
        match value {
            CorsairSessionState_CSS_Closed => Ok(CSS::Closed),
            CorsairSessionState_CSS_Connected => Ok(CSS::Connected),
            CorsairSessionState_CSS_Connecting => Ok(CSS::Connecting),
            CorsairSessionState_CSS_ConnectionLost => Ok(CSS::ConnectionLost),
            CorsairSessionState_CSS_ConnectionRefused => Ok(CSS::ConnectionRefused),
            CorsairSessionState_CSS_Invalid => Ok(CSS::Invalid),
            CorsairSessionState_CSS_Timeout => Ok(CSS::Timeout),
            _ => Err(UnknownError)
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorsairDeviceType {
    Unknown = CorsairDeviceType_CDT_Unknown,
    Keyboard = CorsairDeviceType_CDT_Keyboard,
    Mouse = CorsairDeviceType_CDT_Mouse,
    Mousemat = CorsairDeviceType_CDT_Mousemat,
    Headset = CorsairDeviceType_CDT_Headset,
    HeadsetStand = CorsairDeviceType_CDT_HeadsetStand,
    FanLedController = CorsairDeviceType_CDT_FanLedController,
    LedController = CorsairDeviceType_CDT_LedController,
    MemoryModule = CorsairDeviceType_CDT_MemoryModule,
    Cooler = CorsairDeviceType_CDT_Cooler,
    Motherboard = CorsairDeviceType_CDT_Motherboard,
    GraphicsCard = CorsairDeviceType_CDT_GraphicsCard,
    Touchbar = CorsairDeviceType_CDT_Touchbar
}

impl From<sys::CorsairDeviceType> for CorsairDeviceType {
    fn from(value: sys::CorsairDeviceType) -> Self {
        match value {
            CorsairDeviceType_CDT_Unknown => Self::Unknown,
            CorsairDeviceType_CDT_Keyboard => Self::Keyboard,
            CorsairDeviceType_CDT_Mouse => Self::Mouse,
            CorsairDeviceType_CDT_Mousemat => Self::Mousemat,
            CorsairDeviceType_CDT_Headset => Self::Headset,
            CorsairDeviceType_CDT_HeadsetStand => Self::HeadsetStand,
            CorsairDeviceType_CDT_FanLedController => Self::FanLedController,
            CorsairDeviceType_CDT_LedController => Self::LedController,
            CorsairDeviceType_CDT_MemoryModule => Self::MemoryModule,
            CorsairDeviceType_CDT_Cooler => Self::Cooler,
            CorsairDeviceType_CDT_Motherboard => Self::Motherboard,
            CorsairDeviceType_CDT_GraphicsCard => Self::GraphicsCard,
            CorsairDeviceType_CDT_Touchbar => Self::Touchbar,
            _ => Self::Unknown
        }
    }
}

#[derive(Debug)]
pub struct CorsairDeviceInfo {
    pub type_: CorsairDeviceType,
    pub id: CString,
    pub serial: String,
    pub model: String,
    pub ledCount: i32,
    pub channelCount: i32,
}

impl From<sys::CorsairDeviceInfo> for CorsairDeviceInfo {
    fn from(mut value: sys::CorsairDeviceInfo) -> Self {
        let model = unsafe {
            std::ffi::CStr::from_ptr((&mut value.model) as *mut i8)
        };
        let model = model.to_str().unwrap().into();
        let serial = unsafe {
            std::ffi::CStr::from_ptr((&mut value.serial) as *mut i8)
        };
        let serial = serial.to_str().unwrap().into();
        let id = unsafe {
            std::ffi::CStr::from_ptr((&mut value.id) as *mut i8)
        };
        let id = id.to_owned();
        CorsairDeviceInfo {
            type_: value.type_.into(),
            id,
            serial,
            model,
            ledCount: value.ledCount,
            channelCount: value.channelCount,
        }
    }
}


#[must_use]
pub unsafe fn CorsairSetLedColors(
    device_id: *mut c_char,
    led_colors: Vec<sys::CorsairLedColor>
) -> Result<(), CorsairError> {
    CorsairError::try_from(sys::CorsairSetLedColors(device_id, led_colors.len() as i32, led_colors.as_ptr()))
}

#[must_use]
pub fn CorsairGetLedPositions(
    device_id: *mut c_char,
) -> Result<Vec<sys::CorsairLedPosition>, CorsairError> {
    let mut leds = Vec::with_capacity(CORSAIR_DEVICE_LEDCOUNT_MAX as usize);
    let mut size: i32 = 0;
    let err = unsafe {
        let err = CorsairError::try_from(sys::CorsairGetLedPositions(device_id, CORSAIR_DEVICE_LEDCOUNT_MAX as i32, leds.as_mut_ptr(), &mut size as *mut i32));
        leds.set_len(size as usize);
        err
    };
    err.map(|_| leds)
}

#[must_use]
pub unsafe fn CorsairGetDevices() -> Result<Vec<CorsairDeviceInfo>, CorsairError> {
    let mut devices = Vec::with_capacity(CORSAIR_DEVICE_COUNT_MAX as usize);
    let mut size: i32 = 0;
    // TODO: Allow caller to pass filter
    let filter = sys::CorsairDeviceFilter {
        deviceTypeMask: !0,
    };
    sys::CorsairGetDevices(&filter, CORSAIR_DEVICE_COUNT_MAX as i32, devices.as_mut_ptr(), &mut size as *mut i32).into_ce()?;
    devices.set_len(size as usize);
    let devices = devices.into_iter().map(|device| device.into()).collect();
    Ok(devices)
}

type CorsairSessionStateChangedHandler = Box<dyn Fn(CorsairSessionState, sys::CorsairSessionDetails) -> ()>;

#[must_use]
pub unsafe fn CorsairConnect(
    onStateChanged: Option<CorsairSessionStateChangedHandler>,
    context: *mut c_void,
) -> Result<(), CorsairError> {
    if let Some(funct) = onStateChanged {
        SSC_HANDLER_FUNCT = Some(funct)
    }
    sys::CorsairConnect(Some(corsair_handler), context).into_ce()
}

static mut SSC_HANDLER_FUNCT: Option<CorsairSessionStateChangedHandler> = None;

unsafe extern fn corsair_handler(
    _context: *mut c_void,
    event_data: *const sys::CorsairSessionStateChanged,
) {
    let data = *event_data;
    let new_state = CorsairSessionState::try_from(data.state).unwrap();
    if let Some(funct) = &mut SSC_HANDLER_FUNCT {
        funct(new_state, data.details)
    }
}
