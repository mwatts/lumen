#[cfg(test)]
mod test;

use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

use crate::runtime::scheduler::SchedulerDependentAlloc;

#[native_implemented::function(erlang:make_ref/0)]
pub fn result(process: &Process) -> Term {
    process.next_reference()
}
