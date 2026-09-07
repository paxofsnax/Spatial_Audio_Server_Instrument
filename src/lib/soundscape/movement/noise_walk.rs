use audio;
use mindtree_utils::noise_walk;
use metres::Metres;
use nannou::prelude::*;
use std::time;
use super::BoundingRect;
use utils::{duration_to_secs, pt2};

// The point and vector types in exhibition space.
type Point = Point2<Metres>;
type Vector = Vector2<Metres>;

/// A movement implementation that wanders smoothly through Perlin noise.
///
/// Two independent 1-D noise signals form a drifting direction vector and the
/// sound travels along it at `speed`, contained within the installation area
/// scaled by `normalised_dimensions`. When the noise vector passes through
/// zero the sound briefly stalls and changes heading, giving the walk its
/// organic "breathing" quality.
#[derive(Debug)]
pub struct NoiseWalk {
    /// The rate at which the sound travels in metres per second.
    pub speed: f64,
    /// The rate at which the direction of travel changes (noise phase units per second).
    ///
    /// Low values produce long, lazy arcs; high values produce twitchy wandering.
    pub wobble_scale: f64,
    /// Describes the area within which the sound may roam using a normalised value.
    ///
    /// `0.0` means all points will be in the center.
    /// `1.0` means the sound may roam to the bounds of the installation area.
    pub normalised_dimensions: Vector2<f64>,
    /// Whether or not the sound's orientation should follow its direction of travel.
    pub directional: bool,
    /// State that is updated during a call to `update`.
    state: State,
}

/// State that is updated during a call to `update`.
#[derive(Debug)]
struct State {
    /// The current phase of each of the independent noise signals driving the direction.
    phase: Vector2<f64>,
    /// The most recently sampled direction vector, used for the sound's orientation.
    direction: Vector2<f64>,
    /// The current location of the sound.
    location: Point,
}

impl NoiseWalk {
    /// Create a new **NoiseWalk** movement type.
    ///
    /// The `phase_offset` decorrelates the walk from other noise walking sounds so
    /// that simultaneous walkers do not move in lockstep.
    pub fn new(
        speed: f64,
        wobble_scale: f64,
        normalised_dimensions: Vector2<f64>,
        directional: bool,
        phase_offset: Vector2<f64>,
        installation_bounding_rect: &BoundingRect,
    ) -> Self
    {
        // Antopia: guard against degenerate wobble values (e.g. loaded from
        // project state) which would otherwise freeze the walk entirely.
        let wobble_scale = wobble_scale.max(0.0);
        let (middle, _half) = middle_and_half_dimensions(installation_bounding_rect, normalised_dimensions);
        let middle = pt2::to_metres(middle);
        let state = State {
            phase: phase_offset,
            direction: vec2(0.0, 0.0),
            location: middle,
        };
        NoiseWalk {
            speed,
            wobble_scale,
            normalised_dimensions,
            directional,
            state,
        }
    }
}

// The middle of the walkable area and its dimensions, in f64 space.
fn middle_and_half_dimensions(
    bounding_rect: &BoundingRect,
    normalised_dimensions: Vector2<f64>,
) -> (Point2<f64>, Vector2<f64>)
{
    let middle = bounding_rect.middle();
    let width = (bounding_rect.width() * normalised_dimensions.x).0;
    let height = (bounding_rect.height() * normalised_dimensions.y).0;
    let half = Vector2 { x: width * 0.5, y: height * 0.5 };
    let middle = Point2 { x: middle.x.0, y: middle.y.0 };
    (middle, half)
}

impl NoiseWalk {
    /// The current location and orientation of the **NoiseWalk** for use within the audio
    /// engine's DBAP calculations.
    pub fn position(&self) -> audio::sound::Position {
        let point = self.state.location;
        let radians = if self.directional {
            let dir = self.state.direction;
            dir.y.atan2(dir.x) as f32
        } else {
            0.0
        };
        audio::sound::Position { point, radians }
    }

    /// Update the `NoiseWalk` state for the given past amount of time.
    pub fn update(&mut self, delta_time: &time::Duration, installation_area: &BoundingRect) {
        let NoiseWalk {
            speed,
            wobble_scale,
            normalised_dimensions,
            directional: _,
            ref mut state,
        } = *self;

        let dt = duration_to_secs(delta_time);

        // Advance the noise phases at the wobble rate. This determines how quickly the
        // direction of travel changes.
        state.phase = state.phase + vec2(wobble_scale * dt, wobble_scale * dt);

        // Sample the two independent noise signals to form the direction of travel.
        let vx = noise_walk(state.phase.x);
        let vy = noise_walk(state.phase.y);
        let direction = vec2(vx, vy);
        state.direction = direction;

        // Travel along the direction at `speed`, integrating the current location.
        let location = pt2::to_f64(state.location);
        let mut new_location = location + direction * (speed * dt);

        // Contain the position within the installation area scaled by the normalised
        // dimensions.
        let (middle, half) =
            middle_and_half_dimensions(installation_area, normalised_dimensions);
        let (min_x, max_x) = (middle.x - half.x, middle.x + half.x);
        let (min_y, max_y) = (middle.y - half.y, middle.y + half.y);
        new_location.x = new_location.x.max(min_x).min(max_x);
        new_location.y = new_location.y.max(min_y).min(max_y);
        state.location = pt2::to_metres(new_location);
    }
}
