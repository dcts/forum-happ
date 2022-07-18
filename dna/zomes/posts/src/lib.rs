use hdk::prelude::*;

#[hdk_entry_helper]
struct Post {
    title: String,
    content: String,
}

#[hdk_entry_defs]
#[unit_enum(UnitTypes)]
enum EntryTypes {
    Post(Post),
}

#[derive(Debug, Deserialize)]
struct CreatePostInput {
    post: Post,
    channel: String,
}

#[hdk_link_types]
enum LinkTypes {
    PathToChannel,
}

#[hdk_extern]
fn create_post(create_post_input: CreatePostInput) -> ExternResult<ActionHash> {
    // 1 create post entry
    let post = create_post_input.post;
    let channel = create_post_input.channel;
    let entry_type = EntryTypes::Post(post);
    let create_entry_action_hash = create_entry(entry_type)?;
    
    // 2 add post to path
    let path = Path::from(format!("all_posts.{}", channel)); // Builds the path "all_posts.<CHANNEL>"
    let typed_path = path.typed(
        LinkTypes::PathToChannel
    )?;
    typed_path.ensure()?;

    // 3 return post action hash
    Ok(create_entry_action_hash)
}

/**
 * DON'T TOUCH
 */
pub use posts_zome;
