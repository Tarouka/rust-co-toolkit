/**
ItemTypeStruct:
	<ItemID>
	<ItemName, Max=16>
	<ProfessionRequired>
	<ProfiencyLevelRequired> 
	<LevelRequired>
	<SexRequired, 1=?, 2=?> 
	<StrRequired>
	<AgiRequired>
	<VitRequired>
	<SpiRequired>
	<Bitmask...001|0<IsSellDisabled>1|0<DoesItemNeverDropOnDeath>1|0<ShouldShowImportantSellHint>1|0<ShouldShowImportantDropHint>1|0<IsUnstoreable>1|0<IsUntradeable>>
	<Weight> 
	<BuyPrice>
	<ActionID?> 
	<MaxPhysAtk>
	<MinPhysAtk>
	<PhysDef> 
	<Accuracy>
	<Dodge>
	<HPRestored>
	<MPRestored>
	<AmountDurability1_99>
	<AmountLimitDurability2_99> 
	<Status?>
	<Gem1?> 
	<Gem2?> 
	<Magic1?> 
	<Magic2?>
	<Magic3?> 
	<MagicAtk>
	<MagicDef%>
	<AttackRange>
	<AttackSpeed>
	<FrayMode?> 
	<RepairMode?>
	<TypeMask?>
	<BuyCPsPrice>
	<Type>
	<Description|None, Max=128>
	<Unknown?>
**/


pub struct ItemType {
	pub header: ItemTypeHeader,

	pub entries: Vec<ItemTypeEntry>
}

/*
	Amount=<Amount>
*/
pub struct ItemTypeHeader {
	pub amount: u32
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemTypeEntry {
	pub id: u32,
	pub name: String,
	pub profession_req: u8,
	pub proficiency_req: u8,
	pub lvl_req: u8,
	pub sex_req: u8,
	pub str_req: u16,
	pub agi_req: u16,
	pub vit_req: u16,
	pub spi_req: u16,

	pub is_sell_disabled: bool,
	pub does_item_never_drop_on_death: bool,
	pub should_show_important_sell_hint: bool,
	pub should_show_important_drop_hint: bool,
	pub is_unstoreable: bool,
	pub is_untradeable: bool,

	pub weight: u16,
	pub buy_price: u32,
	pub action_id: u32,
	pub max_phys_atk: u16,
	pub min_phys_atk: u16,
	pub phys_def: u16,
	pub accuracy: u8,
	pub dodge: u8,
	pub hp_restored: u16,
	pub mp_restored: u16,
	pub amount: u16,
	pub amount_limit: u16,
	pub status: u16,
	pub gem1: u8,
	pub gem2: u8,
	pub magic1: u8,
	pub magic2: u8,
	pub magic3: u8,
	pub magic_atk: u16,
	pub magic_def: u16,
	pub atk_range: u8,
	pub atk_speed: u16,
	pub fray_mode: u32,
	pub repair_mode: u32,
	pub type_mask: u32,
	pub buy_cps_price: u16,
	pub type_name: String,
	pub description: String,
	pub unknown_1: u8
}

pub mod parser {
	use super::{ItemTypeEntry, ItemTypeHeader, ItemType};

	use datfiles::parser::*;

	use nom::IResult;
	use nom::IResult::Done;
	use std::num::ParseIntError;

	use regex::*;

