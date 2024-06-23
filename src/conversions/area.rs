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
use crate::conversions::length::*;

/// These are the units of the weight dimension.
/// We define each of these as static constants for ease of use elsewhere and because they are
/// intrinsically immutable

pub const SQ_METRE: Unit = Unit {
    name: "sq_metre",
    dimension: Dimension::Length,
    system: System::Metric,
    to_base: None,
    from_base: None,
    to_system_base: None,
    from_system_base: None,
};
pub const SQ_CENTIMETRE: Unit = Unit {
    name: "sq_centimetre",
    dimension: Dimension::Length,
    system: System::Metric,
    to_base: Some(|v| CENTIMETRE.to_base.unwrap()(CENTIMETRE.to_base.unwrap()(v))),
    from_base: Some(|v| CENTIMETRE.from_base.unwrap()(CENTIMETRE.from_base.unwrap()(v))),
    to_system_base: None,
    from_system_base: None,
};
pub const SQ_MILLIMETRE: Unit = Unit {
    name: "sq_millimetre",
    dimension: Dimension::Length,
    system: System::Metric,
    to_base: Some(|v| MILLIMETRE.to_base.unwrap()(MILLIMETRE.to_base.unwrap()(v))),
    from_base: Some(|v| MILLIMETRE.from_base.unwrap()(MILLIMETRE.from_base.unwrap()(v))),
    to_system_base: None,
    from_system_base: None,
};
pub const SQ_MICROMETRE: Unit = Unit {
    name: "sq_micrometre",
    dimension: Dimension::Length,
    system: System::Metric,
    to_base: Some(|v| MICROMETRE.to_base.unwrap()(MICROMETRE.to_base.unwrap()(v))),
    from_base: Some(|v| MICROMETRE.from_base.unwrap()(MICROMETRE.from_base.unwrap()(v))),
    to_system_base: None,
    from_system_base: None,
};
pub const HECTARE: Unit = Unit {
    name: "hectare",
    dimension: Dimension::Length,
    system: System::Metric,
    to_base: Some(|v| v * 10000.0),
    from_base: Some(|v| v / 10000.0),
    to_system_base: None,
    from_system_base: None,
};

pub const SQ_KILOMETRE: Unit = Unit {
    name: "sq_kilometre",
    dimension: Dimension::Length,
    system: System::Metric,
    to_base: Some(|v| KILOMETRE.to_base.unwrap()(KILOMETRE.to_base.unwrap()(v))),
    from_base: Some(|v| KILOMETRE.from_base.unwrap()(KILOMETRE.from_base.unwrap()(v))),
    to_system_base: None,
    from_system_base: None,
};

// Imperial units

pub const SQ_YARD: Unit = Unit {
    name: "sq_yard",
    dimension: Dimension::Length,
    system: System::Imperial,
    to_base: Some(|v| YARD.to_base.unwrap()(YARD.to_base.unwrap()(v))),
    from_base: Some(|v| YARD.from_base.unwrap()(YARD.from_base.unwrap()(v))),
    to_system_base: None,
    from_system_base: None,
};
pub const SQ_FOOT: Unit = Unit {
    name: "sq_foot",
    dimension: Dimension::Length,
    system: System::Imperial,
    to_base: Some(|v| FOOT.to_base.unwrap()(FOOT.to_base.unwrap()(v))),
    from_base: Some(|v| FOOT.from_base.unwrap()(FOOT.from_base.unwrap()(v))),
    to_system_base: Some(|v| FOOT.to_system_base.unwrap()(FOOT.to_system_base.unwrap()(v))),
    from_system_base: Some(|v| FOOT.to_system_base.unwrap()(FOOT.to_system_base.unwrap()(v))),
};
pub const SQ_INCH: Unit = Unit {
    name: "sq_inch",
    dimension: Dimension::Length,
    system: System::Imperial,
    to_base: Some(|v| INCH.to_base.unwrap()(INCH.to_base.unwrap()(v))),
    from_base: Some(|v| INCH.from_base.unwrap()(INCH.from_base.unwrap()(v))),
    to_system_base: Some(|v| INCH.to_system_base.unwrap()(INCH.to_system_base.unwrap()(v))),
    from_system_base: Some(|v| INCH.to_system_base.unwrap()(INCH.to_system_base.unwrap()(v))),
};
pub const SQ_MILE: Unit = Unit {
    name: "sq_mile",
    dimension: Dimension::Length,
    system: System::Imperial,
    to_base: Some(|v| MILE.to_base.unwrap()(MILE.to_base.unwrap()(v))),
    from_base: Some(|v| MILE.from_base.unwrap()(MILE.from_base.unwrap()(v))),
    to_system_base: Some(|v| MILE.to_system_base.unwrap()(MILE.to_system_base.unwrap()(v))),
    from_system_base: Some(|v| MILE.to_system_base.unwrap()(MILE.to_system_base.unwrap()(v))),
};
pub const SQ_NAUTICAL_MILE: Unit = Unit {
    name: "sq_nm",
    dimension: Dimension::Length,
    system: System::Imperial,
    to_base: Some(|v| NAUTICAL_MILE.to_base.unwrap()(NAUTICAL_MILE.to_base.unwrap()(v))),
    from_base: Some(|v| NAUTICAL_MILE.from_base.unwrap()(NAUTICAL_MILE.from_base.unwrap()(v))),
    to_system_base: Some(|v| NAUTICAL_MILE.to_system_base.unwrap()(NAUTICAL_MILE.to_system_base.unwrap()(v))),
    from_system_base: Some(|v| NAUTICAL_MILE.to_system_base.unwrap()(NAUTICAL_MILE.to_system_base.unwrap()(v))),
};

