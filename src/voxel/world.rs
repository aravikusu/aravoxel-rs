use crate::engine::resource::model::Model;

/// A World of Chunks is essentially... everything.
/// The world the player explores.
pub struct World {
    chunk_model: Model,
}
