use address::Address;
use rustc_serialize::{Encodable, Encoder};

#[derive(Debug)]
pub struct Customer {
    email: String,
    email_opt_in: bool,
    use_shipping_for_billing: bool,
    billing: Address,
    shipping: Address,
}

impl Customer {
    pub fn with_email<T: Into<String>>(email: T) -> Customer {
        Customer {
            email: email.into(),
            email_opt_in: false,
            use_shipping_for_billing: true,
            billing: Address::new(),
            shipping: Address::new()
        }
    }
}

impl Encodable for Customer {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_struct("Customer", 1, |s| {
            s.emit_struct_field("Email", 0, |s| self.email.encode(s))?;
            s.emit_struct_field("EmailOptIn", 1, |s| self.email_opt_in.encode(s))?;
            s.emit_struct_field("UseShippingForBilling", 2, |s| self.use_shipping_for_billing.encode(s))?;
            s.emit_struct_field("Billing", 3, |s| self.billing.encode(s))?;
            s.emit_struct_field("Shipping", 4, |s| self.shipping.encode(s))?;
            Ok(())
        })
    }
}