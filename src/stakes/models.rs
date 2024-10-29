use crate::common::table::{AnySort, CycleSort, SortDirection};
use std::fmt;

#[derive(Clone)]
#[allow(dead_code)]
pub enum StakesSort {
    StakeDesc,
    StakeAsc,
}

impl SortDirection for StakesSort {
    fn is_desc(&self) -> bool {
        matches!(self, StakesSort::StakeDesc)
    }
    fn is_active(&self) -> bool {
        true
    }
}

impl fmt::Display for StakesSort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StakesSort::StakeDesc => {
                write!(f, "STAKE_DESC")
            }
            StakesSort::StakeAsc => {
                write!(f, "STAKE_ASC")
            }
        }
    }
}

impl CycleSort for StakesSort {
    fn cycle(&self) -> AnySort {
        match self {
            StakesSort::StakeDesc => AnySort::Stakes(StakesSort::StakeAsc),
            StakesSort::StakeAsc => AnySort::Stakes(StakesSort::StakeDesc),
        }
    }
}

impl TryFrom<String> for StakesSort {
    type Error = &'static str;
    fn try_from(str: String) -> Result<StakesSort, Self::Error> {
        match str.as_str() {
            "STAKE_ASC" => Ok(StakesSort::StakeAsc),
            "STAKE_DESC" => Ok(StakesSort::StakeDesc),
            _ => Err("Unable to parse the StakesSort from string"),
        }
    }
}
