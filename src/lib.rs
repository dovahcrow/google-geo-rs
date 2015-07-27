#![feature(slice_patterns)]
extern crate hyper;
extern crate serde;
#[macro_use] extern crate wrapped_enum;

use std::io::Read;

#[derive(Default, Debug)]
pub struct Address {
    street_number: String,
    route: String,
    neighborhood: String,
    locality: String,
    postal_code: String,
    postal_code_suffix: String,
    country: String,
    administrative_area_level_2: String,
    administrative_area_level_1: String
}


use std::error::Error;

pub fn lookup(latitude: f64, longitude: f64) -> Result<Address, Box<Error>> {
    let client = hyper::Client::new();
    let mut res = try!(client.get(&format!("http://maps.googleapis.com/maps/api/geocode/json?sensor=false&latlng={},{}", latitude, longitude)).send());
    let mut jsonbuf = String::new();
    try!(res.read_to_string(&mut jsonbuf));
    let js: serde::json::Value = try!(serde::json::from_str(&jsonbuf));
    let obj = js.as_object().unwrap();
    let result = obj.get("results").unwrap().as_array().unwrap();
    let address_components = result[0].as_object().unwrap().get("address_components").unwrap().as_array().unwrap();
    let mut retval: Address = Default::default();

    for item in address_components.iter() {
        let obj = item.as_object().unwrap();
        let types = obj.get("types").unwrap().as_array().unwrap();
        let long_name = obj.get("long_name").unwrap().as_string().unwrap();
        match types[0].as_string().unwrap() {
            "street_number" => {
                retval.street_number = long_name.into();
            }
            "route" => {
                retval.route = long_name.into();
            }
            "neighborhood" => {
                retval.neighborhood = long_name.into();
            }
            "locality" => {
                retval.locality = long_name.into();
            }
            "administrative_area_level_2" => {
                retval.administrative_area_level_2 = long_name.into();
            }
            "administrative_area_level_1" => {
                retval.administrative_area_level_1 = long_name.into();
            }
            "country" => {
                retval.country = long_name.into();
            }
            "postal_code" => {
                retval.postal_code = long_name.into();
            }
            "postal_code_suffix" => {
                retval.postal_code_suffix = long_name.into();
            }
            _ => {
                unreachable!()
            }
        }
    }
    Ok(retval)
}

#[test]
fn it_works() {
    let ret = lookup(41.962665, -87.655719).unwrap();
    assert_eq!(ret.street_number, "4422-4444".to_owned());
    assert_eq!(ret.postal_code, "60640".to_owned());
}
