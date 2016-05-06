use rustc_serialize::{Encodable, Encoder};

#[derive(Debug)]
pub struct ShipNotes {
    order_notes: Option<String>,
}

impl ShipNotes {
    pub fn new() -> ShipNotes {
        ShipNotes {
            order_notes: None,
        }
    }
}

impl Encodable for ShipNotes {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_struct("ShipNotes", 1, |s| {
            s.emit_struct_field("OrderNotes", 0, |s| self.order_notes.encode(s))?;
            Ok(())
        })
    }
}