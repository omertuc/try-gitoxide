use git_object::{
    bstr::{BString, ByteVec},
    tree, Object,
};
use git_repository::{actor::Signature, refs::transaction::PreviousValue};

fn main() {
    let _ = std::fs::remove_dir_all("/tmp/haha");
    let repo = git_repository::init("/tmp/haha").unwrap();
    let mut tree = git_object::Tree::empty();
    let blob = git_object::Blob {
        data: Vec::from_slice("hello world"),
    };
    let oid = repo.write_object(blob).unwrap();

    tree.entries.push(tree::Entry {
        oid: oid.into(),
        mode: tree::EntryMode::Blob,
        filename: BString::from("hello.txt"),
    });

    let tree_obj = Object::from(tree);
    let oid = repo.write_object(tree_obj).unwrap();

    let x = git_object::Commit {
        tree: oid.into(),
        parents: Vec::new().into(),
        author: Signature::empty(),
        committer: Signature::empty(),
        encoding: None,
        message: "Hello".into(),
        extra_headers: Vec::new(),
    };

    let oid = repo.write_object(Object::from(x)).unwrap();

    repo.reference("refs/heads/main", oid, PreviousValue::Any, "creating main branch")
        .unwrap();
}
