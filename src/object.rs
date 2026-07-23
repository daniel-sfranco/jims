use sha::{Digest, Sha1};
use crate::repository;

pub trait JimsObject {
    fn serialize();
    fn deserialize();
}