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
    pub fn offset_to_frame_center_before_trimming(&self) -> (u32, u32) {
        if !self.trimmed {
            return (0, 0)
        }

        // size of x and y trimming in pixels:
        let trim_x = self.source.x;
        let trim_y = self.source.y;

        // move back the frame position by trimming amount:
        let og_start_x = self.frame.x - trim_x;
        let og_start_y = self.frame.y - trim_y;

        // original width and height without trimming:
        let og_start_w = self.source.w;
        let og_start_h = self.source.h;

        // calculate original center:
        let center_x = og_start_x + og_start_w / 2;
        let center_y = og_start_y + og_start_h / 2;


        let trimmed_center_x = self.frame.x + self.frame.w / 2;
        let trimmed_center_y = self.frame.y + self.frame.h / 2;

        let offset_x = center_x - trimmed_center_x;
        let offset_y = center_y - trimmed_center_y;

        (offset_x, offset_y)
    }


    fn frame_center_before_trimming(&self) -> (u32, u32) {

        // if not trimmed, just return the frame center:
        if !self.trimmed {
            let cx = self.frame.x + self.frame.w / 2;
            let cy = self.frame.y + self.frame.h / 2;
            return (cx, cy)
        }

        // size of x and y trimming in pixels:
        let trim_x = self.source.x;
        let trim_y = self.source.y;

        // move back the frame position by trimming amount:
        let og_start_x = self.frame.x - trim_x;
        let og_start_y = self.frame.y - trim_y;

        // original width and height without trimming:
        let og_start_w = self.source.w;
        let og_start_h = self.source.h;

        // calculate original center:
        let center_x = og_start_x + og_start_w / 2;
        let center_y = og_start_y + og_start_h / 2;
        (center_x, center_y)
    }

}
