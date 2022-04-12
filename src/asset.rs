//! Contains Bevy asset-related items for Rhai.

use bevy::{asset::*, reflect::TypeUuid};
use rhai::*;

/// A compiled Rhai script [`Asset`].
#[derive(Debug, Clone, TypeUuid)]
#[uuid = "fe38f050-725f-490e-8233-0692482b13aa"]
pub struct RhaiScript {
  /// The compiled script.
  pub ast: AST,
}

impl RhaiScript {
  /// Creates a new `RhaiScript`.
  pub fn new<S: AsRef<str>>(
    engine: &Engine,
    source: S,
  ) -> Result<Self, ParseError> {
    Ok(Self {
      ast: engine.compile(source)?,
    })
  }
}

/// Handles loading and compiling [`RhaiScript`]s.
#[derive(Debug)]
pub struct RhaiScriptLoader {
  /// A custom engine to use when compiling script assets.
  pub engine: Engine,
}

impl Default for RhaiScriptLoader {
  fn default() -> Self {
    Self {
      engine: Engine::new_raw(),
    }
  }
}

impl AssetLoader for RhaiScriptLoader {
  fn extensions(&self) -> &[&str] {
    &["rhai"]
  }

  fn load<'a>(
    &'a self,
    bytes: &'a [u8],
    load_context: &'a mut LoadContext,
  ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
    Box::pin(async move {
      let asset =
        RhaiScript::new(&self.engine, &String::from_utf8(bytes.to_vec())?)?;

      load_context.set_default_asset(LoadedAsset::new(asset));
      Ok(())
    })
  }
}
