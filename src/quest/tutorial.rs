use super::{Event, Quest};
use crate::item::key::Key;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WinBattle;

#[typetag::serde]
impl Quest for WinBattle {
    fn description(&self) -> String {
        "win a battle".to_string()
    }

    fn handle(&mut self, event: &Event) -> bool {
        matches!(event, Event::BattleWon { .. })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuySword;

#[typetag::serde]
impl Quest for BuySword {
    fn description(&self) -> String {
        "buy a sword".to_string()
    }

    fn handle(&mut self, event: &Event) -> bool {
        if let Event::ItemBought { item: Key::Sword } = event {
            return true;
        }
        false
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsePotion;

#[typetag::serde]
impl Quest for UsePotion {
    fn description(&self) -> String {
        "use a potion".to_string()
    }

    fn handle(&mut self, event: &Event) -> bool {
        if let Event::ItemUsed { item: Key::Potion } = event {
            return true;
        }
        false
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FindChest;

#[typetag::serde]
impl Quest for FindChest {
    fn description(&self) -> String {
        "find a chest".to_string()
    }

    fn handle(&mut self, event: &Event) -> bool {
        matches!(event, Event::ChestFound)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VisitTomb;

#[typetag::serde]
impl Quest for VisitTomb {
    fn description(&self) -> String {
        "visit the tomb of a fallen hero".to_string()
    }

    fn handle(&mut self, event: &Event) -> bool {
        matches!(event, Event::TombtsoneFound)
    }
}
