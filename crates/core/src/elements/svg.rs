use freya_engine::prelude::*;
use freya_native_core::real_dom::NodeImmutable;
use freya_node_state::{
    FontStyleState,
    StyleState,
};
use torin::prelude::LayoutNode;

use super::utils::ElementUtils;
use crate::dom::DioxusNode;

pub struct SvgElement;

impl ElementUtils for SvgElement {
    fn render(
        self,
        layout_node: &LayoutNode,
        node_ref: &DioxusNode,
        canvas: &Canvas,
        _font_collection: &mut FontCollection,
        font_manager: &FontMgr,
        _default_fonts: &[String],
        _scale_factor: f32,
    ) {
        let area = layout_node.visible_area();
        let node_style = &*node_ref.get::<StyleState>().unwrap();
        let font_style = &*node_ref.get::<FontStyleState>().unwrap();

        let x = area.min_x();
        let y = area.min_y();
        if let Some(svg_data) = &node_style.svg_data {
            let svg_dom = svg::Dom::from_bytes(svg_data.as_slice(), font_manager);
            if let Ok(svg_dom) = svg_dom {
                canvas.save_layer(&SaveLayerRec::default());
                canvas.translate((x, y));
                canvas.scale((
                    area.width() / svg_dom.inner().fContainerSize.fWidth,
                    area.height() / svg_dom.inner().fContainerSize.fHeight,
                ));
                svg_dom.render(canvas);
                let mut paint = Paint::default();
                paint.set_color(font_style.color);
                paint.set_blend_mode(BlendMode::SrcIn);
                canvas.draw_paint(&paint);
                canvas.restore();
            }
        }
    }
}
