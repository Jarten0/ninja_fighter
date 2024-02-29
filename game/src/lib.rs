pub mod game_data {
    use bevy_ecs::schedule::ExecutorKind;
    use bevy_ecs::schedule::LogLevel;
    use bevy_ecs::schedule::Schedule;
    use bevy_ecs::schedule::ScheduleBuildSettings;
    use bevy_ecs::world::World;
    use components::*;
    use engine::schedule::ScheduleTag;

    pub fn init_components_and_resources(world: &mut World) {
        components::init_components(world);
    }

    pub fn schedule_builders() -> Vec<fn(&mut Schedule) -> ScheduleTag> {
        vec![tick_schedule, frame_schedule, init_schedule]
    }

    pub(crate) fn tick_schedule(sched: &mut Schedule) -> ScheduleTag {
        // Configuration block
        sched
            .set_build_settings(TICK_SETTINGS.clone())
            .set_executor_kind(ExecutorKind::MultiThreaded);

        // Systems block
        sched
            .add_systems(engine::systems::update)
            .add_systems(collider::collider_mesh::update)
            .add_systems(debug::update)
            .add_systems(collider::update);

        ScheduleTag::Tick
    }

    pub(crate) fn frame_schedule(draw_sched: &mut Schedule) -> ScheduleTag {
        draw_sched
            .set_build_settings(FRAME_SETTINGS.clone())
            .set_executor_kind(ExecutorKind::SingleThreaded);

        draw_sched
            .add_systems(render::draw)
            .add_systems(debug::draw);

        ScheduleTag::Frame
    }

    pub(crate) fn init_schedule(init_sched: &mut Schedule) -> ScheduleTag {
        init_sched
            .set_build_settings(INIT_SETTINGS.clone())
            .set_executor_kind(ExecutorKind::MultiThreaded);

        init_sched
            .add_systems(debug::init)
            .add_systems(protag::init);

        ScheduleTag::Init
    }

    pub(crate) static TICK_SETTINGS: ScheduleBuildSettings = ScheduleBuildSettings {
        ambiguity_detection: LogLevel::Warn,
        hierarchy_detection: LogLevel::Warn,
        use_shortnames: false,
        report_sets: true,
    };
    pub(crate) static FRAME_SETTINGS: ScheduleBuildSettings = ScheduleBuildSettings {
        ambiguity_detection: LogLevel::Warn,
        hierarchy_detection: LogLevel::Warn,
        use_shortnames: false,
        report_sets: true,
    };
    pub(crate) static INIT_SETTINGS: ScheduleBuildSettings = ScheduleBuildSettings {
        ambiguity_detection: LogLevel::Warn,
        hierarchy_detection: LogLevel::Warn,
        use_shortnames: false,
        report_sets: true,
    };
}
