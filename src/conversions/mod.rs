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
#![allow(dead_code)]

use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};

use log::warn;
use strum_macros::{Display, EnumIter};

use crate::conversions::System::Metric;

pub(crate) mod mass;
pub(crate) mod length;
pub(crate) mod area;
pub(crate) mod temperature;
pub(crate) mod volume;
mod power;
mod torque;
mod force;
mod energy;

/// There are multiple measurement systems.
/// You can convert both within and between measurement systems.
/// While conversions with in a system are usually whole number factors, 12 inches = 1 foot,
/// conversions between system are often not exact, and require many decimal places.
/// We acknowledge this difference and so convert between units of the same system using a base within
/// that system, otherwise we use a common SI base if possible.
#[derive(Debug, Default, PartialEq, Clone)]
pub(crate) enum System {
    #[default]
    Metric,
    Imperial,
    US, // Only used for Volume
}

impl System {
    fn is_default(&self) -> bool {
        self == &Metric
    }
}
/// Dimension describe the physical attribute that can be measured.
/// You can only convert from one unit to another if it is in the same dimension
#[derive(Clone, Debug, Default, Display, EnumIter, PartialEq)]
pub(crate) enum Dimension {
    #[default]
    Length,
    Area,
    Mass,
    Volume,
    Temperature,
    Power,
    Torque,
    Force,
    Energy
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub(crate) enum ConversionDirection {
    #[default]
    From,
    To,
}

/// A unit is the basic unit that can be converted to or from
///
#[derive(Debug, Default, Clone)]
pub struct Unit {
    pub(crate) name: &'static str,
    /// The dimension the unit measures
    dimension: Dimension,
    /// The system the unit belongs to
    system: System,
    /// Function that converts this unit to the base unit
    /// All Units that share a dimension must convert to the same base.
    to_base: Option<fn(f64) -> f64>,
    /// Function that converts this unit from the base unit
    from_base: Option<fn(f64) -> f64>,
    /// Function that converts this unit to the system base unit
    /// All Units that share a system and dimension must convert to the same system base.
    to_system_base: Option<fn(f64) -> f64>,
    /// Function that converts this unit from the system base unit
    from_system_base: Option<fn(f64) -> f64>,
}
impl Unit {
    /// Convenience functions to convert to milli, micro, kilo, mega etc
    fn to_milli(v: f64) -> f64 {
        v * 1e3
    }
    fn from_milli(v: f64) -> f64 {
        v / 1e3
    }
    fn to_micro(v: f64) -> f64 {
        v * 1e6
    }
    fn from_micro(v: f64) -> f64 {
        v / 1e6
    }
    fn to_nano(v: f64) -> f64 {
        v * 1e9
    }
    fn from_nano(v: f64) -> f64 {
        v / 1e9
    }
    fn to_kilo(v: f64) -> f64 {
        v / 1e3
    }
    fn from_kilo(v: f64) -> f64 {
        v * 1e3
    }
    fn to_mega(v: f64) -> f64 {
        v / 1e6
    }
    fn from_mega(v: f64) -> f64 {
        v * 1e6
    }
    fn to_giga(v: f64) -> f64 {
        v / 1e9
    }
    fn from_giga(v: f64) -> f64 {
        v * 1e9
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name)
    }
}
impl PartialEq for Unit {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

pub(crate) fn convert(value: &f64, from: &Unit, to: &Unit) -> f64 {
    if from == to {
        return value.clone()
    }

    // Let's get the functions we need to convert the value to and from the base unit
    // Funtions may be None if the unit is the base
    let (to_base, from_base) =

        // We need to see if these are of the same system and have a system base unit.
        if from.system == to.system && from.dimension == to.dimension && !from.system.is_default() {
            (&from.to_system_base, &to.from_system_base)
        } else {
            (&from.to_base, &to.from_base)
        }
    ;

    let mut result = *value;
    if let Some(f) = to_base {
        result = f(result);
    }
    if let Some(f) = from_base {
        result = f(result);
    }
    result

}

pub(crate) fn get_units(dimension: &Option<Dimension>) -> Vec<&'static Unit> {
    match dimension {
        Some(Dimension::Length) => {
            length::get_all()
        }
        Some(Dimension::Area) => {
            area::get_all()
        }
        Some(Dimension::Mass) => {
            mass::get_all()
        }
        Some(Dimension::Volume) => {
            volume::get_all()
        }
        Some(Dimension::Temperature) => {
            temperature::get_all()
        }
        Some(Dimension::Power) => {
            power::get_all()
        }
        Some(Dimension::Torque) => {
            torque::get_all()
        }
        Some(Dimension::Force) => {
            force::get_all()
        }
        Some(Dimension::Energy) => {
            energy::get_all()
        }
        None => {
            warn!("Trying to convert, but no dimension set");
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_near;
    use crate::conversions::convert;
    use crate::conversions::mass::*;

    #[test]
    fn test_fn() {
        assert_near!(convert(&1.0, &OUNCE, &GRAM), 28.349523);
        assert_eq!(convert(&2.0, &OUNCE, &POUND), 0.125);
    }
    #[test]
    fn test_base_units() {
        assert_eq!(convert(&1.0, &KILOGRAM, &GRAM), 1000.0);
        assert_eq!(convert(&2.0, &KILOTONNE, &KILOGRAM), 2000000.0);
    }
    #[test]
    fn test_mixed_units() {
        assert_near!(convert(&1.0, &TONNE, &TON), 0.984207);
        assert_eq!(convert(&2.0, &TON, &TON_SHORT), 2.240);
    }
}
