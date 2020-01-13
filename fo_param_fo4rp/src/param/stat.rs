#![deny(dead_code)]
use super::prelude::*;
pub use crate::basic_impl::stat::*;

invar!(TIMEOUT_READY, 0, "Закончился");
invar!(BONUS_ZERO, 0, "НетБонуса");
invar!(BONUS_RUSH, 1, "БонусЗаВыбросАдреналина");

impl Strength {
    pub fn make_formula(&self) -> impl CrOp {
        let low_life = less_or_equal(
            misc::LifeCurrent.base(),
            (LifeMax.base() + Strength.base() + Endurance.base() * int(2)) / int(2),
        );
        let rush_condition = perk::AdrenalineRush.present()
            & not_equal(timeout::Battle.base(), TIMEOUT_READY)
            & "МалоЗдоровья".part(low_life);
        let rush_bonus = "ОтВыбросаАдреналина".part(if_else(
            "ВыбросАдреналинаДействует".part(rush_condition),
            BONUS_RUSH,
            BONUS_ZERO,
        ));

        self.sum() + rush_bonus
    }
}

invar!(DAMAGED_PERCEPTION, 1, "ПовреждённоеВосприятие");

impl Perception {
    pub fn make_formula(&self) -> impl CrOp {
        let maybe_damaged = if_else(damage::Eye.present(), DAMAGED_PERCEPTION, self.sum());
        "ОтВосприятия".part(maybe_damaged)
            + "ОтНочнойПерсоны".part(traits::NightPerson.make_bonus().compat())
    }
}

impl Endurance {
    // default .sum() formula
}

impl Charisma {
    // default .sum() formula
}

impl Intellect {
    pub fn make_formula(&self) -> impl CrOp {
        "ОтИнтеллекта".part(self.sum())
            + "ОтНочнойПерсоны".part(traits::NightPerson.make_bonus().compat())
    }
}

impl Agility {
    // default .sum() formula
}

impl Luck {
    // default .sum() formula
}

// Original
//invar!(HP_PER_STR, 1, "ЗдоровьеЗаСилу");
//invar!(HP_PER_END, 2, "ЗдоровьеЗаВыносливость");

// Roleplay
invar!(HP_PER_STR, 4, "ЗдоровьеЗаСилу");
invar!(HP_PER_END, 8, "ЗдоровьеЗаВыносливость");

impl LifeMax {
    pub fn make_formula(&self) -> impl CrOp {
        self.sum()
            + "ОтСилы".part(Strength.base() * HP_PER_STR)
            + "ОтВыносливости".part(Endurance.base() * HP_PER_END)
    }
}

invar!(APPOINTS_BASE, 100, "БазовыеОД");

impl ActionPointsMax {
    pub fn make_formula(&self) -> impl CrOp {
        self.sum() + APPOINTS_BASE // + Agility.base() / 2
    }
}

invar!(SMALL_FRAME_CW_MALUS_DIV, 4, "ХилоеТелоШтрафМаксВесаОтСилы");
invar!(CW_PER_STR, 10, "МаксВесЗаСилу");
invar!(CW_BASE, 15, "БазовыйМаксВес");

impl WeightMax {
    pub fn make_formula(&self) -> impl CrOp {
        let small_frame = traits::SmallFrame.base() * SMALL_FRAME_CW_MALUS_DIV;
        let from_strength = Strength.base() * (CW_PER_STR - small_frame);
        "ОтМаксВеса".part(max(self.sum(), int(0)))
            + int(1000) * (CW_BASE + "ОтСилы".part(from_strength))
    }
}

invar!(SEQUENCE_PER_PERCEPTION, 2, "ПорядокДействийЗаВосприятие");
impl Sequence {
    pub fn make_formula(&self) -> impl CrOp {
        self.sum() + Perception.calc() * SEQUENCE_PER_PERCEPTION
    }
}

invar!(MELEE_DAMAGE_BASE, 3, "БазовыйРукопашныйУрон");
invar!(MELEE_DAMAGE_PER_STR, 2, "РукопашныйУронЗаСилу");
impl MeleeDamage {
    pub fn make_formula(&self) -> impl CrOp {
        self.sum() + MELEE_DAMAGE_BASE + "ОтСилы".part(Strength.base() * MELEE_DAMAGE_PER_STR)
    }
}

impl HealingRate {
    pub fn make_formula(&self) -> impl CrOp {
        let endurance = "Выносливость".part(Endurance.calc());
        let from_endurance = "ОтВыносливости".part(max(int(1), endurance / int(3)));
        self.sum() + from_endurance
    }
}

impl CriticalChance {
    pub fn make_formula(&self) -> impl CrOp {
        self.sum() + "Удача".part(Luck.calc())
    }
}
impl CriticalMax {
    // default .sum() formula
}
