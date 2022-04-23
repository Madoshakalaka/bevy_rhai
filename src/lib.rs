//! Provides an easy way to integrate [Rhai] with [Bevy].
//!
//! [Rhai]: https://github.com/rhaiscript/rhai
//! [Bevy]: https://github.com/bevyengine/bevy

#![forbid(missing_docs)]

use core::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use bevy_app::{App, Plugin};
use bevy_asset::{
  AddAsset, AssetLoader, BoxedFuture, LoadContext, LoadedAsset,
};
use bevy_ecs::prelude::Component;
use bevy_reflect::TypeUuid;
use rhai::{Engine, Scope, AST};

/// Adds [`StandardScript`] and its corresponding [`ScriptLoader`].
pub struct StandardScriptPlugin;

impl Plugin for StandardScriptPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_asset::<StandardScript>()
      .init_asset_loader::<ScriptLoader<StandardScript>>();
  }
}

/// Custom Rhai script asset.
pub trait Script: Send + Sync + TypeUuid + 'static {
  /// Creates a [`Script`] from an [`AST`].
  fn from_ast(ast: AST) -> Self
  where
    Self: Sized;
}

/// Handles loading [`Script`] assets.
pub struct ScriptLoader<T> {
  /// Valid file extensions for scripts.
  pub extensions: Vec<&'static str>,
  /// [`Engine`] used to compile scripts into [`AST`]s.
  pub engine: Engine,
  _t: PhantomData<T>,
}

impl<T> ScriptLoader<T>
where
  T: Script,
{
  /// Creates a default [`ScriptLoader`].
  pub fn new() -> Self {
    Self {
      extensions: vec![],
      ..Default::default()
    }
  }

  /// Uses the provided [`Engine`].
  ///
  /// ## Example
  ///
  /// ```
  /// # use bevy_rhai::*;
  /// # use rhai::*;
  /// # fn main() {
  /// ScriptLoader::<StandardScript>::new()
  ///   .with_engine({
  ///     let mut engine = Engine::new_raw();
  ///     engine.disable_symbol("eval");
  ///     engine
  ///   });
  /// # }
  /// ```
  pub fn with_engine(mut self, engine: Engine) -> Self {
    self.engine = engine;
    self
  }

  /// Adds a single extension.
  ///
  /// ## Example
  ///
  /// ```
  /// # use bevy_rhai::*;
  /// # fn main() {
  /// ScriptLoader::<StandardScript>::new()
  ///   .add_extension("rhai");
  /// # }
  /// ```
  pub fn add_extension(mut self, extension: &'static str) -> Self {
    self.extensions.push(extension);
    self
  }

  /// Adds multiple extensions.
  ///
  /// ## Example
  ///
  /// ```
  /// # use bevy_rhai::*;
  /// # fn main() {
  /// ScriptLoader::<StandardScript>::new()
  ///   .add_extensions(&["rhai"]);
  /// # }
  /// ```
  pub fn add_extensions(
    mut self,
    extensions: impl AsRef<[&'static str]>,
  ) -> Self {
    for extension in extensions.as_ref() {
      self.extensions.push(extension);
    }
    self
  }
}

impl<T> Default for ScriptLoader<T>
where
  T: Script,
{
  fn default() -> Self {
    Self {
      extensions: vec!["rhai"],
      engine: Engine::new_raw(),
      _t: Default::default(),
    }
  }
}

impl<T> AssetLoader for ScriptLoader<T>
where
  T: Script,
{
  #[inline]
  fn extensions(&self) -> &[&str] {
    &self.extensions
  }

  fn load<'a>(
    &'a self,
    bytes: &'a [u8],
    load_context: &'a mut LoadContext,
  ) -> BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
    Box::pin(async move {
      let ast = self.engine.compile(core::str::from_utf8(bytes)?)?;
      load_context.set_default_asset(LoadedAsset::new(T::from_ast(ast)));
      Ok(())
    })
  }
}

/// Standard Rhai script asset.
#[derive(Debug, Clone, TypeUuid)]
#[uuid = "fe38f050-725f-490e-8233-0692482b13aa"]
pub struct StandardScript {
  /// [`AST`] of the compiled script.
  pub ast: AST,
}

impl Script for StandardScript {
  fn from_ast(ast: AST) -> Self {
    Self { ast }
  }
}

/// Standard Rhai [`Engine`].
#[derive(Debug, Default, Component)]
pub struct StandardEngine(pub Engine);

impl StandardEngine {
  /// Creates a new [`StandardEngine`] with the provided [`Engine`].
  ///
  /// ## Example
  ///
  /// ```
  /// # use bevy_rhai::*;
  /// # use rhai::*;
  /// # fn main() {
  /// StandardEngine::with_engine({
  ///   let mut engine = Engine::new_raw();
  ///   engine.disable_symbol("eval");
  ///   engine
  /// });
  /// # }
  /// ```
  pub fn with_engine(engine: Engine) -> Self {
    Self(engine)
  }
}

impl Deref for StandardEngine {
  type Target = Engine;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for StandardEngine {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

/// Standard Rhai [`Scope`].
#[derive(Debug, Default, Component)]
pub struct StandardScope(pub Scope<'static>);

impl StandardScope {
  /// Creates a new [`StandardScope`] with the provided [`Scope`].
  ///
  /// ## Example
  ///
  /// ```
  /// # use bevy_rhai::*;
  /// # use rhai::*;
  /// # fn main() {
  /// StandardScope::with_scope({
  ///   let mut scope = Scope::new();
  ///   scope.push("life", 42_i64);
  ///   scope
  /// });
  /// # }
  /// ```
  pub fn with_scope(scope: Scope<'static>) -> Self {
    Self(scope)
  }
}

impl Deref for StandardScope {
  type Target = Scope<'static>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for StandardScope {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}
