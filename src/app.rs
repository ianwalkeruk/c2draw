use crate::export::{DiagramExporter, MermaidExporter, PlantUmlExporter};
use crate::model::{ContainerType, Diagram, DiagramType, Element, ElementType, Position, Relationship};
use crate::ui::canvas::Canvas;
use eframe::egui;
use egui::{CentralPanel, Color32, Context, Id, SidePanel, TopBottomPanel};

/// Main application state
pub struct C2DrawApp {
    diagram: Diagram,
    canvas: Canvas,
    selected_element: Option<crate::model::ElementId>,
    file_path: Option<std::path::PathBuf>,
    show_export_window: bool,
    export_content: String,
    export_title: String,
}

impl Default for C2DrawApp {
    fn default() -> Self {
        let mut app = Self {
            diagram: Diagram::default(),
            canvas: Canvas::new(),
            selected_element: None,
            file_path: None,
            show_export_window: false,
            export_content: String::new(),
            export_title: String::new(),
        };
        // Add some example elements
        app.add_example_elements();
        app
    }
}

impl C2DrawApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn add_example_elements(&mut self) {
        // Add a person
        let person = Element::new(
            ElementType::person("User", "A user of the system"),
            Position::new(50.0, 50.0),
        );
        let person_id = person.id;
        self.diagram.add_element(person);

        // Add a system
        let system = Element::new(
            ElementType::system("My System", "The main software system"),
            Position::new(300.0, 50.0),
        );
        let system_id = system.id;
        self.diagram.add_element(system);

