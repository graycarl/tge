
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum WrapMode {
    Repeat,
    MirroredRepeat,
    ClampToEdge,
    MirrorClampToEdge,
    ClampToBorder,
}

impl WrapMode {

    pub(crate) fn to_flag(&self) -> u32 {
        match self {
            WrapMode::Repeat => glow::REPEAT,
            WrapMode::MirroredRepeat => glow::MIRRORED_REPEAT,
            WrapMode::ClampToEdge => glow::CLAMP_TO_EDGE,
            WrapMode::MirrorClampToEdge => glow::MIRROR_CLAMP_TO_EDGE,
            WrapMode::ClampToBorder => glow::CLAMP_TO_BORDER,
        }
    }

}

impl Default for WrapMode {

    fn default() -> Self {
        WrapMode::Repeat
    }

}

#[derive(Debug, Default, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Wrap {
    pub horizontal: WrapMode,
    pub vertical: WrapMode,
    pub depth: WrapMode,
}

impl Wrap {

    pub fn new(horizontal: WrapMode, vertical: WrapMode, depth: WrapMode) -> Self {
        Self { horizontal, vertical, depth }
    }

    pub fn uv(horizontal: WrapMode, vertical: WrapMode) -> Self {
        Self::new(horizontal, vertical, WrapMode::default())
    }

}
