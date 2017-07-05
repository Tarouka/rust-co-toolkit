
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonsterEntry {
	name: String,
	size_add: u8,
	zoom_pct: u16,
	max_life: u32,
	lvl: u16,
	born_action: u16,
	born_effect: String,
	born_sound: String,
	act_res_ctrl: u8,
	asb: u8,
	adb: u8,
	body_type: u8,
	type_id: String,
	anti_type: u8,
	armet: u8,
	armet_color: u8,
	r_weapon: u8,
	l_weapon: u8,
	l_weapon_color: u8,
	misc: u8,
	mount: u8,
	battle_lvl: u8,
	extra_xp: u8,
	stc_type: u8
}

pub mod parser {
	use ini::Ini;
	use super::MonsterEntry;
	use std::str::FromStr;
	use std::collections::HashMap;
	use datfiles::parser::ParserSerializable;

	pub enum GetKeyError {
		KeyNotFound(String),
		FailedToParse(String)
	}

	fn get_str_key(props: &HashMap<String, String>, key: &str) -> Result<String, GetKeyError> {
		match props.get(key) {
			Some(found_key) => {
				return Result::Ok(found_key.to_owned());
			},

			None => {
				return Result::Err(GetKeyError::KeyNotFound(String::from(key)));
			}
		}
	}

	fn get_parsed_key<T>(props: &HashMap<String, String>, key: &str) -> Result<T, GetKeyError>
		where T: FromStr {
		let found_key = get_str_key(props, key)?;

		match T::from_str(&found_key) {
			Ok(parsed) => {
				return Result::Ok(parsed);
			},

			_ => {
				return Result::Err(GetKeyError::FailedToParse(String::from(key)));
			}
		}
	}

	pub fn get_all_monster_entries(file: Ini) -> Result<Vec<MonsterEntry>, GetKeyError> {
		let mut monsters: Vec<MonsterEntry> = Vec::new();

		for (section, props) in &file {
			let monster_entry = MonsterEntry {
				name:				section.to_owned().unwrap(),
				size_add:			get_parsed_key::<u8>(props, "SizeAdd")?,
				zoom_pct:			get_parsed_key::<u16>(props, "ZoomPercent")?,
				max_life:			get_parsed_key::<u32>(props, "MaxLife")?,
				lvl:				get_parsed_key::<u16>(props, "Level")?,
				born_action:		get_parsed_key::<u16>(props, "BornAction")?,
				act_res_ctrl:		get_parsed_key::<u8>(props, "ActResCtrl")?,
				asb:				get_parsed_key::<u8>(props, "ASB")?,
				adb:				get_parsed_key::<u8>(props, "ADB")?,
				body_type:			get_parsed_key::<u8>(props, "BodyType")?,
				type_id:			get_str_key(props, "TypeID")?,
				anti_type:			get_parsed_key::<u8>(props, "AntiType")?,
				armet:				get_parsed_key::<u8>(props, "Armet")?,
				armet_color:		get_parsed_key::<u8>(props, "ArmetColor").unwrap_or(0),
				r_weapon:			get_parsed_key::<u8>(props, "RWeapon")?,
				l_weapon:			get_parsed_key::<u8>(props, "LWeapon")?,
				l_weapon_color:		get_parsed_key::<u8>(props, "LWeaponColor").unwrap_or(0),
				misc:				get_parsed_key::<u8>(props, "Misc")?,
				mount:				get_parsed_key::<u8>(props, "Mount")?,
				battle_lvl:			get_parsed_key::<u8>(props, "BattleLev")?,
				extra_xp:			get_parsed_key::<u8>(props, "ExtraExp")?,
				stc_type:			get_parsed_key::<u8>(props, "StcType")?,

				born_effect:		get_str_key(props, "BornEffect")?,
				born_sound:			get_str_key(props, "BornSound")?
			};

			monsters.push(monster_entry);
		}

		Result::Ok(monsters)
	}

	pub fn serialize_monster_entries_to_str(entries: &Vec<MonsterEntry>) -> String {
		let mut ret_string: String = String::new();

		for entry in entries {
			ret_string.push_str(&entry.serialize_to_string());
			ret_string.push_str("\n\n");
		}

		ret_string
	}

