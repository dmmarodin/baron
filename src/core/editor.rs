use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy_editor_pls::controls::EditorControls;
use bevy_editor_pls::controls;

pub struct SetupEditorPlugin;

impl Plugin for SetupEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EditorPlugin).insert_resource(editor_controls());
    }
}

fn editor_controls() -> EditorControls {
    let mut editor_controls = EditorControls::default_bindings();
    editor_controls.unbind(controls::Action::PlayPauseEditor);

    editor_controls.insert(controls::Action::PlayPauseEditor, controls::Binding {
        input: controls::UserInput::Single(controls::Button::Keyboard(KeyCode::Escape)),
        conditions: vec![controls::BindingCondition::ListeningForText(false)],
    });

    editor_controls
}