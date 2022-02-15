#[derive(PartialEq, Eq, Hash)]
/**
 * Units can give support to other allied units - or hinder enemy units (negative support).
 * For example, some units can give ranged support to melee units, others can spread terror and hopelessness among the ranks of the enemy.
 * Different types of support affect different types of units.
 * A single unit can only be affected once by each support type, but different support types do stack (so, for example, a single unit can be affected both by leadership and ranged support, but not two instances of leadership).
 */
pub enum SupportType {
    /**
     * Gives power to allied melee units. Works only when attacking.
     */
    Ranged { amount: usize, units: usize },
    /**
     * Takes away power from enemy melee units.
     */
    Armor { amount: usize, units: usize },
    /**
     * Takes away power from enemy ranged units.
     * (Ranged unit type, not to be confused with the identically named ranged support type. For example, units of the siege engine type can give ranged support, too.)
     */
    Shield { amount: usize, units: usize },
    /**
     * Takes away power from enemy melee units. Works only when defending.
     */
    Web { amount: usize, units: usize },
    /**
     * Gives power to most allied units, except: siege engines, fortifications and ships.
     */
    Leadership { amount: usize, units: usize },
    /**
     * Takes away power from most enemy units, except: unseen units (Elves, mostly), siege engines, fortifications and ships.
     */
    Terror { amount: usize, units: usize },
    /**
     * Gives power to allied infantry. Works only when defending.
     * Takes away power from most enemy units, except: flying units, siege engines, fortifications and ships.
     */
    Battlements { amount: usize, units: usize },
}