    impl ParserSerializable for ItemTypeEntry {
    	fn serialize_to_string(&self) -> String {
    		let mut ret_string: String = String::new();

    		serializer_append_field!(self, ret_string, id);
    		serializer_append_field!(self, ret_string, name);
    		serializer_append_field!(self, ret_string, profession_req);
    		serializer_append_field!(self, ret_string, proficiency_req);
    		serializer_append_field!(self, ret_string, lvl_req);
    		serializer_append_field!(self, ret_string, sex_req);
    		serializer_append_field!(self, ret_string, str_req);
    		serializer_append_field!(self, ret_string, agi_req);
    		serializer_append_field!(self, ret_string, vit_req);
    		serializer_append_field!(self, ret_string, spi_req);

    		let calculated_action_mask = 
    			if self.is_sell_disabled { 0x20 } else { 0x00 } |
    			if self.does_item_never_drop_on_death { 0x10 } else { 0x00 } |
    			if self.should_show_important_sell_hint { 0x08 } else { 0x00 } |
    			if self.should_show_important_drop_hint { 0x04 } else { 0x00 } |
    			if self.is_unstoreable { 0x02 } else { 0x00 } |
    			if self.is_untradeable { 0x01 } else { 0x00 };

    		ret_string.push_str(&calculated_action_mask.to_string());
    		ret_string.push_str(" ");

			serializer_append_field!(self, ret_string, weight);
			serializer_append_field!(self, ret_string, buy_price);
			serializer_append_field!(self, ret_string, action_id);
			serializer_append_field!(self, ret_string, max_phys_atk);
			serializer_append_field!(self, ret_string, min_phys_atk);
			serializer_append_field!(self, ret_string, phys_def);
			serializer_append_field!(self, ret_string, accuracy);
			serializer_append_field!(self, ret_string, dodge);
			serializer_append_field!(self, ret_string, hp_restored);
			serializer_append_field!(self, ret_string, mp_restored);
			serializer_append_field!(self, ret_string, amount);
			serializer_append_field!(self, ret_string, amount_limit);
			serializer_append_field!(self, ret_string, status);
			serializer_append_field!(self, ret_string, gem1);
			serializer_append_field!(self, ret_string, gem2);
			serializer_append_field!(self, ret_string, magic1);
			serializer_append_field!(self, ret_string, magic2);
			serializer_append_field!(self, ret_string, magic3);
			serializer_append_field!(self, ret_string, magic_atk);
			serializer_append_field!(self, ret_string, magic_def);
			serializer_append_field!(self, ret_string, atk_range);
			serializer_append_field!(self, ret_string, atk_speed);
			serializer_append_field!(self, ret_string, fray_mode);
			serializer_append_field!(self, ret_string, repair_mode);
			serializer_append_field!(self, ret_string, type_mask);
			serializer_append_field!(self, ret_string, buy_cps_price);
			serializer_append_field!(self, ret_string, type_name);
			
			ret_string.push_str(&append_tildes_to(self.description.to_string()));
			ret_string.push_str(" ");

			serializer_append_field_last!(self, ret_string, unknown_1);

    		ret_string
    	}
    }

    impl ParserSerializable for ItemTypeHeader {
    	fn serialize_to_string(&self) -> String {
    		let mut ret_string: String = String::new();

    		ret_string.push_str("Amount=");
    		ret_string.push_str(&self.amount.to_string());

    		ret_string
    	}
    }

    impl ParserSerializable for ItemType {
    	fn serialize_to_string(&self) -> String {
    		let mut ret_string: String = String::new();

    		ret_string.push_str(&self.header.serialize_to_string());

    		for entry in &self.entries {
    			ret_string.push_str("\r\n");
    			ret_string.push_str(&entry.serialize_to_string());
    		}

    		ret_string
    	}
    }

    pub fn item_type_header<'a>(val: &'a [u8]) -> Result<ItemTypeHeader, ParseIntError> {
		let val_as_str = String::from_utf8_lossy(val).into_owned();
    	let reg_parser_items = Regex::new(r"Amount=(\d+)").unwrap();
    	let captures = reg_parser_items.captures(&val_as_str).unwrap();
    	let amount_cap = String::from(&captures[1]);

    	let entry = ItemTypeHeader {
    		amount:						amount_cap.parse::<u32>()?
    	};

    	Result::Ok(entry)
    }

