// db_publisher_mod.rs

//! persistent storage of "my trusted publishers urls"
//! be careful if the struct changes, then there must be an migration upgrade of the data
//! url is better than login, because is unique

#![allow(dead_code)]

use lazy_static::lazy_static;
use unwrap::unwrap;

use crate::common_structs_mod::PublisherItemData;

lazy_static! {
    static ref DB_PUBLISHERS: sled::Tree = crate::db_sled_mod::DB_SLED.open_tree(b"publishers").unwrap();
}

pub fn insert(publisher_url: &str, value: &PublisherItemData) -> anyhow::Result<()> {
    let value = serde_json::to_vec(value)?;
    DB_PUBLISHERS.insert(publisher_url, value)?;
    Ok(())
}

pub fn read(publisher_url: &str) -> anyhow::Result<Option<PublisherItemData>> {
    let data = DB_PUBLISHERS.get(publisher_url)?;
    match data {
        Some(data) => Ok(Some(serde_json::from_slice(&data)?)),
        None => Ok(None),
    }
}

pub fn delete(publisher_url: &str) {
    unwrap!(DB_PUBLISHERS.remove(publisher_url));
}

pub fn exists(publisher_url: &str) -> bool {
    unwrap!(DB_PUBLISHERS.contains_key(publisher_url))
}

pub fn list() -> anyhow::Result<Vec<PublisherItemData>> {
    let mut vec = vec![];
    for x in DB_PUBLISHERS.iter() {
        let (_key, value) = x?;
        let v: PublisherItemData = serde_json::from_slice(&value)?;
        vec.push(v);
    }
    Ok(vec)
}

pub fn init_with_some_default() {
    unwrap!(DB_PUBLISHERS.clear());
    let json = r##"
    [
     {
        "publisher_url": "https://github.com/JohnTitor",
        "note": "rust-lang.org: Crates.io team"
    },  {
        "publisher_url": "https://github.com/aidanhs",
        "note": "rust-lang.org: core, infra"
    },  {
        "publisher_url": "https://github.com/alexcrichton",
        "note": "rust-lang.org: std, cargo"
    },  {
        "publisher_url": "https://github.com/ashleygwilliams",
        "note": "rust-lang.org: core"
    },  {
        "publisher_url": "https://github.com/brson",
        "note": "early core, compiler, std"
    },  {
        "publisher_url": "https://github.com/burntsushi",
        "note": "rust-lang.org: std, ripgrep"
    },  {
        "publisher_url": "https://github.com/carols10cents",
        "note": "rust-lang.org: core, the book"
    },  {
        "publisher_url": "https://github.com/centril",
        "note": "early lang design"
    },  {
        "publisher_url": "https://github.com/dpc",
        "note": "cargo-crev author"
    },  {
        "publisher_url": "https://github.com/dtolnay",
        "note": "rust-lang.org: Library API team, serde, std standard library"
    },  {
        "publisher_url": "https://github.com/fitzgen",
        "note": "rust-lang.org: webassembly"
    },  {
        "publisher_url": "https://github.com/joshtriplett",
        "note": "rust-lang.org: lang design, cargo"
    },  {
        "publisher_url": "https://github.com/kornelski",
        "note": "lib.rs author"
    },  {
        "publisher_url": "https://github.com/llogiq",
        "note": "rust-lang.org: moderation"
    },  {
        "publisher_url": "https://github.com/lucianobestia",
        "note": "web.crev.dev, cargo_crev_reviews author"
    },  {
        "publisher_url": "https://github.com/manishearth",
        "note": "rust-lang.org: core"
    },  {
        "publisher_url": "https://github.com/mark-simulacrum",
        "note": "rust-lang.org: core, infra, release"
    },  {
        "publisher_url": "https://github.com/nikomatsakis",
        "note": "rust-lang.org: core, compiler"
    },  {
        "publisher_url": "https://github.com/nrc",
        "note": "rust-lang.org: core, rustfmt"
    },  {
        "publisher_url": "https://github.com/pietroalbini",
        "note": "rust-lang.org: core, infra"
    },  {
        "publisher_url": "https://github.com/pnkfelix",
        "note": "rust-lang.org: compiler, lang design"
    },  {
        "publisher_url": "https://github.com/ralfjung",
        "note": "rust-lang.org: unsafe code"
    },  {
        "publisher_url": "https://github.com/steveklabnik",
        "note": "rust-lang.org: core, the book"
    },  {
        "publisher_url": "https://github.com/tarcieri",
        "note": "rust-lang.org: Secure Code working group"
    },  {
        "publisher_url": "https://github.com/withoutboats",
        "note": "rust-lang.org: async"
    }
]
"##;

    let vec_p: Vec<PublisherItemData> = unwrap!(serde_json::from_str(json));
    for x in vec_p.iter() {
        unwrap!(insert(&x.publisher_url, x));
    }
}
