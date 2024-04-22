#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use std::{ffi::CString, os::raw::{c_char, c_uint, c_void}};

pub mod sys;

pub const CORSAIR_STRING_SIZE_S: c_uint = sys::CORSAIR_STRING_SIZE_S;
pub const CORSAIR_STRING_SIZE_M: c_uint = sys::CORSAIR_STRING_SIZE_M;
pub const CORSAIR_LAYER_PRIORITY_MAX: c_uint = sys::CORSAIR_LAYER_PRIORITY_MAX;
pub const CORSAIR_DEVICE_COUNT_MAX: c_uint = sys::CORSAIR_DEVICE_COUNT_MAX;
pub const CORSAIR_DEVICE_LEDCOUNT_MAX: c_uint = sys::CORSAIR_DEVICE_LEDCOUNT_MAX;

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum SysCorsairError {
    NotConnected = sys::CorsairError_CE_NotConnected,
    NoControl = sys::CorsairError_CE_NoControl,
    IncompatibleProtocol = sys::CorsairError_CE_IncompatibleProtocol,
    InvalidArguments = sys::CorsairError_CE_InvalidArguments,
    InvalidOperation = sys::CorsairError_CE_InvalidOperation,
    DeviceNotFound = sys::CorsairError_CE_DeviceNotFound,
    NotAllowed = sys::CorsairError_CE_NotAllowed,
    UnknownError = u32::MAX
}

#[derive(Debug, Clone, Copy)]
pub enum CorsairError {
    Sys(SysCorsairError),

}

pub trait IntoCE {
    fn into_ce(self) -> Result<(), CorsairError>;
}

impl IntoCE for sys::CorsairError {
    fn into_ce(self) -> Result<(), CorsairError> {
        CorsairError::try_from(self)
    }
}

