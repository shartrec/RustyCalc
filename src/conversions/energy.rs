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

// Energy unit constants
pub static JOULE: Unit = Unit {
    name: "Joule",
    dimension: Dimension::Energy,
    system: System::Metric,
    to_base: None,
    from_base: None,
    to_system_base: None,
    from_system_base: None,
};
pub static KILOJOULE: Unit = Unit {
    name: "Kilojoule",
    dimension: Dimension::Energy,
    system: System::Metric,
    to_base: Some(Unit::from_kilo),
    from_base: Some(Unit::to_kilo),
    to_system_base: None,
    from_system_base: None,
};
pub static MEGAJOULE: Unit = Unit {
    name: "Megajoule",
    dimension: Dimension::Energy,
    system: System::Metric,
    to_base: Some(Unit::from_mega),
    from_base: Some(Unit::to_mega),
    to_system_base: None,
    from_system_base: None,
};
pub static GIGAJOULE: Unit = Unit {
    name: "Gigajoule",
    dimension: Dimension::Energy,
    system: System::Metric,
    to_base: Some(Unit::from_giga),
    from_base: Some(Unit::to_giga),
    to_system_base: None,
    from_system_base: None,
};

// Imperial units
pub static BRITISH_THERMAL_UNIT: Unit = Unit {
    name: "BTU",
    dimension: Dimension::Energy,
    system: System::Imperial,
    to_base: Some(|v| v * 1055.05585262),
    from_base: Some(|v| v / 1055.05585262),
    to_system_base: None,
    from_system_base: None,
};
pub static CALORIE: Unit = Unit {
    name: "Calorie",
    dimension: Dimension::Energy,
    system: System::Imperial,
    to_base: Some(|v| v * 4.184),
    from_base: Some(|v| v / 4.184),
    to_system_base: None,
    from_system_base: None,
};

pub(crate) fn get_all() -> Vec<&'static Unit> {
    vec![&JOULE, &KILOJOULE, &MEGAJOULE, &GIGAJOULE,
         &BRITISH_THERMAL_UNIT, &CALORIE,
    ]
}

#[cfg(test)]
mod tests {
    use crate::assert_near;
    use crate::conversions::convert;
    use crate::conversions::energy::{BRITISH_THERMAL_UNIT, CALORIE, GIGAJOULE, JOULE, KILOJOULE, MEGAJOULE};

    #[test]
    fn test_metric_energy_units() {
        assert_near!(convert(&1.0, &KILOJOULE, &JOULE), 1000.0);
        assert_near!(convert(&1.0, &MEGAJOULE, &JOULE), 1_000_000.0);
        assert_near!(convert(&1.0, &GIGAJOULE, &JOULE), 1_000_000_000.0);
    }

    #[test]
    fn test_imperial_energy_units() {
        assert_near!(convert(&1.0, &BRITISH_THERMAL_UNIT, &JOULE), 1055.05585262);
        assert_near!(convert(&1000.0, &CALORIE, &JOULE), 4184.0);
    }

    #[test]
    fn test_metric_to_imperial_energy_units() {
        assert_near!(convert(&1.0, &JOULE, &BRITISH_THERMAL_UNIT), 0.00094781712);
        assert_near!(convert(&1000.0, &JOULE, &BRITISH_THERMAL_UNIT), 0.94781712);
        assert_near!(convert(&1.0, &KILOJOULE, &BRITISH_THERMAL_UNIT), 0.94781712);
    }

    #[test]
    fn test_imperial_to_metric_energy_units() {
        assert_near!(convert(&1.0, &BRITISH_THERMAL_UNIT, &JOULE), 1055.05585262);
        assert_near!(convert(&10.0, &BRITISH_THERMAL_UNIT, &KILOJOULE), 10.5505585262);
        assert_near!(convert(&100.0, &BRITISH_THERMAL_UNIT, &MEGAJOULE), 0.105505585262);
    }
}
