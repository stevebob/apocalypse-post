use game::*;
use content_types::*;
use ecs_content::Entity;

use behaviour::LeafResolution;
use math::Direction;
use math::Coord;

const X_RANGE: isize = 2;
const Y_RANGE: isize = 3;

pub fn car_chace<K: KnowledgeRenderer>() -> BehaviourLeaf<K> {
    BehaviourLeaf::new(move |input| {

        let entity = input.ecs.entity(input.entity_id);

        let position = entity.copy_position().unwrap();
        let knowledge = entity.borrow_simple_npc_knowledge().unwrap();
        let level_knowledge = knowledge.level(input.level_id);
        let speed = entity.copy_current_speed().unwrap();
        let max_speed = entity.copy_max_speed().unwrap();

        let target = if let Some(t) = level_knowledge.any_target() {
            t
        } else {
            // if you can't see a target, do nothing
            return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::ChangeSpeed(input.entity_id, ChangeSpeed::Decelerate)))
        };

        let next_coord = position + Coord::new(1, 0);
        if level_knowledge.get_with_default(next_coord).solid() {
            if position.y > target.y + Y_RANGE {
                return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::Steer(input.entity_id, SteerDirection::Up)))
            } else {
                return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::Steer(input.entity_id, SteerDirection::Down)))
            }
        }

        if position.x > target.x + X_RANGE {
            // we are in front of the target

            if speed > 1 {
                return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::ChangeSpeed(input.entity_id, ChangeSpeed::Decelerate)))
            }

            if position.y > target.y + Y_RANGE {
                if speed == 0 {
                    return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::ChangeSpeed(input.entity_id, ChangeSpeed::Accelerate)))
                }
                return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::Steer(input.entity_id, SteerDirection::Up)))
            } else if position.y < target.y - Y_RANGE {
                if speed == 0 {
                    return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::ChangeSpeed(input.entity_id, ChangeSpeed::Accelerate)))
                }
                return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::Steer(input.entity_id, SteerDirection::Down)))
            }

        } else if position.x < target.x - X_RANGE {
            // we are behind the target

            // speed up to our max speed
            if speed < max_speed {
                return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::ChangeSpeed(input.entity_id, ChangeSpeed::Accelerate)))
            }

            if position.y > target.y + Y_RANGE {
                return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::Steer(input.entity_id, SteerDirection::Up)))
            } else if position.y < target.y - Y_RANGE {
                return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::Steer(input.entity_id, SteerDirection::Down)))
            }

        } else {
            // we are lined up with the target

            if position.y > target.y + Y_RANGE {
                return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::Steer(input.entity_id, SteerDirection::Up)))
            } else if position.y < target.y - Y_RANGE {
                return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::Steer(input.entity_id, SteerDirection::Down)))
            }

            let weapon_slots = entity.borrow_weapon_slots().unwrap();

            if position.y > target.y + 1 {
                let gun_id = *weapon_slots.get(Direction::North).unwrap();
                return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::FireGun {
                    gun_id: gun_id,
                    shooter_id: input.entity_id,
                    direction: Direction::North,
                }));
            } else if position.y < target.y - 1 {
                let gun_id = *weapon_slots.get(Direction::South).unwrap();
                return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::FireGun {
                    gun_id: gun_id,
                    shooter_id: input.entity_id,
                    direction: Direction::South,
                }));
            } else {
                return LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::Steer(input.entity_id, SteerDirection::Up)))
            }
        }

        LeafResolution::Yield(MetaAction::ActionArgs(ActionArgs::Null))
    })
}
