use winit::event::{VirtualKeyCode, ScanCode};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum KeyCode {
    Esc,

    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,

    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    PrintScreen,
    ScrollLock,
    /// Pause or Break
    Pause,

    /// ` or ~
    Grave,
    /// - or _
    Minus,
    /// = or +
    Equals,
    /// [ or {
    LBracket,
    /// ] or }
    RBracket,
    /// \ or |
    Backslash,
    /// ; or :
    Semicolon,
    /// ' or "
    Apostrophe,
    /// , or <
    Comma,
    /// . or >
    Period,
    /// / or ?
    Slash,

    Tab,
    CapsLock,
    LShift,
    RShift,
    LCtrl,
    RCtrl,
    /// or left option on macOS
    LAlt,
    /// or right option on macOs
    RAlt,
    /// or left command on macOS
    LWin,
    /// or right command on macOS
    RWin,
    /// or delete on macOS
    Backspace,
    /// or return on macOS
    Enter,
    Space,
    Menu,

    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,

    Up,
    Down,
    Left,
    Right,

    NumLock,
    NumpadSlash,
    NumpadMultiply,
    NumpadMinus,
    NumpadDecimal,
    NumpadAdd,
    NumpadEnter,
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

    Other(u32),
}

impl From<(Option<VirtualKeyCode>, ScanCode)> for KeyCode {

