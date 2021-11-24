mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use std::io::{Read, BufWriter};
use std::rc::Rc;
use std::cell::RefCell;
use crate::layout::Dimensions;

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

    let root_node = html::parse(html.to_string());
    let stylesheet = css::parse(css.to_string());
    let style_root = style::style_tree(&root_node, &stylesheet);
    let layout_root = layout::layout_tree(&style_root, viewport.clone());

    viewport.clone().borrow_mut().content.height = 600.0;

    let canvas = painting::paint(&layout_root, viewport.borrow().content.clone());
    let (width, height) = (canvas.width as u32, canvas.height as u32);

}