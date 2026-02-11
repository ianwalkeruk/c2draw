use crate::model::{Element, ElementId, Position, Relationship, Size};
use egui::{Color32, Pos2, Rect, Response, Stroke, StrokeKind, Ui, Vec2};
use std::collections::HashMap;

/// Canvas for drawing and editing diagrams
pub struct Canvas {
    pub offset: Vec2,
    pub scale: f32,
    dragging: Option<ElementId>,
    /// If Some(source_id), we're in relationship creation mode waiting for target
    pub relationship_source: Option<ElementId>,
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            offset: Vec2::ZERO,
            scale: 1.0,
            dragging: None,
            relationship_source: None,
        }
    }
}

impl Canvas {
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if we're in relationship creation mode
    pub fn is_in_relationship_mode(&self) -> bool {
        self.relationship_source.is_some()
    }

    /// Start relationship creation mode
    pub fn start_relationship(&mut self, source_id: ElementId) {
        self.relationship_source = Some(source_id);
    }

    /// Cancel relationship creation mode
    pub fn cancel_relationship(&mut self) {
        self.relationship_source = None;
    }

    /// Render the canvas with all elements and relationships
    /// Returns the ID of an element clicked for relationship (if in relationship mode), or None
    pub fn render(
        &mut self,
        ui: &mut Ui,
        elements: &mut HashMap<ElementId, Element>,
        relationships: &[Relationship],
        selected_element: &mut Option<ElementId>,
    ) -> Option<ElementId> {
        let available_size = ui.available_size();
        let (response, painter) = ui.allocate_painter(available_size, egui::Sense::click_and_drag());

        let canvas_rect = response.rect;
        let relationship_mode = self.relationship_source.is_some();

        // Fill canvas background
        painter.rect_filled(canvas_rect, 0.0, Color32::from_gray(245));

        // Draw grid
        self.draw_grid(&painter, canvas_rect);

        // Clip to canvas area
        let clip_rect = canvas_rect;

        // Draw relationships first (so they appear behind elements)
        for rel in relationships {
            if let (Some(source), Some(target)) = (elements.get(&rel.source_id), elements.get(&rel.target_id)) {
                self.draw_relationship(&painter, source, target, rel, clip_rect);
            }
        }

        // Draw preview relationship if in relationship mode
        if let Some(source_id) = self.relationship_source {
            if let Some(source) = elements.get(&source_id) {
                if let Some(mouse_pos) = response.hover_pos() {
                    self.draw_preview_relationship(&painter, source, mouse_pos);
                }
            }
        }

        // Draw elements
        let mut element_responses: Vec<(ElementId, Response)> = Vec::new();

        for element in elements.values_mut() {
            let element_response = self.draw_element(ui, element, clip_rect, selected_element, relationship_mode);
            element_responses.push((element.id, element_response));
        }

        // Handle interactions
        let mut clicked_element_for_relationship: Option<ElementId> = None;

        for (id, response) in element_responses {
            if response.drag_started() {
                self.dragging = Some(id);
                if !relationship_mode {
                    *selected_element = Some(id);
                }
            }

            if response.dragged() {
                if let Some(element) = elements.get_mut(&id) {
                    let delta = response.drag_delta();
                    element.position = Position::new(
                        element.position.x + delta.x,
                        element.position.y + delta.y,
                    );
                }
            }

            if response.drag_stopped() {
                self.dragging = None;
            }

            if response.clicked() {
                if relationship_mode {
                    // In relationship mode, check if this is a valid target
                    if let Some(source_id) = self.relationship_source {
                        if source_id != id {
                            clicked_element_for_relationship = Some(id);
                        }
                    }
                } else {
                    // Normal selection mode
                    *selected_element = Some(id);
                }
            }
        }

        // Deselect when clicking on empty canvas (only in normal mode)
        if response.clicked() && !response.dragged() && !relationship_mode {
            *selected_element = None;
        }

        clicked_element_for_relationship
    }

