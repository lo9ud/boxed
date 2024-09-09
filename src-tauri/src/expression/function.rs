use super::{
    parser::Position,
    value::{Value, ValueType},
    ExpressionError, ExpressionResult,
};

pub enum Function {
    // Statistic functions
    Sum,
    Mean,
    Median,
    Mode,
    // Min,
    // Max,
    // Range,
    // Count,
    // Q1,
    // Q3,

    // // Trigonometric functions
    // Sin,
    // Cos,
    // Tan,
    // Asin,
    // Acos,
    // Atan,
    // Atan2,
    // Sinh,
    // Cosh,
    // Tanh,
    // Asinh,
    // Acosh,
    // Atanh,

    // // Exponential and logarithmic functions
    // Exp,
    // Ln,
    // Log10,
    // Log2,
    // Log,

    // // String functions
    // Concat,
    // Length,
    // Lower,
    // Upper,
    // Trim,
    // Replace,
    // Substring,
    // Split,
    // Join,
    // Contains,
    // StartsWith,
    // EndsWith,
    // IndexOf,
    // LastIndexOf,
    // PadStart,
    // PadEnd,
    // Repeat,
    // Reverse,
    // Format,
}

impl Function {
    pub fn from_str(name: &str) -> Option<Self> {
        match name {
            "sum" => Some(Self::Sum),
            "mean" => Some(Self::Mean),
            "median" => Some(Self::Median),
            "mode" => Some(Self::Mode),
            _ => None,
        }
    }

    pub fn return_type(&self) -> ValueType {
        match self {
            Self::Sum => ValueType::Number,
            Self::Mean => ValueType::Number,
            Self::Median => ValueType::Number,
            Self::Mode => ValueType::Number,
            _ => ValueType::Null,
        }
    }
}
