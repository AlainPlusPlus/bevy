//! This crate provides a set of core widgets for Bevy UI, such as buttons, checkboxes, and sliders.
//! These widgets have no inherent styling, it's the responsibility of the user to add styling
//! appropriate for their game or application.
//!
//! # State Management
//!
//! Most of the widgets use external state management: this means that the widgets do not
//! automatically update their own internal state, but instead rely on the app to update the widget
//! state (as well as any other related game state) in response to a change event emitted by the
//! widget. The primary motivation for this is to avoid two-way data binding in scenarios where the
//! user interface is showing a live view of dynamic data coming from deeper within the game engine.

// Note on naming: the `Core` prefix is used on components that would normally be internal to the
// styled/opinionated widgets that use them. Components which are directly exposed to users above
// the widget level, like `SliderValue`, should not have the `Core` prefix.

mod core_button;
mod core_checkbox;
mod core_radio;
mod core_slider;

use bevy_app::{App, Plugin};

pub use core_button::{CoreButton, CoreButtonPlugin};
pub use core_checkbox::{CoreCheckbox, CoreCheckboxPlugin, SetChecked, ToggleChecked};
pub use core_radio::{CoreRadio, CoreRadioGroup, CoreRadioGroupPlugin};
pub use core_slider::{
    CoreSlider, CoreSliderDragState, CoreSliderPlugin, CoreSliderThumb, SetSliderValue,
    SliderRange, SliderStep, SliderValue, TrackClick,
};

/// A plugin that registers the observers for all of the core widgets. If you don't want to
/// use all of the widgets, you can import the individual widget plugins instead.
pub struct CoreWidgetsPlugin;

impl Plugin for CoreWidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            CoreButtonPlugin,
            CoreCheckboxPlugin,
            CoreRadioGroupPlugin,
            CoreSliderPlugin,
        ));
    }
}
