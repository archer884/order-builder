#![feature(question_mark)]

extern crate grabinput;
extern crate rustc_serialize;

mod address;
mod cart;
mod customer;
mod order;
mod ship_notes;
mod shipping_method;

use order::Order;
use rustc_serialize::json;

fn main() {
    let input: Vec<_> = grabinput::by_lines(std::env::args().nth(1))
        .map(|line| Order::from_email(line.trim()))
        .collect(); 
    
    println!("{}", json::encode(&input).unwrap());
}

#[cfg(test)]
mod tests {
    use order::Order;
    use rustc_serialize::json;
    
    static TEST_CONTENT: &'static str = r#"[{"Label": null,
   "Cart": [
     {
       "Sku": "NGOLD",
       "Qty": 1
     }
   ],
   "Customer": {
     "Email": "KIMSCOOKED@GMAIL.COM",
     "EmailOptIn": false,
     "UseShippingForBilling": true,
     "Billing": {
       "FirstName": "JP",
       "LastName": "Apology",
       "Company": null,
       "AddressLine1": "3232 W Royal Ln",
       "AddressLine2": null,
       "AddressLine3": null,
       "City": "Irving",
       "StateOrProvince": "TX",
       "PostalCode": "75063",
       "CountryCode": "USA",
       "Phone": "18003184823",
       "Verified": null
     },
     "Shipping": {
       "FirstName": "JP",
       "LastName": "Apology",
       "Company": null,
       "AddressLine1": "3232 W Royal Ln",
       "AddressLine2": null,
       "AddressLine3": null,
       "City": "Irving",
       "StateOrProvince": "TX",
       "PostalCode": "75063",
       "CountryCode": "USA",
       "Phone": "18003184823",
       "Verified": null
     }
   },
   "ShipNotes": {
     "OrderNotes": null
   },
   "ShippingMethod": {
     "ShippingOption": "0"
   }
 },
 {
   "Label": null,
   "Cart": [
     {
       "Sku": "NGOLD",
       "Qty": 1
     }
   ],
   "Customer": {
     "Email": "HTRDLNC@GMAIL.COM",
     "EmailOptIn": false,
     "UseShippingForBilling": true,
     "Billing": {
       "FirstName": "JP",
       "LastName": "Apology",
       "Company": null,
       "AddressLine1": "3232 W Royal Ln",
       "AddressLine2": null,
       "AddressLine3": null,
       "City": "Irving",
       "StateOrProvince": "TX",
       "PostalCode": "75063",
       "CountryCode": "USA",
       "Phone": "18003184823",
       "Verified": null
     },
     "Shipping": {
       "FirstName": "JP",
       "LastName": "Apology",
       "Company": null,
       "AddressLine1": "3232 W Royal Ln",
       "AddressLine2": null,
       "AddressLine3": null,
       "City": "Irving",
       "StateOrProvince": "TX",
       "PostalCode": "75063",
       "CountryCode": "USA",
       "Phone": "18003184823",
       "Verified": null
     }
   },
   "ShipNotes": {
     "OrderNotes": null
   },
   "ShippingMethod": {
     "ShippingOption": "0"
   }
 }
]"#;

    #[test]
    fn serialization_works() {
        assert_eq!(
            TEST_CONTENT.replace(" ", "").replace("\n", ""),
            format!("{}", json::as_pretty_json(&vec![
                Order::from_email("KIMSCOOKED@GMAIL.COM"),
                Order::from_email("HTRDLNC@GMAIL.COM")
            ])).replace(" ", "").replace("\n", "")
        );
    }
}