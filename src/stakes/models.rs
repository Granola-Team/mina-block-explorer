use crate::common::table::AnySort;
use crate::common::table::{NegateSort, SortDirection};
use std::fmt;

pub enum EpochStyleVariant {
    Primary,
    Secondary,
}

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

impl NegateSort for StakesSort {
    fn negate(&self) -> AnySort {
        match self {
            StakesSort::StakeDesc => AnySort::Stakes(StakesSort::StakeAsc),
            StakesSort::StakeAsc => AnySort::Stakes(StakesSort::StakeDesc),
        }
    }
}
