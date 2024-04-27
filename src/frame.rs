use crate::rect::Rect;

/// Boundaries and properties of a packed texture.
#[derive(Clone, Debug)]
pub struct Frame<K> {
    /// Key used to uniquely identify this frame.
    pub key: K,
    /// Rectangle describing the texture coordinates and size.
    pub frame: Rect,
    /// True if the texture was rotated during packing.
    /// If it was rotated, it was rotated 90 degrees clockwise.
    pub rotated: bool,
    /// True if the texture was trimmed during packing.
    pub trimmed: bool,

    // (x, y) is the trimmed frame position at original image
    // (w, h) is original image size
    //
    //            w
    //     +--------------+
    //     | (x, y)       |
    //     |  ^           |
    //     |  |           |
    //     |  *********   |
    //     |  *       *   |  h
    //     |  *       *   |
    //     |  *********   |
    //     |              |
    //     +--------------+
    /// Source texture size before any trimming.
    pub source: Rect,
}


impl <K> Frame<K> {

    /// Returns an offset in pixels to non-trimmed frame center coordinate
    /// from trimmed frame center coordinate (trimmed frame's center is considered to be the origin).
    ///
    /// This can be useful if you want to center your trimmed frame in relation to the original non-trimmed frame.
    /// For example, if you have many animation images with the same dimensions, but their inner pixels can be trimmed,
    /// drawing the trimmed frames in a fixed position the same way as seen in the non-trimmed frames requires
    /// calculating this offset and applying it to the drawing position.
    ///
    /// When drawing your trimmed frames to a fixed position so that the fixed position will
    /// be at the center of your trimmed frame, adding this offset to the fixed position
    /// transforms the trimmed frame to its non-trimmed position.
    ///
    /// If no trimming was done for the frame, `(0,0)` is returned.
    pub fn trimmed_center_to_non_trimmed_center_offset(&self) -> (i32, i32) {
        if !self.trimmed {
            return (0, 0)
        }

        let trim_x = self.source.x;
        let trim_y = self.source.y;

        let txw = trim_x + self.frame.w / 2;
        let txh = trim_y + self.frame.h / 2;

        let ocx = self.source.w / 2;
        let ocy = self.source.h / 2;

        let offset_x: i32 = (ocx as i32 - txw as i32);
        let offset_y: i32 = (ocy as i32 - txh as i32);

        (offset_x, offset_y)
    }
}
