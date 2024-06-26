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

// Force unit constants
pub static NEWTON: Unit = Unit {
    name: "Newton",
    dimension: Dimension::Force,
    system: System::Metric,
    to_base: None,
    from_base: None,
    to_system_base: None,
    from_system_base: None,
};
pub static KILONEWTON: Unit = Unit {
    name: "Kilonewton",
    dimension: Dimension::Force,
    system: System::Metric,
    to_base: Some(Unit::from_kilo),
    from_base: Some(Unit::to_kilo),
    to_system_base: None,
    from_system_base: None,
};
pub static MEGANEWTON: Unit = Unit {
    name: "Meganewton",
    dimension: Dimension::Force,
    system: System::Metric,
    to_base: Some(Unit::from_mega),
    from_base: Some(Unit::to_mega),
    to_system_base: None,
    from_system_base: None,
};
pub static GIGANEWTON: Unit = Unit {
    name: "Giganewton",
    dimension: Dimension::Force,
    system: System::Metric,
    to_base: Some(Unit::from_giga),
    from_base: Some(Unit::to_giga),
    to_system_base: None,
    from_system_base: None,
};

// Imperial units
pub static POUND_FORCE: Unit = Unit {
    name: "Pound",
    dimension: Dimension::Force,
    system: System::Imperial,
    to_base: Some(|v| v * 4.4482216152605),
    from_base: Some(|v| v / 4.4482216152605),
    to_system_base: None,
    from_system_base: None,
};
pub static OUNCE_FORCE: Unit = Unit {
    name: "Ounce",
    dimension: Dimension::Force,
    system: System::Imperial,
    to_base: Some(|v| v * 0.278013851),
    from_base: Some(|v| v / 0.278013851),
    to_system_base: Some(|v| v / 16.0),
    from_system_base: Some(|v| v * 16.0),
};

pub(crate) fn get_all() -> Vec<&'static Unit> {
    vec![&NEWTON, &KILONEWTON, &MEGANEWTON, &GIGANEWTON,
         &POUND_FORCE, &OUNCE_FORCE,
    ]
}

#[cfg(test)]
mod tests {
    use crate::assert_near;
    use crate::conversions::convert;
    use crate::conversions::force::{NEWTON, KILONEWTON, MEGANEWTON, GIGANEWTON, POUND_FORCE, OUNCE_FORCE};

    #[test]
    fn test_metric_force_units() {
        assert_near!(convert(&1.0, &KILONEWTON, &NEWTON), 1000.0);
        assert_near!(convert(&1.0, &MEGANEWTON, &NEWTON), 1_000_000.0);
        assert_near!(convert(&1.0, &GIGANEWTON, &NEWTON), 1_000_000_000.0);
    }

    #[test]
    fn test_imperial_force_units() {
        assert_near!(convert(&1.0, &POUND_FORCE, &NEWTON), 4.4482216152605);
        assert_near!(convert(&16.0, &OUNCE_FORCE, &POUND_FORCE), 1.0);
    }

    #[test]
    fn test_metric_to_imperial_force_units() {
        assert_near!(convert(&1.0, &NEWTON, &POUND_FORCE), 0.2248089431);
        assert_near!(convert(&1000.0, &NEWTON, &POUND_FORCE), 224.8089431);
        assert_near!(convert(&1.0, &KILONEWTON, &POUND_FORCE), 224.8089431);
    }

    #[test]
    fn test_imperial_to_metric_force_units() {
        assert_near!(convert(&1.0, &POUND_FORCE, &NEWTON), 4.4482216152605);
        assert_near!(convert(&10.0, &POUND_FORCE, &KILONEWTON), 0.044482216152605);
        assert_near!(convert(&100.0, &POUND_FORCE, &MEGANEWTON), 0.00044482216152605);
    }
}
