use openharmony_ability::xcomponent::KeyCode as Keycode;

use crate::keyboard::{Key, KeyCode, KeyLocation, NativeKeyCode};

/// Map an OHOS keycode to tao's logical `Key`.
///
/// Issue Eulogizethesun/tauri#109: letters, digits and punctuation used to
/// fall through to `Unidentified` on the assumption that a "Unicode character"
/// path (Android's `getUnicodeChar`) would resolve them first — but the NDK
/// XComponent key event carries no produced character on OHOS, so that path
/// never existed here and the fallback was the only path. The keycodes
/// themselves are unambiguous, so map them to base characters directly. This
/// is the unshifted character (no layout/caps awareness); consumers needing
/// the produced text use the IME events.
pub fn to_logical(keycode: Keycode) -> Key<'static> {
  use openharmony_ability::xcomponent::KeyCode::*;

  let native = NativeKeyCode::Ohos(i32::from(keycode));

  match keycode {
    // Using `BrowserHome` instead of `GoHome` according to
    // https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key/Key_Values
    Home => Key::BrowserHome,
    Back => Key::BrowserBack,

    //-------------------------------------------------------------------------------
    // Letters, digits and punctuation: OHOS keycodes are layout-independent and
    // unambiguous, so map them to their base `Character` (issue #109 — was
    // Unidentified because the Android-style Unicode path doesn't exist here).
    Key0 => Key::Character("0".into()),
    Key1 => Key::Character("1".into()),
    Key2 => Key::Character("2".into()),
    Key3 => Key::Character("3".into()),
    Key4 => Key::Character("4".into()),
    Key5 => Key::Character("5".into()),
    Key6 => Key::Character("6".into()),
    Key7 => Key::Character("7".into()),
    Key8 => Key::Character("8".into()),
    Key9 => Key::Character("9".into()),
    Star => Key::Character("*".into()),
    Pound => Key::Character("#".into()),
    A => Key::Character("a".into()),
    B => Key::Character("b".into()),
    C => Key::Character("c".into()),
    D => Key::Character("d".into()),
    E => Key::Character("e".into()),
    F => Key::Character("f".into()),
    G => Key::Character("g".into()),
    H => Key::Character("h".into()),
    I => Key::Character("i".into()),
    J => Key::Character("j".into()),
    K => Key::Character("k".into()),
    L => Key::Character("l".into()),
    M => Key::Character("m".into()),
    N => Key::Character("n".into()),
    O => Key::Character("o".into()),
    P => Key::Character("p".into()),
    Q => Key::Character("q".into()),
    R => Key::Character("r".into()),
    S => Key::Character("s".into()),
    T => Key::Character("t".into()),
    U => Key::Character("u".into()),
    V => Key::Character("v".into()),
    W => Key::Character("w".into()),
    X => Key::Character("x".into()),
    Y => Key::Character("y".into()),
    Z => Key::Character("z".into()),
    Comma => Key::Character(",".into()),
    Period => Key::Character(".".into()),
    Grave => Key::Character("`".into()),
    Minus => Key::Character("-".into()),
    Equals => Key::Character("=".into()),
    LeftBracket => Key::Character("[".into()),
    RightBracket => Key::Character("]".into()),
    Backslash => Key::Character("\\".into()),
    Semicolon => Key::Character(";".into()),
    Apostrophe => Key::Character("'".into()),
    Slash => Key::Character("/".into()),
    At => Key::Character("@".into()),
    Plus => Key::Character("+".into()),
    //-------------------------------------------------------------------------------
    DpadUp => Key::ArrowUp,
    DpadDown => Key::ArrowDown,
    DpadLeft => Key::ArrowLeft,
    DpadRight => Key::ArrowRight,
    DpadCenter => Key::Enter,

    VolumeUp => Key::AudioVolumeUp,
    VolumeDown => Key::AudioVolumeDown,
    Power => Key::Power,
    Camera => Key::Camera,
    // Clear => Key::Named(NamedKey::Clear),
    AltLeft => Key::Alt,
    AltRight => Key::Alt,
    ShiftLeft => Key::Shift,
    ShiftRight => Key::Shift,
    Tab => Key::Tab,
    Space => Key::Space,
    Sym => Key::Symbol,
    Explorer => Key::LaunchWebBrowser,
    Envelope => Key::LaunchMail,
    Enter => Key::Enter,
    Del => Key::Backspace,

    // According to https://developer.android.com/reference/android/view/KeyEvent#KEYCODE_NUM
    // Num => Key::Named(NamedKey::Alt),

    // Headsethook => Key::Named(NamedKey::HeadsetHook),
    // Focus => Key::Named(NamedKey::CameraFocus),

    // Notification => Key::Named(NamedKey::Notification),
    // Search => Key::Named(NamedKey::BrowserSearch),
    MediaPlayPause => Key::MediaPlayPause,
    MediaStop => Key::MediaStop,
    MediaNext => Key::MediaTrackNext,
    MediaPrevious => Key::MediaTrackPrevious,
    MediaRewind => Key::MediaRewind,
    MediaFastForward => Key::MediaFastForward,
    Mute => Key::MicrophoneVolumeMute,
    PageUp => Key::PageUp,
    PageDown => Key::PageDown,

    Escape => Key::Escape,
    ForwardDel => Key::Delete,
    CtrlLeft => Key::Control,
    CtrlRight => Key::Control,
    CapsLock => Key::CapsLock,
    ScrollLock => Key::ScrollLock,
    MetaLeft => Key::Super,
    MetaRight => Key::Super,
    Function => Key::Fn,
    SysRq => Key::PrintScreen,
    Break => Key::Pause,
    MoveHome => Key::Home,
    MoveEnd => Key::End,
    Insert => Key::Insert,
    Forward => Key::BrowserForward,
    MediaPlay => Key::MediaPlay,
    MediaPause => Key::MediaPause,
    MediaClose => Key::MediaClose,
    MediaEject => Key::Eject,
    MediaRecord => Key::MediaRecord,
    F1 => Key::F1,
    F2 => Key::F2,
    F3 => Key::F3,
    F4 => Key::F4,
    F5 => Key::F5,
    F6 => Key::F6,
    F7 => Key::F7,
    F8 => Key::F8,
    F9 => Key::F9,
    F10 => Key::F10,
    F11 => Key::F11,
    F12 => Key::F12,
    NumLock => Key::NumLock,
    // Numpad keys: logical = the character the key produces (NumLock on).
    // `to_location` still reports `KeyLocation::Numpad` for disambiguation.
    Numpad0 => Key::Character("0".into()),
    Numpad1 => Key::Character("1".into()),
    Numpad2 => Key::Character("2".into()),
    Numpad3 => Key::Character("3".into()),
    Numpad4 => Key::Character("4".into()),
    Numpad5 => Key::Character("5".into()),
    Numpad6 => Key::Character("6".into()),
    Numpad7 => Key::Character("7".into()),
    Numpad8 => Key::Character("8".into()),
    Numpad9 => Key::Character("9".into()),
    NumpadDivide => Key::Character("/".into()),
    NumpadMultiply => Key::Character("*".into()),
    NumpadSubtract => Key::Character("-".into()),
    NumpadAdd => Key::Character("+".into()),
    NumpadDot => Key::Character(".".into()),
    NumpadComma => Key::Character(",".into()),
    NumpadEnter => Key::Enter,
    NumpadEquals => Key::Character("=".into()),
    NumpadLeftParen => Key::Character("(".into()),
    NumpadRightParen => Key::Character(")".into()),

    VolumeMute => Key::AudioVolumeMute,
    Info => Key::Info,
    ChannelUp => Key::ChannelUp,
    ChannelDown => Key::ChannelDown,
    ZoomIn => Key::ZoomIn,
    ZoomOut => Key::ZoomOut,
    TV => Key::TV,
    // Guide => Key::Named(NamedKey::Guide),
    // Dvr => Key::Named(NamedKey::DVR),
    // Bookmark => Key::Named(NamedKey::BrowserFavorites),
    // Captions => Key::Named(NamedKey::ClosedCaptionToggle),
    // Settings => Key::Named(NamedKey::Settings),
    // TvPower => Key::Named(NamedKey::TVPower),
    // TvInput => Key::Named(NamedKey::TVInput),
    // StbPower => Key::Named(NamedKey::STBPower),
    // StbInput => Key::Named(NamedKey::STBInput),
    // AvrPower => Key::Named(NamedKey::AVRPower),
    // AvrInput => Key::Named(NamedKey::AVRInput),
    // ProgRed => Key::Named(NamedKey::ColorF0Red),
    // ProgGreen => Key::Named(NamedKey::ColorF1Green),
    // ProgYellow => Key::Named(NamedKey::ColorF2Yellow),
    // ProgBlue => Key::Named(NamedKey::ColorF3Blue),
    // AppSwitch => Key::Named(NamedKey::AppSwitch),
    // LanguageSwitch => Key::Named(NamedKey::GroupNext),
    // MannerMode => Key::Named(NamedKey::MannerMode),
    // Keycode3dMode => Key::Named(NamedKey::TV3DMode),
    // Contacts => Key::Named(NamedKey::LaunchContacts),
    Calendar => Key::LaunchCalendar,
    // Music => Key::Named(NamedKey::LaunchMusicPlayer),
    // Calculator => Key::Named(NamedKey::LaunchApplication2),
    ZenkakuHankaku => Key::ZenkakuHankaku,
    // Eisu => Key::Named(NamedKey::Eisu),
    Muhenkan => Key::NonConvert,
    Henkan => Key::Convert,
    KatakanaHiragana => Key::HiraganaKatakana,
    // Kana => Key::Named(NamedKey::KanjiMode),
    BrightnessDown => Key::BrightnessDown,
    BrightnessUp => Key::BrightnessUp,
    // MediaAudioTrack => Key::Named(NamedKey::MediaAudioTrack),
    Sleep => Key::Standby,
    Wakeup => Key::WakeUp,
    // Pairing => Key::Named(NamedKey::Pairing),
    // MediaTopMenu => Key::Named(NamedKey::MediaTopMenu),
    // LastChannel => Key::Named(NamedKey::MediaLast),
    // TvDataService => Key::Named(NamedKey::TVDataService),
    // VoiceAssist => Key::Named(NamedKey::VoiceDial),
    // TvRadioService => Key::Named(NamedKey::TVRadioService),
    // TvTeletext => Key::Named(NamedKey::Teletext),
    // TvNumberEntry => Key::Named(NamedKey::TVNumberEntry),
    // TvTerrestrialAnalog => Key::Named(NamedKey::TVTerrestrialAnalog),
    // TvTerrestrialDigital => Key::Named(NamedKey::TVTerrestrialDigital),
    // TvSatellite => Key::Named(NamedKey::TVSatellite),
    // TvSatelliteBs => Key::Named(NamedKey::TVSatelliteBS),
    // TvSatelliteCs => Key::Named(NamedKey::TVSatelliteCS),
    // TvSatelliteService => Key::Named(NamedKey::TVSatelliteToggle),
    // TvNetwork => Key::Named(NamedKey::TVNetwork),
    // TvAntennaCable => Key::Named(NamedKey::TVAntennaCable),
    // TvInputHdmi1 => Key::Named(NamedKey::TVInputHDMI1),
    // TvInputHdmi2 => Key::Named(NamedKey::TVInputHDMI2),
    // TvInputHdmi3 => Key::Named(NamedKey::TVInputHDMI3),
    // TvInputHdmi4 => Key::Named(NamedKey::TVInputHDMI4),
    // TvInputComposite1 => Key::Named(NamedKey::TVInputComposite1),
    // TvInputComposite2 => Key::Named(NamedKey::TVInputComposite2),
    // TvInputComponent1 => Key::Named(NamedKey::TVInputComponent1),
    // TvInputComponent2 => Key::Named(NamedKey::TVInputComponent2),
    // TvInputVga1 => Key::Named(NamedKey::TVInputVGA1),
    // TvAudioDescription => Key::Named(NamedKey::TVAudioDescription),
    // TvAudioDescriptionMixUp => Key::Named(NamedKey::TVAudioDescriptionMixUp),
    // TvAudioDescriptionMixDown => Key::Named(NamedKey::TVAudioDescriptionMixDown),
    // TvZoomMode => Key::Named(NamedKey::ZoomToggle),
    // TvContentsMenu => Key::Named(NamedKey::TVContentsMenu),
    // TvMediaContextMenu => Key::Named(NamedKey::TVMediaContext),
    // TvTimerProgramming => Key::Named(NamedKey::TVTimer),
    Help => Key::Help,
    // NavigatePrevious => Key::Named(NamedKey::NavigatePrevious),
    // NavigateNext => Key::Named(NamedKey::NavigateNext),
    // NavigateIn => Key::Named(NamedKey::NavigateIn),
    // NavigateOut => Key::Named(NamedKey::NavigateOut),
    // MediaSkipForward => Key::Named(NamedKey::MediaSkipForward),
    // MediaSkipBackward => Key::Named(NamedKey::MediaSkipBackward),
    // MediaStepForward => Key::Named(NamedKey::MediaStepForward),
    // MediaStepBackward => Key::Named(NamedKey::MediaStepBackward),
    Cut => Key::Cut,
    Copy => Key::Copy,
    Paste => Key::Paste,
    Refresh => Key::BrowserRefresh,

    // -----------------------------------------------------------------
    // Keycodes that don't have a logical Key mapping
    // -----------------------------------------------------------------
    Unknown => Key::Unidentified(native),

    // Can be added on demand
    // SoftLeft => Key::Unidentified(native),
    // SoftRight => Key::Unidentified(native),
    Menu => Key::Unidentified(native),

    // Pictsymbols => Key::Unidentified(native),
    // SwitchCharset => Key::Unidentified(native),

    // -----------------------------------------------------------------
    // Gamepad events should be exposed through a separate API, not
    // keyboard events
    // ButtonA => Key::Unidentified(native),
    // ButtonB => Key::Unidentified(native),
    // ButtonC => Key::Unidentified(native),
    // ButtonX => Key::Unidentified(native),
    // ButtonY => Key::Unidentified(native),
    // ButtonZ => Key::Unidentified(native),
    // ButtonL1 => Key::Unidentified(native),
    // ButtonR1 => Key::Unidentified(native),
    // ButtonL2 => Key::Unidentified(native),
    // ButtonR2 => Key::Unidentified(native),
    // ButtonThumbl => Key::Unidentified(native),
    // ButtonThumbr => Key::Unidentified(native),
    // ButtonStart => Key::Unidentified(native),
    // ButtonSelect => Key::Unidentified(native),
    // ButtonMode => Key::Unidentified(native),
    // // -----------------------------------------------------------------
    // Window => Key::Unidentified(native),

    // Button1 => Key::Unidentified(native),
    // Button2 => Key::Unidentified(native),
    // Button3 => Key::Unidentified(native),
    // Button4 => Key::Unidentified(native),
    // Button5 => Key::Unidentified(native),
    // Button6 => Key::Unidentified(native),
    // Button7 => Key::Unidentified(native),
    // Button8 => Key::Unidentified(native),
    // Button9 => Key::Unidentified(native),
    // Button10 => Key::Unidentified(native),
    // Button11 => Key::Unidentified(native),
    // Button12 => Key::Unidentified(native),
    // Button13 => Key::Unidentified(native),
    // Button14 => Key::Unidentified(native),
    // Button15 => Key::Unidentified(native),
    // Button16 => Key::Unidentified(native),
    Yen => Key::Unidentified(native),
    Ro => Key::Unidentified(native),

    // Assist => Key::Unidentified(native),

    // Keycode11 => Key::Unidentified(native),
    // Keycode12 => Key::Unidentified(native),

    // StemPrimary => Key::Unidentified(native),
    // Stem1 => Key::Unidentified(native),
    // Stem2 => Key::Unidentified(native),
    // Stem3 => Key::Unidentified(native),

    // DpadUpLeft => Key::Unidentified(native),
    // DpadDownLeft => Key::Unidentified(native),
    // DpadUpRight => Key::Unidentified(native),
    // DpadDownRight => Key::Unidentified(native),

    // SoftSleep => Key::Unidentified(native),

    // SystemNavigationUp => Key::Unidentified(native),
    // SystemNavigationDown => Key::Unidentified(native),
    // SystemNavigationLeft => Key::Unidentified(native),
    // SystemNavigationRight => Key::Unidentified(native),

    // AllApps => Key::Unidentified(native),
    // ThumbsUp => Key::Unidentified(native),
    // ThumbsDown => Key::Unidentified(native),
    // ProfileSwitch => Key::Unidentified(native),

    // It's always possible that new versions of Android could introduce
    // key codes we can't know about at compile time.
    _ => Key::Unidentified(native),
  }
}

