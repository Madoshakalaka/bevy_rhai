//! Contains Bevy component-related items for Rhai.

use std::ops::{Deref, DerefMut};

use bevy::prelude::{Bundle, Component};
use rhai::*;

/// An [`Engine`] component.
#[derive(Debug, Component)]
pub struct RhaiEngine(pub rhai::Engine);

impl RhaiEngine {
  /// Creates a `RhaiEngine` from a closure that returns a custom [`Engine`].
  pub fn with_engine(mut f: impl FnMut() -> Engine) -> Self {
    Self(f())
  }
}

impl Default for RhaiEngine {
  fn default() -> Self {
    Self(Engine::new_raw())
  }
}

impl Deref for RhaiEngine {
  type Target = Engine;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for RhaiEngine {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

/// An [`Scope`] component.
#[derive(Debug, Clone, Hash, Default, Component)]
pub struct RhaiScope(pub rhai::Scope<'static>);

impl Deref for RhaiScope {
  type Target = Scope<'static>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for RhaiScope {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

/// An [`AST`] component.
#[derive(Debug, Clone, Default, Component)]
pub struct RhaiAST(pub rhai::AST);

impl Deref for RhaiAST {
  type Target = AST;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for RhaiAST {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

/// A Rhai script component bundle.
#[derive(Debug, Default, Bundle)]
pub struct RhaiBundle {
  /// The [`RhaiEngine`].
  pub engine: RhaiEngine,
  /// The [`RhaiScope`].
  pub scope: RhaiScope,
}