impl SysCorsairError {
    fn try_from(value: u32) -> Result<(), SysCorsairError> {
        use SysCorsairError as CE;
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

impl CorsairError {
    fn try_from(value: u32) -> Result<(), CorsairError> {
        SysCorsairError::try_from(value).map_err(|sys_err| CorsairError::Sys(sys_err))
    }
}


#[repr(u32)]
pub enum CorsairEventId {
    Invalid = sys::CorsairEventId_CEI_Invalid,
    DeviceConnectionStatusChangedEvent = sys::CorsairEventId_CEI_DeviceConnectionStatusChangedEvent,
    KeyEvent = sys::CorsairEventId_CEI_KeyEvent,
}

#[repr(u32)]
pub enum CorsairMacroKeyId {
    Invalid = sys::CorsairMacroKeyId_CMKI_Invalid,
    CMKI1 = sys::CorsairMacroKeyId_CMKI_1,
    CMKI2 = sys::CorsairMacroKeyId_CMKI_2,
    CMKI3 = sys::CorsairMacroKeyId_CMKI_3,
    CMKI4 = sys::CorsairMacroKeyId_CMKI_4,
    CMKI5 = sys::CorsairMacroKeyId_CMKI_5,
    CMKI6 = sys::CorsairMacroKeyId_CMKI_6,
    CMKI7 = sys::CorsairMacroKeyId_CMKI_7,
    CMKI8 = sys::CorsairMacroKeyId_CMKI_8,
    CMKI9 = sys::CorsairMacroKeyId_CMKI_9,
    CMKI10 = sys::CorsairMacroKeyId_CMKI_10,
    CMKI11 = sys::CorsairMacroKeyId_CMKI_11,
    CMKI12 = sys::CorsairMacroKeyId_CMKI_12,
    CMKI13 = sys::CorsairMacroKeyId_CMKI_13,
    CMKI14 = sys::CorsairMacroKeyId_CMKI_14,
    CMKI15 = sys::CorsairMacroKeyId_CMKI_15,
    CMKI16 = sys::CorsairMacroKeyId_CMKI_16,
    CMKI17 = sys::CorsairMacroKeyId_CMKI_17,
    CMKI18 = sys::CorsairMacroKeyId_CMKI_18,
    CMKI19 = sys::CorsairMacroKeyId_CMKI_19,
    CMKI20 = sys::CorsairMacroKeyId_CMKI_20,
}

impl TryFrom<u32> for CorsairMacroKeyId {
    type Error = CorsairError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            sys::CorsairMacroKeyId_CMKI_Invalid => CorsairMacroKeyId::Invalid,
            sys::CorsairMacroKeyId_CMKI_1 => CorsairMacroKeyId::CMKI1,
            sys::CorsairMacroKeyId_CMKI_2 => CorsairMacroKeyId::CMKI2,
            sys::CorsairMacroKeyId_CMKI_3 => CorsairMacroKeyId::CMKI3,
            sys::CorsairMacroKeyId_CMKI_4 => CorsairMacroKeyId::CMKI4,
            sys::CorsairMacroKeyId_CMKI_5 => CorsairMacroKeyId::CMKI5,
            sys::CorsairMacroKeyId_CMKI_6 => CorsairMacroKeyId::CMKI6,
            sys::CorsairMacroKeyId_CMKI_7 => CorsairMacroKeyId::CMKI7,
            sys::CorsairMacroKeyId_CMKI_8 => CorsairMacroKeyId::CMKI8,
            sys::CorsairMacroKeyId_CMKI_9 => CorsairMacroKeyId::CMKI9,
            sys::CorsairMacroKeyId_CMKI_10 => CorsairMacroKeyId::CMKI10,
            sys::CorsairMacroKeyId_CMKI_11 => CorsairMacroKeyId::CMKI11,
            sys::CorsairMacroKeyId_CMKI_12 => CorsairMacroKeyId::CMKI12,
            sys::CorsairMacroKeyId_CMKI_13 => CorsairMacroKeyId::CMKI13,
            sys::CorsairMacroKeyId_CMKI_14 => CorsairMacroKeyId::CMKI14,
            sys::CorsairMacroKeyId_CMKI_15 => CorsairMacroKeyId::CMKI15,
            sys::CorsairMacroKeyId_CMKI_16 => CorsairMacroKeyId::CMKI16,
            sys::CorsairMacroKeyId_CMKI_17 => CorsairMacroKeyId::CMKI17,
            sys::CorsairMacroKeyId_CMKI_18 => CorsairMacroKeyId::CMKI18,
            sys::CorsairMacroKeyId_CMKI_19 => CorsairMacroKeyId::CMKI19,
            sys::CorsairMacroKeyId_CMKI_20 => CorsairMacroKeyId::CMKI20,
            _ => todo!()
        })
    }
}

#[repr(u32)]
pub enum CorsairDataType {
    Boolean = sys::CorsairDataType_CT_Boolean,
    Int32 = sys::CorsairDataType_CT_Int32,
    Float64 = sys::CorsairDataType_CT_Float64,
    String = sys::CorsairDataType_CT_String,
    BooleanArray = sys::CorsairDataType_CT_Boolean_Array,
    Int32Array = sys::CorsairDataType_CT_Int32_Array,
    Float64Array = sys::CorsairDataType_CT_Float64_Array,
    StringArray = sys::CorsairDataType_CT_String_Array,
}

#[repr(u32)]
pub enum CorsairPropertyFlag {
    None = sys::CorsairPropertyFlag_CPF_None,
    CanRead = sys::CorsairPropertyFlag_CPF_CanRead,
    CanWrite = sys::CorsairPropertyFlag_CPF_CanWrite,
    Indexed = sys::CorsairPropertyFlag_CPF_Indexed
}

#[repr(u32)]
pub enum CorsairPhysicalLayout {
    Invalid = sys::CorsairPhysicalLayout_CPL_Invalid,
    US = sys::CorsairPhysicalLayout_CPL_US,
    UK = sys::CorsairPhysicalLayout_CPL_UK,
    JP = sys::CorsairPhysicalLayout_CPL_JP,
    KR = sys::CorsairPhysicalLayout_CPL_KR,
    BR = sys::CorsairPhysicalLayout_CPL_BR
}

