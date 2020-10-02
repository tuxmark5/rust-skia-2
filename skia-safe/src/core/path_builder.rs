use crate::{prelude::*, scalar, Path, PathDirection, PathFillType, Point, RRect, Rect};
use skia_bindings as sb;
use skia_bindings::SkPathBuilder;

pub use skia_bindings::SkPathBuilder_ArcSize as ArcSize;
#[test]
fn test_arc_size_naming() {
    let _ = ArcSize::Large;
}

pub type PathBuilder = Handle<SkPathBuilder>;
unsafe impl Send for PathBuilder {}
unsafe impl Sync for PathBuilder {}

impl NativeDrop for SkPathBuilder {
    fn drop(&mut self) {
        unsafe { sb::C_SkPathBuilder_destruct(self) }
    }
}

impl PathBuilder {
    pub fn new() -> Self {
        Self::from_native(unsafe { SkPathBuilder::new() })
    }

    pub fn snapshot(&mut self) -> Path {
        let mut path = Path::default();
        unsafe { sb::C_SkPathBuilder_snapshot(self.native_mut(), path.native_mut()) }
        path
    }

    pub fn detach(&mut self) -> Path {
        let mut path = Path::default();
        unsafe { sb::C_SkPathBuilder_detach(self.native_mut(), path.native_mut()) }
        path
    }

    pub fn set_fill_type(&mut self, ft: PathFillType) -> &mut Self {
        self.native_mut().fFillType = ft;
        self
    }

    pub fn set_is_volatile(&mut self, is_volatile: bool) -> &mut Self {
        self.native_mut().fIsVolatile = is_volatile;
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        unsafe {
            self.native_mut().reset();
        }
        self
    }

    pub fn move_to(&mut self, pt: impl Into<Point>) -> &mut Self {
        unsafe {
            self.native_mut().moveTo(pt.into().into_native());
        }
        self
    }

    pub fn line_to(&mut self, pt: impl Into<Point>) -> &mut Self {
        unsafe {
            self.native_mut().lineTo(pt.into().into_native());
        }
        self
    }

    pub fn quad_to(&mut self, p1: impl Into<Point>, p2: impl Into<Point>) -> &mut Self {
        unsafe {
            self.native_mut()
                .quadTo(p1.into().into_native(), p2.into().into_native());
        }
        self
    }

    pub fn conic_to(&mut self, p1: impl Into<Point>, p2: impl Into<Point>, w: scalar) -> &mut Self {
        unsafe {
            self.native_mut()
                .conicTo(p1.into().into_native(), p2.into().into_native(), w);
        }
        self
    }

    pub fn cubic_to(
        &mut self,
        p1: impl Into<Point>,
        p2: impl Into<Point>,
        p3: impl Into<Point>,
    ) -> &mut Self {
        unsafe {
            self.native_mut().cubicTo(
                p1.into().into_native(),
                p2.into().into_native(),
                p3.into().into_native(),
            )
        };
        self
    }

    pub fn close(&mut self) -> &mut Self {
        unsafe {
            self.native_mut().close();
        }
        self
    }

    pub fn r_line_to(&mut self, pt: impl Into<Point>) -> &mut Self {
        unsafe {
            self.native_mut().rLineTo(pt.into().into_native());
        }
        self
    }

    pub fn r_quad_to(&mut self, pt1: impl Into<Point>, pt2: impl Into<Point>) -> &mut Self {
        unsafe {
            self.native_mut()
                .rQuadTo(pt1.into().into_native(), pt2.into().into_native());
        }
        self
    }

    pub fn r_conic_to(
        &mut self,
        pt1: impl Into<Point>,
        pt2: impl Into<Point>,
        w: scalar,
    ) -> &mut Self {
        unsafe {
            self.native_mut()
                .rConicTo(pt1.into().into_native(), pt2.into().into_native(), w);
        }
        self
    }

    pub fn r_cubic_to(
        &mut self,
        pt1: impl Into<Point>,
        pt2: impl Into<Point>,
        pt3: impl Into<Point>,
    ) -> &mut Self {
        unsafe {
            self.native_mut().rCubicTo(
                pt1.into().into_native(),
                pt2.into().into_native(),
                pt3.into().into_native(),
            );
        }
        self
    }

    pub fn arc_to(
        &mut self,
        oval: impl AsRef<Rect>,
        start_angle_deg: scalar,
        sweep_angle_deg: scalar,
        force_move_to: bool,
    ) -> &mut Self {
        unsafe {
            self.native_mut().arcTo(
                oval.as_ref().native(),
                start_angle_deg,
                sweep_angle_deg,
                force_move_to,
            );
        }
        self
    }

