wai_bindgen_rust::export!("polyvalid.wai");

pub struct Polyvalid;

impl polyvalid::Polyvalid for Polyvalid {
    fn is_name_valid(input_string: String) -> bool {
        crate::is_name_valid(input_string)
    }
}
