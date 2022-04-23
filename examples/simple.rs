use bevy::{core::FixedTimestep, prelude::*};
use bevy_rhai::*;
use rhai::plugin::*;

#[derive(Debug, Clone, Copy, Default, Component)]
pub struct Counter {
  count: i64,
}

#[rhai::export_module]
mod counter_api {
  #[rhai_fn(get = "count", pure)]
  pub fn count(counter: &mut Counter) -> i64 {
    counter.count
  }

  #[rhai_fn(set = "count", pure)]
  pub fn set_count(counter: &mut Counter, count: i64) {
    counter.count = count
  }
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(StandardScriptPlugin)
    .add_startup_system(setup)
    .add_system_set(
      SystemSet::new()
        .with_run_criteria(FixedTimestep::steps_per_second(2.0))
        .with_system(increment),
    )
    .run()
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  let increment: Handle<StandardScript> = asset_server.load("increment.rhai");
  asset_server.watch_for_changes().unwrap();

  commands
    .spawn()
    .insert(StandardEngine::with_engine({
      let mut engine = Engine::new_raw();
      engine.set_strict_variables(true);
      engine
        .disable_symbol("eval")
        .register_type_with_name::<Counter>("Coutner")
        .register_global_module(exported_module!(counter_api).into());
      engine
    }))
    .insert(StandardScope::default())
    .insert(Counter::default())
    .insert(increment);
}

fn increment(
  scripts: Res<Assets<StandardScript>>,
  mut query: Query<(
    &StandardEngine,
    &Handle<StandardScript>,
    &mut StandardScope,
    &mut Counter,
  )>,
) {
  for (engine, script, mut scope, mut counter) in query.iter_mut() {
    if let Some(script) = scripts.get(script) {
      scope.set_or_push("counter", *counter);
      engine.run_ast_with_scope(&mut scope, &script.ast).unwrap();
      *counter = scope.get_value("counter").unwrap();
      info!("Counter: {:?}", counter);
    }
  }
}
