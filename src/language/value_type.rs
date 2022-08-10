use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::ops::{Add, Div, Mul, Neg, Not, Sub};
use std::str::FromStr;

#[allow(dead_code)]
pub enum ValueType {
    Str(String),
    Number(i32),
}

impl Add for ValueType {
    type Output = Result<ValueType, String>;

    fn add(self, other: ValueType) -> Self::Output {
        match (self, other) {
            (ValueType::Number(rh), ValueType::Number(lh)) => Ok(ValueType::Number(rh + lh)),
            (ValueType::Str(rh), ValueType::Str(lh)) => Ok(ValueType::Str(format!("{}{}", rh, lh))),
            (ValueType::Str(rh), ValueType::Number(lh)) => {
                let number_string = i32::from_str(rh.as_str());

                if let Result::Ok(number) = number_string {
                    Ok(ValueType::Number(number + lh))
                } else {
                    Err(format!("Cannot add string: {} to number: {}.", rh, lh))
                }
            }
            (ValueType::Number(rh), ValueType::Str(lh)) => {
                let number_string = i32::from_str(lh.as_str());

                if let Result::Ok(number) = number_string {
                    Ok(ValueType::Number(number + rh))
                } else {
                    Err(format!("Cannot add string: {} to number: {}.", lh, rh))
                }
            }
        }
    }
}

impl Sub for ValueType {
    type Output = Result<ValueType, String>;

    fn sub(self, other: ValueType) -> Self::Output {
        match (self, other) {
            (ValueType::Number(rh), ValueType::Number(lh)) => Ok(ValueType::Number(rh - lh)),
            (ValueType::Str(rh), ValueType::Number(lh)) => {
                let number_string = i32::from_str(rh.as_str());

                if let Result::Ok(number) = number_string {
                    Ok(ValueType::Number(number - lh))
                } else {
                    Err(format!("Cannot subtract string: {} to number: {}.", rh, lh))
                }
            }
            (ValueType::Number(rh), ValueType::Str(lh)) => {
                let number_string = i32::from_str(lh.as_str());

                if let Result::Ok(number) = number_string {
                    Ok(ValueType::Number(number - rh))
                } else {
                    Err(format!("Cannot subtract string: {} to number: {}.", lh, rh))
                }
            }
            _ => Err(String::from("Can only subtract numbers.")),
        }
    }
}

impl Mul for ValueType {
    type Output = Result<ValueType, String>;

    fn mul(self, other: ValueType) -> Self::Output {
        match (self, other) {
            (ValueType::Number(rh), ValueType::Number(lh)) => Ok(ValueType::Number(rh * lh)),
            (ValueType::Str(rh), ValueType::Number(lh)) => {
                let number_string = i32::from_str(rh.as_str());

                if let Result::Ok(number) = number_string {
                    Ok(ValueType::Number(number * lh))
                } else {
                    Err(format!("Cannot multiply string: {} to number: {}.", rh, lh))
                }
            }
            (ValueType::Number(rh), ValueType::Str(lh)) => {
                let number_string = i32::from_str(lh.as_str());

                if let Result::Ok(number) = number_string {
                    Ok(ValueType::Number(number * rh))
                } else {
                    Err(format!("Cannot multiply string: {} to number: {}.", lh, rh))
                }
            }
            _ => Err(String::from("Can only multiply numbers.")),
        }
    }
}

impl Div for ValueType {
    type Output = Result<ValueType, String>;

    fn div(self, other: ValueType) -> Self::Output {
        match (self, other) {
            (ValueType::Number(rh), ValueType::Number(lh)) => Ok(ValueType::Number(rh / lh)),
            (ValueType::Str(rh), ValueType::Number(lh)) => {
                let number_string = i32::from_str(rh.as_str());

                if let Result::Ok(number) = number_string {
                    Ok(ValueType::Number(number / lh))
                } else {
                    Err(format!("Cannot divide string: {} to number: {}.", rh, lh))
                }
            }
            (ValueType::Number(rh), ValueType::Str(lh)) => {
                let number_string = i32::from_str(lh.as_str());

                if let Result::Ok(number) = number_string {
                    Ok(ValueType::Number(number / rh))
                } else {
                    Err(format!("Cannot divide string: {} to number: {}.", lh, rh))
                }
            }
            _ => Err(String::from("Can only divide numbers.")),
        }
    }
}

