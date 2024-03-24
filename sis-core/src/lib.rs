use std::time::Duration;

use cgmath::Rad;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

#[repr(u16)]
#[derive(Debug, Clone, Copy)]
pub enum VirtualKey {
    // https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
    MLButton = VK_LBUTTON.0,
    MRButton = VK_RBUTTON.0,
    Cancel = VK_CANCEL.0,
    MMButton = VK_MBUTTON.0,
    MX1Button = VK_XBUTTON1.0,
    MX2Button = VK_XBUTTON2.0,
    Backspace = VK_BACK.0,
    Tab = VK_TAB.0,
    Clear = VK_CLEAR.0,
    Return = VK_RETURN.0,
    Shift = VK_SHIFT.0,
    Control = VK_CONTROL.0,
    Alt = VK_MENU.0,
    Pause = VK_PAUSE.0,
    CapsLock = VK_CAPITAL.0,
    Kana = VK_KANA.0,
    //Hangul = VK_HANGUL.0, VK_KANA = VK_HANGUL = 0x15?
    ImeOn = VK_IME_ON.0,
    ImeJunja = VK_JUNJA.0,
    ImeFinal = VK_FINAL.0,
    ImeHanja = VK_HANJA.0,
    //ImeKanji = VK_KANJI.0, VK_KANJI = VK_HANJA = 0x19?
    ImeOff = VK_IME_OFF.0,
    Esc = VK_ESCAPE.0,
    ImeConvert = VK_CONVERT.0,
    ImeNonConvert = VK_NONCONVERT.0,
    ImeAccept = VK_ACCEPT.0,
    ImeModeChange = VK_MODECHANGE.0,
    Space = VK_SPACE.0,
    PageUp = VK_PRIOR.0,
    PageDown = VK_NEXT.0,
    End = VK_END.0,
    Home = VK_HOME.0,
    ArrowLeft = VK_LEFT.0,
    ArrowUp = VK_UP.0,
    ArrowRight = VK_RIGHT.0,
    ArrowDown = VK_DOWN.0,
    Select = VK_SELECT.0,
    Print = VK_PRINT.0,
    Execute = VK_EXECUTE.0,
    PrintScreen = VK_SNAPSHOT.0,
    Insert = VK_INSERT.0,
    Delete = VK_DELETE.0,
    Help = VK_HELP.0,
    N0 = VK_0.0,
    N1 = VK_1.0,
    N2 = VK_2.0,
    N3 = VK_3.0,
    N4 = VK_4.0,
    N5 = VK_5.0,
    N6 = VK_6.0,
    N7 = VK_7.0,
    N8 = VK_8.0,
    N9 = VK_9.0,
    A = VK_A.0,
    B = VK_B.0,
    C = VK_C.0,
    D = VK_D.0,
    E = VK_E.0,
    F = VK_F.0,
    G = VK_G.0,
    H = VK_H.0,
    I = VK_I.0,
    J = VK_J.0,
    K = VK_K.0,
    L = VK_L.0,
    M = VK_M.0,
    N = VK_N.0,
    O = VK_O.0,
    P = VK_P.0,
    Q = VK_Q.0,
    R = VK_R.0,
    S = VK_S.0,
    T = VK_T.0,
    U = VK_U.0,
    V = VK_V.0,
    W = VK_W.0,
    X = VK_X.0,
    Y = VK_Y.0,
    Z = VK_Z.0,
    LWin = VK_LWIN.0,
    RWin = VK_RWIN.0,
    Apps = VK_APPS.0,
    Sleep = VK_SLEEP.0,
    NP0 = VK_NUMPAD0.0,
    NP1 = VK_NUMPAD1.0,
    NP2 = VK_NUMPAD2.0,
    NP3 = VK_NUMPAD3.0,
    NP4 = VK_NUMPAD4.0,
    NP5 = VK_NUMPAD5.0,
    NP6 = VK_NUMPAD6.0,
    NP7 = VK_NUMPAD7.0,
    NP8 = VK_NUMPAD8.0,
    NP9 = VK_NUMPAD9.0,
    Mult = VK_MULTIPLY.0,
    Add = VK_ADD.0,
    Separator = VK_SEPARATOR.0,
    Sub = VK_SUBTRACT.0,
    Dec = VK_DECIMAL.0,
    Div = VK_DIVIDE.0,
    F1 = VK_F1.0,
    F2 = VK_F2.0,
    F3 = VK_F3.0,
    F4 = VK_F4.0,
    F5 = VK_F5.0,
    F6 = VK_F6.0,
    F7 = VK_F7.0,
    F8 = VK_F8.0,
    F9 = VK_F9.0,
    F10 = VK_F10.0,
    F11 = VK_F11.0,
    F12 = VK_F12.0,
    F13 = VK_F13.0,
    F14 = VK_F14.0,
    F15 = VK_F15.0,
    F16 = VK_F16.0,
    F17 = VK_F17.0,
    F18 = VK_F18.0,
    F19 = VK_F19.0,
    F20 = VK_F20.0,
    F21 = VK_F21.0,
    F22 = VK_F22.0,
    F23 = VK_F23.0,
    F24 = VK_F24.0,
    NumLock = VK_NUMLOCK.0,
    Scroll = VK_SCROLL.0,
    OemSpecific92 = 0x92,
    OemSpecific93 = 0x93,
    OemSpecific94 = 0x94,
    OemSpecific95 = 0x95,
    OemSpecific96 = 0x96,
    LShift = VK_LSHIFT.0,
    RShift = VK_RSHIFT.0,
    LControl = VK_LCONTROL.0,
    RControl = VK_RCONTROL.0,
    LMenu = VK_LMENU.0,
    RMenu = VK_RMENU.0,
    BrowserBack = VK_BROWSER_BACK.0,
    BrowserForward = VK_BROWSER_FORWARD.0,
    BrowserRefresh = VK_BROWSER_REFRESH.0,
    BrowserStop = VK_BROWSER_STOP.0,
    BrowserSearch = VK_BROWSER_SEARCH.0,
    BrowserFavorites = VK_BROWSER_FAVORITES.0,
    BrowserHome = VK_BROWSER_HOME.0,
    VolMute = VK_VOLUME_MUTE.0,
    VolDown = VK_VOLUME_DOWN.0,
    VolUp = VK_VOLUME_UP.0,
    MediaNextTrack = VK_MEDIA_NEXT_TRACK.0,
    MediaPrevTrack = VK_MEDIA_PREV_TRACK.0,
    MediaStop = VK_MEDIA_STOP.0,
    MediaPlayPause = VK_MEDIA_PLAY_PAUSE.0,
    LaunchMail = VK_LAUNCH_MAIL.0,
    LaunchMediaSelect = VK_LAUNCH_MEDIA_SELECT.0,
    App1 = VK_LAUNCH_APP1.0,
    App2 = VK_LAUNCH_APP2.0,
    Oem1 = VK_OEM_1.0,
    OemPlus = VK_OEM_PLUS.0,
    OemComma = VK_OEM_COMMA.0,
    OemMinus = VK_OEM_MINUS.0,
    OemPeriod = VK_OEM_PERIOD.0,
    Oem2 = VK_OEM_2.0,
    Oem3 = VK_OEM_3.0,
    Oem4 = VK_OEM_4.0,
    Oem5 = VK_OEM_5.0,
    Oem6 = VK_OEM_6.0,
    Oem7 = VK_OEM_7.0,
    Oem8 = VK_OEM_8.0,
    OemSpecificE1 = 0xE1,
    Oem102 = VK_OEM_102.0,
    OemSpecificE3 = 0xE3,
    OemSpecificE4 = 0xE4,
    ImeProcessKey = VK_PROCESSKEY.0,
    OemSpecificE6 = 0xE6,
    Packet = VK_PACKET.0,
    OemSpecificE9 = 0xE9,
    OemSpecificEA = 0xEA,
    OemSpecificEB = 0xEB,
    OemSpecificEC = 0xEC,
    OemSpecificED = 0xED,
    OemSpecificEE = 0xEE,
    OemSpecificEF = 0xEF,
    OemSpecificF0 = 0xF0,
    OemSpecificF1 = 0xF1,
    OemSpecificF2 = 0xF2,
    OemSpecificF3 = 0xF3,
    OemSpecificF4 = 0xF4,
    OemSpecificF5 = 0xF5,
    Attn = VK_ATTN.0,
    CrSel = VK_CRSEL.0,
    ExSel = VK_EXSEL.0,
    EraseEof = VK_EREOF.0,
    Play = VK_PLAY.0,
    Zoom = VK_ZOOM.0,
    PA1 = VK_PA1.0,
    OemClear = VK_OEM_CLEAR.0,
}