/// Map an OHOS keycode to tao's physical `KeyCode` (issue
/// Eulogizethesun/tauri#109). OHOS keycodes are layout-independent scan
/// positions — exactly what `physical_key` means — so the mapping is direct.
/// Unmapped keys fall back to `Unidentified(NativeKeyCode::Ohos(..))`, which
/// still carries the raw OHOS code for downstream consumers.
pub fn to_physical(keycode: Keycode) -> KeyCode {
  use openharmony_ability::xcomponent::KeyCode::*;

  match keycode {
    Key0 => KeyCode::Digit0,
    Key1 => KeyCode::Digit1,
    Key2 => KeyCode::Digit2,
    Key3 => KeyCode::Digit3,
    Key4 => KeyCode::Digit4,
    Key5 => KeyCode::Digit5,
    Key6 => KeyCode::Digit6,
    Key7 => KeyCode::Digit7,
    Key8 => KeyCode::Digit8,
    Key9 => KeyCode::Digit9,
    A => KeyCode::KeyA,
    B => KeyCode::KeyB,
    C => KeyCode::KeyC,
    D => KeyCode::KeyD,
    E => KeyCode::KeyE,
    F => KeyCode::KeyF,
    G => KeyCode::KeyG,
    H => KeyCode::KeyH,
    I => KeyCode::KeyI,
    J => KeyCode::KeyJ,
    K => KeyCode::KeyK,
    L => KeyCode::KeyL,
    M => KeyCode::KeyM,
    N => KeyCode::KeyN,
    O => KeyCode::KeyO,
    P => KeyCode::KeyP,
    Q => KeyCode::KeyQ,
    R => KeyCode::KeyR,
    S => KeyCode::KeyS,
    T => KeyCode::KeyT,
    U => KeyCode::KeyU,
    V => KeyCode::KeyV,
    W => KeyCode::KeyW,
    X => KeyCode::KeyX,
    Y => KeyCode::KeyY,
    Z => KeyCode::KeyZ,
    Comma => KeyCode::Comma,
    Period => KeyCode::Period,
    Grave => KeyCode::Backquote,
    Minus => KeyCode::Minus,
    Equals => KeyCode::Equal,
    LeftBracket => KeyCode::BracketLeft,
    RightBracket => KeyCode::BracketRight,
    Backslash => KeyCode::Backslash,
    Semicolon => KeyCode::Semicolon,
    Apostrophe => KeyCode::Quote,
    Slash => KeyCode::Slash,
    Space => KeyCode::Space,
    Enter => KeyCode::Enter,
    Del => KeyCode::Backspace,
    Tab => KeyCode::Tab,
    Escape => KeyCode::Escape,
    ForwardDel => KeyCode::Delete,
    Insert => KeyCode::Insert,
    MoveHome => KeyCode::Home,
    MoveEnd => KeyCode::End,
    PageUp => KeyCode::PageUp,
    PageDown => KeyCode::PageDown,
    CapsLock => KeyCode::CapsLock,
    ScrollLock => KeyCode::ScrollLock,
    NumLock => KeyCode::NumLock,
    // D-pad arrows (device-verified: the pre-completion build reported
    // Unidentified(Ohos(2012/2013)) for real arrow presses).
    DpadUp => KeyCode::ArrowUp,
    DpadDown => KeyCode::ArrowDown,
    DpadLeft => KeyCode::ArrowLeft,
    DpadRight => KeyCode::ArrowRight,
    // Media / volume keys (Camera has no tao KeyCode variant — stays
    // Unidentified(native); Power is mapped below with the completion set).
    VolumeUp => KeyCode::AudioVolumeUp,
    VolumeDown => KeyCode::AudioVolumeDown,
    VolumeMute => KeyCode::AudioVolumeMute,
    MediaPlayPause => KeyCode::MediaPlayPause,
    Home => KeyCode::BrowserHome,
    Back => KeyCode::BrowserBack,
    // Full keycode-table completion (issue #109 follow-up audit, 2026-09-11):
    // every remaining OHOS variant that has a direct tao KeyCode counterpart.
    // Cross-table name notes: OHOS MediaNext/MediaPrevious → tao MediaTrack*;
    // SysRq/Break → PrintScreen/Pause; Sleep/Wakeup → Sleep/WakeUp;
    // KatakanaHiragana → KanaMode (the combined JIS key, per its doc);
    // ZenkakuHankaku → Lang5 (doc: "Japanese word-processing: Zenkaku/Hankaku");
    // Menu → ContextMenu; Calc → LaunchApp2 (doc: "labelled Calculator");
    // Star/Pound → NumpadStar/NumpadHash (phone/remote * and #);
    // NumpadLeftParen/RightParen → NumpadParenLeft/Right; Power → Power.
    // NOT mappable (tao KeyCode has no variant; logical Key may still map):
    // Explorer/Calendar/Spreadsheet/WordProcessor (Launch*), MediaPlay/Pause/
    // Rewind/FastForward/Close/Record, Redo, Brightness*, ZoomIn/Out, New/
    // Exit/Save/SpellCheck, Camera, DpadCenter, Sym — these stay
    // Unidentified(native), which still carries the raw OHOS code. TV/remote
    // codes (Red/Green/Blue/Yellow, Channel*, TV/VCR/DVD, …) likewise.
    SysRq => KeyCode::PrintScreen,
    Break => KeyCode::Pause,
    Function => KeyCode::Fn,
    Power => KeyCode::Power,
    Sleep => KeyCode::Sleep,
    Wakeup => KeyCode::WakeUp,
    Forward => KeyCode::BrowserForward,
    Refresh => KeyCode::BrowserRefresh,
    Bookmarks => KeyCode::BrowserFavorites,
    Menu => KeyCode::ContextMenu,
    Cut => KeyCode::Cut,
    Copy => KeyCode::Copy,
    Paste => KeyCode::Paste,
    Undo => KeyCode::Undo,
    Envelope => KeyCode::LaunchMail,
    Calc => KeyCode::LaunchApp2,
    MediaStop => KeyCode::MediaStop,
    MediaNext => KeyCode::MediaTrackNext,
    MediaPrevious => KeyCode::MediaTrackPrevious,
    MediaEject => KeyCode::Eject,
    Help => KeyCode::Help,
    Star => KeyCode::NumpadStar,
    Pound => KeyCode::NumpadHash,
    NumpadLeftParen => KeyCode::NumpadParenLeft,
    NumpadRightParen => KeyCode::NumpadParenRight,
    // JIS / ISO physical keys (logical Key has no variants — physical
    // identity is still lossless and correct).
    Yen => KeyCode::IntlYen,
    Ro => KeyCode::IntlRo,
    Key102nd => KeyCode::IntlBackslash,
    Muhenkan => KeyCode::NonConvert,
    Henkan => KeyCode::Convert,
    KatakanaHiragana => KeyCode::KanaMode,
    ZenkakuHankaku => KeyCode::Lang5,
    // USB HID / Sun extension keys present on some desktop keyboards.
    Open => KeyCode::Open,
    Find => KeyCode::Find,
    Again => KeyCode::Again,
    Props => KeyCode::Props,
    F1 => KeyCode::F1,
    F2 => KeyCode::F2,
    F3 => KeyCode::F3,
    F4 => KeyCode::F4,
    F5 => KeyCode::F5,
    F6 => KeyCode::F6,
    F7 => KeyCode::F7,
    F8 => KeyCode::F8,
    F9 => KeyCode::F9,
    F10 => KeyCode::F10,
    F11 => KeyCode::F11,
    F12 => KeyCode::F12,
    AltLeft => KeyCode::AltLeft,
    AltRight => KeyCode::AltRight,
    ShiftLeft => KeyCode::ShiftLeft,
    ShiftRight => KeyCode::ShiftRight,
    CtrlLeft => KeyCode::ControlLeft,
    CtrlRight => KeyCode::ControlRight,
    MetaLeft => KeyCode::SuperLeft,
    MetaRight => KeyCode::SuperRight,
    Numpad0 => KeyCode::Numpad0,
    Numpad1 => KeyCode::Numpad1,
    Numpad2 => KeyCode::Numpad2,
    Numpad3 => KeyCode::Numpad3,
    Numpad4 => KeyCode::Numpad4,
    Numpad5 => KeyCode::Numpad5,
    Numpad6 => KeyCode::Numpad6,
    Numpad7 => KeyCode::Numpad7,
    Numpad8 => KeyCode::Numpad8,
    Numpad9 => KeyCode::Numpad9,
    NumpadDivide => KeyCode::NumpadDivide,
    NumpadMultiply => KeyCode::NumpadMultiply,
    NumpadSubtract => KeyCode::NumpadSubtract,
    NumpadAdd => KeyCode::NumpadAdd,
    NumpadDot => KeyCode::NumpadDecimal,
    NumpadComma => KeyCode::NumpadComma,
    NumpadEnter => KeyCode::NumpadEnter,
    NumpadEquals => KeyCode::NumpadEqual,
    _ => KeyCode::Unidentified(NativeKeyCode::Ohos(i32::from(keycode))),
  }
}

