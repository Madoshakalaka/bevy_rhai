//! Provides an easy way to use [Rhai] with [Bevy].
//!
//! [Rhai]: https://github.com/rhaiscript/rhai
//! [Bevy]: https://github.com/bevyengine/bevy

#![forbid(missing_docs)]

pub mod asset;
pub mod components;

use asset::*;
use bevy::prelude::*;

/// Adds Rhai functionality to an application.
pub struct RhaiPlugin;

impl Plugin for RhaiPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_asset::<RhaiScript>()
      .init_asset_loader::<RhaiScriptLoader>();
  }
}

pub mod prelude {
  //! Contains commonly used items.

  use super::*;

  pub use super::RhaiPlugin;
  pub use asset::*;
  pub use components::*;
}
