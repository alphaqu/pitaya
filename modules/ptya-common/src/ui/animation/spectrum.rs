use crate::ui::animation::interpolation::Interpolation;
use crate::ui::animation::lerp::Lerp;

#[non_exhaustive]
pub struct Spectrum<V: Clone> {
	pub stops: Vec<InterpRangeStop<V>>,
}

impl<V: Clone> Spectrum<V> {
	pub fn single(value: V) -> Spectrum<V> {
		Spectrum::new(vec![InterpRangeStop {
			level: 0.0,
			value
		}])
	}
	
	pub fn new(mut stops: Vec<InterpRangeStop<V>>) -> Spectrum<V> {
		if stops.is_empty() {
			panic!("must atleast be 1 stop.")
		}

		Spectrum {
			stops,
		}
	}
	
	pub fn get(&self, t: f32) -> V {
		self.revolve_get(t, |from, to, t| {
			from.clone()
		})
	}
	
	#[inline(always)]
	fn revolve_get(&self, t: f32, between_resolver: impl Fn(&V, &V, f32) -> V) -> V {
		let first_stop = self.stops.first().unwrap();
		if t <= first_stop.level {
			return first_stop.value.clone();
		}

		let last_stop = self.stops.last().unwrap();
		if t >= last_stop.level {
			return last_stop.value.clone();
		}

		let mut prev_value = &first_stop.value;
		let mut prev_level = first_stop.level;

		// Skip the first entry as that is the first stop
		for stop in self.stops.iter().skip(1) {
			if stop.level == t {
				return stop.value.clone();
			} else if stop.level < t {
				prev_value = &stop.value;
				prev_level = stop.level;
				continue;
			} else {
				let local_t = (t - prev_level) / (stop.level - prev_level);
				return between_resolver(prev_value, &stop.value, local_t);
				//let interpolated_t = self.interpolation.get(local_t);
				//return V::lerp(prev_value, &stop.value, interpolated_t);
			}
		}

		// this should actually never happen but whatever.
		last_stop.value.clone()
	}
}

impl<V: Clone + Default> Default for Spectrum<V> {
	fn default() -> Self {
		Spectrum::single(V::default())
	}
}


#[non_exhaustive]
pub struct LerpSpectrum<V: Lerp> {
	pub spectrum: Spectrum<V>,
	pub interpolation: Interpolation,
}

impl<V: Lerp> LerpSpectrum<V> {
	pub fn single(value: V) -> LerpSpectrum<V> {
		LerpSpectrum {
			spectrum: Spectrum::single(value),
			interpolation: Interpolation::Step
		}
	}
	
	pub fn new(stops: Vec<InterpRangeStop<V>>, interpolation: Interpolation) -> LerpSpectrum<V> {
		LerpSpectrum {
			spectrum: Spectrum::new(stops),
			interpolation
		}
	}

	pub fn get(&self, t: f32) -> V {
		self.spectrum.revolve_get(t,  |from, to, t| {
			V::lerp_static(from, to, self.interpolation.get(t))
		})
	}
}

impl<V: Lerp + Default> Default for LerpSpectrum<V> {
	fn default() -> Self {
		LerpSpectrum::single(V::default())
	}
}

pub struct InterpRangeStop<V> {
	pub level: f32,
	pub value: V
}