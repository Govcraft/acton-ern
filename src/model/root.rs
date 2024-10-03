use std::fmt;
use std::hash::Hash;

use derive_more::{AsRef, From, Into};
use mti::prelude::*;

use crate::errors::ErnError;


#[derive(AsRef, From, Into, Eq, Debug, PartialEq, Clone, Hash, Default, PartialOrd)]
pub struct EntityRoot {
    name: MagicTypeId,
}

impl EntityRoot {
    pub fn name(&self) -> &MagicTypeId {
        &self.name
    }
    pub fn as_str(&self) -> &str {
        &self.name
    }


    pub fn new(value: String) -> Result<Self, ErnError> {
        Ok(EntityRoot {
            name: value.create_type_id::<V7>(),
        })
    }
}

impl fmt::Display for EntityRoot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = &self.name;
        write!(f, "{id}")
    }
}

//
impl std::str::FromStr for EntityRoot {
    type Err = ErnError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(EntityRoot {
            name: s.create_type_id::<V7>(),
        })
    }
}
