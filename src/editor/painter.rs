use eframe::{egui::{self, Painter}, epaint::{vec2, Color32, Rect, Stroke, Vec2}};

use super::{elements::{Position, RealPosition}, Background};

pub struct EditorPainter {
    internal_painter: Painter,    
    area: Rect,
    scaling: f32,
    offset: Vec2,
}

impl EditorPainter {
    pub fn frow_ui_with_offset(ui: &egui::Ui, offset: Vec2) -> Self {
        let margin = ui.spacing().window_margin;
        let available_rect = ui.available_rect_before_wrap();
        let total_left_top = available_rect.left_top() - margin.left_top();
        let total_right_bottom = available_rect.right_bottom() - margin.right_bottom();
        let total_area = Rect::from_min_max(total_left_top, total_right_bottom);
        
        Self {
            internal_painter: ui.painter_at(total_area),
            area: total_area,
            scaling: 20.0,
            offset,
        }
    }
}

impl EditorPainter {
    pub fn circle(&self, position: Position, radius: f32, fill_color: Color32, stroke: Stroke) {
        let real_position = position.get_real_position(self.scaling, self.area) + self.offset;
        let real_radius = radius * self.scaling;
        let real_stroke = Stroke::new(stroke.width * self.scaling, stroke.color);

        self.internal_painter.circle(real_position, real_radius, fill_color, real_stroke);
    }

    pub fn circle_filled(&self, position: Position, radius: f32, fill_color: Color32) {
        let real_position = position.get_real_position(self.scaling, self.area) + self.offset;
        let real_radius = radius * self.scaling;
        
        self.internal_painter.circle_filled(real_position, real_radius, fill_color);
    }    

    pub fn circle_stroke(&self, position: Position, radius: f32, stroke: Stroke) {
        let real_position = position.get_real_position(self.scaling, self.area) + self.offset;
        let real_radius = radius * self.scaling;
        let real_stroke = Stroke::new(stroke.width * self.scaling, stroke.color);

        self.internal_painter.circle_stroke(real_position, real_radius, real_stroke);
    }    
}

impl EditorPainter {
    pub fn background(&self, background: Background) {
        match background {
            Background::None => (),
            Background::Grid(spacing) => self.grid(spacing),
            Background::Dots(spacing) => self.dots(spacing),
        }
    }
    
    fn grid(&self, spacing: i32) {
        
    }
    
    fn dots(&self, spacing: i32) {
        // TODO: calculate offset mod spacing and apply that instead of offset
        let start_height = self.area.top() + self.offset.y;
        let start_width = self.area.left() + self.offset.x;
        
        let start = self.area.left_top() + self.offset;
        let end = self.area.right_bottom() + self.offset;
        
        let actual_spacing = spacing * self.scaling as i32;

        for i in ((start.y as i32)..(end.y as i32)).step_by(actual_spacing as usize) {
            self.internal_painter.circle_filled(vec2(400.0, i as f32).to_pos2(), 2.0, Color32::LIGHT_GRAY);
        }
    }
}