impl Not for ValueType {
    type Output = Result<ValueType, String>;

    fn not(self) -> Self::Output {
        match self {
            _ => Err(String::from("Cannot apply NOT to non-boolean type.")),
        }
    }
}

impl Neg for ValueType {
    type Output = Result<ValueType, String>;

    fn neg(self) -> Self::Output {
        match self {
            ValueType::Number(ref number) => Ok(ValueType::Number(-*number)),
            _ => Err(String::from("Cannot apply NEG to non-boolean type.")),
        }
    }
}

impl PartialEq for ValueType {
    fn eq(&self, other: &ValueType) -> bool {
        match (self, other) {
            (&ValueType::Str(ref rh), &ValueType::Str(ref lh)) => rh == lh,
            (&ValueType::Number(rh), &ValueType::Number(lh)) => rh == lh,

            (&ValueType::Number(rh), &ValueType::Str(ref lh)) => {
                let number_string = i32::from_str(lh.as_str()).unwrap();
                rh == number_string
            }

            (&ValueType::Str(ref rh), &ValueType::Number(lh)) => {
                let number_string = i32::from_str(rh.as_str()).unwrap();
                lh == number_string
            }
        }
    }

    fn ne(&self, other: &ValueType) -> bool {
        match (self, other) {
            (&ValueType::Number(rh), &ValueType::Number(lh)) => rh != lh,
            (&ValueType::Str(ref rh), &ValueType::Str(ref lh)) => rh != lh,

            (&ValueType::Number(rh), &ValueType::Str(ref lh)) => {
                let number_string = i32::from_str(lh.as_str()).unwrap();
                rh != number_string
            }

            (&ValueType::Str(ref rh), &ValueType::Number(lh)) => {
                let number_string = i32::from_str(rh.as_str()).unwrap();
                lh != number_string
            }
        }
    }
}

impl PartialOrd for ValueType {
    fn partial_cmp(&self, other: &ValueType) -> Option<Ordering> {
        if self > other {
            Some(Ordering::Greater)
        } else if self < other {
            Some(Ordering::Less)
        } else if self == other {
            Some(Ordering::Equal)
        } else {
            None
        }
    }

    fn gt(&self, other: &ValueType) -> bool {
        match (self, other) {
            (&ValueType::Number(rh), &ValueType::Number(lh)) => rh > lh,
            (&ValueType::Number(rh), &ValueType::Str(ref lh)) => {
                let number_string = i32::from_str(lh.as_str()).unwrap();
                rh > number_string
            }

            (&ValueType::Str(ref rh), &ValueType::Number(lh)) => {
                let number_string = i32::from_str(rh.as_str()).unwrap();
                lh > number_string
            }
            _ => false,
        }
    }

    fn ge(&self, other: &ValueType) -> bool {
        match (self, other) {
            (&ValueType::Number(rh), &ValueType::Number(lh)) => rh >= lh,
            (&ValueType::Number(rh), &ValueType::Str(ref lh)) => {
                let number_string = i32::from_str(lh.as_str()).unwrap();
                rh >= number_string
            }

            (&ValueType::Str(ref rh), &ValueType::Number(lh)) => {
                let number_string = i32::from_str(rh.as_str()).unwrap();
                lh >= number_string
            }
            _ => false,
        }
    }

    fn lt(&self, other: &ValueType) -> bool {
        match (self, other) {
            (&ValueType::Number(rh), &ValueType::Number(lh)) => rh < lh,
            (&ValueType::Number(rh), &ValueType::Str(ref lh)) => {
                let number_string = i32::from_str(lh.as_str()).unwrap();
                rh < number_string
            }

            (&ValueType::Str(ref rh), &ValueType::Number(lh)) => {
                let number_string = i32::from_str(rh.as_str()).unwrap();
                lh < number_string
            }
            _ => false,
        }
    }

    fn le(&self, other: &ValueType) -> bool {
        match (self, other) {
            (&ValueType::Number(rh), &ValueType::Number(lh)) => rh <= lh,
            (&ValueType::Number(rh), &ValueType::Str(ref lh)) => {
                let number_string = i32::from_str(lh.as_str()).unwrap();
                rh <= number_string
            }

            (&ValueType::Str(ref rh), &ValueType::Number(lh)) => {
                let number_string = i32::from_str(rh.as_str()).unwrap();
                lh <= number_string
            }
            _ => false,
        }
    }
}
