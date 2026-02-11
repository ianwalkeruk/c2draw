//! UI tests for C2Draw using egui-kittest
//!
//! These tests verify UI interactions including:
//! - Basic UI rendering
//! - Button clicks
//! - Checkbox interactions
//! - Element creation through UI

use egui::accesskit::Toggled;
use egui_kittest::{Harness, kittest::{Queryable, NodeT}};

/// Basic test that creates a Harness and renders a simple UI
/// Verifies that the harness can be created and the UI renders without panicking
#[test]
fn basic_harness_creation_and_render() {
    let mut harness = Harness::new_ui(|ui| {
        ui.label("Hello, C2Draw!");
        ui.button("Click me");
    });

    // Run the harness to process any pending events
    harness.run();

    // Verify the UI elements exist by querying them
    let label = harness.get_by_label("Hello, C2Draw!");
    assert!(label.accesskit_node().role() == egui::accesskit::Role::Label);

    let button = harness.get_by_label("Click me");
    assert!(button.accesskit_node().role() == egui::accesskit::Role::Button);
}

/// Test that interacts with a checkbox
/// Verifies checkbox state changes when clicked
#[test]
fn checkbox_interaction() {
    let checked = false;
    let mut harness = Harness::new_ui_state(|ui, checked: &mut bool| {
        ui.checkbox(checked, "Toggle me!");
    }, checked);

    // Initial state should be unchecked
    let checkbox = harness.get_by_label("Toggle me!");
    assert_eq!(checkbox.accesskit_node().toggled(), Some(Toggled::False));

    // Click the checkbox
    checkbox.click();
    harness.run();

    // Verify state changed
    assert!(*harness.state(), "Checkbox state should be true after click");

    // Query again after run to verify the UI reflects the new state
    let checkbox = harness.get_by_label("Toggle me!");
    assert_eq!(checkbox.accesskit_node().toggled(), Some(Toggled::True));
}

/// Test that interacts with a button
/// Verifies button click updates state
#[test]
fn button_interaction() {
    let click_count = 0u32;
    let mut harness = Harness::new_ui_state(|ui, count: &mut u32| {
        ui.label(format!("Clicked: {} times", count));
        if ui.button("Increment").clicked() {
            *count += 1;
        }
    }, click_count);

    // Initial state
    harness.run();
    assert_eq!(*harness.state(), 0, "Initial click count should be 0");

    // Click the button
    let button = harness.get_by_label("Increment");
    button.click();
    harness.run();

    // Verify count incremented
    assert_eq!(*harness.state(), 1, "Click count should be 1 after first click");

    // Click multiple times
    for _ in 0..4 {
        let button = harness.get_by_label("Increment");
        button.click();
        harness.run();
    }

    assert_eq!(*harness.state(), 5, "Click count should be 5 after 5 total clicks");
}

/// Test that verifies multiple UI elements can be queried
#[test]
fn multiple_element_query() {
    let harness = Harness::new_ui(|ui| {
        ui.vertical(|ui| {
            ui.label("First label");
            ui.label("Second label");
            ui.label("Third label");
            ui.button("First button");
            ui.button("Second button");
        });
    });

    // Query all labels
    let labels: Vec<_> = harness.query_all_by_label_contains("label").collect();
    assert_eq!(labels.len(), 3, "Should find 3 labels");

    // Query all buttons
    let buttons: Vec<_> = harness.query_all_by_role(egui::accesskit::Role::Button).collect();
    assert_eq!(buttons.len(), 2, "Should find 2 buttons");
}

/// Test that simulates UI interaction with text elements
#[test]
fn text_edit_interaction() {
    let text = String::from("Initial text");
    let mut harness = Harness::new_ui_state(|ui, text: &mut String| {
        ui.text_edit_singleline(text);
    }, text);

    harness.run();

    // Verify text field exists by querying for text input role
    let text_input = harness.query_by_role(egui::accesskit::Role::TextInput);
    assert!(text_input.is_some(), "Text input field should exist");
}

/// Integration test that combines multiple UI interactions
#[test]
fn complex_ui_interaction() {
    #[derive(Debug)]
    struct AppState {
        counter: i32,
        enabled: bool,
        text: String,
    }

    let state = AppState {
        counter: 0,
        enabled: false,
        text: String::from("Hello"),
    };

    let mut harness = Harness::new_ui_state(|ui, state: &mut AppState| {
        ui.heading("Complex UI Test");

        // Checkbox to enable/disable counter
        ui.checkbox(&mut state.enabled, "Enable counter");

        // Counter display and buttons
        ui.horizontal(|ui| {
            ui.label(format!("Counter: {}", state.counter));
            if ui.button("Increment").clicked() && state.enabled {
                state.counter += 1;
            }
            if ui.button("Decrement").clicked() && state.enabled {
                state.counter -= 1;
            }
        });

        // Text edit
        ui.text_edit_singleline(&mut state.text);
    }, state);

    harness.run();
    assert_eq!(harness.state().counter, 0);
    assert!(!harness.state().enabled);

    // Try to increment (should not work when disabled)
    let increment_btn = harness.get_by_label("Increment");
    increment_btn.click();
    harness.run();
    assert_eq!(harness.state().counter, 0, "Counter should not change when disabled");

    // Enable the counter
    let checkbox = harness.get_by_label("Enable counter");
    checkbox.click();
    harness.run();
    assert!(harness.state().enabled);

    // Now increment should work
    let increment_btn = harness.get_by_label("Increment");
    increment_btn.click();
    harness.run();
    assert_eq!(harness.state().counter, 1);

    // Decrement
    let decrement_btn = harness.get_by_label("Decrement");
    decrement_btn.click();
    harness.run();
    assert_eq!(harness.state().counter, 0);
}
