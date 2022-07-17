use hdk::prelude::*;

#[hdk_entry_helper]
struct Comment {
  comment: String
}

#[hdk_entry_defs]
#[unit_enum(UnitTypes)]
enum EntryTypes {
    Comment(Comment),
}

/**
 * DON'T TOUCH
 */
pub use comments_zome;
