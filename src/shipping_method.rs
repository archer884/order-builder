use rustc_serialize::{Encodable, Encoder};

#[derive(Debug)]
pub struct ShippingMethod {
    shipping_option: String,
}

impl ShippingMethod {
    pub fn new() -> ShippingMethod {
        ShippingMethod {
            shipping_option: "0".to_owned()
        }
    }
}

impl Encodable for ShippingMethod {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_struct("ShippingMethod", 1, |s| {
            s.emit_struct_field("ShippingOption", 0, |s| self.shipping_option.encode(s))?;
            Ok(())
        })
    }
}