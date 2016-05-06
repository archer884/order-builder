use rustc_serialize::{Encodable, Encoder};

#[derive(Debug)]
pub struct Address {
    first_name: String,
    last_name: String,
    company: Option<String>,
    address_line_1: Option<String>,
    address_line_2: Option<String>,
    address_line_3: Option<String>,
    city: String,
    state_or_province: String,
    postal_code: String,
    country_code: String,
    phone: String,
    verified: Option<bool>,
}

impl Address {
    pub fn new() -> Address {
        Address {
            first_name: "JP".to_owned(),
            last_name: "Apology".to_owned(),
            company: None,
            address_line_1: Some("3232 W Royal Ln".to_owned()),
            address_line_2: None,
            address_line_3: None,
            city: "Irving".to_owned(),
            state_or_province: "TX".to_owned(),
            postal_code: "75063".to_owned(),
            country_code: "USA".to_owned(),
            phone: "8003184823".to_owned(),
            verified: None,
        }
    }
}

impl Encodable for Address {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_struct("Address", 1, |s| {
            s.emit_struct_field("FirstName", 0, |s| self.first_name.encode(s))?;
            s.emit_struct_field("LastName", 1, |s| self.last_name.encode(s))?;
            s.emit_struct_field("Company", 2, |s| self.company.encode(s))?;
            s.emit_struct_field("AddressLine1", 3, |s| self.address_line_1.encode(s))?;
            s.emit_struct_field("AddressLine2", 4, |s| self.address_line_2.encode(s))?;
            s.emit_struct_field("AddressLine3", 5, |s| self.address_line_3.encode(s))?;
            s.emit_struct_field("City", 6, |s| self.city.encode(s))?;
            s.emit_struct_field("StateOrProvince", 7, |s| self.state_or_province.encode(s))?;
            s.emit_struct_field("PostalCode", 8, |s| self.postal_code.encode(s))?;
            s.emit_struct_field("CountryCode", 9, |s| self.country_code.encode(s))?;
            s.emit_struct_field("Phone", 10, |s| self.phone.encode(s))?;
            s.emit_struct_field("Verified", 11, |s| self.verified.encode(s))?;
            Ok(())
        })
    }
}