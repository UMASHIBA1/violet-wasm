mod utils;

use wasm_bindgen::prelude::*;

use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use std::io::{Read, BufWriter};
use std::rc::Rc;
use std::cell::RefCell;
use crate::layout::Dimensions;

use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

mod dom;
mod html;
mod css;
mod style;
mod layout;
mod painting;

#[wasm_bindgen]
pub fn main(html: &str, css: &str) { // 後でhtmlとcssを受け取る

    let mut viewport = Rc::new(RefCell::new(Dimensions::default()));
    viewport.clone().borrow_mut().content.width = 800.0;
    viewport.clone().borrow_mut().content.height = 600.0;

    log(&format!("html: {}", html));
    log(&format!("css: {}", css));

    let root_node = html::parse(html.to_string());
    log(&format!("root_node: {:?}", root_node));
    let stylesheet = css::parse(css.to_string());
    log(&format!("stylesheet: {:?}", stylesheet));

    let style_root = style::style_tree(&root_node, &stylesheet);

    log(&format!("style_root: {:?}", style_root));

    let layout_root = layout::layout_tree(&style_root, viewport.clone());

    log(&format!("layout_root: {:?}", layout_root));


    viewport.clone().borrow_mut().content.height = 600.0;

    let canvas = painting::paint(&layout_root, viewport.borrow().content.clone());
    
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas_el = document.get_element_by_id("canvas").unwrap();

    // https://developer.mozilla.org/ja/docs/Web/API/CanvasRenderingContext2D
    let canvas_el: web_sys::HtmlCanvasElement = canvas_el
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    
        let context = canvas_el
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

        log(&canvas.pixels.len().to_string().as_str());

        let mut index = 0;
    for p in canvas.pixels {
        let x = index % 800;
        let y = (index - x) / 800 + 1;
        index += 1;

        // log(x.to_string().as_str());
        // log(y.to_string().as_str());
        // log(&format!("rgba({}, {}, {}, {})", p.r, p.g, p.b, p.a));

               
        context.set_fill_style(&format!("rgba({}, {}, {}, {})", p.r, p.g, p.b, p.a).into());
        // セルのx座標とy座標を計算して
        context.fill_rect(
            x as f64,
            y as f64,
            1.0 as f64,
            1.0 as f64,
        );
    }

}