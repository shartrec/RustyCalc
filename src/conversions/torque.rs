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

// Torque unit constants
pub static NEWTON_METRE: Unit = Unit {
    name: "newton_metre",
    dimension: Dimension::Torque,
    system: System::Metric,
    to_base: None,
    from_base: None,
    to_system_base: None,
    from_system_base: None,
};

// Imperial units
pub static FOOT_POUND: Unit = Unit {
    name: "foot_pound",
    dimension: Dimension::Torque,
    system: System::Imperial,
    to_base: Some(|v| v * 1.3558179483314),
    from_base: Some(|v| v / 1.3558179483314),
    to_system_base: None,
    from_system_base: None,
};
pub static INCH_POUND: Unit = Unit {
    name: "inch_pound",
    dimension: Dimension::Torque,
    system: System::Imperial,
    to_base: Some(|v| v * 0.1129848290276167),
    from_base: Some(|v| v / 0.1129848290276167),
    to_system_base: Some(|v| v / 12.0),
    from_system_base: Some(|v| v * 12.0),
};

pub(crate) fn get_all() -> Vec<&'static Unit> {
    vec![&NEWTON_METRE, &FOOT_POUND, &INCH_POUND,
    ]
}

#[cfg(test)]
mod tests {
    use crate::assert_near;
    use crate::conversions::convert;
    use crate::conversions::torque::{FOOT_POUND, INCH_POUND, NEWTON_METRE};

    #[test]
    fn test_imperial_torque_units() {
        assert_near!(convert(&1.0, &FOOT_POUND, &NEWTON_METRE), 1.3558179483314);
        assert_near!(convert(&12.0, &INCH_POUND, &FOOT_POUND), 1.0);
    }

    #[test]
    fn test_metric_to_imperial_torque_units() {
        assert_near!(convert(&1.0, &NEWTON_METRE, &FOOT_POUND), 0.7375621492772656);
        assert_near!(convert(&1000.0, &NEWTON_METRE, &FOOT_POUND), 737.5621492772656);
    }

    #[test]
    fn test_imperial_to_metric_torque_units() {
        assert_near!(convert(&1.0, &FOOT_POUND, &NEWTON_METRE), 1.3558179483314);
    }
}
