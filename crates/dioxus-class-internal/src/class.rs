use std::fmt::Display;
use std::ops::Add;
use dioxus::prelude::*;
use dioxus::dioxus_core::AttributeValue;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Class(pub Vec<String>);

impl Display for Class {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_class())
    }
}

impl Class {
    pub const NONE: Class = Class(vec![]);

    pub fn to_class(&self) -> String {
        self.0.join(" ")
    }
    pub fn validate(&mut self) -> () {
        //TODO: validate the string format for css
        self.0.sort_unstable();
        self.0.dedup();
    }
    pub fn validated(&mut self) -> Self {
        self.validate();
        self.to_owned()
    }
    pub fn append(&mut self, v: &str) -> () {
        self.0.push(v.to_string());
    }
}

impl IntoAttributeValue for Class {
    fn into_value(self) -> AttributeValue {
        AttributeValue::Text(self.to_class())
    }
}

impl IntoAttributeValue for &Class {
    fn into_value(self) -> AttributeValue {
        AttributeValue::Text(self.to_class())
    }
}

impl From<Vec<&str>> for Class {
    fn from(v: Vec<&str>) -> Self {
        Self(v.iter().map(|x| x.to_string()).collect::<Vec<String>>())
    }
}

impl From<Vec<String>> for Class {
    fn from(v: Vec<String>) -> Self {
        Self(v)
    }
}

impl From<&[&str]> for Class {
    fn from(v: &[&str]) -> Self {
        Self(v.iter().map(|x| x.to_string()).collect::<Vec<String>>())
    }
}

impl From<&[String]> for Class {
    fn from(v: &[String]) -> Self {
        Self(v.to_vec())
    }
}

impl Add<&str> for Class {
    type Output = Self;

    fn add(self, rhs: &str) -> Self::Output {
        let mut result = self.clone();
        result.append(rhs);
        result.validated()
    }
}

impl Add<String> for Class {
    type Output = Self;

    fn add(self, rhs: String) -> Self::Output {
        let mut result = self.clone();
        result.append(&rhs);
        result.validated()
    }
}

impl Add<Vec<&str>> for Class {
    type Output = Self;

    fn add(self, rhs: Vec<&str>) -> Self::Output {
        let mut result = self.clone();
        for part in rhs {
            result.append(part);
        }
        result.validated()
    }
}

impl Add<Vec<String>> for Class {
    type Output = Self;

    fn add(self, rhs: Vec<String>) -> Self::Output {
        let mut result = self.clone();
        for part in rhs {
            result.append(&part);
        }
        result.validated()
    }
}

impl Add<Class> for Class {
    type Output = Self;

    fn add(self, rhs: Class) -> Self::Output {
        let mut result = self.clone();
        for part in rhs.0 {
            result.append(&part);
        }
        result.validated()
    }
}

impl Add<&Class> for Class {
    type Output = Self;

    fn add(self, rhs: &Class) -> Self::Output {
        let mut result = self.clone();
        for part in rhs.0.iter() {
            result.append(&part);
        }
        result.validated()
    }
}

impl Add<Option<&str>> for Class {
    type Output = Self;

    fn add(self, rhs: Option<&str>) -> Self::Output {
        match rhs {
            Some(rhs) => self + rhs,
            None => self,
        }
    }
}

impl Add<Option<String>> for Class {
    type Output = Self;

    fn add(self, rhs: Option<String>) -> Self::Output {
        match rhs {
            Some(rhs) => self + rhs,
            None => self,
        }
    }
}

impl Add<Option<Vec<&str>>> for Class {
    type Output = Self;

    fn add(self, rhs: Option<Vec<&str>>) -> Self::Output {
        match rhs {
            Some(rhs) => self + rhs,
            None => self,
        }
    }
}

impl Add<Option<Vec<String>>> for Class {
    type Output = Self;

    fn add(self, rhs: Option<Vec<String>>) -> Self::Output {
        match rhs {
            Some(rhs) => self + rhs,
            None => self,
        }
    }
}

impl Add<Option<Class>> for Class {
    type Output = Self;

    fn add(self, rhs: Option<Class>) -> Self::Output {
        match rhs {
            Some(rhs) => self + rhs,
            None => self,
        }
    }
}

impl Add<Option<&Class>> for Class {
    type Output = Self;

    fn add(self, rhs: Option<&Class>) -> Self::Output {
        match rhs {
            Some(rhs) => self + rhs,
            None => self,
        }
    }
}

impl Add<&Option<&Class>> for Class {
    type Output = Self;

    #[allow(suspicious_double_ref_op)]
    fn add(self, rhs: &Option<&Class>) -> Self::Output {
        match rhs {
            Some(rhs) => self + rhs.clone(),
            None => self,
        }
    }
}