    fn from((key_code, scan_code): (Option<VirtualKeyCode>, ScanCode)) -> Self {
        match key_code {
            Some(key_code) => {
                match key_code {
                    VirtualKeyCode::Key1 => KeyCode::Num1,
                    VirtualKeyCode::Key2 => KeyCode::Num2,
                    VirtualKeyCode::Key3 => KeyCode::Num3,
                    VirtualKeyCode::Key4 => KeyCode::Num4,
                    VirtualKeyCode::Key5 => KeyCode::Num5,
                    VirtualKeyCode::Key6 => KeyCode::Num6,
                    VirtualKeyCode::Key7 => KeyCode::Num7,
                    VirtualKeyCode::Key8 => KeyCode::Num8,
                    VirtualKeyCode::Key9 => KeyCode::Num9,
                    VirtualKeyCode::Key0 => KeyCode::Num0,
                    VirtualKeyCode::A => KeyCode::A,
                    VirtualKeyCode::B => KeyCode::B,
                    VirtualKeyCode::C => KeyCode::C,
                    VirtualKeyCode::D => KeyCode::D,
                    VirtualKeyCode::E => KeyCode::E,
                    VirtualKeyCode::F => KeyCode::F,
                    VirtualKeyCode::G => KeyCode::G,
                    VirtualKeyCode::H => KeyCode::H,
                    VirtualKeyCode::I => KeyCode::I,
                    VirtualKeyCode::J => KeyCode::J,
                    VirtualKeyCode::K => KeyCode::K,
                    VirtualKeyCode::L => KeyCode::L,
                    VirtualKeyCode::M => KeyCode::M,
                    VirtualKeyCode::N => KeyCode::N,
                    VirtualKeyCode::O => KeyCode::O,
                    VirtualKeyCode::P => KeyCode::P,
                    VirtualKeyCode::Q => KeyCode::Q,
                    VirtualKeyCode::R => KeyCode::R,
                    VirtualKeyCode::S => KeyCode::S,
                    VirtualKeyCode::T => KeyCode::T,
                    VirtualKeyCode::U => KeyCode::U,
                    VirtualKeyCode::V => KeyCode::V,
                    VirtualKeyCode::W => KeyCode::W,
                    VirtualKeyCode::X => KeyCode::X,
                    VirtualKeyCode::Y => KeyCode::Y,
                    VirtualKeyCode::Z => KeyCode::Z,
                    VirtualKeyCode::Escape => KeyCode::Esc,
                    VirtualKeyCode::F1 => KeyCode::F1,
                    VirtualKeyCode::F2 => KeyCode::F2,
                    VirtualKeyCode::F3 => KeyCode::F3,
                    VirtualKeyCode::F4 => KeyCode::F4,
                    VirtualKeyCode::F5 => KeyCode::F5,
                    VirtualKeyCode::F6 => KeyCode::F6,
                    VirtualKeyCode::F7 => KeyCode::F7,
                    VirtualKeyCode::F8 => KeyCode::F8,
                    VirtualKeyCode::F9 => KeyCode::F9,
                    VirtualKeyCode::F10 => KeyCode::F10,
                    VirtualKeyCode::F11 => KeyCode::F11,
                    VirtualKeyCode::F12 => KeyCode::F12,
                    VirtualKeyCode::F13 => KeyCode::F13,
                    VirtualKeyCode::F14 => KeyCode::F14,
                    VirtualKeyCode::F15 => KeyCode::F15,
                    VirtualKeyCode::F16 => KeyCode::F16,
                    VirtualKeyCode::F17 => KeyCode::F17,
                    VirtualKeyCode::F18 => KeyCode::F18,
                    VirtualKeyCode::F19 => KeyCode::F19,
                    VirtualKeyCode::F20 => KeyCode::F20,
                    VirtualKeyCode::F21 => KeyCode::F21,
                    VirtualKeyCode::F22 => KeyCode::F22,
                    VirtualKeyCode::F23 => KeyCode::F23,
                    VirtualKeyCode::F24 => KeyCode::F24,
                    VirtualKeyCode::Snapshot => KeyCode::PrintScreen,
                    VirtualKeyCode::Scroll => KeyCode::ScrollLock,
                    VirtualKeyCode::Pause => KeyCode::Pause,
                    VirtualKeyCode::Insert => KeyCode::Insert,
                    VirtualKeyCode::Home => KeyCode::Home,
                    VirtualKeyCode::Delete => KeyCode::Delete,
                    VirtualKeyCode::End => KeyCode::End,
                    VirtualKeyCode::PageDown => KeyCode::PageDown,
                    VirtualKeyCode::PageUp => KeyCode::PageUp,
                    VirtualKeyCode::Left => KeyCode::Left,
                    VirtualKeyCode::Up => KeyCode::Up,
                    VirtualKeyCode::Right => KeyCode::Right,
                    VirtualKeyCode::Down => KeyCode::Down,
                    VirtualKeyCode::Back => KeyCode::Backspace,
                    VirtualKeyCode::Return => KeyCode::Enter,
                    VirtualKeyCode::Space => KeyCode::Space,
                    VirtualKeyCode::Compose => KeyCode::Other(scan_code),
                    VirtualKeyCode::Caret => KeyCode::Other(scan_code),
                    VirtualKeyCode::Numlock => KeyCode::NumLock,
                    VirtualKeyCode::Numpad0 => KeyCode::Numpad0,
                    VirtualKeyCode::Numpad1 => KeyCode::Numpad1,
                    VirtualKeyCode::Numpad2 => KeyCode::Numpad2,
                    VirtualKeyCode::Numpad3 => KeyCode::Numpad3,
                    VirtualKeyCode::Numpad4 => KeyCode::Numpad4,
                    VirtualKeyCode::Numpad5 => KeyCode::Numpad5,
                    VirtualKeyCode::Numpad6 => KeyCode::Numpad6,
                    VirtualKeyCode::Numpad7 => KeyCode::Numpad7,
                    VirtualKeyCode::Numpad8 => KeyCode::Numpad8,
                    VirtualKeyCode::Numpad9 => KeyCode::Numpad9,
                    VirtualKeyCode::AbntC1 => KeyCode::Other(scan_code),
                    VirtualKeyCode::AbntC2 => KeyCode::Other(scan_code),
                    VirtualKeyCode::Add => KeyCode::NumpadAdd,
                    VirtualKeyCode::Apostrophe => KeyCode::Apostrophe,
                    VirtualKeyCode::Apps => KeyCode::Menu,
                    VirtualKeyCode::At => KeyCode::Other(scan_code),
                    VirtualKeyCode::Ax => KeyCode::Other(scan_code),
                    VirtualKeyCode::Backslash => KeyCode::Backslash,
                    VirtualKeyCode::Calculator => KeyCode::Other(scan_code),
                    VirtualKeyCode::Capital => KeyCode::CapsLock,
                    VirtualKeyCode::Colon => KeyCode::Other(scan_code),
                    VirtualKeyCode::Comma => KeyCode::Comma,
                    VirtualKeyCode::Convert => KeyCode::Other(scan_code),
                    VirtualKeyCode::Decimal => KeyCode::NumpadDecimal,
                    VirtualKeyCode::Divide => KeyCode::NumpadSlash,
                    VirtualKeyCode::Equals => KeyCode::Equals,
                    VirtualKeyCode::Grave => KeyCode::Grave,
                    VirtualKeyCode::Kana => KeyCode::Other(scan_code),
                    VirtualKeyCode::Kanji => KeyCode::Other(scan_code),
                    VirtualKeyCode::LAlt => KeyCode::LAlt,
                    VirtualKeyCode::LBracket => KeyCode::LBracket,
                    VirtualKeyCode::LControl => KeyCode::LCtrl,
                    VirtualKeyCode::LShift => KeyCode::LShift,
                    VirtualKeyCode::LWin => KeyCode::LWin,
                    VirtualKeyCode::Mail => KeyCode::Other(scan_code),
                    VirtualKeyCode::MediaSelect => KeyCode::Other(scan_code),
                    VirtualKeyCode::MediaStop => KeyCode::Other(scan_code),
                    VirtualKeyCode::Minus => KeyCode::Minus,
                    VirtualKeyCode::Multiply => KeyCode::NumpadMultiply,
                    VirtualKeyCode::Mute => KeyCode::Other(scan_code),
                    VirtualKeyCode::MyComputer => KeyCode::Other(scan_code),
                    VirtualKeyCode::NavigateForward => KeyCode::Other(scan_code),
                    VirtualKeyCode::NavigateBackward => KeyCode::Other(scan_code),
                    VirtualKeyCode::NextTrack => KeyCode::Other(scan_code),
                    VirtualKeyCode::NoConvert => KeyCode::Other(scan_code),
                    VirtualKeyCode::NumpadComma => KeyCode::Other(scan_code),
                    VirtualKeyCode::NumpadEnter => KeyCode::NumpadEnter,
                    VirtualKeyCode::NumpadEquals => KeyCode::Other(scan_code),
                    VirtualKeyCode::OEM102 => KeyCode::Other(scan_code),
                    VirtualKeyCode::Period => KeyCode::Period,
                    VirtualKeyCode::PlayPause => KeyCode::Other(scan_code),
                    VirtualKeyCode::Power => KeyCode::Other(scan_code),
                    VirtualKeyCode::PrevTrack => KeyCode::Other(scan_code),
                    VirtualKeyCode::RAlt => KeyCode::RAlt,
                    VirtualKeyCode::RBracket => KeyCode::RBracket,
                    VirtualKeyCode::RControl => KeyCode::RCtrl,
                    VirtualKeyCode::RShift => KeyCode::RShift,
                    VirtualKeyCode::RWin => KeyCode::RWin,
                    VirtualKeyCode::Semicolon => KeyCode::Semicolon,
                    VirtualKeyCode::Slash => KeyCode::Slash,
                    VirtualKeyCode::Sleep => KeyCode::Other(scan_code),
                    VirtualKeyCode::Stop => KeyCode::Other(scan_code),
                    VirtualKeyCode::Subtract => KeyCode::NumpadMinus,
                    VirtualKeyCode::Sysrq => KeyCode::Other(scan_code),
                    VirtualKeyCode::Tab => KeyCode::Tab,
                    VirtualKeyCode::Underline => KeyCode::Other(scan_code),
                    VirtualKeyCode::Unlabeled => KeyCode::Other(scan_code),
                    VirtualKeyCode::VolumeDown => KeyCode::Other(scan_code),
                    VirtualKeyCode::VolumeUp => KeyCode::Other(scan_code),
                    VirtualKeyCode::Wake => KeyCode::Other(scan_code),
                    VirtualKeyCode::WebBack => KeyCode::Other(scan_code),
                    VirtualKeyCode::WebFavorites => KeyCode::Other(scan_code),
                    VirtualKeyCode::WebForward => KeyCode::Other(scan_code),
                    VirtualKeyCode::WebHome => KeyCode::Other(scan_code),
                    VirtualKeyCode::WebRefresh => KeyCode::Other(scan_code),
                    VirtualKeyCode::WebSearch => KeyCode::Other(scan_code),
                    VirtualKeyCode::WebStop => KeyCode::Other(scan_code),
                    VirtualKeyCode::Yen => KeyCode::Other(scan_code),
                    VirtualKeyCode::Copy => KeyCode::Other(scan_code),
                    VirtualKeyCode::Paste => KeyCode::Other(scan_code),
                    VirtualKeyCode::Cut => KeyCode::Other(scan_code),
                }
            }
            None => KeyCode::Other(scan_code),
        }
    }

}