#[repr(u32)]
pub enum CorsairLogicalLayout {
    Invalid = sys::CorsairLogicalLayout_CLL_Invalid,
    USInt = sys::CorsairLogicalLayout_CLL_US_Int,
    NA = sys::CorsairLogicalLayout_CLL_NA,
    EU = sys::CorsairLogicalLayout_CLL_EU,
    UK = sys::CorsairLogicalLayout_CLL_UK,
    BE = sys::CorsairLogicalLayout_CLL_BE,
    BR = sys::CorsairLogicalLayout_CLL_BR,
    CH = sys::CorsairLogicalLayout_CLL_CH,
    CN = sys::CorsairLogicalLayout_CLL_CN,
    DE = sys::CorsairLogicalLayout_CLL_DE,
    ES = sys::CorsairLogicalLayout_CLL_ES,
    FR = sys::CorsairLogicalLayout_CLL_FR,
    IT = sys::CorsairLogicalLayout_CLL_IT,
    ND = sys::CorsairLogicalLayout_CLL_ND,
    RU = sys::CorsairLogicalLayout_CLL_RU,
    JP = sys::CorsairLogicalLayout_CLL_JP,
    KR = sys::CorsairLogicalLayout_CLL_KR,
    TW = sys::CorsairLogicalLayout_CLL_TW,
    MEX = sys::CorsairLogicalLayout_CLL_MEX
}

#[repr(u32)]
pub enum CorsairChannelDeviceType {
    Invalid = sys::CorsairChannelDeviceType_CCDT_Invalid,
    HdFan = sys::CorsairChannelDeviceType_CCDT_HD_Fan,
    SpFan = sys::CorsairChannelDeviceType_CCDT_SP_Fan,
    LlFan = sys::CorsairChannelDeviceType_CCDT_LL_Fan,
    MlFan = sys::CorsairChannelDeviceType_CCDT_ML_Fan,
    QlFan = sys::CorsairChannelDeviceType_CCDT_QL_Fan,
    Led8SeriesFan = sys::CorsairChannelDeviceType_CCDT_8LedSeriesFan,
    Strip = sys::CorsairChannelDeviceType_CCDT_Strip,
    Dap = sys::CorsairChannelDeviceType_CCDT_DAP,
    Pump = sys::CorsairChannelDeviceType_CCDT_Pump,
    Dram = sys::CorsairChannelDeviceType_CCDT_DRAM,
    WaterBlock = sys::CorsairChannelDeviceType_CCDT_WaterBlock,
    QxFan = sys::CorsairChannelDeviceType_CCDT_QX_Fan
}

pub struct CorsairVersion {
    pub major: i32,
    pub minor: i32,
    pub patch: i32
}

impl From<sys::CorsairVersion> for CorsairVersion {
    fn from(value: sys::CorsairVersion) -> Self {
        CorsairVersion {
            major: value.major,
            minor: value.minor,
            patch: value.patch,
        }
    }
}

impl From<CorsairVersion> for sys::CorsairVersion {
    fn from(value: CorsairVersion) -> Self {
        sys::CorsairVersion {
            major: value.major,
            minor: value.minor,
            patch: value.patch,
        }
    }
}

