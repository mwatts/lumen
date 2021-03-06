use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::closure::*;
use liblumen_alloc::erts::term::prelude::*;

pub fn anonymous_closure(process: &Process) -> Term {
    process.anonymous_closure_with_env_from_slice(
        super::module(),
        INDEX,
        OLD_UNIQUE,
        UNIQUE,
        ARITY,
        CLOSURE_NATIVE,
        process.pid().into(),
        &[],
    )
}

pub fn returned() -> Term {
    Atom::str_to_term("anonymous_0")
}

const INDEX: Index = 0;
const OLD_UNIQUE: OldUnique = 1;
const UNIQUE: Unique = [
    0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF,
];

#[native_implemented::function(test:0-1-0123456789ABCDEF0123456789ABCDEF/0)]
fn result() -> Term {
    returned()
}
