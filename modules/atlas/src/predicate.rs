use crate::feature::{FeatureFields, Value};

#[ derive(Clone)]
pub enum ValueSelector<V: Clone> {
    Constant(V),
}

impl<V: Clone> ValueSelector<V> {
    pub fn get_value(&self, zoom: f32, fields: &FeatureFields) -> V {
        match self {
            ValueSelector::Constant(value) => value.clone(),
        }
    }
}

pub struct Predicate {
    pub field: String,
    pub condition: Condition,
}

pub enum Condition {
    Exists,
    Number(NumberCondition),
    String(StringCondition),
}

impl Condition {
    pub fn check(&self, value: Option<&Value>) -> bool {
        match self {
            Condition::Exists => value.is_some(),
            Condition::Number(number) => {
                if let Some(value) = value {
                    if let Some(value) = value.to_f64() {
                        return number.check(value);
                    }
                }

                false
            }
            Condition::String(string) => {
                if let Some(value) = value {
                    if let Some(value) = value.to_str() {
                        return string.check(value);
                    }
                }

                false
            }
        }
    }
}

pub enum NumberCondition {
    Eq(f64),
    NotEq(f64),
    Greater(f64),
    GreaterOrEq(f64),
    Lesser(f64),
    LesserOrEq(f64),
}

impl NumberCondition {
    pub fn check(&self, value: f64) -> bool {
        match *self {
            NumberCondition::Eq(v) => value == v,
            NumberCondition::NotEq(v) => value != v,
            NumberCondition::Greater(v) => value > v,
            NumberCondition::GreaterOrEq(v) => value >= v,
            NumberCondition::Lesser(v) => value < v,
            NumberCondition::LesserOrEq(v) => value <= v,
        }
    }
}

pub enum StringCondition {
    Eq(String),
    NotEq(String),
}

impl StringCondition {
    pub fn check(&self, value: &str) -> bool {
        match self {
            StringCondition::Eq(v) => value == v,
            StringCondition::NotEq(v) => value != v,
        }
    }
}