#[cfg(test)]
mod tests {
    use crate::assert_near;
    use crate::conversions::area::*;
    use crate::conversions::convert;

    pub const YARDS_PER_METRE: f64 = 1.093613;

    #[test]
    fn test_all_metric_areas() {
        assert_near!(convert(&23.66, &SQ_MICROMETRE, &SQ_MILLIMETRE), 0.00002366, 1e-10);
        assert_near!(convert(&23.66, &SQ_MILLIMETRE, &SQ_METRE), 0.00002366, 1e-10);
        assert_near!(convert(&23.66, &SQ_CENTIMETRE, &SQ_METRE), 0.002366, 1e-10);
        assert_near!(convert(&23.66, &SQ_METRE, &SQ_KILOMETRE), 0.00002366, 1e-10);
        assert_eq!(convert(&23.66, &SQ_KILOMETRE, &HECTARE), 2366.0);
        assert_eq!(convert(&23.66, &HECTARE, &SQ_METRE), 236600.0);
        // and back
        assert_eq!(convert(&23.66, &HECTARE, &SQ_KILOMETRE), 0.2366);
        assert_eq!(convert(&23.66, &SQ_METRE, &HECTARE), 0.002366);
        assert_eq!(convert(&23.66, &SQ_KILOMETRE, &SQ_METRE), 23660000.0);
        assert_eq!(convert(&23.66, &SQ_METRE, &SQ_CENTIMETRE), 236600.0);
        assert_eq!(convert(&23.66, &SQ_METRE, &SQ_MILLIMETRE), 23660000.0);
        assert_eq!(convert(&23.66, &SQ_MILLIMETRE, &SQ_MICROMETRE), 23660000.0);
    }
    #[test]
    fn test_all_imperial_areas() {
        assert_eq!(convert(&23.66, &SQ_YARD, &SQ_FOOT), 23.66 * 9.0);
        assert_eq!(convert(&23.66, &SQ_INCH, &SQ_FOOT), 23.66 / 1144.0);
        assert_eq!(convert(&23.66, &SQ_FOOT, &SQ_INCH), 23.66 * 144.0);
        assert_eq!(convert(&23.66, &SQ_INCH, &SQ_FOOT), 23.66 / 1144.0);
        assert_eq!(convert(&23.66, &SQ_YARD, &SQ_INCH), 23.66 * 1296.0);
        assert_near!(convert(&23.66, &SQ_YARD, &SQ_MILE), 0.01344318_f64.powf(2.0));
        assert_near!(convert(&23.66, &SQ_MILE, &SQ_INCH), 1499097.6_f64.powf(2.0));

    }
    #[test]
    fn test_all_imperial_to_metre() {
        assert_near!(convert(&23.66, &SQ_INCH, &SQ_METRE), 23.66 / (YARDS_PER_METRE * 36.0).powf(2.0));
        assert_near!(convert(&23.66, &SQ_FOOT, &SQ_METRE), 23.66 / (YARDS_PER_METRE * 3.0).powf(2.0));
        assert_near!(convert(&23.66, &SQ_YARD, &SQ_METRE), 23.66 / YARDS_PER_METRE.powf(2.0));
        assert_near!(convert(&23.66, &SQ_MILE, &SQ_KILOMETRE), 23.66 * 1.609344439_f64.powf(2.0));
        assert_near!(convert(&23.66, &SQ_NAUTICAL_MILE, &SQ_KILOMETRE), 23.66 * 1.852001576_f64.powf(2.0));
        // and back
        assert_near!(convert(&23.66, &SQ_METRE, &SQ_INCH), 23.66 * (YARDS_PER_METRE * 36.0).powf(2.0));
        assert_near!(convert(&23.66, &SQ_METRE, &SQ_FOOT), 23.66 * (YARDS_PER_METRE * 3.0).powf(2.0));
        assert_near!(convert(&23.66, &SQ_METRE, &SQ_YARD), 23.66 * YARDS_PER_METRE.powf(2.0));
        assert_near!(convert(&23.66, &SQ_KILOMETRE, &SQ_MILE), 23.66 / 1.609344439_f64.powf(2.0));
        assert_near!(convert(&23.66, &SQ_KILOMETRE, &SQ_NAUTICAL_MILE), 23.66 / 1.852001576_f64.powf(2.0));
    }
}