	pub fn item_type_entry<'a>(val: &'a [u8]) -> Result<ItemTypeEntry, usize> {
		let val_as_str = String::from_utf8_lossy(val).into_owned();
		let reg_parser_items = Regex::new(r"(\S+)\s*").unwrap();
		let mut packed_strs: Vec<String> = Vec::new();

		for cap in reg_parser_items.captures_iter(&val_as_str) {
			packed_strs.push(String::from(&cap[1]));
		}

		let action_mask = parse_match_to_u8(&packed_strs, 10)?;
		let entry = ItemTypeEntry {
			id:										parse_match_to_u32(&packed_strs, 0)?,
			name:									packed_strs[1].clone(),
			profession_req:							parse_match_to_u8(&packed_strs, 2)?,
			proficiency_req:						parse_match_to_u8(&packed_strs, 3)?,
			lvl_req:								parse_match_to_u8(&packed_strs, 4)?,
			sex_req:								parse_match_to_u8(&packed_strs, 5)?,
			str_req:								parse_match_to_u16(&packed_strs, 6)?,
			agi_req:								parse_match_to_u16(&packed_strs, 7)?,
			vit_req:								parse_match_to_u16(&packed_strs, 8)?,
			spi_req:								parse_match_to_u16(&packed_strs, 9)?,

			is_sell_disabled:						(action_mask & 0x20) > 0,
			does_item_never_drop_on_death:			(action_mask & 0x10) > 0,
			should_show_important_sell_hint:		(action_mask & 0x08) > 0,
			should_show_important_drop_hint:		(action_mask & 0x04) > 0,
			is_unstoreable:							(action_mask & 0x02) > 0,
			is_untradeable:							(action_mask & 0x01) > 0,

			weight:									parse_match_to_u16(&packed_strs, 11)?,
			buy_price:								parse_match_to_u32(&packed_strs, 12)?,
			action_id:								parse_match_to_u32(&packed_strs, 13)?,
			max_phys_atk:							parse_match_to_u16(&packed_strs, 14)?,
			min_phys_atk:							parse_match_to_u16(&packed_strs, 15)?,
			phys_def:								parse_match_to_u16(&packed_strs, 16)?,
			accuracy:								parse_match_to_u8(&packed_strs, 17)?,
			dodge:									parse_match_to_u8(&packed_strs, 18)?,
			hp_restored:							parse_match_to_u16(&packed_strs, 19)?,
			mp_restored:							parse_match_to_u16(&packed_strs, 20)?,
			amount:									parse_match_to_u16(&packed_strs, 21)?,
			amount_limit:							parse_match_to_u16(&packed_strs, 22)?,
			status:									parse_match_to_u16(&packed_strs, 23)?,
			gem1:									parse_match_to_u8(&packed_strs, 24)?,
			gem2:									parse_match_to_u8(&packed_strs, 25)?,
			magic1:									parse_match_to_u8(&packed_strs, 26)?,
			magic2:									parse_match_to_u8(&packed_strs, 27)?,
			magic3:									parse_match_to_u8(&packed_strs, 28)?,
			magic_atk:								parse_match_to_u16(&packed_strs, 29)?,
			magic_def:								parse_match_to_u16(&packed_strs, 30)?,
			atk_range:								parse_match_to_u8(&packed_strs, 31)?,
			atk_speed:								parse_match_to_u16(&packed_strs, 32)?,
			fray_mode:								parse_match_to_u32(&packed_strs, 33)?,
			repair_mode:							parse_match_to_u32(&packed_strs, 34)?,
			type_mask:								parse_match_to_u32(&packed_strs, 35)?,
			buy_cps_price:							parse_match_to_u16(&packed_strs, 36)?,
			type_name:								packed_strs[37].clone(),
			description:							remove_tildes_from(packed_strs[38].clone()),
			unknown_1:								parse_match_to_u8(&packed_strs, 39)?
		};

		Result::Ok(entry)
	}


	pub fn parse_item_type<'a>(bytes: &'a [u8]) -> Option<ItemType> {
		let mut entries: Vec<ItemTypeEntry> = Vec::new();
		let bytes_split = split_bytes_by_lines(Vec::from(bytes));
		let header = ItemTypeHeader { amount: 30 };

		match bytes_split.get(0) {
			Some(line) => {
				match item_type_header(&line) {
					Result::Ok(item_type_header_parsed) => {
						let header = item_type_header_parsed;
					},

					Result::Err(err) => {
						println!("Error while parsing header: {:?}", err);
						return None;
					}
				}
			},

			None => {
				return None;
			}
		}

		for idx in 1..bytes_split.len() {
			let line = bytes_split.get(idx).unwrap();

			match item_type_entry(&line) {
				Result::Ok(item_type_entry_parsed) => {
					entries.push(item_type_entry_parsed);
				},

				Result::Err(idx) => {
					println!("Error at field index {}", idx);
				}
			}
		}

		Some(ItemType { header: header, entries: entries })
	}

	#[cfg(test)]
	mod tests {
		use super::{item_type_header, item_type_entry};
		use datfiles::parser::ParserSerializable;

		macro_rules! assert_header_amount_eq {
			( $str_to_parse:expr, $expected:expr ) => ({
	    		let header_bytes = String::from($str_to_parse).into_bytes();
				let parsed_header = item_type_header(&header_bytes).unwrap();

				assert_eq!($expected, parsed_header.amount);
			})
		}

		macro_rules! assert_item_type_field_eq {
			( $str_to_parse:expr, $field:ident, $expected:expr ) => ({
	    		let item_type_bytes = String::from($str_to_parse).into_bytes();
				let parsed_item_entry = item_type_entry(&item_type_bytes).unwrap();

				assert_eq!($expected, parsed_item_entry.$field);
			})
		}

		const SAMPLE_ITEM_ENTRY_1: &str = "100000 FirstItem 11 0 15 0 40 0 500 120 63 92 11 1 190 8000 140 0 80 4000 0 1 11 143 0 2 33 0 5 5000 0 1 1000 0 67 54 0 Helmet None 5";
		const SAMPLE_ITEM_ENTRY_2: &str = "150000 SecondItem 41 12 70 1 240 120 0 600 21 175 90000 2 2000 19000 0 16 0 150 3000 1099 1199 100 1 22 0 4 55 0 1300 25 15000 255 11 0 128 Backsword Description~without~tildes 6";
		const SAMPLE_ITEM_ENTRY_3: &str = "2000000 ThirdItem 145 20 130 2 0 300 90 0 10 2000 1824 3 0 0 17000 20 13 0 10000 3599 3699 78 11 0 3 44 0 10000 60 0 500 8 0 180 2000 Shootingbow None 7";

		const SAMPLE_HEADER_ENTRY_1: &str = "Amount=1239";
		const SAMPLE_HEADER_ENTRY_2: &str = "Amount=0";
		const SAMPLE_HEADER_ENTRY_3: &str = "Amount=99999";

		#[test]
		fn parse_header_will_return_right_val() {
			assert_header_amount_eq!(SAMPLE_HEADER_ENTRY_1, 1239);
			assert_header_amount_eq!(SAMPLE_HEADER_ENTRY_2, 0);
			assert_header_amount_eq!(SAMPLE_HEADER_ENTRY_3, 99999);
		}

		#[test]
		fn parse_header_serialize_will_return_initial_header_1() {
			let header_bytes = String::from(SAMPLE_HEADER_ENTRY_1).into_bytes();
			let parsed_header = item_type_header(&header_bytes).unwrap();
			let reserialized_line = parsed_header.serialize_to_string();

			assert_eq!(SAMPLE_HEADER_ENTRY_1, reserialized_line);
		}

		#[test]
		fn parse_header_serialize_will_return_initial_header_2() {
			let header_bytes = String::from(SAMPLE_HEADER_ENTRY_2).into_bytes();
			let parsed_header = item_type_header(&header_bytes).unwrap();
			let reserialized_line = parsed_header.serialize_to_string();

			assert_eq!(SAMPLE_HEADER_ENTRY_2, reserialized_line);
		}

		#[test]
		fn parse_header_serialize_will_return_initial_header_3() {
			let header_bytes = String::from(SAMPLE_HEADER_ENTRY_3).into_bytes();
			let parsed_header = item_type_header(&header_bytes).unwrap();
			let reserialized_line = parsed_header.serialize_to_string();

			assert_eq!(SAMPLE_HEADER_ENTRY_3, reserialized_line);
		}

		#[test]
		fn parse_item_entry_will_return_right_id() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, id, 100000);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, id, 150000);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, id, 2000000);
		}

		#[test]
		fn parse_item_entry_will_return_right_name() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, name, "FirstItem");
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, name, "SecondItem");
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, name, "ThirdItem");
		}

		#[test]
		fn parse_item_entry_will_return_right_profession_req() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, profession_req, 11);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, profession_req, 41);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, profession_req, 145);
		}

		#[test]
		fn parse_item_entry_will_return_right_proficiency_req() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, proficiency_req, 0);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, proficiency_req, 12);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, proficiency_req, 20);
		}

		#[test]
		fn parse_item_entry_will_return_right_lvl_req() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, lvl_req, 15);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, lvl_req, 70);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, lvl_req, 130);
		}

		#[test]
		fn parse_item_entry_will_return_right_sex_req() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, sex_req, 0);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, sex_req, 1);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, sex_req, 2);
		}

		#[test]
		fn parse_item_entry_will_return_right_str_req() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, str_req, 40);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, str_req, 240);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, str_req, 0);
		}

		#[test]
		fn parse_item_entry_will_return_right_agi_req() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, agi_req, 0);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, agi_req, 120);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, agi_req, 300);
		}

		#[test]
		fn parse_item_entry_will_return_right_vit_req() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, vit_req, 500);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, vit_req, 0);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, vit_req, 90);
		}

		#[test]
		fn parse_item_entry_will_return_right_spi_req() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, spi_req, 120);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, spi_req, 600);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, spi_req, 0);
		}

		#[test]
		fn parse_item_entry_will_return_right_is_sell_disabled() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, is_sell_disabled, true);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, is_sell_disabled, false);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, is_sell_disabled, false);
		}

		#[test]
		fn parse_item_entry_will_return_right_does_item_never_drop_on_death() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, does_item_never_drop_on_death, true);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, does_item_never_drop_on_death, true); 
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, does_item_never_drop_on_death, false);
		}

		#[test]
		fn parse_item_entry_will_return_right_should_show_important_sell_hint() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, should_show_important_sell_hint, true);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, should_show_important_sell_hint, false);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, should_show_important_sell_hint, true);
		}

		#[test]
		fn parse_item_entry_will_return_right_should_show_important_drop_hint() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, should_show_important_drop_hint, true); 
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, should_show_important_drop_hint, true);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, should_show_important_drop_hint, false);
		}

		#[test]
		fn parse_item_entry_will_return_right_is_unstoreable() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, is_unstoreable, true);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, is_unstoreable, false);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, is_unstoreable, true);
		}

		#[test]
		fn parse_item_entry_will_return_right_is_untradeable() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, is_untradeable, true);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, is_untradeable, true);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, is_untradeable, false);
		}

		#[test]
		fn parse_item_entry_will_return_right_weight() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, weight, 92);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, weight, 175);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, weight, 2000);
		}

		#[test]
		fn parse_item_entry_will_return_right_buy_price() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, buy_price, 11);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, buy_price, 90000);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, buy_price, 1824);
		}

		#[test]
		fn parse_item_entry_will_return_right_action_id() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, action_id, 1);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, action_id, 2);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, action_id, 3);
		}

		#[test]
		fn parse_item_entry_will_return_right_max_phys_atk() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, max_phys_atk, 190);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, max_phys_atk, 2000);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, max_phys_atk, 0);
		}

		#[test]
		fn parse_item_entry_will_return_right_min_phys_atk() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, min_phys_atk, 8000);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, min_phys_atk, 19000);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, min_phys_atk, 0);
		}

		#[test]
		fn parse_item_entry_will_return_right_phys_def() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, phys_def, 140);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, phys_def, 0);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, phys_def, 17000);
		}

		#[test]
		fn parse_item_entry_will_return_right_accuracy() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, accuracy, 0);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, accuracy, 16);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, accuracy, 20);
		}

		#[test]
		fn parse_item_entry_will_return_right_dodge() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, dodge, 80);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, dodge, 0);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, dodge, 13);
		}

		#[test]
		fn parse_item_entry_will_return_right_hp_restored() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, hp_restored, 4000);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, hp_restored, 150);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, hp_restored, 0);
		}

		#[test]
		fn parse_item_entry_will_return_right_mp_restored() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, mp_restored, 0);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, mp_restored, 3000);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, mp_restored, 10000);
		}

		#[test]
		fn parse_item_entry_will_return_right_amount() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, amount, 1);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, amount, 1099);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, amount, 3599);
		}

		#[test]
		fn parse_item_entry_will_return_right_amount_limit() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, amount_limit, 11);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, amount_limit, 1199);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, amount_limit, 3699);
		}

		#[test]
		fn parse_item_entry_will_return_right_status() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, status, 143);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, status, 100);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, status, 78);
		}

		#[test]
		fn parse_item_entry_will_return_right_gem1() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, gem1, 0);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, gem1, 1);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, gem1, 11);
		}

		#[test]
		fn parse_item_entry_will_return_right_gem2() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, gem2, 2);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, gem2, 22);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, gem2, 0);
		}

		#[test]
		fn parse_item_entry_will_return_right_magic1() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, magic1, 33);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, magic1, 0);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, magic1, 3);
		}

		#[test]
		fn parse_item_entry_will_return_right_magic2() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, magic2, 0);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, magic2, 4);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, magic2, 44);
		}

		#[test]
		fn parse_item_entry_will_return_right_magic3() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, magic3, 5);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, magic3, 55);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, magic3, 0);
		}

		#[test]
		fn parse_item_entry_will_return_right_magic_atk() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, magic_atk, 5000);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, magic_atk, 0);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, magic_atk, 10000);
		}

		#[test]
		fn parse_item_entry_will_return_right_magic_def() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, magic_def, 0);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, magic_def, 1300);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, magic_def, 60);
		}

		#[test]
		fn parse_item_entry_will_return_right_atk_range() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, atk_range, 1);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, atk_range, 25);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, atk_range, 0);
		}

		#[test]
		fn parse_item_entry_will_return_right_atk_speed() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, atk_speed, 1000);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, atk_speed, 15000);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, atk_speed, 500);
		}

		#[test]
		fn parse_item_entry_will_return_right_fray_mode() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, fray_mode, 0);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, fray_mode, 255);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, fray_mode, 8);
		}

		#[test]
		fn parse_item_entry_will_return_right_repair_mode() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, repair_mode, 67);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, repair_mode, 11);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, repair_mode, 0);
		}

		#[test]
		fn parse_item_entry_will_return_right_type_mask() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, type_mask, 54);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, type_mask, 0);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, type_mask, 180);
		}

		#[test]
		fn parse_item_entry_will_return_right_buy_cps_price() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, buy_cps_price, 0);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, buy_cps_price, 128);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, buy_cps_price, 2000);
		}

		#[test]
		fn parse_item_entry_will_return_right_type_name() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, type_name, "Helmet");
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, type_name, "Backsword");
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, type_name, "Shootingbow");
		}

		#[test]
		fn parse_item_entry_will_return_right_description() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, description, "None");
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, description, "Description without tildes");
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, description, "None");
		}

		#[test]
		fn parse_item_entry_will_return_right_unknown_1() {
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_1, unknown_1, 5);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_2, unknown_1, 6);
			assert_item_type_field_eq!(SAMPLE_ITEM_ENTRY_3, unknown_1, 7);
		}

		#[test]
		fn parse_item_entry_serialize_will_return_initial_line_1() {
			let item_type_bytes = String::from(SAMPLE_ITEM_ENTRY_1).into_bytes();
			let parsed_item_entry = item_type_entry(&item_type_bytes).unwrap();
			let reserialized_line = parsed_item_entry.serialize_to_string();

			assert_eq!(SAMPLE_ITEM_ENTRY_1, reserialized_line);
		}

		#[test]
		fn parse_item_entry_serialize_will_return_initial_line_2() {
			let item_type_bytes = String::from(SAMPLE_ITEM_ENTRY_2).into_bytes();
			let parsed_item_entry = item_type_entry(&item_type_bytes).unwrap();
			let reserialized_line = parsed_item_entry.serialize_to_string();

			assert_eq!(SAMPLE_ITEM_ENTRY_2, reserialized_line);
		}

		#[test]
		fn parse_item_entry_serialize_will_return_initial_line_3() {
			let item_type_bytes = String::from(SAMPLE_ITEM_ENTRY_3).into_bytes();
			let parsed_item_entry = item_type_entry(&item_type_bytes).unwrap();
			let reserialized_line = parsed_item_entry.serialize_to_string();

			assert_eq!(SAMPLE_ITEM_ENTRY_3, reserialized_line);
		}
	}
}

pub mod encoder {
	use super::{ItemType, ItemTypeEntry, ItemTypeHeader};
	use serde_json;

	pub fn decode_item_type_to_json(item_type: &ItemType) -> Vec<u8> {
		let json_item_type = serde_json::to_string_pretty(&item_type.entries).unwrap();
		println!("{}", item_type.entries.len());

		json_item_type.into_bytes()
	}

	pub fn encode_item_type_from_json(item_type: Vec<u8>) -> ItemType {
		let json_item_type = String::from_utf8(item_type).unwrap();
		let item_type_entries: Vec<ItemTypeEntry> = serde_json::from_str(&json_item_type).unwrap();

		ItemType { header: ItemTypeHeader { amount: item_type_entries.len() as u32 }, entries: item_type_entries }
	}
}