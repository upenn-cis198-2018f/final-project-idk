const MAX_KEYS: u16 = 112;

// Unknown key string
const UK: &'static str = "<UK>";

pub const KEY_NAMES: [&'static str; MAX_KEYS as usize] = [
    UK, "<ESC>",
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "=",
    "<Backspace>", "<Tab>",
    "q", "w", "e", "r", "t", "y", "u", "i", "o", "p",
    "[", "]", "<Enter>", "<LCtrl>",
    "a", "s", "d", "f", "g", "h", "j", "k", "l", ";",
    "'", "`", "<LShift>",
    "\\", "z", "x", "c", "v", "b", "n", "m", ",", ".", "/",
    "<RShift>",
    "<KP*>",
    "<LAlt>", " ", "<CapsLock>",
    "<F1>", "<F2>", "<F3>", "<F4>", "<F5>", "<F6>", "<F7>", "<F8>", "<F9>", "<F10>",
    "<NumLock>", "<ScrollLock>",
    "<KP7>", "<KP8>", "<KP9>",
    "<KP->",
    "<KP4>", "<KP5>", "<KP6>",
    "<KP+>",
    "<KP1>", "<KP2>", "<KP3>", "<KP0>",
    "<KP.>",
    UK, UK, UK,
    "<F11>", "<F12>",
    UK, UK, UK, UK, UK, UK, UK,
    "<KPEnter>", "<RCtrl>", "<KP/>", "<SysRq>", "<RAlt>", UK,
    "<Home>", "<Up>", "<PageUp>", "<Left>", "<Right>", "<End>", "<Down>",
    "<PageDown>", "<Insert>", "<Delete>"
];
