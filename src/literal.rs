use std::ops;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(f32),
    String(String),
    Bool(bool),
    Null
}

impl Literal {
    pub fn to_string(&self) -> String {
        match self {
            Self::Number(x) => {
                if x.to_string().ends_with(".0") || !x.to_string().contains('.') {
                    return format!("{:.1}", x);
                } else {
                    return format!("{}", x);
                }
            }
            Self::String(x) => x.to_string(),
            Self::Bool(x) => x.to_string(),
            Self::Null => "null".to_string()
        }
    }

    pub fn literal_type(&self) -> String {
        match self {
            Self::Number(_) => "number".to_string(),
            Self::String(_) => "string".to_string(),
            Self::Bool(_) => "bool".to_string(),
            Self::Null => "null".to_string(),
        }
    }

    pub fn is_double(&self) -> bool {
        match self {
            Literal::Number(_) => true,
            _ => false
        }
    }

    pub fn is_bool(&self) -> bool {
        match self {
            Literal::Bool(_) => true,
            _ => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Literal::String(_) => true,
            _ => false,
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            Literal::Null => true,
            _ => false,
        }
    }
}

impl ops::Add<Literal> for Literal {
    type Output = Result<Literal, String>;

    fn add(self, rhs: Literal) -> Self::Output {
        match (self, rhs) {
            (Literal::Number(lhs), Literal::Number(rhs)) => Ok(Literal::Number(lhs + rhs)),
            (Literal::String(lhs), Literal::String(rhs)) => Ok(Literal::String(lhs + &rhs)),
            (lhs, rhs) => Err(format!("Cannot add '{}' and '{}'", lhs.literal_type(), rhs.literal_type()))
        }
    }
}

impl ops::Sub<Literal> for Literal {
    type Output = Result<Literal, String>;

    fn sub(self, rhs: Literal) -> Self::Output {
        match (self, rhs) {
            (Literal::Number(lhs), Literal::Number(rhs)) => Ok(Literal::Number(lhs - rhs)),
            (lhs, rhs) => Err(format!("Cannot subtract '{}' from '{}'", rhs.literal_type(), lhs.literal_type())),
        }
    }
}

impl ops::Neg for Literal {
    type Output = Result<Literal, String>;

    fn neg(self) -> Self::Output {
        match self {
            Literal::Number(x) => Ok(Literal::Number(-x)),
            Literal::Bool(x) => Ok(Literal::Bool(!x)),
            Literal::String(_) => Err("Cannot negate a string.".to_string()),
            Literal::Null => Err("Cannot negate a nil.".to_string())
        }
    }
}

impl ops::Mul for Literal {
    type Output = Result<Literal, String>;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Number(x * y)),
            (lhs, rhs) => Err(format!("Cannot multiply '{}' by '{}'", lhs.literal_type(), rhs.literal_type())),
            
        }
    }
}

impl ops::Div for Literal {
    type Output = Result<Literal, String>;

    fn div(self, rhs: Self) -> Self::Output {
        match(self, rhs) {
            (Literal::Number(x), Literal::Number(y)) => Ok(Literal::Number(x / y)),
            (lhs, rhs) => Err(format!("Cannot multiply '{}' by '{}'", lhs.literal_type(), rhs.literal_type()))
        }
    }
}

impl PartialOrd<Self> for Literal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Literal::Number(x), Literal::Number(y)) => {
                if x > y {
                    Some(Ordering::Greater)
                } else if x < y {
                    Some(Ordering::Less)
                } else {
                    Some(Ordering::Equal)
                }
            }
            (_, _) => None,
        }
    }
}