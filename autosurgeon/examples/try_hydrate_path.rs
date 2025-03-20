use std::collections::BTreeMap;

use autosurgeon::{reconcile, hydrate, hydrate_path, Reconcile, Hydrate, Prop};
use automerge;

#[derive(Reconcile, Hydrate, Clone, Debug, Eq, PartialEq)]
pub enum AddressKind {
    Home,
    Work(String),
    Other,
}

#[derive(Reconcile, Hydrate, Clone, Debug, Eq, PartialEq)]
struct ContactBook {
    #[key]
    id: String,
    contacts: BTreeMap<String, Contact>,
}

#[derive(Reconcile, Hydrate, Clone, Debug, Eq, PartialEq)]
struct Contact {
    name: String,
    addresses: Vec<Address>,
    id: String,
}

#[derive(Reconcile, Hydrate, Clone, Debug, Eq, PartialEq)]
struct Address {
    line_one: String,
    line_two: String,
    kind: AddressKind,
}
fn main() {

    let mut doc = automerge::Automerge::new();        

    // Make up some data
    let bob = Contact {
        name: "bob".to_string(),
        id: "one".to_string(),
        addresses: vec![Address {
            line_one: "line one".to_string(),
            line_two: "line two".to_string(),
            kind: AddressKind::Home,
        }],
    };
    let alice = Contact {
        name: "alice".to_string(),
        id: "two".to_string(),
        addresses: vec![Address {
            line_one: "33 Rockefeller Plaza".to_string(),
            line_two: "New York".to_string(),
            kind: AddressKind::Work("Suite 420".to_string()),
        }],
    };

    let contacts = ContactBook {
        contacts: BTreeMap::from_iter(vec![
            (bob.id.to_string(), bob.clone()),
            (alice.id.to_string(), alice.clone()),
        ]),
        id: "My Contact Book".to_string()
    };

    // Now insert it into the document
    let _tx_res = doc
        .transact_with::<_, _, automerge::AutomergeError, _>(
            |_| automerge::transaction::CommitOptions::default().with_message("Set Contact Book".to_owned()),
            |tx| {
                reconcile(tx, &contacts).unwrap();
                Ok(())
            });

    println!("We made the following document: {:#?}", doc.dump());


    // Let's hydrate the whole document but naively use the hydrate_path
    let contact_book: ContactBook = hydrate(&doc).unwrap();
    println!("That hydrates back to this via hydrate: {:#?}", contact_book);

    // Let's hydrate the whole document but naively use the hydrate_path
    let contact_book: Result<Option<ContactBook>, _> = hydrate_path(&doc, &automerge::ROOT, vec![
    ].into_iter());
    println!("That hydrates back to this via hydrate_path: {:#?}", contact_book);

    let s_visit_alice: Result<Option<Contact>, _> = hydrate_path(&doc, &automerge::ROOT, vec![
        "contacts".into(),
        "two".into(),
    ].into_iter());

    println!("Let's Visit Alice at: {:#?}", s_visit_alice);
}