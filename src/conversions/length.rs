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

pub static METRE: Unit = Unit {
    name: "Metre",
    dimension: Dimension::Length,
    system: System::Metric,
    to_base: None,
    from_base: None,
    to_system_base: None,
    from_system_base: None,
};
pub static CENTIMETRE: Unit = Unit {
    name: "Centimetre",
    dimension: Dimension::Length,
    system: System::Metric,
    to_base: Some(|v| v / 100.0),
    from_base: Some(|v| v * 100.0),
    to_system_base: None,
    from_system_base: None,
};
pub static MILLIMETRE: Unit = Unit {
    name: "Millimetre",
    dimension: Dimension::Length,
    system: System::Metric,
    to_base: Some(Unit::from_milli),
    from_base: Some(Unit::to_milli),
    to_system_base: None,
    from_system_base: None,
};
pub static MICROMETRE: Unit = Unit {
    name: "Micrometre",
    dimension: Dimension::Length,
    system: System::Metric,
    to_base: Some(Unit::from_micro),
    from_base: Some(Unit::to_micro),
    to_system_base: None,
    from_system_base: None,
};
pub static KILOMETRE: Unit = Unit {
    name: "Kilometre",
    dimension: Dimension::Length,
    system: System::Metric,
    to_base: Some(Unit::from_kilo),
    from_base: Some(Unit::to_kilo),
    to_system_base: None,
    from_system_base: None,
};
static METRES_PER_LIGHTYEAR: f64 = 9460730472580800.0;
pub static LIGHTYEAR: Unit = Unit {
    name: "Lightyear",
    dimension: Dimension::Length,
    system: System::Metric,
    to_base: Some(|v| v * METRES_PER_LIGHTYEAR),
    from_base: Some(|v| v / METRES_PER_LIGHTYEAR),
    to_system_base: None,
    from_system_base: None,
};
static METRES_PER_PARSEC: f64 = 30856775814913670.0;
pub static PARSEC: Unit = Unit {
    name: "Parsec",
    dimension: Dimension::Length,
    system: System::Metric,
    to_base: Some(|v| v * METRES_PER_PARSEC),
    from_base: Some(|v| v / METRES_PER_PARSEC),
    to_system_base: None,
    from_system_base: None,
};

// Imperial units

static YARDS_PER_METRE: f64 = 1.093613;
pub static YARD: Unit = Unit {
    name: "Yard",
    dimension: Dimension::Length,
    system: System::Imperial,
    to_base: Some(|v| v / YARDS_PER_METRE),
    from_base: Some(|v| v * YARDS_PER_METRE),
    to_system_base: None,
    from_system_base: None,
};
pub static FOOT: Unit = Unit {
    name: "Foot",
    dimension: Dimension::Length,
    system: System::Imperial,
    to_base: Some(|v| v / (YARDS_PER_METRE * 3.0)),
    from_base: Some(|v| v * (YARDS_PER_METRE * 3.0)),
    to_system_base: Some(|v| v / 3.0),
    from_system_base: Some(|v| v * 3.0),
};
pub static INCH: Unit = Unit {
    name: "Inch",
    dimension: Dimension::Length,
    system: System::Imperial,
    to_base: Some(|v| v / (YARDS_PER_METRE * 36.0)),
    from_base: Some(|v| v * (YARDS_PER_METRE * 36.0)),
    to_system_base: Some(|v| v / 36.0),
    from_system_base: Some(|v| v * 36.0),
};
pub static MILE: Unit = Unit {
    name: "Mile",
    dimension: Dimension::Length,
    system: System::Imperial,
    to_base: Some(|v| v / (YARDS_PER_METRE / 1760.0)),
    from_base: Some(|v| v * (YARDS_PER_METRE / 1760.0)),
    to_system_base: Some(|v| v * 1760.0),
    from_system_base: Some(|v| v / 1760.0),
};
pub static NAUTICAL_MILE: Unit = Unit {
    name: "Nm",
    dimension: Dimension::Length,
    system: System::Imperial,
    to_base: Some(|v| v / (YARDS_PER_METRE / 2025.373)),
    from_base: Some(|v| v * (YARDS_PER_METRE / 2025.373)),
    to_system_base: Some(|v| v * 2025.373),
    from_system_base: Some(|v| v / 2025.373),
};

