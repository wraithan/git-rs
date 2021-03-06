use crate::errors::Result;

pub mod commit;
pub mod blob;
pub mod tree;
pub mod tag;

pub enum Type {
    Commit,
    Tree,
    Blob,
    Tag
}

pub enum Object {
    Commit(commit::Commit),
    Tree(tree::Tree),
    Blob(blob::Blob),
    Tag(tag::Tag)
}

impl Type {
    pub fn load<T: std::io::Read>(&self, stream: &mut T) -> Result<Object> {
        match &self {
            Type::Commit => {
                let xs = commit::Commit::load(stream)?;
                Ok(Object::Commit(xs))
            },
            Type::Tree => {
                let xs = tree::Tree::load(stream)?;
                Ok(Object::Tree(xs))
            },
            Type::Tag => {
                let xs = tag::Tag::load(stream)?;
                Ok(Object::Tag(xs))
            },
            Type::Blob => {
                let xs = blob::Blob::load(stream)?;
                Ok(Object::Blob(xs))
            }
        }
    }
}
