use egui::{Rect, Vec2};

pub struct JustifyList {
    pub rect: Rect,
    pub vertical: bool,
    pub gap: f32,
}

impl JustifyList {
	pub fn horizontal(rect: Rect, gap: f32) -> JustifyList {
		JustifyList {
			rect,
			vertical: false,
			gap
		}
	}

	pub fn vertical(rect: Rect, gap: f32) -> JustifyList {
		JustifyList {
			rect,
			vertical: true,
			gap
		}
	}

    pub fn apply(&self, entries: &[JustifyEntry]) -> Vec<Rect> {
        let size = entries.len();

        let content_length = if self.vertical {
            self.rect.height()
        } else {
            self.rect.width()
        };

        let mut total_flex = 0.0;
        for entry in entries {
            total_flex += entry.size;
        }
        let content_length = content_length;

	    // gap between entry border
	    let entry_gap = self.gap / 2.0;

        let mut out = Vec::with_capacity(size);
        let mut min = self.rect.min;
        for (i, entry) in entries.iter().enumerate() {
            let entry_size = content_length * (entry.size / total_flex);
            let mut rect = Rect::from_min_size(
                min,
                if self.vertical {
                    min.y += entry_size;
                    Vec2::new(self.rect.width(), entry_size)
                } else {
                    min.x += entry_size;
                    Vec2::new(entry_size, self.rect.height())
                },
            );

	        // if not first apply above gap
	        if i != 0 {
		        if self.vertical {
			        rect.min.y += entry_gap;
		        } else  {
			        rect.min.x += entry_gap;
		        }
	        }

	        // if not last apply after gap
	        if i != size - 1 {
		        if self.vertical {
			        rect.max.y -= entry_gap;
		        } else  {
			        rect.max.x -= entry_gap;
		        }
	        }
            out.push(rect);
        }
        out
    }
}

pub struct JustifyEntry {
	pub size: f32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{Pos2, vec2};

    #[test]
    fn basic() {
        let list = JustifyList {
            rect: Rect::from_min_size(Pos2::ZERO, Vec2::new(10.0, 10.0)),
            vertical: false,
            gap: 0.0,
        };

	    let out = list.apply(&[
		    JustifyEntry { size: 1.0 },
		    JustifyEntry { size: 1.0 }
	    ]);

	    assert_eq!(out, vec![
		    Rect::from_min_size(Pos2::ZERO, Vec2::new(5.0, 10.0)),
		    Rect::from_min_size(Pos2::new(5.0, 0.0), Vec2::new(5.0, 10.0))
	    ])
    }

	#[test]
	fn gap_test() {
		let list = JustifyList {
			rect: Rect::from_min_size(Pos2::ZERO, Vec2::new(10.0, 10.0)),
			vertical: false,
			gap: 2.0,
		};

		let out = list.apply(&[
			JustifyEntry { size: 1.0 },
			JustifyEntry { size: 1.0 }
		]);

		assert_eq!(out, vec![
			Rect::from_min_size(Pos2::ZERO, Vec2::new(4.0, 10.0)),
			Rect::from_min_size(Pos2::new(6.0, 0.0), Vec2::new(4.0, 10.0))
		])
	}

	#[test]
	fn grow_test() {
		let list = JustifyList {
			rect: Rect::from_min_size(Pos2::ZERO, Vec2::new(10.0, 10.0)),
			vertical: false,
			gap: 0.0,
		};

		let out = list.apply(&[
			JustifyEntry { size: 4.0 },
			JustifyEntry { size: 1.0 }
		]);

		assert_eq!(out, vec![
			Rect::from_min_size(Pos2::ZERO, Vec2::new(8.0, 10.0)),
			Rect::from_min_size(Pos2::new(8.0, 0.0), Vec2::new(2.0, 10.0))
		])
	}

	#[test]
	fn grow_gap() {
		let list = JustifyList {
			rect: Rect::from_min_size(Pos2::ZERO, Vec2::new(10.0, 10.0)),
			vertical: false,
			gap: 2.0,
		};

		let out = list.apply(&[
			JustifyEntry { size: 4.0 },
			JustifyEntry { size: 1.0 }
		]);

		assert_eq!(out, vec![
			Rect::from_min_size(Pos2::ZERO, Vec2::new(7.0, 10.0)),
			Rect::from_min_size(Pos2::new(9.0, 0.0), Vec2::new(1.0, 10.0))
		])
	}
}