#[repr(u32)]
pub enum CorsairAccessLevel {
    Shared = sys::CorsairAccessLevel_CAL_Shared,
    ExclusiveLightingControl = sys::CorsairAccessLevel_CAL_ExclusiveLightingControl,
    ExclusiveKeyEventsListening = sys::CorsairAccessLevel_CAL_ExclusiveKeyEventsListening,
    ExclusiveLightingControlAndKeyEventsListening = sys::CorsairAccessLevel_CAL_ExclusiveLightingControlAndKeyEventsListening
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorsairSessionState {
    Closed = sys::CorsairSessionState_CSS_Closed,
    Connected = sys::CorsairSessionState_CSS_Connected,
    Connecting = sys::CorsairSessionState_CSS_Connecting,
    ConnectionLost = sys::CorsairSessionState_CSS_ConnectionLost,
    ConnectionRefused = sys::CorsairSessionState_CSS_ConnectionRefused,
    Invalid = sys::CorsairSessionState_CSS_Invalid,
    Timeout = sys::CorsairSessionState_CSS_Timeout
}

#[repr(u32)]
pub enum CorsairDevicePropertyId {
    Invalid = sys::CorsairDevicePropertyId_CDPI_Invalid,
    PropertyArray = sys::CorsairDevicePropertyId_CDPI_PropertyArray,
    MicEnabled = sys::CorsairDevicePropertyId_CDPI_MicEnabled,
    SurroundSoundEnabled = sys::CorsairDevicePropertyId_CDPI_SurroundSoundEnabled,
    SidetoneEnabled = sys::CorsairDevicePropertyId_CDPI_SidetoneEnabled,
    EqualizerPreset = sys::CorsairDevicePropertyId_CDPI_EqualizerPreset,
    PhysicalLayout = sys::CorsairDevicePropertyId_CDPI_PhysicalLayout,
    LogicalLayout = sys::CorsairDevicePropertyId_CDPI_LogicalLayout,
    MacroKeyArray = sys::CorsairDevicePropertyId_CDPI_MacroKeyArray,
    BatteryLevel = sys::CorsairDevicePropertyId_CDPI_BatteryLevel,
    ChannelLedCount = sys::CorsairDevicePropertyId_CDPI_ChannelLedCount,
    ChannelDeviceCount = sys::CorsairDevicePropertyId_CDPI_ChannelDeviceCount,
    ChannelDeviceLedCountArray = sys::CorsairDevicePropertyId_CDPI_ChannelDeviceLedCountArray,
    ChannelDeviceTypeArray = sys::CorsairDevicePropertyId_CDPI_ChannelDeviceTypeArray,
}

#[derive(Debug)]
pub struct UnknownError;

impl TryFrom<sys::CorsairSessionState> for CorsairSessionState {
    type Error = UnknownError;

