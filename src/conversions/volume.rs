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

pub static LITRE: Unit = Unit {
    name: "Litre",
    dimension: Dimension::Volume,
    system: System::Metric,
    to_base: None,
    from_base: None,
    to_system_base: None,
    from_system_base: None,
};
pub static CU_METRE: Unit = Unit {
    name: "Cubic Metre",
    dimension: Dimension::Volume,
    system: System::Metric,
    to_base: Some(|v| v * 1000.0),
    from_base: Some(|v| v / 1000.0),
    to_system_base: None,
    from_system_base: None,
};
pub static CUBIC_CENTIMETRE: Unit = Unit {
    name: "CC",
    dimension: Dimension::Volume,
    system: System::Metric,
    to_base: Some(|v| v / 1000.0),
    from_base: Some(|v| v * 1000.0),
    to_system_base: None,
    from_system_base: None,
};
pub static CUBIC_KILOMETRE: Unit = Unit {
    name: "Cubic Kilometre",
    dimension: Dimension::Volume,
    system: System::Metric,
    to_base: Some(|v| v * 1_000_000.0 * 1_000_000.0),
    from_base: Some(|v| v / (1_000_000.0 * 1_000_000.0)),
    to_system_base: None,
    from_system_base: None,
};

pub static KILO_LITRE: Unit = Unit {
    name: "Kilo Litre",
    dimension: Dimension::Volume,
    system: System::Metric,
    to_base: Some(Unit::from_kilo),
    from_base: Some(Unit::to_kilo),
    to_system_base: None,
    from_system_base: None,
};

pub static MEGA_LITRE: Unit = Unit {
    name: "Mega Litre",
    dimension: Dimension::Volume,
    system: System::Metric,
    to_base: Some(Unit::from_mega),
    from_base: Some(Unit::to_mega),
    to_system_base: None,
    from_system_base: None,
};

pub static GIGA_LITRE: Unit = Unit {
    name: "Giga Litre",
    dimension: Dimension::Volume,
    system: System::Metric,
    to_base: Some(Unit::from_giga),
    from_base: Some(Unit::to_giga),
    to_system_base: None,
    from_system_base: None,
};

// Imperial units

pub static IMP_FL_OUNCE: Unit = Unit {
    name: "Imp Fl Ounce",
    dimension: Dimension::Volume,
    system: System::Imperial,
    to_base: Some(|v| v / 35.19507973),
    from_base: Some(|v| v * 35.19507973),
    to_system_base: None,
    from_system_base: None,
};
pub static IMP_CUBIC_INCH: Unit = Unit {
    name: "Cubic Inch",
    dimension: Dimension::Volume,
    system: System::Imperial,
    to_base: Some(|v| v / 61.02374409),
    from_base: Some(|v| v * 61.02374409),
    to_system_base: Some(|v| v * 1.733871455),
    from_system_base: Some(|v| v * 1.733871455),
};
pub static IMP_PINT: Unit = Unit {
    name: "Imp Pint",
    dimension: Dimension::Volume,
    system: System::Imperial,
    to_base: Some(|v| v / 1.759753986),
    from_base: Some(|v| v * 1.759753986),
    to_system_base: Some(|v| v * 20.0),
    from_system_base: Some(|v| v / 20.0),
};
pub static IMP_QUART: Unit = Unit {
    name: "Imp Quart",
    dimension: Dimension::Volume,
    system: System::Imperial,
    to_base: Some(|v| v / 0.8798769932),
    from_base: Some(|v| v * 0.8798769932),
    to_system_base: Some(|v| v * 40.0),
    from_system_base: Some(|v| v / 40.0),
};
pub static IMP_GALLON: Unit = Unit {
    name: "Imp Gallon",
    dimension: Dimension::Volume,
    system: System::Imperial,
    to_base: Some(|v| v / 0.2199692483),
    from_base: Some(|v| v * 0.2199692483),
    to_system_base: Some(|v| v * 160.0),
    from_system_base: Some(|v| v / 160.0),
};

// US units
pub static US_FL_OUNCE: Unit = Unit {
    name: "US Fl Ounce",
    dimension: Dimension::Volume,
    system: System::US,
    to_base: Some(|v| v / 33.81402270),
    from_base: Some(|v| v * 33.81402270),
    to_system_base: None,
    from_system_base: None,
};
pub static US_PINT: Unit = Unit {
    name: "US Pint",
    dimension: Dimension::Volume,
    system: System::US,
    to_base: Some(|v| v / 2.113376419),
    from_base: Some(|v| v * 2.113376419),
    to_system_base: Some(|v| v * 16.0),
    from_system_base: Some(|v| v / 16.0),
};
pub static US_QUART: Unit = Unit {
    name: "US Quart",
    dimension: Dimension::Volume,
    system: System::US,
    to_base: Some(|v| v / 1.056688209),
    from_base: Some(|v| v * 1.056688209),
    to_system_base: Some(|v| v * 32.0),
    from_system_base: Some(|v| v / 32.0),
};
pub static US_GALLON: Unit = Unit {
    name: "US Gallon",
    dimension: Dimension::Volume,
    system: System::US,
    to_base: Some(|v| v / 0.2641720524),
    from_base: Some(|v| v * 0.2641720524),
    to_system_base: Some(|v| v * 128.0),
    from_system_base: Some(|v| v / 128.0),
};

