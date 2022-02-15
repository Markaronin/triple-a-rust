#[derive(PartialEq, Eq, Hash)]
/**
 * Targeted attacks are special types of combat maneuvers (like cavalry charge or siege bombardment) that your units can perform against specific types of enemies.
 * Targeted attacks work similarly to regular attacks: a dice is rolled for each attack, and if the roll is a success, the enemy must select a casualty.
 * Targeted attacks, however, have a limited set of targets, and casualties must be selected from among those units (e.g. walls or fortresses, in the case of siege).
 * In some cases this allows you to target stronger units directly, without having to clear weaker units first.
 */
pub enum TargetedAttack {
    /**
     * Some cavalry units can perform a charge against enemy units with infantry type.
     * Fires in the first battle round only, and only when attacking. Will not fire if enemy has fortifications present (unless the respective map option is turned off).
     */
    Charge { amount: usize },
    /**
     * Some units can try to outflank the battle line of the enemy, attacking the vulnerable ranged infantry and siege engines directly.
     * Will not fire if enemy has fortifications present (unless the respective map option is turned off).
     */
    Flank { amount: usize },
    /**
     * Massive units enter the battlefield by simply trampling over the enemy army.
     * Targets both infantry and cavalry, fires in the first battle round only, and only when attacking.
     * Will not fire if enemy has fortifications present (unless the respective map option is turned off).
     */
    Trample { attacks: usize, amount: usize },
    /**
     * Heavy infantry in closed formation (especially if armed with spears or pikes) can effectively stop cavalry charges.
     * Targets are units performing a charge special attack. Fires in the first battle round only, and only when defending.
     * At most 1 unit can fire per target (one charging cavalry can only try to get impaled on one spearwall at a time), so if you have more units with formation than there are enemy units with charge, only a part of your units will fire.
     */
    Formation { amount: usize },
    /**
     * Archers can try to shoot down attacking enemy air units. Only works when defending.
     */
    AntiAir { amount: usize },
    /**
     * Some units, like siege engines or ents, can rapidly dismantle defensive buildings: walls and fortresses.
     * Ancient fortifications are, however, impregnable to anything the Third Age could come up with.
     * Fires only when attacking, and in the first battle round only.
     * Casualties die instantly, regardless of hitpoints.
     */
    Siege { attacks: usize, amount: usize },
    /**
     * Powerful beings, like Maiar or the Nazgûl, fight their own divine magic-duel during the battle.
     * Targets are other units with duel targeted attack.
     * Casualties die instantly, regardless of hitpoints.
     * Watch out for this, your most powerful units can easily get killed by this ability.
     */
    Duel { amount: usize },
}
