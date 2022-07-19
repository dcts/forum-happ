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
    Anchor(Anchor)
}

#[derive(Debug, Deserialize)]
struct CreatePostInput {
    post: Post,
    channel: String,
}

#[hdk_link_types]
enum LinkTypes {
    PathToChannel,
    ChannelToPost,  
}

#[hdk_entry_helper]
struct Anchor(String);

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

    let _create_link_action_hash = create_link(
        typed_path.path_entry_hash()?,
        create_entry_action_hash.clone(),
        LinkTypes::ChannelToPost,
        ()
    )?;

    // 3 return post action hash
    Ok(create_entry_action_hash)
}

#[hdk_extern]
fn get_channel_posts(channel: String) -> ExternResult<Vec<ActionHash>> {

    // 🚧 WHY THIS TESTS GREEN ALTHOUGH TYPO?
    // let anchor = Anchor(format!("alsl_posts.{}", channel));
    // let anchor_hash = create_entry(EntryTypes::Anchor(anchor))?;
    // => hash_entry(PATH) => to get the same anchor

    // get root hash
    // let root_hash = hdk::hash_path::path::root_hash()?;

    let channel_path = Path::from(format!("all_posts.{}", channel));
    let channel_typed_path = channel_path.typed(LinkTypes::ChannelToPost)?;
    let anchor_hash = channel_typed_path.path_entry_hash()?;

    let all_links = get_links(
        anchor_hash,
        LinkTypes::ChannelToPost,
        None
    )?;

    let action_hashes = all_links.iter()
        .map(|link| {
            ActionHash::from(link.clone().target)
        })
        .collect();

    Ok(action_hashes)
}


#[hdk_extern]
fn get_all_channels(_: ()) -> ExternResult<Vec<String>> {
    // WES SOLUTION
    // Path::from("all_posts")
    //     .typed(LinkTypes::PathToChannel)?
    //     .children_paths()?
    //     .iter()
    //     .filter_map(|path| path.leaf())
    //     .map(|component| String::try_from(component))
    //     .collect::<Result<Vec<String>, SerializedBytesError>>()
    //     .map_err(|_err| {
    //         wasm_error!(WasmErrorInner::Guest(String::from(
    //             "serialized bytes error converting component to string"
    //         )))
    //     })

    // MY SOLUTION
    let path = Path::from("all_posts".to_string());
    let typed_path = path.clone().typed(
        LinkTypes::PathToChannel
    )?;

    let mut channels: Vec<String> = Vec::new();
    let child_paths = typed_path.children_paths()?;
    for child_path in child_paths {
        let component = child_path.leaf();
        match component {
            Some(component) => {
                let maybe_string = String::try_from(component);
                match maybe_string {
                    Ok(string) => channels.push(string),
                    _ => (),
                }
            },
            _ => ()
        }
    }

    Ok(channels)

    // DOES THIS WORK TOO???
    // - construction your own anchor
    // - then getting all links from the anchor
    // - filter by LinkTypes::PathToChannel
    // - get all the targets => extract the name of the path somehow? How?
    // let a = hdk::hash_path::anchor::Anchor::from(path);

}

#[derive(Debug, Deserialize)]
struct UpdatePostInput {
    updated_post: Post,
    post_to_update: ActionHash,
}

#[hdk_extern] 
fn update_post(update_post_input: UpdatePostInput) -> ExternResult<ActionHash> {
    let updated_post = update_post_input.updated_post;
    let post_to_update = update_post_input.post_to_update;
    let update_post_action_hash = update_entry(
        post_to_update, 
        EntryTypes::Post(updated_post),
    )?;

    Ok(update_post_action_hash)
}   
/**
 * DON'T TOUCH
 */
pub use posts_zome;