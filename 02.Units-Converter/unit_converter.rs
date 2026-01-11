// Unit conversion functions

/// Convert kilometers to meters
pub fn km2m(km: f64) -> f64 {
    km * 1000.0
}

/// Convert meters to kilometers
pub fn m2km(m: f64) -> f64 {
    m / 1000.0
}

/// Convert miles to kilometers
pub fn miles2km(miles: f64) -> f64 {
    miles * 1.60934
}

/// Convert kilometers to miles
pub fn km2miles(km: f64) -> f64 {
    km / 1.60934
}

/// Convert Celsius to Fahrenheit
pub fn celsius2fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

/// Convert Fahrenheit to Celsius
pub fn fahrenheit2celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

/// Convert kilograms to pounds
pub fn kg2lbs(kg: f64) -> f64 {
    kg * 2.20462
}

/// Convert pounds to kilograms
pub fn lbs2kg(lbs: f64) -> f64 {
    lbs / 2.20462
}
