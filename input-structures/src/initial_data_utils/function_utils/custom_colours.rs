use std::fmt::{Formatter , Debug, Result};
pub struct PrimaryColor {
    pub foreground_color: ForegroundColor,
    pub background_color: BackgroundColor,
}

impl PrimaryColor {
    /// new PrimaryColor Object
    pub fn new(
        foreground_color: ForegroundColor,
        background_color: BackgroundColor,
    ) -> PrimaryColor {
        PrimaryColor {
            foreground_color,
            background_color,
        }
    }
        /// Implicitly formatted text:隐式格式化文本
        pub fn print(&self, text: &str) {
            print!(
                "\x1b[1;{};{}m{}\x1b[0m",
                self.foreground_color.value(),
                self.background_color.value(),
                text,
            )
        }
        /// Implicitly formatted text:隐式格式化文本
        pub fn println(&self, text: &str) {
            println!(
                "\x1b[1;{};{}m{}\x1b[0m",
                self.foreground_color.value(),
                self.background_color.value(),
                text,
            )
        }
        ///Formatted return string:格式化返回字符串
        pub fn format(&self, text: &str) -> String {
            format!(
                "\x1b[1;{};{}m{}\x1b[0m",
                self.foreground_color.value(),
                self.background_color.value(),
                text,
            )
        }
}
// Foreground Color:前景色（文本颜色）
pub enum ForegroundColor {
    /// <span style="color:black;background:gray">黑色<span>
    Black = 30,
    /// <span style="color:red;background:gray">红色<span>
    Red = 31,
    /// <span style="color:green;background:gray">绿色<span>
    Green = 32,
    /// <span style="color:yellow;background:gray">黄色<span>
    Yellow = 33,
    /// <span style="color:blue;background:gray">蓝色<span>
    Blue = 34,
    /// <span style="color:magenta;background:gray">品红<span>
    Magenta = 35,
    /// <span style="color:cyan;background:gray">青色<span>
    Cyan = 36,
    /// <span style="color:white;background:gray">白色<span>
    White = 37,
    /// <span style="color:white;background:gray">白色<span>
    Default = 39,
    /// <span style="color:white;background:gray">白色<span>
    Reset = 0,
}

impl ForegroundColor {
    // return enum value
    pub fn value(&self) -> u8 {
        match self {
            ForegroundColor::Black => 30,
            ForegroundColor::Red => 31,
            ForegroundColor::Green => 32,
            ForegroundColor::Yellow => 33,
            ForegroundColor::Blue => 34,
            ForegroundColor::Magenta => 35,
            ForegroundColor::Cyan => 36,
            ForegroundColor::White => 37,
            ForegroundColor::Default => 39,
            ForegroundColor::Reset => 0,
        }
    }
    pub fn fgcolour_name(&self) -> String {
        let str_representation = match self {
            ForegroundColor::Black => "Black",
            ForegroundColor::Red => "Red",
            ForegroundColor::Green => "Green",
            ForegroundColor::Yellow => "Yellow",
            ForegroundColor::Blue => "Blue",
            ForegroundColor::Magenta => "Magenta",
            ForegroundColor::Cyan => "Cyan",
            ForegroundColor::White => "White",
            ForegroundColor::Default => "Default",
            ForegroundColor::Reset => "Reset",
        };
        str_representation.to_string()
    }
}

// Background Color:背景颜色
pub enum BackgroundColor {
    /// <span style="color:black;background:gray">黑色<span>
    Black = 40,
    /// <span style="color:red;background:gray">红色<span>
    Red = 41,
    /// <span style="color:green;background:gray">绿色<span>
    Green = 42,
    /// <span style="color:yellow;background:gray">黄色<span>
    Yellow = 43,
    /// <span style="color:blue;background:gray">蓝色<span>
    Blue = 44,
    /// <span style="color:magenta;background:gray">品红<span>
    Magenta = 45,
    /// <span style="color:cyan;background:gray">青色<span>
    Cyan = 46,
    /// <span style="color:white;background:gray">白色<span>
    White = 47,
    /// <span style="color:white;background:gray">白色<span>
    Default = 49,
    /// <span style="color:white;background:gray">白色<span>
    Reset = 0,
}

impl BackgroundColor {
    // return enum value
    pub fn value(&self) -> u8 {
        match self {
            BackgroundColor::Black => 40,
            BackgroundColor::Red => 41,
            BackgroundColor::Green => 42,
            BackgroundColor::Yellow => 44,
            BackgroundColor::Blue => 44,
            BackgroundColor::Magenta => 45,
            BackgroundColor::Cyan => 46,
            BackgroundColor::White => 47,
            BackgroundColor::Default => 49,
            BackgroundColor::Reset => 0,
        }
    }
    pub fn bgcolour_name(&self) -> String {
        let str_representation = match self {
            BackgroundColor::Black => "Black",
            BackgroundColor::Red => "Red",
            BackgroundColor::Green => "Green",
            BackgroundColor::Yellow => "Yellow",
            BackgroundColor::Blue => "Blue",
            BackgroundColor::Magenta => "Magenta",
            BackgroundColor::Cyan => "Cyan",
            BackgroundColor::White => "White",
            BackgroundColor::Default => "Default",
            BackgroundColor::Reset => "Reset",
        };
        str_representation.to_string()
    }
}
impl Debug for BackgroundColor {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "BackgroundColor: {}", self.bgcolour_name())
    }
}
impl Debug for ForegroundColor {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "ForegroundColor: {}", self.fgcolour_name())
    }
}
