use permute_mmo_rs::util::SpawnInfo;
use serde::Deserialize;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Deserialize, Clone, Default, Debug)]
pub struct Spawner {
    #[serde(rename = "Seed")]
    pub seed: String,
    #[serde(rename = "Species")]
    pub species: u16,
    #[serde(rename = "BaseCount")]
    pub base_count: usize,
    #[serde(rename = "BaseTable")]
    pub base_table: String,
    #[serde(rename = "BonusCount")]
    pub bonus_count: usize,
    #[serde(rename = "BonusTable")]
    pub bonus_table: String,
}

impl From<Spawner> for Rc<RefCell<SpawnInfo>> {
    fn from(spawner: Spawner) -> Self {
        let mut table = if spawner.base_table.starts_with("0x") {
            u64::from_str_radix(&spawner.base_table[2..], 16).unwrap_or_default()
        } else {
            u64::from_str_radix(&spawner.base_table, 16).unwrap_or_default()
        };
        let bonus = if spawner.bonus_table.starts_with("0x") {
            u64::from_str_radix(&spawner.bonus_table[2..], 16).unwrap_or_default()
        } else {
            u64::from_str_radix(&spawner.bonus_table, 16).unwrap_or_default()
        };
        let is_outbreak = !(table != 0 && table != 0xCBF29CE484222645)
            && !(bonus != 0 && bonus != 0xCBF29CE484222645);
        if is_outbreak {
            if table < 1000 {
                table = spawner.species as u64;
            }
            SpawnInfo::get_mo(table, spawner.base_count)
        } else {
            SpawnInfo::get_mmo(table, spawner.base_count, bonus, spawner.bonus_count)
        }
    }
}