    fn draw_grid(&self, painter: &egui::Painter, rect: Rect) {
        let grid_spacing = 20.0 * self.scale;
        let grid_color = Color32::from_gray(220);

        // Vertical lines
        let mut x = rect.min.x + (self.offset.x % grid_spacing);
        while x < rect.max.x {
            painter.line_segment(
                [Pos2::new(x, rect.min.y), Pos2::new(x, rect.max.y)],
                Stroke::new(1.0, grid_color),
            );
            x += grid_spacing;
        }

        // Horizontal lines
        let mut y = rect.min.y + (self.offset.y % grid_spacing);
        while y < rect.max.y {
            painter.line_segment(
                [Pos2::new(rect.min.x, y), Pos2::new(rect.max.x, y)],
                Stroke::new(1.0, grid_color),
            );
            y += grid_spacing;
        }
    }

    fn draw_element(
        &self,
        ui: &mut Ui,
        element: &Element,
        clip_rect: Rect,
        selected_element: &Option<ElementId>,
        relationship_mode_active: bool,
    ) -> Response {
        let rect = Rect::from_min_size(
            element.position.to_pos2(),
            element.size.to_vec2(),
        );

        // Skip if not visible
        if !clip_rect.intersects(rect) {
            return ui.interact(rect, ui.id().with(element.id), egui::Sense::hover());
        }

        let is_selected = selected_element.map_or(false, |id| id == element.id);
        // Highlight if selected or if it's the relationship source
        let is_relationship_source = self.relationship_source.map_or(false, |id| id == element.id);
        let highlight = is_selected || is_relationship_source;

        let (bg_color, border_color) = crate::ui::element_colors(element, highlight);

        // Draw shadow
        let shadow_rect = rect.translate(Vec2::new(3.0, 3.0));
        ui.painter().rect_filled(shadow_rect, 4.0, Color32::from_black_alpha(30));

        // Draw element background
        ui.painter().rect_filled(rect, 4.0, bg_color);

        // Draw border (thicker if selected or in relationship mode)
        let stroke_width = if highlight { 3.0 } else { 2.0 };
        let final_border_color = if is_relationship_source {
            Color32::from_rgb(0, 150, 0) // Green highlight for relationship source
        } else {
            border_color
        };
        ui.painter().rect_stroke(
            rect,
            4.0,
            Stroke::new(stroke_width, final_border_color),
            StrokeKind::Middle,
        );

        // Draw icon
        let icon = crate::ui::get_element_icon(element);
        let icon_pos = rect.min + Vec2::new(8.0, 8.0);
        ui.painter().text(
            icon_pos,
            egui::Align2::LEFT_TOP,
            icon,
            egui::FontId::proportional(20.0),
            Color32::BLACK,
        );

        // Draw name
        let name_pos = rect.min + Vec2::new(8.0, 36.0);
        ui.painter().text(
            name_pos,
            egui::Align2::LEFT_TOP,
            element.name(),
            egui::FontId::proportional(13.0),
            Color32::BLACK,
        );

        // Draw description (truncated)
        let desc = truncate_text(element.description(), 25);
        let desc_pos = rect.min + Vec2::new(8.0, 54.0);
        ui.painter().text(
            desc_pos,
            egui::Align2::LEFT_TOP,
            desc,
            egui::FontId::proportional(10.0),
            Color32::from_gray(80),
        );

        // Interaction
        ui.interact(rect, ui.id().with(element.id), egui::Sense::click_and_drag())
    }

