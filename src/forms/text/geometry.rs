use std::ops::Range;

use gpui::{Bounds, Pixels, Point};

use super::{TextInput, element::selection_rects};

impl TextInput {
    /// The caret's box in window coordinates, once laid out.
    pub fn caret_bounds(&self) -> Option<Bounds<Pixels>> {
        self.bounds_for(self.cursor())
    }

    /// An offset's box in the window, from the last layout; later offsets clamp to its end.
    pub fn bounds_for(&self, offset: usize) -> Option<Bounds<Pixels>> {
        let layout = self.layout.as_ref()?;
        let laid = layout.lines.len();
        let at = self.position_for(self.display_offset(offset).min(laid))?;
        Some(Bounds::new(
            layout.bounds.origin + at - self.scroll,
            gpui::size(Pixels::ZERO, layout.line_height),
        ))
    }

    /// A range's boxes in the window, a line each, from the last layout; later offsets clamp to its end.
    pub fn bounds_for_range(&self, range: Range<usize>) -> Vec<Bounds<Pixels>> {
        let Some(layout) = self.layout.as_ref() else {
            return Vec::new();
        };
        if layout.placeholder {
            return Vec::new();
        }
        let laid = layout.lines.len();
        let (start, end) = (
            self.display_offset(range.start).min(laid),
            self.display_offset(range.end).min(laid),
        );
        selection_rects(
            &layout.lines,
            start,
            end,
            layout.line_height,
            layout.bounds.size.width,
        )
        .into_iter()
        .map(|rect| Bounds::new(rect.origin + layout.bounds.origin - self.scroll, rect.size))
        .collect()
    }

    /// Top-left of a display offset, relative to the text's origin.
    pub(crate) fn position_for(&self, display: usize) -> Option<Point<Pixels>> {
        let layout = self.layout.as_ref()?;
        if layout.placeholder {
            return Some(Point::default());
        }
        layout
            .lines
            .visual_position_for_byte_index(display, layout.line_height)
    }

    /// The content offset nearest a window position.
    pub(crate) fn offset_for_point(&self, point: Point<Pixels>) -> usize {
        let Some(layout) = self.layout.as_ref() else {
            return 0;
        };
        if layout.placeholder {
            return 0;
        }
        let local = point - layout.bounds.origin + self.scroll;
        if local.y < Pixels::ZERO {
            return 0;
        }
        let index = match layout
            .lines
            .closest_byte_index_for_pixel_point(local, layout.line_height)
        {
            Ok(index) | Err(index) => index,
        };
        self.content_offset(index.min(layout.lines.len()))
    }
}
