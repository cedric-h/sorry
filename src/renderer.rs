use crate::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct MeshBundle {
    pub ent: usize,
    pub appearance: String,
    pub size: Vector2<f32>,
    pub iso: Isometry2<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct RenderData {
    pub ents: Vec<MeshBundle>,
}
js_serializable!(RenderData);
