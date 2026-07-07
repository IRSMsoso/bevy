use bevy_ecs::define_label;

define_label!(
    #[diagnostic::on_unimplemented(
        note = "consider annotating `{Self}` with `#[derive(ObserverSet)]`"
    )]
    ObserverSet,
);