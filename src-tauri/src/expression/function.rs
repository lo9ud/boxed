use super::{
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

    fn check_args(&self, args: &Vec<Value>) -> ExpressionResult<()> {
        match self {
            Self::Sum | Self::Mean | Self::Median | Self::Mode => {
                if args.is_empty() {
                    Err(ExpressionError::missing_arguments())
                } else {
                    match args[0].value_type() {
                        ValueType::Number => {
                            for arg in args.iter().skip(1) {
                                if arg.value_type() != ValueType::Number {
                                    return Err(ExpressionError::type_error(
                                        &ValueType::Number,
                                        &arg.value_type(),
                                        arg.position(),
                                    ));
                                }
                            }
                            Ok(())
                        }
                        ValueType::Column(t) => match t {
                            ValueType::Number if args.length() == 1 => Ok(()),
                            _ => Err(ExpressionError::ArgumentError(
                                "Expected a single numerical column argument".to_string(),
                            )),
                        },
                        _ => Err(ExpressionError::ArgumentError(
                            "Expected number arguments".to_string(),
                        )),
                    }
                }
            }
        }
    }

    pub fn eval(&self, args: Vec<Value>) -> ExpressionResult<Value> {
        self.check_args(&args)?;
        match self {
            Self::Sum => {
                let sum = args.iter().fold(0.0, |acc, v| acc + v.as_number());
                Value::Number(sum)
            }
            Self::Mean => {
                let sum = args.iter().fold(0.0, |acc, v| acc + v.as_number());
                Value::Number(sum / args.len() as f64)
            }
            Self::Median => {
                let mut sorted = args.iter().map(|v| v.as_number()).collect::<Vec<_>>();
                sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let mid = sorted.len() / 2;
                if sorted.len() % 2 == 0 {
                    Value::Number((sorted[mid - 1] + sorted[mid]) / 2.0)
                } else {
                    Value::Number(sorted[mid])
                }
            }
            Self::Mode => {
                let mut counts = std::collections::HashMap::new();
                for v in args {
                    *counts.entry(v).or_insert(0) += 1;
                }
                let max = counts.values().max().unwrap();
                let mode = counts.iter().find(|(_, &count)| count == max).unwrap().0;
                mode.clone()
            }
        }
    }
}
