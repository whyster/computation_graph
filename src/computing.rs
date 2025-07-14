use std::cmp::PartialEq;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub(crate) enum ComputingDomain {
    Classical,
    Quantum,
    Conflict
}


pub(crate) trait Computable {
    fn get_domain(&self) -> ComputingDomain;
}

impl ComputingDomain {
    pub(crate) fn compare(&self, other: &Self) -> Self {
        if let ComputingDomain::Conflict = self {
            return ComputingDomain::Conflict;
        }
        if let ComputingDomain::Conflict = other {
            return ComputingDomain::Conflict;
        }
        if self != other {
            return ComputingDomain::Conflict;
        }
        
        // self should equal other, so it doesn't matter which we return a copy of
        *self
    }
}
