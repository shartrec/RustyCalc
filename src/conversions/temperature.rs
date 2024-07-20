/*
 * Copyright (c) 2024.
 *
 * Copyright 2024 Trevor Campbell
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
 * associated documentation files (the “Software”), to deal in the Software without restriction,
 * including without limitation the rights to use, copy, modify, merge, publish, distribute,
 * sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all copies or
 * substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
 * NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT
 * OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 *
 */
use crate::conversions::{Dimension, System, Unit};

/// These are the units of the weight dimension.
/// We define each of these as static constants for ease of use elsewhere and because they are
/// intrinsically immutable

pub static CELSIUS: Unit = Unit {
    name: "Celsius",
    dimension: Dimension::Temp,
    system: System::Metric,
    to_base: None,
    from_base: None,
    to_system_base: None,
    from_system_base: None,
};

pub static KELVIN: Unit = Unit {
    name: "Kelvin",
    dimension: Dimension::Temp,
    system: System::Metric,
    to_base: Some(|v| v - 273.15),
    from_base: Some(|v| v + 273.15),
    to_system_base: None,
    from_system_base: None,
};

pub static FAHRENHEIT: Unit = Unit {
    name: "Fahrenheit",
    dimension: Dimension::Temp,
    system: System::Metric,
    to_base: Some(|v| (v - 32.0) / 9.0 * 5.0),
    from_base: Some(|v| v / 5.0 * 9.0 + 32.0),
    to_system_base: None,
    from_system_base: None,
};

pub(crate) fn get_all() -> Vec<&'static Unit> {
    vec![&CELSIUS, &KELVIN, &FAHRENHEIT,
    ]
}

#[cfg(test)]
mod tests {
    use crate::assert_near;
    use crate::conversions::convert;
    use crate::conversions::temperature::*;

    #[test]
    fn test_all_metric_areas() {
        assert_eq!(convert(&-40.0, &CELSIUS, &FAHRENHEIT), -40.0);
        assert_eq!(convert(&-40.0, &FAHRENHEIT, &CELSIUS), -40.0);
        assert_eq!(convert(&10.0, &CELSIUS, &FAHRENHEIT), 50.0);
        assert_near!(convert(&100.0, &FAHRENHEIT, &CELSIUS), 37.7777778);
        assert_eq!(convert(&100.0, &CELSIUS, &KELVIN), 373.15);
        assert_near!(convert(&100.0, &KELVIN, &CELSIUS), -173.15);
        assert_near!(convert(&550.0, &KELVIN, &CELSIUS), 276.85);
        assert_near!(convert(&70.0, &FAHRENHEIT, &KELVIN), 294.2611111);
    }
}
