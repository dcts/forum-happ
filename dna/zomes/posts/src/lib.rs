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

#[derive(Debug, Deserialize)]
struct CreatePostInput {
  post: Post,
  channel: String,
}

#[hdk_extern]
fn create_post(create_post_input: CreatePostInput) -> ExternResult<ActionHash> {
  let post = create_post_input.post;
  let entry_type = EntryTypes::Post(post);
  let create_entry_action_hash = create_entry(entry_type)?;
  Ok(create_entry_action_hash)
}



/**
 * DON'T TOUCH
 */
pub use posts_zome;
