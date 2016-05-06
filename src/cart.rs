use rustc_serialize::{Encodable, Encoder};

#[derive(Debug)]
pub struct Cart {
    sku: String,
    qty: i32,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            sku: "NGOLD".into(),
            qty: 1,
        }
    }
}

impl Encodable for Cart {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_struct("Cart", 1, |s| {
            s.emit_struct_field("Sku", 0, |s| self.sku.encode(s))?;
            s.emit_struct_field("Qty", 1, |s| self.qty.encode(s))?;
            Ok(())
        })
    }
}