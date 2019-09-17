use crate::artifact;
use crate::drivers::DrawingDriver;
use skia_safe::Canvas;
use std::path::Path;

pub enum PDF {}

impl DrawingDriver for PDF {
    const NAME: &'static str = "pdf";

    fn draw_image(size: (i32, i32), path: &Path, name: &str, func: impl Fn(&mut Canvas)) {
        let mut document = skia_safe::pdf::new_document(None).begin_page(size, None);
        func(document.canvas());
        let data = document.end_page().close();
        artifact::write_file(data.as_bytes(), path, name, "pdf");
    }
}
