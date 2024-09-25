use ratatui::{
    layout::Rect,
    style::{Color, Stylize},
    symbols::Marker,
    widgets::{
        canvas::{Canvas, Map, MapResolution},
        Block, Widget,
    },
};
pub struct MapS {
    x: f64,
    y: f64,
    marker: Marker,
}

impl Widget for &MapS {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        self.map_canvas().render(area, buf);
    }
}

impl MapS {
    pub fn new() -> MapS {
        MapS {
            x: 37.6173,
            y: 55.7558,
            marker: Marker::Braille,
        }
    }

    fn map_canvas(&self) -> impl Widget + '_ {
        Canvas::default()
            .block(Block::bordered().title("Your location"))
            .marker(self.marker)
            .paint(|ctx| {
                ctx.draw(&Map {
                    color: Color::Green,
                    resolution: MapResolution::High,
                });
                ctx.print(self.x, self.y, "".yellow());
            })
            .x_bounds([-180.0, 180.0])
            .y_bounds([-90.0, 90.0])
    }
}
