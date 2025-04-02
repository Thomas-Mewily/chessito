
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Grey,
}

pub enum Layer {
    Foreground,
    Background,
}


pub fn get(color: Color) -> String { get_with_layer(color, Layer::Foreground) }

pub fn get_with_layer(color: Color, layer: Layer) -> String 
{
    let code = match color {
        Color::Black   => 30,
        Color::Red     => 31,
        Color::Green   => 32,
        Color::Yellow  => 33,
        Color::Blue    => 34,
        Color::Magenta => 35,
        Color::Cyan    => 36,
        Color::White   => 37,
        Color::Grey    => 90,
    };

    let layer_code = match layer {
        Layer::Foreground => 0,
        Layer::Background => 10,
    };

    format!("\x1b[{}m", code + layer_code)
}

#[allow(non_camel_case_types)]
pub type str_color = str;

pub const BLACK_FOREGROUND  : &str_color = "\x1b[30m";
pub const RED_FOREGROUND    : &str_color = "\x1b[31m";
pub const GREEN_FOREGROUND  : &str_color = "\x1b[32m";
pub const YELLOW_FOREGROUND : &str_color = "\x1b[33m";
pub const BLUE_FOREGROUND   : &str_color = "\x1b[34m";
pub const MAGENTA_FOREGROUND: &str_color = "\x1b[35m";
pub const CYAN_FOREGROUND   : &str_color = "\x1b[36m";
pub const WHITE_FOREGROUND  : &str_color = "\x1b[37m";
pub const GREY_FOREGROUND   : &str_color = "\x1b[90m";

pub const BLACK_BACKGROUND  : &str_color = "\x1b[40m";
pub const RED_BACKGROUND    : &str_color = "\x1b[41m";
pub const GREEN_BACKGROUND  : &str_color = "\x1b[42m";
pub const YELLOW_BACKGROUND : &str_color = "\x1b[43m";
pub const BLUE_BACKGROUND   : &str_color = "\x1b[44m";
pub const MAGENTA_BACKGROUND: &str_color = "\x1b[45m";
pub const CYAN_BACKGROUND   : &str_color = "\x1b[46m";
pub const WHITE_BACKGROUND  : &str_color = "\x1b[47m";
pub const GREY_BACKGROUND   : &str_color = "\x1b[100m";

pub const COLOR_TITLE: &str_color = "\x1b[35m";

pub const COLOR_ERROR:   &str_color           = "\x1b[31m";
pub const COLOR_HIGHLIGHT_ERROR:   &str_color = "\x1b[47m\x1b[41m";

pub const COLOR_SUCCESS: &str_color           = "\x1b[32m";
pub const COLOR_HIGHLIGHT_SUCCESS: &str_color = "\x1b[47m\x1b[42m";

pub const COLOR_WARNING: &str_color           = "\x1b[33m";
pub const COLOR_HIGHLIGHT_WARNING: &str_color = "\x1b[47m\x1b[43m";

pub const COLOR_INFO: &str_color           = "\x1b[36m";
pub const COLOR_HIGHLIGHT_INFO: &str_color = "\x1b[47m\x1b[46m";

pub const COLOR_BLACK_ON_WHITE : &str_color   = "\x1b[30m\x1b[47m";

pub const COLOR_RESET: &str_color = "\x1b[37m\x1b[40m";
pub const COLOR_RESET_FOREGROUND : &str_color = "\x1b[37m";
pub const COLOR_RESET_BACKGROUND : &str_color = "\x1b[40m";
