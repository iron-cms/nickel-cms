use rustc_serialize::json::{self, Json, ToJson};
use params::Value;
use super::render::BaseDataMap;

#[derive(RustcDecodable, Debug)]
pub struct Validator<T> {
    pub requiered: Option<bool>,
    pub empty: Option<bool>,
    pub min: Option<u32>,
    pub max: Option<u32>,
    pub dafault: Option<T>,
    errors: Option<ErrorValidator>,
}

pub struct ValidateResult(BaseDataMap, ErrorValidator);
pub type VRes = Vec<ValidateResult>;

pub trait Tr {
    fn get_errors(&self);
}

impl<T> Tr for Vec<T> {
    fn get_errors(&self) {
        println!("get_errors");
    }
}

impl<T> Validator<T> {
    pub fn new(validator_rules: BaseDataMap) -> Validator<String> {
        let json_obj: Json = Json::Object(validator_rules);
        let json_str: String = json_obj.to_string();
        json::decode(&json_str).unwrap()
    }

    pub fn validate(&mut self, field: String, value: Option<&Value>) -> ValidateResult {
        // Init Errors
        self.errors = Some(ErrorValidator::new(&field));

        // Invoke validators
        self.requiered(value);

        let mut val = "".to_json();
        if let Some(&Value::String(ref name)) = value {
            val = name.to_json();
        }
        let mut err = ErrorValidator::new(&field);
        if let Some(ref err_results) = self.errors {
            err = err_results.to_owned();
        }
        ValidateResult(btreemap! {
            field.to_owned() => val.to_json()
        }, err)
    }

    /// Requered validator
    fn requiered(&mut self, value: Option<&Value>) {
        if self.requiered.is_some() {
            if value.is_none() {
                if let Some(ref mut error) = self.errors {
                    let msg = format!("Field requiered: {}", error.field);
                    error.add(msg);
                }
            }
        }
    }
}

/// Validator Errors
#[derive(RustcDecodable, Debug, Clone)]
pub struct ErrorValidator {
    pub errors: Vec<String>,
    pub errors_count: Option<u32>,
    pub field : String,
}

/// Validator Errors methods
impl ErrorValidator {
    /// Init error
    fn new(field: &String) -> Self {
        ErrorValidator {
            field: field.to_owned(),
            errors: vec!(),
            errors_count: None,
        }
    }

    /// Add error
    fn add(&mut self, error: String) {
        if self.errors_count.is_none() {
            self.errors = vec!(error);
            self.errors_count = Some(1);
        } else {
            if let Some(count) = self.errors_count {
                self.errors.push(error);
                self.errors_count = Some(count + 1);
            }
        }
    }
}
