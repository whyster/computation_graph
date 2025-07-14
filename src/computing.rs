use std::cmp::PartialEq;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub(crate) enum ComputingDomain {
    Classical,
    Quantum,
    Conflict,
    Unknown // TODO: Figure out a better name to describe the weakest most permissive domain
}


pub(crate) trait Computable {
    fn get_domain(&self) -> ComputingDomain;
}

impl<T: Computable> Computable for Vec<T> {
    fn get_domain(&self) -> ComputingDomain {
       self.iter()
           .map(|x| x.get_domain())
           .fold(ComputingDomain::Unknown, |a, b| a.compare(&b))
    }
}

impl<T: Computable> Computable for Box<T> {
    fn get_domain(&self) -> ComputingDomain {
        self.as_ref().get_domain()
    }
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

        if let ComputingDomain::Unknown = self {
            return *other;
        }
        if let ComputingDomain::Unknown = other {
            return *self;
        }

        // self should equal other, so it doesn't matter which we return a copy of
        *self
    }
}
