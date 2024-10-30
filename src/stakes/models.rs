use crate::common::table::{AnySort, CycleSort, SortDirection};
use std::fmt;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum StakesSort {
    StakeDesc,
    StakeAsc,
    StakeNil,
    BalanceDesc,
    BalanceAsc,
    BalanceNil,
}

impl SortDirection for StakesSort {
    fn is_desc(&self) -> bool {
        matches!(self, StakesSort::StakeDesc) || matches!(self, StakesSort::BalanceDesc)
    }
    fn is_active(&self) -> bool {
        !matches!(self, StakesSort::StakeNil) && !matches!(self, StakesSort::BalanceNil)
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
            StakesSort::BalanceDesc => {
                write!(f, "BALANCE_DESC")
            }
            StakesSort::BalanceAsc => {
                write!(f, "BALANCE_ASC")
            }
            StakesSort::StakeNil => {
                write!(f, "")
            }
            StakesSort::BalanceNil => {
                write!(f, "")
            }
        }
    }
}

impl CycleSort for StakesSort {
    fn cycle(&self) -> AnySort {
        match self {
            StakesSort::StakeDesc => AnySort::Stakes(StakesSort::StakeAsc),
            StakesSort::StakeAsc => AnySort::Stakes(StakesSort::StakeDesc),
            StakesSort::BalanceDesc => AnySort::Stakes(StakesSort::BalanceAsc),
            StakesSort::BalanceAsc => AnySort::Stakes(StakesSort::BalanceDesc),
            StakesSort::BalanceNil => AnySort::Stakes(StakesSort::BalanceDesc),
            StakesSort::StakeNil => AnySort::Stakes(StakesSort::StakeDesc),
        }
    }
}

impl TryFrom<String> for StakesSort {
    type Error = &'static str;
    fn try_from(str: String) -> Result<StakesSort, Self::Error> {
        match str.as_str() {
            "STAKE_ASC" => Ok(StakesSort::StakeAsc),
            "STAKE_DESC" => Ok(StakesSort::StakeDesc),
            "BALANCE_DESC" => Ok(StakesSort::BalanceDesc),
            "BALANCE_ASC" => Ok(StakesSort::BalanceAsc),
            _ => Err("Unable to parse the StakesSort from string"),
        }
    }
}
