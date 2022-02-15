use crate::util::{Coord2D, Size2D};
use druid::piet::{ImageFormat, InterpolationMode};
use druid::widget::{Label, Painter, SizedBox};
use druid::{ImageBuf, PaintCtx, Rect, RenderContext, Widget, WidgetExt};
use lazy_static::lazy_static;
use maplit::hashmap;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref IMAGE_ARRAY: Mutex<HashMap<&'static str, Arc<[u8]>>> = Mutex::new(hashmap!());
}

/**
 * This will explicitly leak the memory - do not call in an unbounded manner
 */
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn get_image_buf(path: String) -> Arc<[u8]> {
    let mut img_arr = IMAGE_ARRAY.lock().unwrap();
    img_arr
        .entry(string_to_static_str(path.clone()))
        .or_insert_with(|| {
            println!("Reading {path}");
            ImageBuf::from_file(path).unwrap().raw_pixels_shared()
        })
        .clone()
}

fn get_tile_path(tile_position: Coord2D) -> String {
    format!("./files/{}_{}.png", tile_position.x, tile_position.y)
}

fn get_src_and_dest_rect(viewport_area: Rect, tile_area: Rect) -> (Rect, Rect) {
    let dst_rect = tile_area.intersect(viewport_area);
    let mut src_rect = Rect {
        x0: 0.0,
        y0: 0.0,
        x1: TILE_SIZE.width as f64,
        y1: TILE_SIZE.height as f64,
    };
    if tile_area.x0 < viewport_area.x0 {
        src_rect.x0 = viewport_area.x0 - tile_area.x0;
    }
    if tile_area.y0 < viewport_area.y0 {
        src_rect.y0 = viewport_area.y0 - tile_area.y0;
    }
    if tile_area.x1 > viewport_area.x1 {
        src_rect.x1 = viewport_area.x1 - tile_area.x0;
    }
    if tile_area.y1 > viewport_area.y1 {
        src_rect.y1 = viewport_area.y1 - tile_area.y0;
    }

    (src_rect, dst_rect)
}

fn paint_tile_with_coordinates(
    ctx: &mut PaintCtx,
    tile_position: Coord2D,
    paint_position: Coord2D,
    viewport_area: Rect,
) {
    let path = get_tile_path(tile_position);

    let img_buf = get_image_buf(path);
    let img = ctx
        .make_image(
            TILE_SIZE.width,
            TILE_SIZE.height,
            &img_buf,
            ImageFormat::RgbaSeparate,
        )
        .unwrap();

    let (src_rect, dst_rect) = get_src_and_dest_rect(
        viewport_area,
        Rect {
            x0: paint_position.x as f64,
            y0: paint_position.y as f64,
            x1: (paint_position.x + TILE_SIZE.width) as f64,
            y1: (paint_position.y + TILE_SIZE.height) as f64,
        },
    );

    ctx.save().unwrap();
    {
        // This is sorta hacky. The ctx comes with some preexisting transforms, but I did the math as if from 0,0
        let current_transform = ctx.current_transform().inverse();
        ctx.transform(current_transform);
    }
    ctx.draw_image_area(&img, src_rect, dst_rect, InterpolationMode::NearestNeighbor);
    ctx.restore().unwrap();
}

const TILE_SIZE: Size2D = Size2D {
    width: 256,
    height: 256,
};

pub fn build_spaces_widget() -> impl Widget<Coord2D> {
    let painter = Painter::new(|ctx, data: &Coord2D, _env| {
        println!("Painting");
        let widget_origin = ctx.window_origin();
        let widget_size = ctx.size();
        let viewport_area = Rect {
            x0: widget_origin.x,
            y0: widget_origin.y,
            x1: widget_origin.x + widget_size.width,
            y1: widget_origin.y + widget_size.height,
        };
        let start_tile_x = data.x / TILE_SIZE.width;
        let end_tile_x = (data.x + widget_size.width as usize) / TILE_SIZE.width;
        let start_tile_y = data.y / TILE_SIZE.height;
        let end_tile_y = (data.y + widget_size.height as usize) / TILE_SIZE.height;
        for tile_x in start_tile_x..=end_tile_x {
            for tile_y in start_tile_y..=end_tile_y {
                let tile_offset_x = (((data.x / TILE_SIZE.width) * TILE_SIZE.width) - data.x)
                    + ((tile_x - start_tile_x) * TILE_SIZE.width);
                let tile_offset_y = (((data.y / TILE_SIZE.height) * TILE_SIZE.height) - data.y)
                    + ((tile_y - start_tile_y) * TILE_SIZE.height);
                paint_tile_with_coordinates(
                    ctx,
                    Coord2D {
                        x: tile_x,
                        y: tile_y,
                    },
                    Coord2D {
                        x: widget_origin.x as usize + tile_offset_x,
                        y: widget_origin.y as usize + tile_offset_y,
                    },
                    viewport_area,
                );
            }
        }
    });
    SizedBox::new(Label::new(""))
        .expand_width()
        .height(1024.0)
        .background(painter)
}