        // Add a relationship
        self.diagram.add_relationship(Relationship::new(
            person_id,
            system_id,
            "Uses",
        ));
    }

    fn new_diagram(&mut self) {
        self.diagram = Diagram::default();
        self.selected_element = None;
        self.file_path = None;
    }

    fn save_diagram(&mut self) {
        if let Some(path) = &self.file_path {
            if let Ok(json) = self.diagram.to_json() {
                let _ = std::fs::write(path, json);
            }
        } else {
            self.save_diagram_as();
        }
    }

    fn save_diagram_as(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("C2Draw Diagram", &["c4d"])
            .add_filter("JSON", &["json"])
            .save_file()
        {
            if let Ok(json) = self.diagram.to_json() {
                let _ = std::fs::write(&path, json);
                self.file_path = Some(path);
            }
        }
    }

    fn open_diagram(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("C2Draw Diagram", &["c4d"])
            .add_filter("JSON", &["json"])
            .pick_file()
        {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(diagram) = Diagram::from_json(&content) {
                    self.diagram = diagram;
                    self.selected_element = None;
                    self.file_path = Some(path);
                }
            }
        }
    }

    fn export_plantuml(&mut self) {
        let exporter = PlantUmlExporter::new();
        self.export_content = exporter.export(&self.diagram);
        self.export_title = "C4-PlantUML Export".to_string();
        self.show_export_window = true;
    }

    fn export_mermaid(&mut self) {
        let exporter = MermaidExporter::new();
        self.export_content = exporter.export(&self.diagram);
        self.export_title = "Mermaid Export".to_string();
        self.show_export_window = true;
    }

    fn add_element(&mut self, element_type: ElementType) {
        let index = self.diagram.elements.len();
        let position = crate::ui::default_element_position(index);
        let element = Element::new(element_type, position);
        self.diagram.add_element(element);
    }

    fn delete_selected(&mut self) {
        if let Some(id) = self.selected_element {
            self.diagram.remove_element(id);
            self.selected_element = None;
        }
    }

    fn render_sidebar(&mut self, ctx: &Context) {
        SidePanel::left("sidebar")
            .default_width(150.0)
            .show(ctx, |ui| {
                ui.heading("Elements");
                ui.separator();

                ui.label("C1 - System Context");
                if ui.button("➕ Person").clicked() {
                    self.add_element(ElementType::person("New Person", "Description"));
                }
                if ui.button("➕ External Person").clicked() {
                    self.add_element(ElementType::external_person("External User", "Description"));
                }
                if ui.button("➕ System").clicked() {
                    self.add_element(ElementType::system("New System", "Description"));
                }
                if ui.button("➕ External System").clicked() {
                    self.add_element(ElementType::external_system("External System", "Description"));
                }

                ui.separator();
                ui.label("C2 - Container");
                if ui.button("➕ Web App").clicked() {
                    self.add_element(ElementType::container(
                        "Web Application",
                        "Description",
                        ContainerType::WebApplication,
                        "React/Spring Boot",
                    ));
                }
                if ui.button("➕ Database").clicked() {
                    self.add_element(ElementType::container(
                        "Database",
                        "Description",
                        ContainerType::Database,
                        "PostgreSQL",
                    ));
                }
                if ui.button("➕ Queue").clicked() {
                    self.add_element(ElementType::container(
                        "Message Queue",
                        "Description",
                        ContainerType::Queue,
                        "RabbitMQ",
                    ));
                }

                ui.separator();
                ui.label("Actions");
                if ui.button("🔗 Add Relationship").clicked() {
                    // Relationship creation mode would be implemented here
                }
                if ui.button("🗑️ Delete Selected").clicked() {
                    self.delete_selected();
                }
            });
    }

    fn render_properties_panel(&mut self, ctx: &Context) {
        SidePanel::right("properties")
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Properties");
                ui.separator();

                if let Some(id) = self.selected_element {
                    if let Some(element) = self.diagram.get_element_mut(id) {
                        ui.label("Type");
                        ui.label(element.element_type.type_name());
                        ui.separator();

                        ui.label("Name");
                        let mut name = element.name().to_string();
                        if ui.text_edit_singleline(&mut name).changed() {
                            element.set_name(name);
                        }

                        ui.label("Description");
                        let mut desc = element.description().to_string();
                        ui.text_edit_multiline(&mut desc);
                        element.set_description(desc);

                        ui.separator();
                        if ui.button("Delete Element").clicked() {
                            self.diagram.remove_element(id);
                            self.selected_element = None;
                        }
                    }
                } else {
                    ui.label("No element selected");
                }
            });
    }

    fn render_menu_bar(&mut self, ctx: &Context) {
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() {
                        self.new_diagram();
                        ui.close();
                    }
                    if ui.button("Open...").clicked() {
                        self.open_diagram();
                        ui.close();
                    }
                    ui.separator();
                    if ui.button("Save").clicked() {
                        self.save_diagram();
                        ui.close();
                    }
                    if ui.button("Save As...").clicked() {
                        self.save_diagram_as();
                        ui.close();
                    }
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Export", |ui| {
                    if ui.button("C4-PlantUML...").clicked() {
                        self.export_plantuml();
                        ui.close();
                    }
                    if ui.button("Mermaid...").clicked() {
                        self.export_mermaid();
                        ui.close();
                    }
                });

                ui.menu_button("View", |ui| {
                    ui.label("Diagram Type");
                    ui.radio_value(&mut self.diagram.diagram_type, DiagramType::SystemContext, "System Context (C1)");
                    ui.radio_value(&mut self.diagram.diagram_type, DiagramType::Container, "Container (C2)");
                });
            });
        });
    }

    fn render_export_window(&mut self, ctx: &Context) {
        if self.show_export_window {
            egui::Window::new(&self.export_title)
                .id(Id::new("export_window"))
                .collapsible(false)
                .resizable(true)
                .default_size([500.0, 400.0])
                .show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut self.export_content)
                                .code_editor()
                                .desired_rows(20),
                        );
                    });

                    ui.horizontal(|ui| {
                        if ui.button("Copy to Clipboard").clicked() {
                            ctx.copy_text(self.export_content.clone());
                        }
                        if ui.button("Close").clicked() {
                            self.show_export_window = false;
                        }
                    });
                });
        }
    }
}

impl eframe::App for C2DrawApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.render_menu_bar(ctx);
        self.render_sidebar(ctx);
        self.render_properties_panel(ctx);

        CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).fill(Color32::from_gray(240)))
            .show(ctx, |ui| {
                // Render the canvas
                self.canvas.render(
                    ui,
                    &mut self.diagram.elements,
                    &self.diagram.relationships,
                    &mut self.selected_element,
                );
            });

        self.render_export_window(ctx);
    }
}