impl VirtualKey {
    pub fn to_vk(self) -> VIRTUAL_KEY {
        VIRTUAL_KEY(self as u16)
    }

    pub fn to_scan(&self) -> u16 {
        use VirtualKey as VK;
        match self {
            VK::B => 0x30,
            VK::P => 0x19,
            VK::J => 0x24,
            VK::I => 0x17,
            VK::Control => 0x1D,
            VK::Alt => 0x38,
            _ => todo!()
        }
    }
}

pub struct ParseError;

impl TryFrom<String> for VirtualKey {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        use VirtualKey as VK;
        Ok(match value.to_lowercase().as_str() {
            "a" => VK::A,
            "b" => VK::B,
            "c" => VK::C,
            "d" => VK::D,
            "e" => VK::E,
            "f" => VK::F,
            "g" => VK::G,
            "h" => VK::H,
            "i" => VK::I,
            "j" => VK::J,
            "k" => VK::K,
            "l" => VK::L,
            "m" => VK::M,
            "n" => VK::N,
            "o" => VK::O,
            "p" => VK::P,
            "q" => VK::Q,
            "r" => VK::R,
            "s" => VK::S,
            "t" => VK::T,
            "u" => VK::U,
            "v" => VK::V,
            "w" => VK::W,
            "x" => VK::X,
            "y" => VK::Y,
            "z" => VK::Z,
            "ctrl" => VK::Control,
            "alt" => VK::Alt,
            "shift" => VK::Shift,
            _ => return Err(ParseError),
        })
    }
}

