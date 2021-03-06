use game::*;
use ecs::*;

pub fn acid(env: RuleEnv, action: &EcsAction, reactions: &mut Vec<Reaction>) -> RuleResult {

    for (entity_id, position) in action.position_profile().insertion_copy_iter() {

        if env.spatial_hash.get(position).acid() {

            let entity = env.ecs.entity(entity_id);

            if entity.contains_tyre_health() {
                reactions.push(Reaction::new(ActionArgs::AcidDamage(entity_id), 0));
            }
        }
    }

    RULE_ACCEPT
}