	impl ParserSerializable for MonsterEntry {
    	fn serialize_to_string(&self) -> String {
    		let mut ret_string: String = String::new();

    		ret_string.push_str("[");
    		ret_string.push_str(&self.name.to_string());
    		ret_string.push_str("]");
    		ret_string.push_str("\nSizeAdd=");
    		ret_string.push_str(&self.size_add.to_string());
    		ret_string.push_str("\nZoomPercent=");
    		ret_string.push_str(&self.zoom_pct.to_string());
    		ret_string.push_str("\nMaxLife=");
    		ret_string.push_str(&self.max_life.to_string());
    		ret_string.push_str("\nLevel=");
    		ret_string.push_str(&self.lvl.to_string());
    		ret_string.push_str("\nBornAction=");
    		ret_string.push_str(&self.born_action.to_string());
    		ret_string.push_str("\nBornEffect=");
    		ret_string.push_str(&self.born_effect);
    		ret_string.push_str("\nBornSound=");
    		ret_string.push_str(&self.born_sound);
    		ret_string.push_str("\nActResCtrl=");
    		ret_string.push_str(&self.act_res_ctrl.to_string());
    		ret_string.push_str("\nASB=");
    		ret_string.push_str(&self.asb.to_string());
    		ret_string.push_str("\nADB=");
    		ret_string.push_str(&self.adb.to_string());
    		ret_string.push_str("\nBodyType=");
    		ret_string.push_str(&self.body_type.to_string());
			ret_string.push_str("\nTypeID=");
    		ret_string.push_str(&self.type_id.to_string());
    		ret_string.push_str("\nAntiType=");
    		ret_string.push_str(&self.anti_type.to_string());
    		ret_string.push_str("\nArmet=");
    		ret_string.push_str(&self.armet.to_string());
    		ret_string.push_str("\nArmetColor=");
    		ret_string.push_str(&self.armet_color.to_string());
    		ret_string.push_str("\nRWeapon=");
    		ret_string.push_str(&self.r_weapon.to_string());
    		ret_string.push_str("\nLWeapon=");
    		ret_string.push_str(&self.l_weapon.to_string());
    		ret_string.push_str("\nLWeaponColor=");
    		ret_string.push_str(&self.l_weapon_color.to_string());
    		ret_string.push_str("\nMisc=");
    		ret_string.push_str(&self.misc.to_string());
    		ret_string.push_str("\nMount=");
    		ret_string.push_str(&self.mount.to_string());
    		ret_string.push_str("\nBattleLev=");
    		ret_string.push_str(&self.battle_lvl.to_string());
    		ret_string.push_str("\nExtraExp=");
    		ret_string.push_str(&self.extra_xp.to_string());
    		ret_string.push_str("\nStcType=");
    		ret_string.push_str(&self.stc_type.to_string());

    		ret_string
    	}
	}

	#[cfg(test)]
	pub mod tests {
		use super::get_all_monster_entries;
		use ini::Ini;
		use datfiles::parser::ParserSerializable;

		const VALID_MONSTER_ENTRY: &str = "
			[SuperMonster]
			SizeAdd=2
			ZoomPercent=90
			MaxLife=1000
			Level=100
			BornAction=315
			BornEffect=MBStandard
			BornSound=none
			ActResCtrl=9
			ASB=5
			ADB=6
			BodyType=0
			TypeID=3029
			AntiType=2
			Armet=0
			ArmetColor=3
			RWeapon=0
			LWeapon=0
			LWeaponColor=3
			Misc=4
			Mount=16
			BattleLev=17
			ExtraExp=100
			StcType=18";

		const MONSTER_ENTRY_UNFORMATTED: &str = "[AMonster]
SizeAdd=2
ZoomPercent=90
MaxLife=1000
Level=100
BornAction=315
BornEffect=MBStandard
BornSound=none
ActResCtrl=9
ASB=5
ADB=6
BodyType=0
TypeID=3029
AntiType=2
Armet=0
ArmetColor=3
RWeapon=0
LWeapon=0
LWeaponColor=3
Misc=4
Mount=16
BattleLev=17
ExtraExp=100
StcType=18";

		macro_rules! assert_monster_entry_val {
			( $expected:expr, $key:ident, $ini_val:expr, $idx:expr ) => ({
				let ini = Ini::load_from_str($ini_val).unwrap();
				if let Result::Ok(monster_entries) = get_all_monster_entries(ini) {
					let first_monster_entry = monster_entries.get($idx).unwrap();

					assert_eq!($expected, first_monster_entry.$key);
				}
			})
		}

		fn test_monster_entry_parse_then_reformat_returns_start_val() {
			let ini = Ini::load_from_str(MONSTER_ENTRY_UNFORMATTED).unwrap();
			if let Result::Ok(monster_entries) = get_all_monster_entries(ini) {
				let first_monster_entry = monster_entries.get(0).unwrap();
				let reformatted_val = first_monster_entry.serialize_to_string();

				assert_eq!(MONSTER_ENTRY_UNFORMATTED, reformatted_val);
			}
			else {
				assert!(false, "Failed to format monster entry");
			}
		}
		
