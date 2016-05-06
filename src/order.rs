use customer::Customer;
use cart::Cart;
use rustc_serialize::{Encodable, Encoder};
use ship_notes::ShipNotes;
use shipping_method::ShippingMethod;

#[derive(Debug)]
pub struct Order {
    label: Option<String>,
    cart: Vec<Cart>,
    customer: Customer,
    ship_notes: ShipNotes,
    shipping_method: ShippingMethod,
}

impl Order {
    pub fn from_email<T: Into<String>>(email: T) -> Order {
        Order {
            label: None,
            cart: vec![Cart::new()],
            customer: Customer::with_email(email),
            ship_notes: ShipNotes::new(),
            shipping_method: ShippingMethod::new(),
        }
    }
}

impl Encodable for Order {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_struct("Order", 1, |s| {
            s.emit_struct_field("Label", 0, |s| self.label.encode(s))?;
            s.emit_struct_field("Cart", 1, |s| self.cart.encode(s))?;
            s.emit_struct_field("Customer", 2, |s| self.customer.encode(s))?;
            s.emit_struct_field("ShipNotes", 3, |s| self.ship_notes.encode(s))?;
            s.emit_struct_field("ShippingMethod", 4, |s| self.shipping_method.encode(s))?;
            Ok(())
        })
    } 
}