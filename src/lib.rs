extern crate libc;
extern crate temperature;
use temperature::{Kelvin,Celcius,Fahrenheit};

#[no_mangle]
pub extern fn convert_to_kelvin(value: f64, is_celcius: bool) -> f64 {
    let m;
    if is_celcius == true
    {
        let c = Celcius::from(value);
        let k = Kelvin::from(c);
        m = k.into();
    } else
    {
        let f = Fahrenheit::from(value);
        let k = Kelvin::from(f);
        m = k.into()
    }
    m
}

#[no_mangle]
pub extern fn convert_to_fahrenheit(kelvinvalue: f64) -> f64 {
    let k = Kelvin::from(kelvinvalue);
    let f = Fahrenheit::from(k);
    f.into()
}

#[no_mangle]
pub extern fn convert_to_celcius(kelvinvalue: f64) -> f64 {
    let k = Kelvin::from(kelvinvalue);
    let c = Celcius::from(k);
    c.into()
}
