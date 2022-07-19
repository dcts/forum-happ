# Questions
- `get_links` => what ordering are the links returned? Random? => not in any particular order, ordering is responsibility of caller / developer
- how to get from path to childPathStrings by following the links. How do we get from links back to path

### Posts => get_channel_posts
Why does this work?
```rust
fn get_channel_posts(channel: String) -> ExternResult<Vec<ActionHash>> {
    // 🚧 WHY THIS TESTS GREEN ALTHOUGH TYPO?
    // let anchor = Anchor(format!("alsl_posts.{}", channel));
    // let anchor_hash = create_entry(EntryTypes::Anchor(anchor))?;

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
```

