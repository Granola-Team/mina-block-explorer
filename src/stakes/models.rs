use crate::common::table::SortDirection;
use std::fmt;

pub enum EpochStyleVariant {
    Primary,
    Secondary,
}

#[derive(Clone)]
#[allow(dead_code)]
pub enum StakesSort {
    StakeDesc,
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
        }
    }
}
