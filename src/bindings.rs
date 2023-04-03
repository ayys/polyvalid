wai_bindgen_rust::export!("polyvalid.wai");

pub struct Polyvalid;

impl polyvalid::Polyvalid for Polyvalid {
    fn is_app_name_valid(input_string: String) -> polyvalid::Output {
        let result = crate::is_app_name_valid(input_string);
        let is_valid = result.is_ok();
        let error_message = result.err().map(|e| e.to_string()).unwrap_or_default();
        polyvalid::Output {
            is_valid,
            error_message,
        }
    }
}
