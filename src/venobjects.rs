use crate::insts::Instructions;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VenObjects {
    Int(i64),
    Float(Float),
    Str(String),
    Class(String, Vec<Instructions>),
    Function(String, Vec<Instructions>),
}
impl VenObjects {
    pub fn get_int(&self) -> Option<i64> {
        match self {
            VenObjects::Int(i) => Some(*i),
            _ => None,
        }
    }
    pub fn get_float(&self) -> Option<f64> {
        match self {
            VenObjects::Float(float) => Some(float.to_primitive()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Float {
    before_point: i64,
    after_point: i64,
}
impl Float {
    pub fn to_primitive(&self) -> f64 {
        let stri = self.before_point.to_string() + "." + self.after_point.to_string().as_str();
        stri.parse::<f64>().expect("Cannot get float value")
    }
    pub fn build(float: f64) -> Self {
        let stri = float.to_string();
        let mut before: String = "0".into();
        let mut after: String = "0".into();
        let mut after_point = false;
        for i in stri.chars() {
            if i == '.' {
                after_point = true;
            } else if after_point && i.is_numeric() {
                after.push(i);
            } else if i.is_numeric() {
                before.push(i);
            } else {
                println!("alpha : {:?}", i);
            }
        }
        if !after_point {
            after = "0".into();
        }
        Float {
            before_point: before.parse::<i64>().expect("Unable to convert to float"),
            after_point: after.parse::<i64>().expect("Unable to convert to float"),
        }
    }
}