    fn try_from(value: sys::CorsairSessionState) -> Result<Self, Self::Error> {
        use CorsairSessionState as CSS;
        match value {
            sys::CorsairSessionState_CSS_Closed => Ok(CSS::Closed),
            sys::CorsairSessionState_CSS_Connected => Ok(CSS::Connected),
            sys::CorsairSessionState_CSS_Connecting => Ok(CSS::Connecting),
            sys::CorsairSessionState_CSS_ConnectionLost => Ok(CSS::ConnectionLost),
            sys::CorsairSessionState_CSS_ConnectionRefused => Ok(CSS::ConnectionRefused),
            sys::CorsairSessionState_CSS_Invalid => Ok(CSS::Invalid),
            sys::CorsairSessionState_CSS_Timeout => Ok(CSS::Timeout),
            _ => Err(UnknownError)
        }
    }
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorsairDeviceType {
    Unknown = sys::CorsairDeviceType_CDT_Unknown,
    Keyboard = sys::CorsairDeviceType_CDT_Keyboard,
    Mouse = sys::CorsairDeviceType_CDT_Mouse,
    Mousemat = sys::CorsairDeviceType_CDT_Mousemat,
    Headset = sys::CorsairDeviceType_CDT_Headset,
    HeadsetStand = sys::CorsairDeviceType_CDT_HeadsetStand,
    FanLedController = sys::CorsairDeviceType_CDT_FanLedController,
    LedController = sys::CorsairDeviceType_CDT_LedController,
    MemoryModule = sys::CorsairDeviceType_CDT_MemoryModule,
    Cooler = sys::CorsairDeviceType_CDT_Cooler,
    Motherboard = sys::CorsairDeviceType_CDT_Motherboard,
    GraphicsCard = sys::CorsairDeviceType_CDT_GraphicsCard,
    Touchbar = sys::CorsairDeviceType_CDT_Touchbar,
    GameController = sys::CorsairDeviceType_CDT_GameController,
    All = sys::CorsairDeviceType_CDT_All
}

impl From<sys::CorsairDeviceType> for CorsairDeviceType {
    fn from(value: sys::CorsairDeviceType) -> Self {
        match value {
            sys::CorsairDeviceType_CDT_Unknown => Self::Unknown,
            sys::CorsairDeviceType_CDT_Keyboard => Self::Keyboard,
            sys::CorsairDeviceType_CDT_Mouse => Self::Mouse,
            sys::CorsairDeviceType_CDT_Mousemat => Self::Mousemat,
            sys::CorsairDeviceType_CDT_Headset => Self::Headset,
            sys::CorsairDeviceType_CDT_HeadsetStand => Self::HeadsetStand,
            sys::CorsairDeviceType_CDT_FanLedController => Self::FanLedController,
            sys::CorsairDeviceType_CDT_LedController => Self::LedController,
            sys::CorsairDeviceType_CDT_MemoryModule => Self::MemoryModule,
            sys::CorsairDeviceType_CDT_Cooler => Self::Cooler,
            sys::CorsairDeviceType_CDT_Motherboard => Self::Motherboard,
            sys::CorsairDeviceType_CDT_GraphicsCard => Self::GraphicsCard,
            sys::CorsairDeviceType_CDT_Touchbar => Self::Touchbar,
            _ => Self::Unknown
        }
    }
}

#[derive(Debug)]
pub struct CorsairDeviceInfo {
    pub type_: CorsairDeviceType,
    pub id: CString,
    pub serial: CString,
    pub model: CString,
    pub ledCount: i32,
    pub channelCount: i32,
}

impl From<sys::CorsairDeviceInfo> for CorsairDeviceInfo {
    fn from(mut value: sys::CorsairDeviceInfo) -> Self {
        let model = unsafe {
            std::ffi::CStr::from_ptr((&mut value.model) as *mut i8)
        };
        let model = model.to_owned();
        let serial = unsafe {
            std::ffi::CStr::from_ptr((&mut value.serial) as *mut i8)
        };
        let serial = serial.to_owned();
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

pub struct CorsairLedColor {
    pub id: sys::CorsairLedLuid,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<CorsairLedColor> for sys::CorsairLedColor {
    fn from(value: CorsairLedColor) -> Self {
        sys::CorsairLedColor {
            id: value.id,
            r: value.r,
            g: value.g,
            b: value.b,
            a: value.a,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CorsairLedPosition {
    pub id: sys::CorsairLedLuid,
    pub cx: f64,
    pub cy: f64
}

impl From<sys::CorsairLedPosition> for CorsairLedPosition {
    fn from(value: sys::CorsairLedPosition) -> Self {
        CorsairLedPosition {
            id: value.id,
            cx: value.cx,
            cy: value.cy
        }
    }
}

#[must_use]
pub fn CorsairDisconnect() -> Result<(), CorsairError> {
    unsafe{CorsairError::try_from(sys::CorsairDisconnect())}
}

#[must_use]
pub fn CorsairGetDeviceInfo(
    mut deviceId: i8,
) -> Result<CorsairDeviceInfo, CorsairError> {
    let mut device_info = sys::CorsairDeviceInfo {
        type_: 0,
        id: [0;128],
        serial: [0;128],
        model: [0;128],
        ledCount: 0,
        channelCount: 0,
    };

    let res = unsafe{
        CorsairError::try_from(sys::CorsairGetDeviceInfo(
            (&mut deviceId) as *mut i8,
            (&mut device_info) as *mut sys::CorsairDeviceInfo
        ))
    };

    res?;

    Ok(device_info.into())
}

#[must_use]
pub unsafe fn CorsairSetLedColors(
    device_id: *mut c_char,
    led_colors: Vec<CorsairLedColor>
) -> Result<(), CorsairError> {
    let led_colors = led_colors.into_iter().map(|lc| lc.into()).collect::<Vec<_>>();
    CorsairError::try_from(sys::CorsairSetLedColors(device_id, led_colors.len() as i32, led_colors.as_ptr()))
}

#[must_use]
pub fn CorsairGetLedPositions(
    device_id: *mut c_char,
) -> Result<Vec<CorsairLedPosition>, CorsairError> {
    let mut leds = Vec::with_capacity(CORSAIR_DEVICE_LEDCOUNT_MAX as usize);
    let mut size: i32 = 0;
    let err = unsafe {
        let err = CorsairError::try_from(sys::CorsairGetLedPositions(device_id, CORSAIR_DEVICE_LEDCOUNT_MAX as i32, leds.as_mut_ptr(), &mut size as *mut i32));
        leds.set_len(size as usize);
        err
    };
    err.map(|_| leds.into_iter().map(|l| l.into()).collect::<Vec<_>>())
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

type CorsairSessionStateChangedHandler = Box<dyn Fn(CorsairSessionState, CorsairSessionDetails) -> ()>;

pub struct CorsairSessionDetails {
    pub client_version: CorsairVersion,
    pub server_version: CorsairVersion,
    pub server_host_version: CorsairVersion,
}

impl From<sys::CorsairSessionDetails> for CorsairSessionDetails {
    fn from(value: sys::CorsairSessionDetails) -> Self {
        CorsairSessionDetails {
            client_version: value.clientVersion.into(),
            server_version: value.serverVersion.into(),
            server_host_version: value.serverHostVersion.into(),
        }
    }
}

impl From<CorsairSessionDetails> for sys::CorsairSessionDetails {
    fn from(value: CorsairSessionDetails) -> Self {
        sys::CorsairSessionDetails {
            clientVersion: value.client_version.into(),
            serverVersion: value.server_version.into(),
            serverHostVersion: value.server_host_version.into(),
        }
    }
}

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
        funct(new_state, data.details.into())
    }
}

#[must_use]
pub fn CorsairGetSessionDetails() -> Result<CorsairSessionDetails, CorsairError> {
    let mut details: sys::CorsairSessionDetails = CorsairSessionDetails {
        client_version: CorsairVersion {
            major: 0,
            minor: 0,
            patch: 0
        },
        server_version: CorsairVersion {
            major: 0,
            minor: 0,
            patch: 0,
        },
        server_host_version: CorsairVersion {
            major: 0,
            minor: 0,
            patch: 0,
        },
    }.into();
    let err = unsafe {
        sys::CorsairGetSessionDetails(&mut details)
    };

    CorsairError::try_from(err)?;

    Ok(details.into())
}

#[must_use]
pub fn CorsairSubscribeForEvents(
    onEvent: sys::CorsairEventHandler,
    context: *mut ::std::os::raw::c_void,
) -> Result<(), CorsairError> {
    CorsairError::try_from(unsafe {
        sys::CorsairSubscribeForEvents(onEvent, context)
    })?;

    Ok(())
}

#[must_use]
pub fn CorsairUnsubscribeFromEvents() -> Result<(), CorsairError> {
    CorsairError::try_from(unsafe {
        sys::CorsairUnsubscribeFromEvents()
    })
}

pub struct CorsairKeyEventConfiguration {
    key_id: CorsairMacroKeyId,
    is_intercepted: bool,
}

impl From<CorsairKeyEventConfiguration> for sys::CorsairKeyEventConfiguration {
    fn from(value: CorsairKeyEventConfiguration) -> Self {
        sys::CorsairKeyEventConfiguration {
            keyId: value.key_id as u32,
            isIntercepted: value.is_intercepted,
        }
    }
}

impl TryFrom<sys::CorsairKeyEventConfiguration> for CorsairKeyEventConfiguration {
    type Error = CorsairError;

    fn try_from(value: sys::CorsairKeyEventConfiguration) -> Result<Self, Self::Error> {
        Ok(CorsairKeyEventConfiguration {
            key_id: value.keyId.try_into()?,
            is_intercepted: value.isIntercepted,
        })
    }
}

#[must_use]
pub fn CorsairConfigureKeyEvent(
    device_id: char,
    config: CorsairKeyEventConfiguration,
) -> Result<(), CorsairError> {
    let config: sys::CorsairKeyEventConfiguration = config.into();
    let device_id = &mut (device_id as i8);
    CorsairError::try_from(unsafe {
        sys::CorsairConfigureKeyEvent(device_id, &config)
    })
}

#[must_use]
pub fn CorsairGetDevicePropertyInfo(
    deviceId: *mut ::std::os::raw::c_char,
    propertyId: u32,
    index: ::std::os::raw::c_uint,
    dataType: *mut u32,
    flags: *mut ::std::os::raw::c_uint,
) -> Result<(), CorsairError> {
    CorsairError::try_from(unsafe {
        sys::CorsairGetDevicePropertyInfo(deviceId, propertyId, index, dataType, flags)
    })
}

#[must_use]
pub fn CorsairReadDeviceProperty(
    deviceId: *mut ::std::os::raw::c_char,
    propertyId: u32,
    index: ::std::os::raw::c_uint,
    property: *mut sys::CorsairProperty,
) -> Result<(), CorsairError> {
    CorsairError::try_from(unsafe {
        sys::CorsairReadDeviceProperty(deviceId, propertyId, index, property)
    })
}

#[must_use]
pub fn CorsairFreeProperty(property: *mut sys::CorsairProperty) -> Result<(), CorsairError> {
    CorsairError::try_from(unsafe {
        sys::CorsairFreeProperty(property)
    })
}

#[must_use]
pub fn CorsairSetLedColorsBuffer(
    deviceId: *mut ::std::os::raw::c_char,
    size: ::std::os::raw::c_int,
    ledColors: *const sys::CorsairLedColor,
) -> Result<(), CorsairError> {
    CorsairError::try_from(unsafe {
        sys::CorsairSetLedColorsBuffer(deviceId, size, ledColors)
    })
}

#[must_use]
pub fn CorsairSetLedColorsFlushBufferAsync(
    callback: sys::CorsairAsyncCallback,
    context: *mut ::std::os::raw::c_void,
) -> Result<(), CorsairError> {
    CorsairError::try_from(unsafe {
        sys::CorsairSetLedColorsFlushBufferAsync(callback, context)
    })
}

#[must_use]
pub fn CorsairGetLedColors(
    deviceId: *mut ::std::os::raw::c_char,
    size: ::std::os::raw::c_int,
    ledColors: *mut sys::CorsairLedColor,
) -> Result<(), CorsairError> {
    CorsairError::try_from(unsafe {
        sys::CorsairGetLedColors(deviceId, size, ledColors)
    })
}

#[must_use]
pub fn CorsairSetLayerPriority(priority: ::std::os::raw::c_uint) -> Result<(), CorsairError> {
    CorsairError::try_from(unsafe {
        sys::CorsairSetLayerPriority(priority)
    })
}

#[must_use]
pub fn CorsairGetLedLuidForKeyName(
    deviceId: *mut ::std::os::raw::c_char,
    keyName: ::std::os::raw::c_char,
    ledId: *mut sys::CorsairLedLuid,
) -> Result<(), CorsairError> {
    CorsairError::try_from(unsafe {
        sys::CorsairGetLedLuidForKeyName(deviceId, keyName, ledId)
    })
}

#[must_use]
pub fn CorsairRequestControl(
    deviceId: *mut ::std::os::raw::c_char,
    accessLevel: u32,
) -> Result<(), CorsairError> {
    CorsairError::try_from(unsafe {
        sys::CorsairRequestControl(deviceId, accessLevel)
    })
}
