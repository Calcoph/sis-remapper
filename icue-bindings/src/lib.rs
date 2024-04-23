#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use std::{os::raw::{c_uint, c_void}, ptr::null_mut};

use types::{CorsairAccessLevel, CorsairDataType, CorsairDeviceId, CorsairDeviceInfo, CorsairDevicePropertyId, CorsairError, CorsairKeyEventConfiguration, CorsairLedColor, CorsairLedLuid, CorsairLedPosition, CorsairProperty, CorsairPropertyFlags, CorsairSessionDetails, CorsairSessionState, CorsairSessionStateChangedHandler, CorsairVersion, IntoCE, KeyName};

mod sys;
pub mod types;

pub const CORSAIR_STRING_SIZE_S: c_uint = sys::CORSAIR_STRING_SIZE_S;
pub const CORSAIR_STRING_SIZE_M: c_uint = sys::CORSAIR_STRING_SIZE_M;
pub const CORSAIR_LAYER_PRIORITY_MAX: c_uint = sys::CORSAIR_LAYER_PRIORITY_MAX;
pub const CORSAIR_DEVICE_COUNT_MAX: c_uint = sys::CORSAIR_DEVICE_COUNT_MAX;
pub const CORSAIR_DEVICE_LEDCOUNT_MAX: c_uint = sys::CORSAIR_DEVICE_LEDCOUNT_MAX;

#[must_use]
pub fn CorsairDisconnect() -> Result<(), CorsairError> {
    unsafe{CorsairError::try_from(sys::CorsairDisconnect())}
}

#[must_use]
pub fn CorsairGetDeviceInfo(
    deviceId: &CorsairDeviceId,
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
            deviceId.get_ptr(),
            (&mut device_info) as *mut sys::CorsairDeviceInfo
        ))
    };

    res?;

    Ok(device_info.into())
}

#[must_use]
pub unsafe fn CorsairSetLedColors(
    device_id: &CorsairDeviceId,
    led_colors: Vec<CorsairLedColor>
) -> Result<(), CorsairError> {
    let led_colors = led_colors.into_iter().map(|lc| lc.into()).collect::<Vec<_>>();
    CorsairError::try_from(sys::CorsairSetLedColors(device_id.get_ptr(), led_colors.len() as i32, led_colors.as_ptr()))
}

#[must_use]
pub fn CorsairGetLedPositions(
    device_id: &CorsairDeviceId,
) -> Result<Vec<CorsairLedPosition>, CorsairError> {
    let mut leds = Vec::with_capacity(CORSAIR_DEVICE_LEDCOUNT_MAX as usize);
    let mut size: i32 = 0;
    let err = unsafe {
        let err = CorsairError::try_from(sys::CorsairGetLedPositions(device_id.get_ptr(), CORSAIR_DEVICE_LEDCOUNT_MAX as i32, leds.as_mut_ptr(), &mut size as *mut i32));
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

#[must_use]
pub fn CorsairConfigureKeyEvent(
    device_id: &CorsairDeviceId,
    config: CorsairKeyEventConfiguration,
) -> Result<(), CorsairError> {
    let config: sys::CorsairKeyEventConfiguration = config.into();
    CorsairError::try_from(unsafe {
        sys::CorsairConfigureKeyEvent(device_id.get_ptr(), &config)
    })
}

#[must_use]
pub fn CorsairGetDevicePropertyInfo(
    deviceId: &CorsairDeviceId,
    propertyId: CorsairDevicePropertyId,
    index: Option<u32>,
) -> Result<(CorsairDataType, CorsairPropertyFlags), CorsairError> {
    let mut data_type = sys::CorsairDataType_CT_Boolean;
    let mut flags = 0;
    let index = index.unwrap_or(0);
    CorsairError::try_from(unsafe {
        sys::CorsairGetDevicePropertyInfo(
            deviceId.get_ptr(),
            propertyId as sys::CorsairDevicePropertyId,
            index,
            &mut data_type,
            &mut flags
        )
    })?;

    let data_type = data_type.try_into()?;
    let flags = CorsairPropertyFlags::from_bits(flags).ok_or(CorsairError::InvalidFlags)?;

    Ok((data_type, flags))
}

#[must_use]
pub fn CorsairReadDeviceProperty(
    device_id: &CorsairDeviceId,
    property_id: CorsairDevicePropertyId,
    index: Option<u32>,
) -> Result<CorsairProperty, CorsairError> {
    let index = index.unwrap_or(0);
    let mut property = sys::CorsairProperty {
        type_: 0,
        value: sys::CorsairDataValue{boolean: false},
    };

    CorsairError::try_from(unsafe {
        sys::CorsairReadDeviceProperty(device_id.get_ptr(), property_id as sys::CorsairDevicePropertyId, index, &mut property)
    })?;

    property.try_into()
}

#[must_use]
pub fn CorsairFreeProperty(mut property: CorsairProperty) -> Result<(), CorsairError> {
    CorsairError::try_from(unsafe {
        let (property, drop_later) = property.to_sys();
        let res = sys::CorsairFreeProperty(&mut property.into());
        std::mem::drop(drop_later);
        res
    })
}

#[must_use]
pub fn CorsairSetLedColorsBuffer(
    device_id: &CorsairDeviceId,
    led_colors: Vec<CorsairLedColor>,
) -> Result<(), CorsairError> {
    let mut led_colors = led_colors.into_iter()
        .map(|led_color| led_color.into())
        .collect::<Vec<_>>();

    CorsairError::try_from(unsafe {
        sys::CorsairSetLedColorsBuffer(
            device_id.get_ptr(),
            led_colors.len() as i32,
            led_colors.as_mut_ptr()
        )
    })
}

#[must_use]
pub fn CorsairSetLedColorsFlushBufferAsync() -> Result<(), CorsairError> {
    // TODO: Allow setting the callback
    CorsairError::try_from(unsafe {
        sys::CorsairSetLedColorsFlushBufferAsync(None, null_mut())
    })
}

#[must_use]
pub fn CorsairGetLedColors(
    device_id: &CorsairDeviceId,
    led_colors: Vec<CorsairLedColor>,
) -> Result<(), CorsairError> {
    let mut led_colors = led_colors.into_iter()
        .map(|led_color| led_color.into())
        .collect::<Vec<_>>();

    CorsairError::try_from(unsafe {
        sys::CorsairGetLedColors(
            device_id.get_ptr(),
            led_colors.len() as i32,
            led_colors.as_mut_ptr()
        )
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
    device_id: &CorsairDeviceId,
    key_name: KeyName,
    led_id: CorsairLedLuid,
) -> Result<(), CorsairError> {
    CorsairError::try_from(unsafe {
        sys::CorsairGetLedLuidForKeyName(
            device_id.get_ptr(),
            key_name.0,
            (&mut led_id.0.into()) as *mut sys::CorsairLedLuid
        )
    })
}

#[must_use]
pub fn CorsairRequestControl(
    deviceId: &CorsairDeviceId,
    accessLevel: CorsairAccessLevel,
) -> Result<(), CorsairError> {
    CorsairError::try_from(unsafe {
        sys::CorsairRequestControl(deviceId.get_ptr(), accessLevel as sys::CorsairAccessLevel)
    })
}