#[derive(Debug, Clone)]
pub enum HotkeySlot {
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    S12,
}

impl HotkeySlot {
    pub fn to_vkey(&self) -> u32 {
        match self {
            HotkeySlot::S1 => VK_F13.0 as u32,
            HotkeySlot::S2 => VK_F14.0 as u32,
            HotkeySlot::S3 => VK_F15.0 as u32,
            HotkeySlot::S4 => VK_F16.0 as u32,
            HotkeySlot::S5 => VK_F17.0 as u32,
            HotkeySlot::S6 => VK_F18.0 as u32,
            HotkeySlot::S7 => VK_F19.0 as u32,
            HotkeySlot::S8 => VK_F20.0 as u32,
            HotkeySlot::S9 => VK_F21.0 as u32,
            HotkeySlot::S10 => VK_F22.0 as u32,
            HotkeySlot::S11 => VK_F23.0 as u32,
            HotkeySlot::S12 => VK_F24.0 as u32,
        }
    }

    pub fn to_id(&self) -> i32 {
        match self {
            HotkeySlot::S1 => 1,
            HotkeySlot::S2 => 2,
            HotkeySlot::S3 => 3,
            HotkeySlot::S4 => 4,
            HotkeySlot::S5 => 5,
            HotkeySlot::S6 => 6,
            HotkeySlot::S7 => 7,
            HotkeySlot::S8 => 8,
            HotkeySlot::S9 => 9,
            HotkeySlot::S10 => 10,
            HotkeySlot::S11 => 11,
            HotkeySlot::S12 => 12,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RippleAnimation {
    pub animation: ColorAnimation,
    pub duration: Duration,
    pub speed: f64,
    pub light_amount: f64
}


#[derive(Debug, Clone)]
pub struct WaveAnimation {
    pub animation: ColorAnimation,
    pub duration: Duration,
    pub speed: f64,
    pub rotation: Rad<f32>,
    pub light_amount: f64,
    pub two_sides: bool,
}

#[derive(Debug, Clone)]
pub struct ColorChangeAnimation {
    pub animation: ColorAnimation,
    pub duration: Duration,
}


#[derive(Debug, Clone)]
pub struct ColorAnimation {
    pub name: String,
    pub keyframes: Vec<Keyframe>
}

#[derive(Debug, Clone, Copy)]
pub struct Keyframe {
    /// timestamp between 0.0 and 1.0
    pub timestamp: f32,
    pub color: RGBAf32
}

pub type RGBA = (u8,u8,u8,u8);
pub type RGBAf32 = (f32,f32,f32,f32);


pub fn rgbau8_to_rgbaf32(color: RGBA) -> RGBAf32 {
    let color = (
        color.0 as f32 / 255.0,
        color.1 as f32 / 255.0,
        color.2 as f32 / 255.0,
        color.3 as f32 / 255.0,
    );
    //let color = srg_to_oklab(color);
    color
}
