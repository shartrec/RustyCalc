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

pub const KILOGRAM: Unit = Unit {
    name: "Kilogram",
    dimension: Dimension::Mass,
    system: System::Metric,
    to_base: None,
    from_base: None,
    to_system_base: None,
    from_system_base: None,
};
pub const GRAM: Unit = Unit {
    name: "Gram",
    dimension: Dimension::Mass,
    system: System::Metric,
    to_base: Some(Unit::to_kilo),
    from_base: Some(Unit::from_kilo),
    to_system_base: None,
    from_system_base: None,
};
pub const MILLIGRAM: Unit = Unit {
    name: "Milligram",
    dimension: Dimension::Mass,
    system: System::Metric,
    to_base: Some(|v| GRAM.to_base.unwrap()(Unit::from_milli(v))),
    from_base: Some(|v| Unit::to_milli(GRAM.from_base.unwrap()(v))),
    to_system_base: None,
    from_system_base: None,
};
pub const MICROGRAM: Unit = Unit {
    name: "Microgram",
    dimension: Dimension::Mass,
    system: System::Metric,
    to_base: Some(|v| GRAM.to_base.unwrap()(Unit::from_micro(v))),
    from_base: Some(|v| Unit::to_micro(GRAM.from_base.unwrap()(v))),
    to_system_base: None,
    from_system_base: None,
};
pub const TONNE: Unit = Unit {
    name: "Ton (metric)",
    dimension: Dimension::Mass,
    system: System::Metric,
    to_base: Some(Unit::from_kilo),
    from_base: Some(Unit::to_kilo),
    to_system_base: None,
    from_system_base: None,
};
pub const KILOTONNE: Unit = Unit {
    name: "Kiloton (metric)",
    dimension: Dimension::Mass,
    system: System::Metric,
    to_base: Some(|v| TONNE.to_base.unwrap()(Unit::from_kilo(v))),
    from_base: Some(|v| Unit::to_kilo(TONNE.from_base.unwrap()(v))),
    to_system_base: None,
    from_system_base: None,
};
pub const MEGATONNE: Unit = Unit {
    name: "Megaton (metric)",
    dimension: Dimension::Mass,
    system: System::Metric,
    to_base: Some(|v| TONNE.to_base.unwrap()(Unit::from_mega(v))),
    from_base: Some(|v| Unit::to_mega(TONNE.from_base.unwrap()(v))),
    to_system_base: None,
    from_system_base: None,
};
pub const GIGATONNE: Unit = Unit {
    name: "Gigaton (metric)",
    dimension: Dimension::Mass,
    system: System::Metric,
    to_base: Some(|v| TONNE.to_base.unwrap()(Unit::from_giga(v))),
    from_base: Some(|v| Unit::to_giga(TONNE.from_base.unwrap()(v))),
    to_system_base: None,
    from_system_base: None,
};

// Imperial

pub const OUNCES_PER_KILO: f64 = 35.2739619495804;
pub const OUNCE: Unit = Unit {
    name: "Ounce",
    dimension: Dimension::Mass,
    system: System::Imperial,
    to_base: Some(|v| v / OUNCES_PER_KILO),
    from_base: Some(|v| v * OUNCES_PER_KILO),
    to_system_base: None,
    from_system_base: None,
};
pub const POUND: Unit = Unit {
    name: "Pound",
    dimension: Dimension::Mass,
    system: System::Imperial,
    to_base: Some( |v| v * 16.0 / OUNCES_PER_KILO),
    from_base: Some( |v| v * OUNCES_PER_KILO / 16.0),
    to_system_base: Some(|v| v * 16.0),
    from_system_base: Some(|v| v / 16.0),
};
pub const TON: Unit = Unit {
    name: "Long Ton",
    dimension: Dimension::Mass,
    system: System::Imperial,
    to_base: Some(|v| v * 2240.0 * 16.0 / OUNCES_PER_KILO),
    from_base: Some(|v| v * OUNCES_PER_KILO / (2240.0 * 16.0)),
    to_system_base: Some(|v| v * 2240.0 * 16.0),
    from_system_base: Some(|v| v / (2240.0 * 16.0)),
};
pub const TON_SHORT: Unit = Unit {
    name: "Short Ton",
    dimension: Dimension::Mass,
    system: System::Imperial,
    to_base: Some(|v| v * 2000.0 * 16.0 / OUNCES_PER_KILO),
    from_base: Some(|v| v * OUNCES_PER_KILO / (2000.0 * 16.0)),
    to_system_base: Some(|v| v * 2000.0 * 16.0),
    from_system_base: Some(|v| v / (2000.0 * 16.0)),
};