pub(crate) fn get_all() -> Vec<&'static Unit> {
    vec![&LITRE, &CU_METRE, &CUBIC_CENTIMETRE, &CUBIC_KILOMETRE,
         &KILO_LITRE, &MEGA_LITRE, &GIGA_LITRE, &IMP_CUBIC_INCH,
         &IMP_FL_OUNCE, &IMP_PINT, &IMP_QUART, &IMP_GALLON,
         &US_FL_OUNCE, &US_PINT, &US_QUART, &US_GALLON,
    ]
}

#[cfg(test)]
mod tests {
    use crate::assert_near;
    use crate::conversions::convert;
    use crate::conversions::volume::{LITRE, CU_METRE, CUBIC_CENTIMETRE, CUBIC_KILOMETRE, KILO_LITRE, MEGA_LITRE, GIGA_LITRE};
    use crate::conversions::volume::{IMP_FL_OUNCE, IMP_CUBIC_INCH, IMP_PINT, IMP_QUART, IMP_GALLON};
    use crate::conversions::volume::{US_FL_OUNCE, US_PINT, US_QUART, US_GALLON};

    #[test]
    fn test_all_metric_volumes() {
        assert_near!(convert(&23.66, &LITRE, &CUBIC_CENTIMETRE), 23660.0);
        assert_near!(convert(&1.0, &CU_METRE, &LITRE), 1000.0);
        assert_near!(convert(&1.0, &CUBIC_CENTIMETRE, &LITRE), 0.001);
        assert_near!(convert(&1.0, &CUBIC_KILOMETRE, &MEGA_LITRE), 1_000_000.0);
        assert_near!(convert(&1.0, &KILO_LITRE, &LITRE), 1000.0);
        assert_near!(convert(&1.0, &MEGA_LITRE, &LITRE), 1_000_000.0);
        assert_near!(convert(&1.0, &GIGA_LITRE, &LITRE), 1_000_000_000.0);
    }

    #[test]
    fn test_all_imperial_volumes() {
        assert_eq!(convert(&15.5, &IMP_PINT, &IMP_GALLON), 1.9375);
        assert_near!(convert(&20.0, &IMP_FL_OUNCE, &IMP_CUBIC_INCH), 34.6774291);
        assert_near!(convert(&1.0, &IMP_PINT, &IMP_FL_OUNCE), 20.0);
        assert_near!(convert(&1.0, &IMP_QUART, &IMP_FL_OUNCE), 40.0);
        assert_near!(convert(&1.0, &IMP_GALLON, &IMP_FL_OUNCE), 160.0);
    }

    #[test]
    fn test_all_us_volumes() {
        assert_near!(convert(&1.0, &US_FL_OUNCE, &US_GALLON), 0.0078125);
        assert_near!(convert(&1.0, &US_PINT, &US_FL_OUNCE), 16.0);
        assert_near!(convert(&1.0, &US_QUART, &US_FL_OUNCE), 32.0);
        assert_near!(convert(&1.0, &US_GALLON, &US_FL_OUNCE), 128.0);
        assert_near!(convert(&10.0, &US_PINT, &US_GALLON), 1.25);
    }

    #[test]
    fn test_metric_to_imperial_volumes() {
        assert_near!(convert(&1.0, &LITRE, &IMP_PINT), 1.759753986);
        assert_near!(convert(&1.0, &CU_METRE, &IMP_GALLON), 219.9692483);
        assert_near!(convert(&1000.0, &CUBIC_CENTIMETRE, &IMP_FL_OUNCE), 35.19507973);
    }

    #[test]
    fn test_imperial_to_metric_volumes() {
        assert_near!(convert(&1.0, &IMP_PINT, &LITRE), 0.56826125);
        assert_near!(convert(&1.0, &IMP_QUART, &LITRE), 1.1365225);
        assert_near!(convert(&1.0, &IMP_GALLON, &LITRE), 4.54609);
    }

    #[test]
    fn test_metric_to_us_volumes() {
        assert_near!(convert(&1.0, &LITRE, &US_PINT), 2.113376419);
        assert_near!(convert(&1.0, &CU_METRE, &US_GALLON), 264.1720524);
        assert_near!(convert(&1000.0, &CUBIC_CENTIMETRE, &US_FL_OUNCE), 33.8140227);
    }

    #[test]
    fn test_us_to_metric_volumes() {
        assert_near!(convert(&1.0, &US_PINT, &LITRE), 0.473176473);
        assert_near!(convert(&1.0, &US_QUART, &LITRE), 0.946352946);
        assert_near!(convert(&1.0, &US_GALLON, &LITRE), 3.785411784);
    }

    #[test]
    fn test_imperial_to_us_volumes() {
        assert_near!(convert(&1.0, &IMP_PINT, &US_PINT), 1.200949925);
        assert_near!(convert(&1.0, &IMP_QUART, &US_QUART), 1.200949925);
        assert_near!(convert(&1.0, &IMP_GALLON, &US_GALLON), 1.200949925);
    }

    #[test]
    fn test_us_to_imperial_volumes() {
        assert_near!(convert(&1.0, &US_PINT, &IMP_PINT), 0.8326741846);
        assert_near!(convert(&1.0, &US_QUART, &IMP_QUART), 0.8326741846);
        assert_near!(convert(&1.0, &US_GALLON, &IMP_GALLON), 0.8326741846);
    }
}
