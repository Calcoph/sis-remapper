use std::ffi::CString;

use crate::sys;

#[derive(Debug)]
pub struct CorsairDeviceId(CString);

impl CorsairDeviceId {
    pub(crate) fn get_ptr(&self) -> *mut i8 {
        self.0.as_ptr() as *mut i8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CorsairLedLuid(pub(crate) u32);

impl From<CorsairLedLuid> for sys::CorsairLedLuid {
    fn from(value: CorsairLedLuid) -> Self {
        value.into()
    }
}

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
    InvalidChar,
    MaxPriorityExceded,
    InvalidDataType,
    InvalidFlags,
    InvalidMacroKeyId,

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
    pub(crate) fn try_from(value: u32) -> Result<(), CorsairError> {
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
            _ => Err(CorsairError::InvalidMacroKeyId)?
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

impl TryFrom<sys::CorsairDataType> for CorsairDataType {
    type Error = CorsairError;

    fn try_from(value: sys::CorsairDataType) -> Result<Self, Self::Error> {
        Ok(match value {
            sys::CorsairDataType_CT_Boolean => CorsairDataType::Boolean,
            sys::CorsairDataType_CT_Int32 => CorsairDataType::Int32,
            sys::CorsairDataType_CT_Float64 => CorsairDataType::Float64,
            sys::CorsairDataType_CT_String => CorsairDataType::String,
            sys::CorsairDataType_CT_Boolean_Array => CorsairDataType::BooleanArray,
            sys::CorsairDataType_CT_Int32_Array => CorsairDataType::Int32Array,
            sys::CorsairDataType_CT_Float64_Array => CorsairDataType::Float64Array,
            sys::CorsairDataType_CT_String_Array => CorsairDataType::StringArray,
            _ => Err(CorsairError::InvalidDataType)?
        })
    }
}

#[repr(u32)]
pub enum CorsairPropertyFlag {
    None = sys::CorsairPropertyFlag_CPF_None,
    CanRead = sys::CorsairPropertyFlag_CPF_CanRead,
    CanWrite = sys::CorsairPropertyFlag_CPF_CanWrite,
    Indexed = sys::CorsairPropertyFlag_CPF_Indexed
}

bitflags::bitflags! {
    pub struct CorsairPropertyFlags: u32 {
        const NONE = 1 << sys::CorsairPropertyFlag_CPF_None;
        const CAN_READ = 1 << sys::CorsairPropertyFlag_CPF_CanRead;
        const CAN_WRITE = 1 << sys::CorsairPropertyFlag_CPF_CanWrite;
        const INDEXED = 1 << sys::CorsairPropertyFlag_CPF_Indexed;
    }
}

pub type CorsairSessionStateChangedHandler = Box<dyn Fn(CorsairSessionState, CorsairSessionDetails) -> ()>;

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

pub struct CorsairKeyEventConfiguration {
    key_id: CorsairMacroKeyId,
    is_intercepted: bool,
}

impl From<CorsairKeyEventConfiguration> for sys::CorsairKeyEventConfiguration {
    fn from(value: CorsairKeyEventConfiguration) -> Self {
        sys::CorsairKeyEventConfiguration {
            keyId: value.key_id as sys::CorsairMacroKeyId,
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
    pub id: CorsairDeviceId,
    pub serial: CString,
    pub model: CString,
    pub ledCount: i32,
    pub channelCount: i32,
}

impl From<sys::CorsairDeviceInfo> for CorsairDeviceInfo {
    fn from(value: sys::CorsairDeviceInfo) -> Self {
        // TODO: Construct CString from raw to avoid extra allocs
        let model = unsafe {
            std::ffi::CStr::from_ptr(value.model.as_ptr())
        };
        let model = model.to_owned();
        let serial = unsafe {
            std::ffi::CStr::from_ptr(value.serial.as_ptr())
        };
        let serial = serial.to_owned();
        let id = unsafe {
            std::ffi::CStr::from_ptr(value.id.as_ptr())
        };
        let id = id.to_owned();
        CorsairDeviceInfo {
            type_: value.type_.into(),
            id: CorsairDeviceId(id),
            serial,
            model,
            ledCount: value.ledCount,
            channelCount: value.channelCount,
        }
    }
}

pub struct CorsairLedColor {
    pub id: CorsairLedLuid,
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<CorsairLedColor> for sys::CorsairLedColor {
    fn from(value: CorsairLedColor) -> Self {
        sys::CorsairLedColor {
            id: value.id.0,
            r: value.r,
            g: value.g,
            b: value.b,
            a: value.a,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CorsairLedPosition {
    pub id: CorsairLedLuid,
    pub cx: f64,
    pub cy: f64
}

impl From<sys::CorsairLedPosition> for CorsairLedPosition {
    fn from(value: sys::CorsairLedPosition) -> Self {
        CorsairLedPosition {
            id: CorsairLedLuid(value.id),
            cx: value.cx,
            cy: value.cy
        }
    }
}

impl From<&mut [bool]> for sys::CorsairDataType_BooleanArray {
    fn from(value: &mut [bool]) -> Self {
        sys::CorsairDataType_BooleanArray {
            count: value.len() as u32,
            items: value.as_mut_ptr(),
        }
    }
}

impl From<sys::CorsairDataType_BooleanArray> for Vec<bool> {
    fn from(value: sys::CorsairDataType_BooleanArray) -> Self {
        unsafe{
            Vec::from_raw_parts(value.items, value.count as usize, value.count as usize)
        }
    }
}

impl From<&mut [i32]> for sys::CorsairDataType_Int32Array {
    fn from(value: &mut [i32]) -> Self {
        sys::CorsairDataType_Int32Array {
            count: value.len() as u32,
            items: value.as_mut_ptr(),
        }
    }
}

impl From<sys::CorsairDataType_Int32Array> for Vec<i32> {
    fn from(value: sys::CorsairDataType_Int32Array) -> Self {
        unsafe{
            Vec::from_raw_parts(value.items, value.count as usize, value.count as usize)
        }
    }
}

impl From<&mut [f64]> for sys::CorsairDataType_Float64Array {
    fn from(value: &mut [f64]) -> Self {
        sys::CorsairDataType_Float64Array {
            count: value.len() as u32,
            items: value.as_mut_ptr(),
        }
    }
}

impl From<sys::CorsairDataType_Float64Array> for Vec<f64> {
    fn from(value: sys::CorsairDataType_Float64Array) -> Self {
        unsafe{
            Vec::from_raw_parts(value.items, value.count as usize, value.count as usize)
        }
    }
}

impl From<sys::CorsairDataType_StringArray> for Vec<CString> {
    fn from(value: sys::CorsairDataType_StringArray) -> Self {
        unsafe{
            Vec::from_raw_parts(value.items, value.count as usize, value.count as usize)
                .into_iter()
                .map(|ptr| CString::from_raw(ptr))
                .collect()
        }
    }
}

pub enum CorsairDataValue {
    Bool(bool),
    I32(i32),
    F64(f64),
    String(CString),
    BoolArray(Vec<bool>),
    I32Array(Vec<i32>),
    F64Array(Vec<f64>),
    StringArray(Vec<CString>),
}

pub struct CorsairProperty(pub CorsairDataValue);

impl CorsairProperty {
    fn get_type(&self) -> CorsairDataType {
        match self.0 {
            CorsairDataValue::Bool(_) => CorsairDataType::Boolean,
            CorsairDataValue::I32(_) => CorsairDataType::Int32,
            CorsairDataValue::F64(_) => CorsairDataType::Float64,
            CorsairDataValue::String(_) => CorsairDataType::String,
            CorsairDataValue::BoolArray(_) => CorsairDataType::BooleanArray,
            CorsairDataValue::I32Array(_) => CorsairDataType::Int32Array,
            CorsairDataValue::F64Array(_) => CorsairDataType::Float64Array,
            CorsairDataValue::StringArray(_) => CorsairDataType::StringArray,
        }
    }

    fn get_value(&mut self) -> (sys::CorsairDataValue, Option<Vec<*mut i8>>) {
        //returned vec is just so it doesn't get dropped before the sys::CorsairDataValue is passed to the SDK
        let val = &mut self.0;
        match val {
            CorsairDataValue::Bool(b) => (sys::CorsairDataValue {boolean: *b}, None),
            CorsairDataValue::I32(i) => (sys::CorsairDataValue {int32: *i}, None),
            CorsairDataValue::F64(f) => (sys::CorsairDataValue {float64: *f}, None),
            CorsairDataValue::String(s) => (sys::CorsairDataValue {string: s.as_ptr() as *mut i8}, None),
            CorsairDataValue::BoolArray(ba) => (sys::CorsairDataValue {boolean_array: <&mut [_]>::into(ba.as_mut())}, None),
            CorsairDataValue::I32Array(ia) => (sys::CorsairDataValue {int32_array: <&mut [_]>::into(ia.as_mut())}, None),
            CorsairDataValue::F64Array(fa) => (sys::CorsairDataValue {float64_array: <&mut [_]>::into(fa.as_mut())}, None),
            CorsairDataValue::StringArray(sa) => {
                let mut pointers = sa.into_iter().map(|cstr| cstr.as_ptr() as *mut i8).collect::<Vec<_>>();
                let s_arr = sys::CorsairDataType_StringArray {
                    count: pointers.len() as u32,
                    items: pointers.as_mut_ptr(),
                };
                (
                    sys::CorsairDataValue {string_array: s_arr},
                    Some(pointers)
                )
            },
        }
    }

    pub(crate) fn to_sys(&mut self) -> (sys::CorsairProperty, Option<Vec<*mut i8>>) {
        //returned vec is just so it doesn't get dropped before the sys::CorsairDataValue is passed to the SDK
        let type_ = self.get_type();
        let (value, drop_later) = self.get_value();

        (
            sys::CorsairProperty {
                type_: type_ as sys::CorsairDataType,
                value,
            },
            drop_later
        )
    }
}

impl TryFrom<sys::CorsairProperty> for CorsairProperty {
    type Error = CorsairError;

    fn try_from(value: sys::CorsairProperty) -> Result<Self, Self::Error> {
        let type_: CorsairDataType = value.type_.try_into()?;
        Ok(unsafe{match type_ {
            CorsairDataType::Boolean => CorsairProperty(CorsairDataValue::Bool(value.value.boolean)),
            CorsairDataType::Int32 => CorsairProperty(CorsairDataValue::I32(value.value.int32)),
            CorsairDataType::Float64 => CorsairProperty(CorsairDataValue::F64(value.value.float64)),
            CorsairDataType::String => CorsairProperty(CorsairDataValue::String(CString::from_raw(value.value.string))),
            CorsairDataType::BooleanArray => CorsairProperty(CorsairDataValue::BoolArray(value.value.boolean_array.into())),
            CorsairDataType::Int32Array => CorsairProperty(CorsairDataValue::I32Array(value.value.int32_array.into())),
            CorsairDataType::Float64Array => CorsairProperty(CorsairDataValue::F64Array(value.value.float64_array.into())),
            CorsairDataType::StringArray => CorsairProperty(CorsairDataValue::StringArray(value.value.string_array.into())),
        }})
    }
}

pub struct Priority(u32);

impl Priority {
    pub fn new(priority: u8) -> Result<Priority, CorsairError> {
        let priority = priority as u32;
        if priority <= sys::CORSAIR_LAYER_PRIORITY_MAX {
            Ok(Priority(priority))
        } else {
            Err(CorsairError::MaxPriorityExceded)
        }
    }
}

pub struct KeyName(pub(crate) i8);

impl KeyName {
    pub fn new(c: char) -> Result<KeyName, CorsairError> {
        match c {
            'A'..='Z' => Ok(KeyName(c as i8)),
            _ => Err(CorsairError::InvalidChar)
        }
    }
}
