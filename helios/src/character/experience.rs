use bevy::prelude::*;

#[derive(Component, Default, Copy, Clone, Debug)]
pub struct CharacterLevel(pub u8);

impl CharacterLevel {
    pub fn experience_for_level_up(&self) -> CharacterExperience {
        CharacterExperience(500 * (self.0 as u32).pow(2))
    }
}

#[derive(Component, Default, Copy, Clone, Debug)]
pub struct CharacterExperience(pub u32);

pub fn level_up(
    mut query: Query<
        (&mut CharacterLevel, &mut CharacterExperience),
        Or<(Changed<CharacterLevel>, Changed<CharacterExperience>)>,
    >,
) {
    for (mut level, mut experience) in query.iter_mut() {
        let level_up_experience = level.experience_for_level_up();
        if experience.0 >= level_up_experience.0 {
            level.0 += 1;
            experience.0 -= level_up_experience.0;
        }
    }
}
