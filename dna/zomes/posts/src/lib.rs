use hdk::prelude::*;

#[hdk_entry_helper]
struct Post {
  title: String,
  content: String,
}

#[hdk_entry_defs]
#[unit_enum(UnitTypes)]
enum EntryTypes {
  Post(Post)
}

/**
 * DON'T TOUCH
 */
pub use posts_zome;
