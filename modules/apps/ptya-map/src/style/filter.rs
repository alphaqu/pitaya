use crate::tile::{Feature, GeomType, Value};

pub struct ComponentFilter {
	pub fields: Vec<FilterEntry>,
	pub id: Option<FilterCondition<u64>>,
	pub geo_type: Option<FilterCondition<GeomType>>
}

impl ComponentFilter {
	pub fn matches(&self, feature: &Feature) -> bool {
		if let Some(id) = &self.id {
			if !id.matches(feature.id.as_ref()) {
				return false;
			}
		}

		if let Some(id) = &self.geo_type {
			if !id.matches(Some(&feature.geometry_type)) {
				return false;
			}
		}

		for field in &self.fields {
			if !field.condition.matches(feature.fields.get(&field.field)) {
				return false;
			};
		}
		
		true
	}
}

pub struct FilterEntry {
	pub field: String,
	pub condition: FilterCondition<Value>
}

pub enum FilterCondition<V: PartialEq> {
	Exists,
	Is(Vec<V>),
	IsNot(Vec<V>),
}

impl<V: PartialEq> FilterCondition<V> {
	#[inline(always)]
	pub fn matches(&self, value: Option<&V>) -> bool {
		match self {
			FilterCondition::Exists => {
				value.is_some()
			}
			FilterCondition::Is(values) => {
				if let Some(value) = value {
					for v in values {
						if value != v {
							return false;
						}
					}
					
					return true;
				} else {
					false
				}
			}
			FilterCondition::IsNot(values) => {
				if let Some(value) = value {
					for v in values {
						if value == v {
							return false;
						}
					}

					return true;
				} else {
					true
				}
			}
		}
	}
}
