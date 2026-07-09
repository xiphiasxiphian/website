type Position = (f64, f64);
type Size = (f64, f64);

#[derive(Debug, Default, PartialEq, PartialOrd)]
pub struct Transform
{
    pub pos: Position,
    pub size: Size,
}

impl Transform
{
    pub fn with_anchor(pos: Position, size: Size, anchor: Anchor) -> Self
    {
        let real_pos = anchor.translate_pos(pos, size);
        Transform { pos: real_pos, size }
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Anchor
{
    #[default]
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}


impl Anchor
{
    fn translate_pos(self, pos @ (x, y): Position, size @ (w, h): Size) -> Position
    {
        match self
        {
            Anchor::TopLeft => pos,
            Anchor::TopRight => (x - w, y),
            Anchor::BottomLeft => (x, y - h),
            Anchor::BottomRight => (x - w, y - h),
            Anchor::Center => (x - (w / 2.0), y - (h / 2.0)),
        }
    }
}
