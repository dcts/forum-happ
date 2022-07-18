use hdk::prelude::*;

#[hdk_entry_helper]
struct Comment {
    comment: String,
}

#[hdk_entry_defs]
#[unit_enum(UnitTypes)]
enum EntryTypes {
    Comment(Comment),
}

#[derive(Debug, Deserialize)]
struct CreateCommentInput {
    comment_on: ActionHash,
    comment: String,
}

#[hdk_link_types]
enum LinkTypes {
    CommentedOnToComment,
}

#[hdk_extern]
fn create_comment(create_comment_input: CreateCommentInput) -> ExternResult<ActionHash> {
    // 1. create comment entry => returns comment_action_hash
    let comment = Comment {
        comment: create_comment_input.comment,
    };
    let entry_type = EntryTypes::Comment(comment);
    let comment_action_hash = create_entry(entry_type)?;

    // 2. link from post_action_hash to comment_action_hash => return create_link_action_hash (which is not needed)
    let post_action_hash = create_comment_input.comment_on;
    create_link(
        post_action_hash, 
        comment_action_hash.clone(), 
        LinkTypes::CommentedOnToComment, 
        ()
    )?;

    // 3. return comment_action_hash
    Ok(comment_action_hash)
}

#[hdk_extern]
fn get_comments_on(action_hash: ActionHash) -> ExternResult<Vec<Record>> {
    // 1. get all links where base hash is our action hash
    // get_links
    let links = get_links(
        action_hash,
        LinkTypes::CommentedOnToComment,
        None
    )?;

    // 2. retrieve all records from the target hash and put it in a vector
    let mut records: Vec<Record> = Vec::new();
    for link in links.iter() {
        let mylink = link.clone();
        let target = mylink.target;
        let link_action_hash = ActionHash::from(target);
        let record = get(link_action_hash, GetOptions::default());
        match record {
            Ok(record_option) => match record_option {
                Some(record) => records.push(record),
                None => (),
            },
            Err(_) => (),
        }
    }

    // 3. return a vector of records
    Ok(records)
}

/**
 * DON'T TOUCH
 */
pub use comments_zome;
