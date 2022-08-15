use std::any::Any;
use std::ops::Range;
use crate::data::FeatureData;
use crate::types::MapVertex;

pub mod fill;
pub mod stroke;

pub(crate) struct StyleAllocation {
    pub old_style: Box<dyn Any + Send>,
    pub vertices_range: Range<usize>,
}


/// A style declares a style of a geometry values.
pub trait Style: Copy + Clone + Any + Send {
    type Input<'a>: Copy + Clone;

    fn get_len(input: Self::Input<'_>) -> usize;
    fn prepare(&mut self, scale: f32);
    fn needs_update(&self, old_styler: Self) -> bool;
    fn compile(
        &self,
        input: Self::Input<'_>,
        v: &mut Vec<MapVertex>,
        i: &mut Vec<u32>,
    );
    fn update(&self, input: Self::Input<'_>, v: &mut [MapVertex], old_styler: Option<Self>);
}


/// A styler submits all of the styles it wants to use on a feature.
pub trait Styler {
    /// Visits all of the features and submits all of their styles.
    ///
    /// **FEATURES SLICE DOES NOT CHANGE AND THIS METHOD SHOULD ALWAYS GIVE SAME RESULT ON THE SAME INPUTS**
    ///
    /// # Arguments
    ///
    /// * `handler`: The handler which should consume all of the feature styles.
    /// * `layer`: The current layer name.
    /// * `features`: The features on this layer.
    fn visit_features<S: StyleHandler>(&self, handler: &mut S, layer: &str, zoom: f32, features: &[FeatureData]);
}

/// A StyleHandler is a consumer of all of the styles that a Styler might submit.
pub trait StyleHandler {
    fn submit<'a, S: Style>(&'a mut self, input: impl Into<S::Input<'a>>, style: S);
}