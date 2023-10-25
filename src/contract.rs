use crate::common;

pub struct Contract {
    pub caller_address: common::address::Address,
    caller: common::address::Address,
    // weird naming is to avoid conflict with rust keyword
    self_: common::address::Address,
    // jumpdests:
}