		#[test]
		fn test_valid_monster_entry_parse_size_add() {
			assert_monster_entry_val!(2, size_add, VALID_MONSTER_ENTRY, 0);
		}

		#[test]
		fn test_valid_monster_entry_parse_zoom_pct() {
			assert_monster_entry_val!(90, zoom_pct, VALID_MONSTER_ENTRY, 0);
		}

		#[test]
		fn test_valid_monster_entry_parse_max_life() {
			assert_monster_entry_val!(1000, max_life, VALID_MONSTER_ENTRY, 0);
		}

		#[test]
		fn test_valid_monster_entry_parse_lvl() {
			assert_monster_entry_val!(100, lvl, VALID_MONSTER_ENTRY, 0);
		}

		#[test]
		fn test_valid_monster_entry_parse_born_action() {
			assert_monster_entry_val!(315, born_action, VALID_MONSTER_ENTRY, 0);
		}

		#[test]
		fn test_valid_monster_entry_parse_act_res_ctrl() {
			assert_monster_entry_val!(9, act_res_ctrl, VALID_MONSTER_ENTRY, 0);
		}

		#[test]
		fn test_valid_monster_entry_parse_asb() {
			assert_monster_entry_val!(5, asb, VALID_MONSTER_ENTRY, 0);
		}

		#[test]
		fn test_valid_monster_entry_parse_adb() {
			assert_monster_entry_val!(6, adb, VALID_MONSTER_ENTRY, 0);
		}

		#[test]
		fn test_valid_monster_entry_parse_body_type() {
			assert_monster_entry_val!(0, body_type, VALID_MONSTER_ENTRY, 0);
		}

		#[test]
		fn test_valid_monster_entry_parse_type_id() {
			assert_monster_entry_val!("3029", type_id, VALID_MONSTER_ENTRY, 0);
		}

		#[test]
		fn test_valid_monster_entry_parse_anti_type() {
			assert_monster_entry_val!(2, anti_type, VALID_MONSTER_ENTRY, 0);
		}

		#[test]
		fn test_valid_monster_entry_parse_armet() {
			assert_monster_entry_val!(0, armet, VALID_MONSTER_ENTRY, 0);
		}

		#[test]
		fn test_valid_monster_entry_parse_armet_color() {
			assert_monster_entry_val!(3, armet_color, VALID_MONSTER_ENTRY, 0);
		}

		#[test]
		fn test_valid_monster_entry_parse_r_weapon() {
			assert_monster_entry_val!(0, r_weapon, VALID_MONSTER_ENTRY, 0);
		}

		#[test]
		fn test_valid_monster_entry_parse_l_weapon() {
			assert_monster_entry_val!(0, l_weapon, VALID_MONSTER_ENTRY, 0);
		}

		#[test]
		fn test_valid_monster_entry_parse_l_weapon_color() {
			assert_monster_entry_val!(3, l_weapon_color, VALID_MONSTER_ENTRY, 0);
		}

		#[test]
		fn test_valid_monster_entry_parse_misc() {
			assert_monster_entry_val!(4, misc, VALID_MONSTER_ENTRY, 0);
		}

		#[test]
		fn test_valid_monster_entry_parse_mount() {
			assert_monster_entry_val!(16, mount, VALID_MONSTER_ENTRY, 0);
		}

		#[test]
		fn test_valid_monster_entry_parse_battle_lvl() {
			assert_monster_entry_val!(17, battle_lvl, VALID_MONSTER_ENTRY, 0);
		}

		#[test]
		fn test_valid_monster_entry_parse_extra_xp() {
			assert_monster_entry_val!(100, extra_xp, VALID_MONSTER_ENTRY, 0);
		}

		#[test]
		fn test_valid_monster_entry_parse_stc_type() {
			assert_monster_entry_val!(18, stc_type, VALID_MONSTER_ENTRY, 0);
		}
	}
}

pub mod encoder {
	use super::MonsterEntry;
	use serde_json;

	pub fn decode_monster_to_json(entries: &Vec<MonsterEntry>) -> Vec<u8> {
		let json_monster = serde_json::to_string_pretty(&entries).unwrap();
		println!("{}", entries.len());

		json_monster.into_bytes()
	}

	pub fn encode_monster_from_json(monster: Vec<u8>) -> Vec<MonsterEntry> {
		let json_monster = String::from_utf8(monster).unwrap();
		let monster_entries: Vec<MonsterEntry> = serde_json::from_str(&json_monster).unwrap();

		monster_entries
	}
}