pub(crate) fn get_all() -> Vec<&'static Unit> {
    vec![&KILOGRAM, &GRAM, &MILLIGRAM, &MICROGRAM,
         &TONNE, &KILOTONNE, &MEGATONNE, &GIGATONNE,
         &OUNCE, &POUND, &TON, &TON_SHORT
    ]
}

#[cfg(test)]
mod tests {
    use crate::assert_near;
    use crate::conversions::convert;
    use crate::conversions::mass::*;

    #[test]
    fn test_all_metric_weights() {
        assert_eq!(convert(&23.66, &MICROGRAM, &MILLIGRAM), 0.02366);
        assert_eq!(convert(&23.66, &MILLIGRAM, &GRAM), 0.02366);
        assert_eq!(convert(&23.66, &GRAM, &KILOGRAM), 0.02366);
        assert_eq!(convert(&23.66, &KILOGRAM, &TONNE), 0.02366);
        assert_eq!(convert(&23.66, &TONNE, &KILOTONNE), 0.02366);
        assert_eq!(convert(&23.66, &KILOTONNE, &MEGATONNE), 0.02366);
        // and back
        assert_eq!(convert(&23.66, &MEGATONNE, &KILOTONNE), 23660.0);
        assert_eq!(convert(&23.66, &KILOTONNE, &TONNE), 23660.0);
        assert_eq!(convert(&23.66, &TONNE, &KILOGRAM), 23660.0);
        assert_eq!(convert(&23.66, &KILOGRAM, &GRAM), 23660.0);
        assert_eq!(convert(&23.66, &GRAM, &MILLIGRAM), 23660.0);
        assert_eq!(convert(&23.66, &MILLIGRAM, &MICROGRAM), 23660.0);
    }
    #[test]
    fn test_all_imperial_weights() {
        assert_eq!(convert(&23.66, &OUNCE, &POUND), 23.66 / 16.0);
        assert_eq!(convert(&23.66, &POUND, &OUNCE), 23.66 * 16.0);
        assert_eq!(convert(&23.66, &TON, &POUND), 23.66 * 2240.0);
        assert_eq!(convert(&23.66, &POUND, &TON), 23.66 / 2240.0);

    }
    #[test]
    fn test_all_imperial_to_kilo() {
        assert_near!(convert(&23.66, &OUNCE, &KILOGRAM), 23.66 / OUNCES_PER_KILO);
        assert_near!(convert(&23.66, &POUND, &KILOGRAM), 23.66 / OUNCES_PER_KILO * 16.0);
        assert_near!(convert(&23.66, &TON, &KILOGRAM), 23.66 / OUNCES_PER_KILO * 16.0 * 2240.0);
        assert_near!(convert(&23.66, &TON_SHORT, &KILOGRAM), 23.66 / OUNCES_PER_KILO * 16.0 * 2000.0);
        // and back
        assert_near!(convert(&23.66, &KILOGRAM, &OUNCE), 23.66 * OUNCES_PER_KILO);
        assert_near!(convert(&23.66, &KILOGRAM, &POUND), 23.66 * OUNCES_PER_KILO / 16.0);
        assert_near!(convert(&23.66, &KILOGRAM, &TON), 23.66 * OUNCES_PER_KILO / (2240.0 * 16.0));
        assert_near!(convert(&23.66, &KILOGRAM, &TON_SHORT), 23.66 * OUNCES_PER_KILO / (2000.0 * 16.0));

    }
}