pub(crate) fn get_all() -> Vec<&'static Unit> {
    vec![&METRE, &CENTIMETRE, &MILLIMETRE, &MICROMETRE,
         &KILOMETRE, &LIGHTYEAR, &PARSEC, &YARD,
         &FOOT, &INCH, &MILE, &NAUTICAL_MILE
    ]
}

#[cfg(test)]
mod tests {
    use crate::assert_near;
    use crate::conversions::convert;
    use crate::conversions::length::*;

    #[test]
    fn test_all_metric_lengths() {
        assert_eq!(convert(&23.66, &MICROMETRE, &MILLIMETRE), 0.02366);
        assert_eq!(convert(&23.66, &MILLIMETRE, &METRE), 0.02366);
        assert_eq!(convert(&23.66, &CENTIMETRE, &METRE), 0.2366);
        assert_eq!(convert(&23.66, &METRE, &KILOMETRE), 0.02366);
        assert_near!(convert(&23.66, &KILOMETRE, &LIGHTYEAR), 2.500864e-12, 1e-8);
        assert_near!(convert(&23.66, &KILOMETRE, &PARSEC), 7.667684e-13, 1e-8);
        // and back
        assert_near!(convert(&23.66, &PARSEC, &KILOMETRE), 7.300713e+14, 1e8);
        assert_near!(convert(&23.66, &LIGHTYEAR, &KILOMETRE), 2.238409e+14, 1e8);
        assert_eq!(convert(&23.66, &KILOMETRE, &METRE), 23660.0);
        assert_eq!(convert(&23.66, &METRE, &CENTIMETRE), 2366.0);
        assert_eq!(convert(&23.66, &METRE, &MILLIMETRE), 23660.0);
        assert_eq!(convert(&23.66, &MILLIMETRE, &MICROMETRE), 23660.0);
    }
    #[test]
    fn test_all_imperial_lengths() {
        assert_eq!(convert(&23.66, &INCH, &FOOT), 23.66 / 12.0);
        assert_eq!(convert(&23.66, &FOOT, &INCH), 23.66 * 12.0);
        assert_eq!(convert(&23.66, &INCH, &FOOT), 23.66 / 12.0);
        assert_eq!(convert(&23.66, &YARD, &INCH), 23.66 * 36.0);
        assert_near!(convert(&23.66, &YARD, &MILE), 0.01344318);
        assert_near!(convert(&23.66, &MILE, &INCH), 1499097.6);

    }
    #[test]
    fn test_all_imperial_to_metre() {
        assert_near!(convert(&23.66, &INCH, &METRE), 0.600964);
        assert_near!(convert(&23.66, &FOOT, &METRE), 7.211569);
        assert_near!(convert(&23.66, &YARD, &METRE), 21.634710);
        assert_near!(convert(&23.66, &MILE, &METRE), 38077.08, 1e-2);
        assert_near!(convert(&23.66, &NAUTICAL_MILE, &METRE), 43818.36, 1e-2);
        // and back
        assert_near!(convert(&23.66, &METRE, &INCH), 931.4958089);
        assert_near!(convert(&23.66, &METRE, &FOOT), 931.4958089 / 12.0);
        assert_near!(convert(&23.66, &METRE, &YARD), 931.4958089 / 36.0);
        assert_near!(convert(&23.66, &METRE, &MILE), 0.01470164241);
        assert_near!(convert(&23.66, &METRE, &NAUTICAL_MILE), 0.01277537797);

    }
}