    pub fn arc_to_tangent(
        &mut self,
        p1: impl Into<Point>,
        p2: impl Into<Point>,
        radius: scalar,
    ) -> &mut Self {
        unsafe {
            self.native_mut()
                .arcTo1(p1.into().into_native(), p2.into().into_native(), radius);
        }
        self
    }

    pub fn arc_to_radius(
        &mut self,
        r: impl Into<Point>,
        x_axis_rotate: scalar,
        large_arc: ArcSize,
        sweep: PathDirection,
        xy: impl Into<Point>,
    ) -> &mut Self {
        unsafe {
            self.native_mut().arcTo2(
                r.into().into_native(),
                x_axis_rotate,
                large_arc,
                sweep,
                xy.into().into_native(),
            );
        }
        self
    }

    pub fn add_arc(
        &mut self,
        oval: impl AsRef<Rect>,
        start_angle_deg: scalar,
        sweep_angle_deg: scalar,
    ) -> &mut Self {
        unsafe {
            self.native_mut()
                .addArc(oval.as_ref().native(), start_angle_deg, sweep_angle_deg);
        }
        self
    }

    pub fn add_rect(
        &mut self,
        rect: impl AsRef<Rect>,
        dir: impl Into<Option<PathDirection>>,
        start_index: impl Into<Option<usize>>,
    ) -> &mut Self {
        let dir = dir.into().unwrap_or(PathDirection::CW);
        let start_index = start_index.into().unwrap_or(0);
        unsafe {
            self.native_mut()
                .addRect(rect.as_ref().native(), dir, start_index.try_into().unwrap());
        }
        self
    }

    pub fn add_oval(
        &mut self,
        rect: impl AsRef<Rect>,
        dir: impl Into<Option<PathDirection>>,
        start_index: impl Into<Option<usize>>,
    ) -> &mut Self {
        let dir = dir.into().unwrap_or(PathDirection::CW);
        // m86: default start index changed from 0 to 1
        let start_index = start_index.into().unwrap_or(1);
        unsafe {
            self.native_mut()
                .addOval(rect.as_ref().native(), dir, start_index.try_into().unwrap());
        }
        self
    }

    pub fn add_rrect(
        &mut self,
        rect: impl AsRef<RRect>,
        dir: impl Into<Option<PathDirection>>,
        start_index: impl Into<Option<usize>>,
    ) -> &mut Self {
        let dir = dir.into().unwrap_or(PathDirection::CW);
        // m86: default start index changed from 0 to 6 or 7 dependening on the path's direction.
        let start_index =
            start_index
                .into()
                .unwrap_or(if dir == PathDirection::CW { 6 } else { 7 });
        unsafe {
            self.native_mut().addRRect(
                rect.as_ref().native(),
                dir,
                start_index.try_into().unwrap(),
            );
        }
        self
    }

    pub fn add_circle(
        &mut self,
        center: impl Into<Point>,
        radius: scalar,
        dir: impl Into<Option<PathDirection>>,
    ) -> &mut Self {
        let center = center.into();
        let dir = dir.into().unwrap_or(PathDirection::CW);
        unsafe {
            self.native_mut().addCircle(center.x, center.y, radius, dir);
        }
        self
    }

    pub fn add_polygon(&mut self, pts: &[Point], is_closed: bool) -> &mut Self {
        unsafe {
            self.native_mut().addPolygon(
                pts.native().as_ptr(),
                pts.len().try_into().unwrap(),
                is_closed,
            );
        }
        self
    }

    pub fn inc_reserve(&mut self, extra_pt_count: usize, extra_verb_count: usize) {
        unsafe {
            self.native_mut().incReserve(
                extra_pt_count.try_into().unwrap(),
                extra_verb_count.try_into().unwrap(),
            )
        }
    }

    #[deprecated(since = "0.35.0", note = "Removed without replacement")]
    pub fn make(
        _points: &[Point],
        _verbs: &[u8],
        _conic_weights: &[scalar],
        _fill_type: PathFillType,
        _is_volatile: impl Into<Option<bool>>,
    ) -> ! {
        panic!("Removed without replacement");
    }
}

#[test]
fn test_creation_snapshot_and_detach() {
    let mut builder = PathBuilder::new();
    let _path = builder.snapshot();
    let _path = builder.detach();
}
