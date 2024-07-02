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

// Power unit constants
pub static WATT: Unit = Unit {
    name: "Watt",
    dimension: Dimension::Power,
    system: System::Metric,
    to_base: None,
    from_base: None,
    to_system_base: None,
    from_system_base: None,
};
pub static KILOWATT: Unit = Unit {
    name: "Kilowatt",
    dimension: Dimension::Power,
    system: System::Metric,
    to_base: Some(Unit::from_kilo),
    from_base: Some(Unit::to_kilo),
    to_system_base: None,
    from_system_base: None,
};
pub static MEGAWATT: Unit = Unit {
    name: "Megawatt",
    dimension: Dimension::Power,
    system: System::Metric,
    to_base: Some(Unit::from_mega),
    from_base: Some(Unit::to_mega),
    to_system_base: None,
    from_system_base: None,
};
pub static GIGAWATT: Unit = Unit {
    name: "Gigawatt",
    dimension: Dimension::Power,
    system: System::Metric,
    to_base: Some(Unit::from_giga),
    from_base: Some(Unit::to_giga),
    to_system_base: None,
    from_system_base: None,
};

// Imperial units
pub static HORSEPOWER: Unit = Unit {
    name: "Horsepower",
    dimension: Dimension::Power,
    system: System::Imperial,
    to_base: Some(|v| v * 745.699872),
    from_base: Some(|v| v / 745.699872),
    to_system_base: None,
    from_system_base: None,
};

pub(crate) fn get_all() -> Vec<&'static Unit> {
    vec![&WATT, &KILOWATT, &MEGAWATT, &GIGAWATT,
         &HORSEPOWER,
    ]
}

#[cfg(test)]
mod tests {
    use crate::assert_near;
    use crate::conversions::convert;
    use crate::conversions::power::{GIGAWATT, HORSEPOWER, KILOWATT, MEGAWATT, WATT};

    #[test]
    fn test_metric_power_units() {
        assert_near!(convert(&1.0, &KILOWATT, &WATT), 1000.0);
        assert_near!(convert(&1.0, &MEGAWATT, &WATT), 1_000_000.0);
        assert_near!(convert(&1.0, &GIGAWATT, &WATT), 1_000_000_000.0);
    }

    #[test]
    fn test_imperial_power_units() {
        assert_near!(convert(&1.0, &HORSEPOWER, &WATT), 745.699872);
        assert_near!(convert(&10.0, &HORSEPOWER, &KILOWATT), 7.45699872);
    }

    #[test]
    fn test_metric_to_imperial_power_units() {
        assert_near!(convert(&1.0, &WATT, &HORSEPOWER), 0.00134102209);
        assert_near!(convert(&1000.0, &WATT, &HORSEPOWER), 1.34102209);
        assert_near!(convert(&1.0, &KILOWATT, &HORSEPOWER), 1.34102209);
    }

    #[test]
    fn test_imperial_to_metric_power_units() {
        assert_near!(convert(&1.0, &HORSEPOWER, &WATT), 745.699872);
        assert_near!(convert(&10.0, &HORSEPOWER, &KILOWATT), 7.45699872);
        assert_near!(convert(&100.0, &HORSEPOWER, &MEGAWATT), 0.0745699872);
    }
}
