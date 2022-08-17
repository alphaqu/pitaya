use std::ops::Range;

pub mod task;


pub fn extend(range: Range<f32>, t: f32) -> f32 {
	let t = t - range.start;
	let t = t / (range.end - range.start);
	t
}