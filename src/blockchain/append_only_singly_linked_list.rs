//! NOTE: the name of this module seems exceedingly stupid and a little unnecessary, however
//! it's a lot better to have a very verbose module name that has a generically named struct
//! that makes sense within it's context, than having a very generic module name with a verbose
//! struct name within (and also a lot less annoying).

pub struct List {}

impl List {
    pub fn new() -> Self {
        Self {}
    }
}