pub fn to_location(keycode: Keycode) -> KeyLocation {
  use openharmony_ability::xcomponent::KeyCode::*;

  match keycode {
    AltLeft => KeyLocation::Left,
    AltRight => KeyLocation::Right,
    ShiftLeft => KeyLocation::Left,
    ShiftRight => KeyLocation::Right,

    CtrlLeft => KeyLocation::Left,
    CtrlRight => KeyLocation::Right,
    MetaLeft => KeyLocation::Left,
    MetaRight => KeyLocation::Right,

    NumLock => KeyLocation::Numpad,
    Numpad0 => KeyLocation::Numpad,
    Numpad1 => KeyLocation::Numpad,
    Numpad2 => KeyLocation::Numpad,
    Numpad3 => KeyLocation::Numpad,
    Numpad4 => KeyLocation::Numpad,
    Numpad5 => KeyLocation::Numpad,
    Numpad6 => KeyLocation::Numpad,
    Numpad7 => KeyLocation::Numpad,
    Numpad8 => KeyLocation::Numpad,
    Numpad9 => KeyLocation::Numpad,
    NumpadDivide => KeyLocation::Numpad,
    NumpadMultiply => KeyLocation::Numpad,
    NumpadSubtract => KeyLocation::Numpad,
    NumpadAdd => KeyLocation::Numpad,
    NumpadDot => KeyLocation::Numpad,
    NumpadComma => KeyLocation::Numpad,
    NumpadEnter => KeyLocation::Numpad,
    NumpadEquals => KeyLocation::Numpad,
    NumpadLeftParen => KeyLocation::Numpad,
    NumpadRightParen => KeyLocation::Numpad,

    _ => KeyLocation::Standard,
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use openharmony_ability::xcomponent::KeyCode::*;

  // ─── to_logical: navigation & media ────────────────────────────────────

  #[test]
  fn to_logical_home_maps_to_browser_home() {
    assert!(matches!(to_logical(Home), Key::BrowserHome));
  }

  #[test]
  fn to_logical_back_maps_to_browser_back() {
    assert!(matches!(to_logical(Back), Key::BrowserBack));
  }

  #[test]
  fn to_logical_dpad_navigation() {
    assert!(matches!(to_logical(DpadUp), Key::ArrowUp));
    assert!(matches!(to_logical(DpadDown), Key::ArrowDown));
    assert!(matches!(to_logical(DpadLeft), Key::ArrowLeft));
    assert!(matches!(to_logical(DpadRight), Key::ArrowRight));
    assert!(matches!(to_logical(DpadCenter), Key::Enter));
  }

  #[test]
  fn to_logical_volume_keys() {
    assert!(matches!(to_logical(VolumeUp), Key::AudioVolumeUp));
    assert!(matches!(to_logical(VolumeDown), Key::AudioVolumeDown));
    assert!(matches!(to_logical(VolumeMute), Key::AudioVolumeMute));
  }

  #[test]
  fn to_logical_media_keys() {
    assert!(matches!(to_logical(MediaPlayPause), Key::MediaPlayPause));
    assert!(matches!(to_logical(MediaStop), Key::MediaStop));
    assert!(matches!(to_logical(MediaNext), Key::MediaTrackNext));
    assert!(matches!(to_logical(MediaPrevious), Key::MediaTrackPrevious));
    assert!(matches!(to_logical(MediaRewind), Key::MediaRewind));
    assert!(matches!(
      to_logical(MediaFastForward),
      Key::MediaFastForward
    ));
    assert!(matches!(to_logical(MediaPlay), Key::MediaPlay));
    assert!(matches!(to_logical(MediaPause), Key::MediaPause));
    assert!(matches!(to_logical(MediaClose), Key::MediaClose));
    assert!(matches!(to_logical(MediaEject), Key::Eject));
    assert!(matches!(to_logical(MediaRecord), Key::MediaRecord));
  }

  #[test]
  fn to_logical_mute_maps_to_microphone_volume_mute() {
    assert!(matches!(to_logical(Mute), Key::MicrophoneVolumeMute));
  }

  // ─── to_logical: modifier keys ────────────────────────────────────────

  #[test]
  fn to_logical_alt_keys() {
    assert!(matches!(to_logical(AltLeft), Key::Alt));
    assert!(matches!(to_logical(AltRight), Key::Alt));
  }

  #[test]
  fn to_logical_shift_keys() {
    assert!(matches!(to_logical(ShiftLeft), Key::Shift));
    assert!(matches!(to_logical(ShiftRight), Key::Shift));
  }

  #[test]
  fn to_logical_ctrl_keys() {
    assert!(matches!(to_logical(CtrlLeft), Key::Control));
    assert!(matches!(to_logical(CtrlRight), Key::Control));
  }

  #[test]
  fn to_logical_meta_keys() {
    assert!(matches!(to_logical(MetaLeft), Key::Super));
    assert!(matches!(to_logical(MetaRight), Key::Super));
  }

  #[test]
  fn to_logical_caps_scroll_num_lock() {
    assert!(matches!(to_logical(CapsLock), Key::CapsLock));
    assert!(matches!(to_logical(ScrollLock), Key::ScrollLock));
    assert!(matches!(to_logical(NumLock), Key::NumLock));
  }

  // ─── to_logical: common keys ──────────────────────────────────────────

  #[test]
  fn to_logical_tab_space_enter() {
    assert!(matches!(to_logical(Tab), Key::Tab));
    assert!(matches!(to_logical(Space), Key::Space));
    assert!(matches!(to_logical(Enter), Key::Enter));
  }

  #[test]
  fn to_logical_del_maps_to_backspace() {
    assert!(matches!(to_logical(Del), Key::Backspace));
  }

  #[test]
  fn to_logical_forward_del_maps_to_delete() {
    assert!(matches!(to_logical(ForwardDel), Key::Delete));
  }

  #[test]
  fn to_logical_escape() {
    assert!(matches!(to_logical(Escape), Key::Escape));
  }

  #[test]
  fn to_logical_function_key() {
    assert!(matches!(to_logical(Function), Key::Fn));
  }

  // ─── to_logical: F-keys ───────────────────────────────────────────────

  #[test]
  fn to_logical_f1_through_f12() {
    assert!(matches!(to_logical(F1), Key::F1));
    assert!(matches!(to_logical(F2), Key::F2));
    assert!(matches!(to_logical(F3), Key::F3));
    assert!(matches!(to_logical(F4), Key::F4));
    assert!(matches!(to_logical(F5), Key::F5));
    assert!(matches!(to_logical(F6), Key::F6));
    assert!(matches!(to_logical(F7), Key::F7));
    assert!(matches!(to_logical(F8), Key::F8));
    assert!(matches!(to_logical(F9), Key::F9));
    assert!(matches!(to_logical(F10), Key::F10));
    assert!(matches!(to_logical(F11), Key::F11));
    assert!(matches!(to_logical(F12), Key::F12));
  }

  // ─── to_logical: page nav & editing ───────────────────────────────────

  #[test]
  fn to_logical_page_up_down() {
    assert!(matches!(to_logical(PageUp), Key::PageUp));
    assert!(matches!(to_logical(PageDown), Key::PageDown));
  }

  #[test]
  fn to_logical_move_home_end_insert() {
    assert!(matches!(to_logical(MoveHome), Key::Home));
    assert!(matches!(to_logical(MoveEnd), Key::End));
    assert!(matches!(to_logical(Insert), Key::Insert));
  }

  #[test]
  fn to_logical_forward_maps_to_browser_forward() {
    assert!(matches!(to_logical(Forward), Key::BrowserForward));
  }

  #[test]
  fn to_logical_sysrq_break() {
    assert!(matches!(to_logical(SysRq), Key::PrintScreen));
    assert!(matches!(to_logical(Break), Key::Pause));
  }

  // ─── to_logical: special buttons ──────────────────────────────────────

  #[test]
  fn to_logical_power_camera() {
    assert!(matches!(to_logical(Power), Key::Power));
    assert!(matches!(to_logical(Camera), Key::Camera));
  }

  #[test]
  fn to_logical_explorer_envelope() {
    assert!(matches!(to_logical(Explorer), Key::LaunchWebBrowser));
    assert!(matches!(to_logical(Envelope), Key::LaunchMail));
  }

  #[test]
  fn to_logical_sym_maps_to_symbol() {
    assert!(matches!(to_logical(Sym), Key::Symbol));
  }

  // ─── to_logical: clipboard & refresh ──────────────────────────────────

  #[test]
  fn to_logical_cut_copy_paste() {
    assert!(matches!(to_logical(Cut), Key::Cut));
    assert!(matches!(to_logical(Copy), Key::Copy));
    assert!(matches!(to_logical(Paste), Key::Paste));
  }

  #[test]
  fn to_logical_refresh_maps_to_browser_refresh() {
    assert!(matches!(to_logical(Refresh), Key::BrowserRefresh));
  }

  // ─── to_logical: TV & brightness ─────────────────────────────────────

  #[test]
  fn to_logical_tv_keys() {
    assert!(matches!(to_logical(TV), Key::TV));
    assert!(matches!(to_logical(ChannelUp), Key::ChannelUp));
    assert!(matches!(to_logical(ChannelDown), Key::ChannelDown));
  }

  #[test]
  fn to_logical_zoom_keys() {
    assert!(matches!(to_logical(ZoomIn), Key::ZoomIn));
    assert!(matches!(to_logical(ZoomOut), Key::ZoomOut));
  }

  #[test]
  fn to_logical_brightness_keys() {
    assert!(matches!(to_logical(BrightnessDown), Key::BrightnessDown));
    assert!(matches!(to_logical(BrightnessUp), Key::BrightnessUp));
  }

  #[test]
  fn to_logical_info() {
    assert!(matches!(to_logical(Info), Key::Info));
  }

  // ─── to_logical: Japanese keys ───────────────────────────────────────

  #[test]
  fn to_logical_japanese_keys() {
    assert!(matches!(to_logical(ZenkakuHankaku), Key::ZenkakuHankaku));
    assert!(matches!(to_logical(Muhenkan), Key::NonConvert));
    assert!(matches!(to_logical(Henkan), Key::Convert));
    assert!(matches!(
      to_logical(KatakanaHiragana),
      Key::HiraganaKatakana
    ));
  }

  // ─── to_logical: calendar & sleep ────────────────────────────────────

  #[test]
  fn to_logical_calendar_maps_to_launch_calendar() {
    assert!(matches!(to_logical(Calendar), Key::LaunchCalendar));
  }

  #[test]
  fn to_logical_sleep_wakeup() {
    assert!(matches!(to_logical(Sleep), Key::Standby));
    assert!(matches!(to_logical(Wakeup), Key::WakeUp));
  }

  #[test]
  fn to_logical_help() {
    assert!(matches!(to_logical(Help), Key::Help));
  }

  // ─── to_logical: unidentified fallbacks ────────────────────────────────

  #[test]
  fn to_logical_unknown_maps_to_unidentified() {
    let result = to_logical(Unknown);
    assert!(matches!(result, Key::Unidentified(_)));
  }

  #[test]
  fn to_logical_numpad_keys_map_characters() {
    // Numpad digits/operators map to their produced character (NumLock on);
    // to_location still reports KeyLocation::Numpad for disambiguation.
    let cases = [
      (Numpad0, "0"),
      (Numpad1, "1"),
      (Numpad2, "2"),
      (Numpad3, "3"),
      (Numpad4, "4"),
      (Numpad5, "5"),
      (Numpad6, "6"),
      (Numpad7, "7"),
      (Numpad8, "8"),
      (Numpad9, "9"),
      (NumpadDivide, "/"),
      (NumpadMultiply, "*"),
      (NumpadSubtract, "-"),
      (NumpadAdd, "+"),
      (NumpadDot, "."),
      (NumpadComma, ","),
      (NumpadEquals, "="),
      (NumpadLeftParen, "("),
      (NumpadRightParen, ")"),
    ];
    for (kc, expected) in cases {
      assert!(
        matches!(to_logical(kc), Key::Character(ref c) if *c == expected),
        "numpad key {kc:?} should map to Character({expected:?})"
      );
    }
    assert!(matches!(to_logical(NumpadEnter), Key::Enter));
  }

  #[test]
  fn to_logical_alpha_keys_map_characters() {
    // Letters/digits map to their base (unshifted) character — issue #109:
    // the Android-style Unicode path doesn't exist on the OHOS NDK, so the
    // keycode mapping is the only source.
    let cases = [
      (A, "a"),
      (Z, "z"),
      (Key0, "0"),
      (Key9, "9"),
      (Star, "*"),
      (Pound, "#"),
    ];
    for (kc, expected) in cases {
      assert!(
        matches!(to_logical(kc), Key::Character(ref c) if *c == expected),
        "key {kc:?} should map to Character({expected:?})"
      );
    }
  }

  #[test]
  fn to_logical_punctuation_keys_map_characters() {
    let cases = [
      (Comma, ","),
      (Period, "."),
      (Grave, "`"),
      (Minus, "-"),
      (Equals, "="),
      (LeftBracket, "["),
      (RightBracket, "]"),
      (Backslash, "\\"),
      (Semicolon, ";"),
      (Apostrophe, "'"),
      (Slash, "/"),
      (At, "@"),
      (Plus, "+"),
    ];
    for (kc, expected) in cases {
      assert!(
        matches!(to_logical(kc), Key::Character(ref c) if *c == expected),
        "key {kc:?} should map to Character({expected:?})"
      );
    }
  }

  #[test]
  fn to_logical_yen_ro_are_unidentified() {
    assert!(matches!(to_logical(Yen), Key::Unidentified(_)));
    assert!(matches!(to_logical(Ro), Key::Unidentified(_)));
  }

  #[test]
  fn to_logical_menu_is_unidentified() {
    assert!(matches!(to_logical(Menu), Key::Unidentified(_)));
  }

  // ─── to_location: modifier location ───────────────────────────────────

  #[test]
  fn to_location_left_modifiers() {
    assert_eq!(to_location(AltLeft), KeyLocation::Left);
    assert_eq!(to_location(ShiftLeft), KeyLocation::Left);
    assert_eq!(to_location(CtrlLeft), KeyLocation::Left);
    assert_eq!(to_location(MetaLeft), KeyLocation::Left);
  }

  #[test]
  fn to_location_right_modifiers() {
    assert_eq!(to_location(AltRight), KeyLocation::Right);
    assert_eq!(to_location(ShiftRight), KeyLocation::Right);
    assert_eq!(to_location(CtrlRight), KeyLocation::Right);
    assert_eq!(to_location(MetaRight), KeyLocation::Right);
  }

  #[test]
  fn to_location_numpad_keys() {
    for kc in [
      NumLock,
      Numpad0,
      Numpad1,
      Numpad2,
      Numpad3,
      Numpad4,
      Numpad5,
      Numpad6,
      Numpad7,
      Numpad8,
      Numpad9,
      NumpadDivide,
      NumpadMultiply,
      NumpadSubtract,
      NumpadAdd,
      NumpadDot,
      NumpadComma,
      NumpadEnter,
      NumpadEquals,
      NumpadLeftParen,
      NumpadRightParen,
    ] {
      assert_eq!(
        to_location(kc),
        KeyLocation::Numpad,
        "numpad key should be Numpad location"
      );
    }
  }

  #[test]
  fn to_location_non_modifier_non_numpad_is_standard() {
    // Regular keys (letters, digits, F-keys, arrows, etc.) are Standard
    for kc in [
      A, B, Key0, Key9, F1, F12, DpadUp, DpadDown, Space, Tab, Enter, Escape, VolumeUp, Home,
      PageUp,
    ] {
      assert_eq!(
        to_location(kc),
        KeyLocation::Standard,
        "regular key should be Standard location"
      );
    }
  }

  // ─── to_physical: physical KeyCode mapping (issue #109) ─────────────────

  #[test]
  fn to_physical_letters_digits_and_punctuation() {
    assert_eq!(to_physical(A), KeyCode::KeyA);
    assert_eq!(to_physical(Z), KeyCode::KeyZ);
    assert_eq!(to_physical(Key0), KeyCode::Digit0);
    assert_eq!(to_physical(Key9), KeyCode::Digit9);
    assert_eq!(to_physical(Grave), KeyCode::Backquote);
    assert_eq!(to_physical(Apostrophe), KeyCode::Quote);
    assert_eq!(to_physical(Equals), KeyCode::Equal);
    assert_eq!(to_physical(Del), KeyCode::Backspace);
    assert_eq!(to_physical(MoveHome), KeyCode::Home);
  }

  #[test]
  fn to_physical_modifiers_have_left_right_identity() {
    assert_eq!(to_physical(AltLeft), KeyCode::AltLeft);
    assert_eq!(to_physical(AltRight), KeyCode::AltRight);
    assert_eq!(to_physical(ShiftLeft), KeyCode::ShiftLeft);
    assert_eq!(to_physical(ShiftRight), KeyCode::ShiftRight);
    assert_eq!(to_physical(CtrlLeft), KeyCode::ControlLeft);
    assert_eq!(to_physical(CtrlRight), KeyCode::ControlRight);
    assert_eq!(to_physical(MetaLeft), KeyCode::SuperLeft);
    assert_eq!(to_physical(MetaRight), KeyCode::SuperRight);
  }

  #[test]
  fn to_physical_numpad_keys() {
    assert_eq!(to_physical(Numpad0), KeyCode::Numpad0);
    assert_eq!(to_physical(Numpad9), KeyCode::Numpad9);
    assert_eq!(to_physical(NumpadEnter), KeyCode::NumpadEnter);
    assert_eq!(to_physical(NumpadDot), KeyCode::NumpadDecimal);
    assert_eq!(to_physical(NumpadEquals), KeyCode::NumpadEqual);
    assert_eq!(to_physical(NumpadDivide), KeyCode::NumpadDivide);
  }

  #[test]
  fn to_physical_arrows_and_media_keys() {
    // Device-verified gap fill (2026-09-11): real arrow presses arrived as
    // Unidentified(Ohos(2012/2013)) before these mappings existed.
    assert_eq!(to_physical(DpadUp), KeyCode::ArrowUp);
    assert_eq!(to_physical(DpadDown), KeyCode::ArrowDown);
    assert_eq!(to_physical(DpadLeft), KeyCode::ArrowLeft);
    assert_eq!(to_physical(DpadRight), KeyCode::ArrowRight);
    assert_eq!(to_physical(VolumeDown), KeyCode::AudioVolumeDown);
    assert_eq!(to_physical(VolumeUp), KeyCode::AudioVolumeUp);
    assert_eq!(to_physical(VolumeMute), KeyCode::AudioVolumeMute);
    assert_eq!(to_physical(MediaPlayPause), KeyCode::MediaPlayPause);
    assert_eq!(to_physical(Home), KeyCode::BrowserHome);
    assert_eq!(to_physical(Back), KeyCode::BrowserBack);
  }

  #[test]
  fn to_physical_completion_set() {
    // Cross-table name remaps from the 2026-09-11 full-table audit.
    assert_eq!(to_physical(SysRq), KeyCode::PrintScreen);
    assert_eq!(to_physical(Break), KeyCode::Pause);
    assert_eq!(to_physical(Function), KeyCode::Fn);
    assert_eq!(to_physical(Sleep), KeyCode::Sleep);
    assert_eq!(to_physical(Wakeup), KeyCode::WakeUp);
    assert_eq!(to_physical(MediaNext), KeyCode::MediaTrackNext);
    assert_eq!(to_physical(MediaPrevious), KeyCode::MediaTrackPrevious);
    assert_eq!(to_physical(MediaEject), KeyCode::Eject);
    assert_eq!(to_physical(Envelope), KeyCode::LaunchMail);
    assert_eq!(to_physical(Menu), KeyCode::ContextMenu);
    assert_eq!(to_physical(Star), KeyCode::NumpadStar);
    assert_eq!(to_physical(Pound), KeyCode::NumpadHash);
    assert_eq!(to_physical(NumpadLeftParen), KeyCode::NumpadParenLeft);
    assert_eq!(to_physical(NumpadRightParen), KeyCode::NumpadParenRight);
    // JIS / ISO.
    assert_eq!(to_physical(Yen), KeyCode::IntlYen);
    assert_eq!(to_physical(Ro), KeyCode::IntlRo);
    assert_eq!(to_physical(Key102nd), KeyCode::IntlBackslash);
    assert_eq!(to_physical(Muhenkan), KeyCode::NonConvert);
    assert_eq!(to_physical(Henkan), KeyCode::Convert);
    assert_eq!(to_physical(KatakanaHiragana), KeyCode::KanaMode);
    assert_eq!(to_physical(ZenkakuHankaku), KeyCode::Lang5);
    // Sun / HID extension keys.
    assert_eq!(to_physical(Open), KeyCode::Open);
    assert_eq!(to_physical(Find), KeyCode::Find);
    assert_eq!(to_physical(Again), KeyCode::Again);
    assert_eq!(to_physical(Props), KeyCode::Props);
    assert_eq!(to_physical(Undo), KeyCode::Undo);
    assert_eq!(to_physical(Copy), KeyCode::Copy);
    assert_eq!(to_physical(Bookmarks), KeyCode::BrowserFavorites);
    assert_eq!(to_physical(Calc), KeyCode::LaunchApp2);
  }

  #[test]
  fn to_physical_unmapped_falls_back_to_native_unidentified() {
    // Keys without a tao KeyCode mapping keep the raw OHOS code attached.
    assert!(matches!(
      to_physical(Unknown),
      KeyCode::Unidentified(NativeKeyCode::Ohos(_))
    ));
  }
}