    fn draw_relationship(
        &self,
        painter: &egui::Painter,
        source: &Element,
        target: &Element,
        rel: &Relationship,
        _clip_rect: Rect,
    ) {
        let source_pos = source.position;
        let target_pos = target.position;
        let source_size = source.size;
        let target_size = target.size;

        let source_center = Pos2::new(
            source_pos.x + source_size.width * 0.5,
            source_pos.y + source_size.height * 0.5,
        );
        let target_center = Pos2::new(
            target_pos.x + target_size.width * 0.5,
            target_pos.y + target_size.height * 0.5,
        );

        // Calculate edge points
        let source_edge = self.calculate_edge_point(source_pos, source_size, target_center);
        let target_edge = self.calculate_edge_point(target_pos, target_size, source_center);

        // Draw line
        painter.line_segment(
            [source_edge, target_edge],
            Stroke::new(2.0, Color32::from_gray(100)),
        );

        // Draw arrowhead
        self.draw_arrowhead(painter, target_edge, source_edge);

        // Draw label
        let mid_point = Pos2::new(
            (source_edge.x + target_edge.x) * 0.5,
            (source_edge.y + target_edge.y) * 0.5,
        );
        painter.text(
            mid_point,
            egui::Align2::CENTER_CENTER,
            &rel.description,
            egui::FontId::proportional(10.0),
            Color32::from_gray(60),
        );
    }

    fn draw_preview_relationship(
        &self,
        painter: &egui::Painter,
        source: &Element,
        mouse_pos: Pos2,
    ) {
        let source_pos = source.position;
        let source_size = source.size;

        let source_center = Pos2::new(
            source_pos.x + source_size.width * 0.5,
            source_pos.y + source_size.height * 0.5,
        );

        // Calculate edge point from source
        let source_edge = self.calculate_edge_point(source_pos, source_size, mouse_pos);

        // Draw dashed preview line
        let preview_color = Color32::from_rgb(0, 150, 0);
        painter.line_segment(
            [source_edge, mouse_pos],
            Stroke::new(2.0, preview_color),
        );

        // Draw preview arrowhead at mouse position
        let direction = (mouse_pos - source_edge).normalized();
        let perpendicular = Vec2::new(-direction.y, direction.x);
        let arrow_size = 10.0;
        let base = mouse_pos - direction * arrow_size;
        let p1 = base + perpendicular * arrow_size * 0.5;
        let p2 = base - perpendicular * arrow_size * 0.5;

        painter.add(egui::Shape::convex_polygon(
            vec![mouse_pos, p1, p2],
            preview_color,
            Stroke::new(1.0, preview_color),
        ));
    }

    fn calculate_edge_point(&self, position: Position, size: Size, target: Pos2) -> Pos2 {
        let center = Pos2::new(
            position.x + size.width * 0.5,
            position.y + size.height * 0.5,
        );

        let direction_vec = target - center;
        let direction = direction_vec.normalized();

        // Calculate intersection with rectangle
        let half_width = size.width * 0.5;
        let half_height = size.height * 0.5;

        let dx = if direction.x.abs() > 0.001 {
            half_width / direction.x.abs()
        } else {
            f32::INFINITY
        };
        let dy = if direction.y.abs() > 0.001 {
            half_height / direction.y.abs()
        } else {
            f32::INFINITY
        };

        let distance = dx.min(dy);
        Pos2::new(
            center.x + direction.x * distance,
            center.y + direction.y * distance,
        )
    }

    fn draw_arrowhead(&self, painter: &egui::Painter, tip: Pos2, from: Pos2) {
        let direction = (tip - from).normalized();
        let perpendicular = Vec2::new(-direction.y, direction.x);

        let arrow_size = 10.0;
        let base = tip - direction * arrow_size;

        let p1 = base + perpendicular * arrow_size * 0.5;
        let p2 = base - perpendicular * arrow_size * 0.5;

        painter.add(egui::Shape::convex_polygon(
            vec![tip, p1, p2],
            Color32::from_gray(100),
            Stroke::new(1.0, Color32::from_gray(100)),
        ));
    }
}

fn truncate_text(text: &str, max_len: usize) -> String {
    if text.chars().count() <= max_len {
        text.to_string()
    } else {
        let truncated: String = text.chars().take(max_len).collect();
        format!("{}...", truncated)
    }
}
