/*
MagicTypeStruct:
	<SkillID>
	<ActionSort>
	<SkillName[16]>
	<IsOffensiveOnUse>
	<IsGroundTargeted>
	<IsMultiTarget>
	<TargetFlag:000Body|Passive|Terrain|None|Self>
	<SkillLevel>
	<MPCost>
	<Power>
	<IntoneEffectDuration>
	<Accuracy>
	<Time?>
	<Range?>
	<MaxDistance>
	<Status?>
	<JobRequired>
	<XPRequired>
	<LevelRequired>
	<SkillType:0=Magic,1=XPSkill,2=Martial>
	<WeaponRequired: 3 first digits of ItemID>
	<ActiveTime?>
	<IsAutoActive:4=WeaponPassiveSkill.0=None>
	<FloorAttribute?>
	<IsAutoLearned>
	<AutoLearnLevel>
	<DropWeapon?>
	<StaminaCost>
	<HitsWithWeapon>
	<UsesItem?>
	<NextSkillIDAutoCast>
	<UseDelay>
	<UseItemNum?>
	<SenderAction?>
	<ShortDescription[64]>
	<Description[256]>
	<IntoneEffect[64]>
	<IntoneSFX[260]>
	<SenderEffect[64]>
	<SenderSFX[260]>
	<TargetDelay?>
	<TargetEffect[64]>
	<TargetSFX[260>
	<GroundEffect[64]>
	<TraceEffect[64]>
	<ScreenRepresent?>
	<IsUsableInMarket>
	<IsStaggering>
*/

pub struct MagicType {
	pub header: MagicTypeHeader,

	pub entries: Vec<MagicTypeEntry>
}

pub struct MagicTypeHeader {
	pub amount: u32
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MagicTypeEntry {
	pub id: u32,
	pub action_sort: u8,
	pub skill_name: String,
	pub is_offensive_on_use: bool,
	pub is_ground_targeted: bool,
	pub is_multi_target: bool,

	pub is_body_target: bool,
	pub is_passive_target: bool,
	pub is_terrain_target: bool,
	pub is_none_target: bool,
	pub is_self_target: bool,

	pub skill_lvl: u8,
	pub mp_cost: u16,
	pub power: i32,
	pub intone_effect_duration: u32,
	pub accuracy: u8,
	pub time: u32,
	pub range: u32,
	pub max_distance: u8,
	pub status: u64,
	pub job_required: u32,
	pub xp_required: u64,
	pub lvl_required: u8,
	pub skill_type: u8,
	pub weapon_required: u16,
	pub active_time: u32,
	pub auto_active: u16,
	pub floor_attribute: u32,
	pub is_auto_learned: bool,
	pub auto_learn_lvl: u8,
	pub drop_weapon: u32,
	pub stamina_cost: u8,
	pub hits_with_weapon: u8,
	pub uses_item: u8,
	pub next_skill_id_auto_cast: u32,
	pub use_delay: u32,
	pub use_item_num: u8,
	pub sender_action: u32,

	pub short_desc: String,
	pub desc: String,
	pub intone_effect: String,
	pub intone_sfx: String,
	pub sender_effect: String,
	pub sender_sfx: String,

	pub target_delay: u32,
	pub target_effect: String,
	pub target_sfx: String,

	pub ground_effect: String,
	pub trace_effect: String,

	pub screen_represent: bool,
	pub is_usable_in_market: bool,
	pub is_staggering: bool
}

pub mod parser {
	use super::{MagicTypeHeader, MagicTypeEntry, MagicType};
	use datfiles::parser;
	use datfiles::parser::*;

	use nom::*;

	impl ParserSerializable for MagicTypeEntry {
    	fn serialize_to_string(&self) -> String {
    		let mut ret_string: String = String::new();

    		serializer_append_field!(self, ret_string, id);
			serializer_append_field!(self, ret_string, action_sort);
			serializer_append_field!(self, ret_string, skill_name);
			serializer_append_field_as_bool!(self, ret_string, is_offensive_on_use);
			serializer_append_field_as_bool!(self, ret_string, is_ground_targeted);
			serializer_append_field_as_bool!(self, ret_string, is_multi_target);

			let calculated_target_flag = 
    			if self.is_body_target { 0x10 } else { 0x00 } |
    			if self.is_passive_target { 0x08 } else { 0x00 } |
    			if self.is_terrain_target { 0x04 } else { 0x00 } |
    			if self.is_none_target { 0x02 } else { 0x00 } |
    			if self.is_self_target { 0x01 } else { 0x00 };

    		ret_string.push_str(&calculated_target_flag.to_string());
    		ret_string.push_str(" ");

			serializer_append_field!(self, ret_string, skill_lvl);
			serializer_append_field!(self, ret_string, mp_cost);
			serializer_append_field!(self, ret_string, power);
			serializer_append_field!(self, ret_string, intone_effect_duration);
			serializer_append_field!(self, ret_string, accuracy);
			serializer_append_field!(self, ret_string, time);
			serializer_append_field!(self, ret_string, range);
			serializer_append_field!(self, ret_string, max_distance);
			serializer_append_field!(self, ret_string, status);
			serializer_append_field!(self, ret_string, job_required);
			serializer_append_field!(self, ret_string, xp_required);
			serializer_append_field!(self, ret_string, lvl_required);
			serializer_append_field!(self, ret_string, skill_type);
			serializer_append_field!(self, ret_string, weapon_required);
			serializer_append_field!(self, ret_string, active_time);
			serializer_append_field!(self, ret_string, auto_active);
			serializer_append_field!(self, ret_string, floor_attribute);
			serializer_append_field_as_bool!(self, ret_string, is_auto_learned);
			serializer_append_field!(self, ret_string, auto_learn_lvl);
			serializer_append_field!(self, ret_string, drop_weapon);
			serializer_append_field!(self, ret_string, stamina_cost);
			serializer_append_field!(self, ret_string, hits_with_weapon);
			serializer_append_field!(self, ret_string, uses_item);
			serializer_append_field!(self, ret_string, next_skill_id_auto_cast);
			serializer_append_field!(self, ret_string, use_delay);
			serializer_append_field!(self, ret_string, use_item_num);
			serializer_append_field!(self, ret_string, sender_action);

			ret_string.push_str(&append_tildes_to(self.short_desc.to_string()));
    		ret_string.push_str(" ");

    		ret_string.push_str(&append_tildes_to(self.desc.to_string()));
    		ret_string.push_str(" ");

			serializer_append_field!(self, ret_string, intone_effect);
			serializer_append_field!(self, ret_string, intone_sfx);
			serializer_append_field!(self, ret_string, sender_effect);
			serializer_append_field!(self, ret_string, sender_sfx);
			serializer_append_field!(self, ret_string, target_delay);
			serializer_append_field!(self, ret_string, target_effect);
			serializer_append_field!(self, ret_string, target_sfx);
			serializer_append_field!(self, ret_string, ground_effect);
			serializer_append_field!(self, ret_string, trace_effect);
			serializer_append_field_as_bool!(self, ret_string, screen_represent);
			serializer_append_field_as_bool!(self, ret_string, is_usable_in_market);
			serializer_append_field_last_as_bool!(self, ret_string, is_staggering);

    		ret_string
    	}
    }

    impl ParserSerializable for MagicTypeHeader {
    	fn serialize_to_string(&self) -> String {
    		let mut ret_string: String = String::new();

    		ret_string.push_str("Amount=");
    		ret_string.push_str(&self.amount.to_string());

    		ret_string
    	}
    }

    impl ParserSerializable for MagicType {
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

	pub fn magic_type_header(input: &[u8]) -> IResult<&[u8], MagicTypeHeader> {
    	do_parse!(input,
    		tag!("Amount=")											>>
    		amount:				parse_str_fragment_crlfeof_to_u32	>>

    		(
    			MagicTypeHeader {
    				amount: amount
    			}
			)
		)
	}

	pub fn magic_type_entry(input: &[u8]) -> IResult<&[u8], MagicTypeEntry> {
		do_parse!(input,
			id: 						add_return_error!(ErrorKind::Custom(1), parse_str_fragment_to_u32)			>>
			action_sort:				add_return_error!(ErrorKind::Custom(2), parse_str_fragment_to_u8)			>>
			skill_name:					add_return_error!(ErrorKind::Custom(3), parse_str_fragment)					>>
			is_offensive_on_use:		add_return_error!(ErrorKind::Custom(4), parse_str_fragment_to_bool)			>>
			is_ground_targeted:			add_return_error!(ErrorKind::Custom(5), parse_str_fragment_to_bool)			>>
			is_multi_target:			add_return_error!(ErrorKind::Custom(6), parse_str_fragment_to_bool)			>>
			target_flag:				add_return_error!(ErrorKind::Custom(7), parse_str_fragment_to_u8)			>>
			skill_lvl:					add_return_error!(ErrorKind::Custom(8), parse_str_fragment_to_u8)			>>
			mp_cost:					add_return_error!(ErrorKind::Custom(9), parse_str_fragment_to_u16)			>>
			power:						add_return_error!(ErrorKind::Custom(10), parse_str_fragment_to_i32)			>>
			intone_effect_duration: 	add_return_error!(ErrorKind::Custom(11), parse_str_fragment_to_u32)			>>
			accuracy:					add_return_error!(ErrorKind::Custom(12), parse_str_fragment_to_u8)			>>
			time:						add_return_error!(ErrorKind::Custom(13), parse_str_fragment_to_u32)			>>
			range:						add_return_error!(ErrorKind::Custom(14), parse_str_fragment_to_u32)			>>
			max_distance:				add_return_error!(ErrorKind::Custom(15), parse_str_fragment_to_u8)			>>
			status:						add_return_error!(ErrorKind::Custom(16), parse_str_fragment_to_u64)			>>
			job_required:				add_return_error!(ErrorKind::Custom(17), parse_str_fragment_to_u32)			>>
			xp_required:				add_return_error!(ErrorKind::Custom(18), parse_str_fragment_to_u64)			>>
			lvl_required:				add_return_error!(ErrorKind::Custom(19), parse_str_fragment_to_u8)			>>
			skill_type:					add_return_error!(ErrorKind::Custom(20), parse_str_fragment_to_u8)			>>
			weapon_required:			add_return_error!(ErrorKind::Custom(21), parse_str_fragment_to_u16)			>>
			active_time:				add_return_error!(ErrorKind::Custom(22), parse_str_fragment_to_u32)			>>
			auto_active:				add_return_error!(ErrorKind::Custom(23), parse_str_fragment_to_u16)			>>
			floor_attribute:			add_return_error!(ErrorKind::Custom(24), parse_str_fragment_to_u32)			>>
			is_auto_learned:			add_return_error!(ErrorKind::Custom(25), parse_str_fragment_to_bool)			>>
			auto_learn_lvl:				add_return_error!(ErrorKind::Custom(26), parse_str_fragment_to_u8)			>>
			drop_weapon:				add_return_error!(ErrorKind::Custom(27), parse_str_fragment_to_u32)			>>
			stamina_cost:				add_return_error!(ErrorKind::Custom(28), parse_str_fragment_to_u8)			>>
			hits_with_weapon:			add_return_error!(ErrorKind::Custom(29), parse_str_fragment_to_u8)			>>
			uses_item:					add_return_error!(ErrorKind::Custom(30), parse_str_fragment_to_u8)			>>
			next_skill_id_auto_cast:	add_return_error!(ErrorKind::Custom(31), parse_str_fragment_to_u32)			>>
			use_delay:					add_return_error!(ErrorKind::Custom(32), parse_str_fragment_to_u32)			>>
			use_item_num:				add_return_error!(ErrorKind::Custom(33), parse_str_fragment_to_u8)			>>
			sender_action:				add_return_error!(ErrorKind::Custom(34), parse_str_fragment_to_u32)			>>
			short_desc:					add_return_error!(ErrorKind::Custom(35), parse_str_fragment)					>>
			desc:						add_return_error!(ErrorKind::Custom(36), parse_str_fragment)					>>
			intone_effect:				add_return_error!(ErrorKind::Custom(37), parse_str_fragment)					>>
			intone_sfx:					add_return_error!(ErrorKind::Custom(38), parse_str_fragment)					>>
			sender_effect:				add_return_error!(ErrorKind::Custom(39), parse_str_fragment)					>>
			sender_sfx:					add_return_error!(ErrorKind::Custom(40), parse_str_fragment)					>>
			target_delay:				add_return_error!(ErrorKind::Custom(41), parse_str_fragment_to_u32)			>>
			target_effect:				add_return_error!(ErrorKind::Custom(42), parse_str_fragment)					>>
			target_sfx:					add_return_error!(ErrorKind::Custom(43), parse_str_fragment)					>>
			ground_effect:				add_return_error!(ErrorKind::Custom(44), parse_str_fragment)					>>
			trace_effect:				add_return_error!(ErrorKind::Custom(45), parse_str_fragment)					>>
			screen_represent:			add_return_error!(ErrorKind::Custom(46), parse_str_fragment_to_bool)			>>
			is_usable_in_market:		add_return_error!(ErrorKind::Custom(47), parse_str_fragment_to_bool)			>>
			is_staggering:				add_return_error!(ErrorKind::Custom(48), parse_str_fragment_crlfeof_to_bool)	>>

			(
				MagicTypeEntry {
					id: id,
					action_sort: action_sort,
					skill_name: skill_name,
					is_offensive_on_use: is_offensive_on_use,
					is_ground_targeted: is_ground_targeted,
					is_multi_target: is_multi_target,

					is_body_target: (target_flag & 0x10) > 0,
					is_passive_target: (target_flag & 0x08) > 0,
					is_terrain_target: (target_flag & 0x04) > 0,
					is_none_target: (target_flag & 0x02) > 0,
					is_self_target: (target_flag & 0x01) > 0,

					skill_lvl: skill_lvl,
					mp_cost: mp_cost,
					power: power,
					intone_effect_duration: intone_effect_duration,
					accuracy: accuracy,
					time: time,
					range: range,
					max_distance: max_distance,
					status: status,
					job_required: job_required,
					xp_required: xp_required,
					lvl_required: lvl_required,
					skill_type: skill_type,
					weapon_required: weapon_required,
					active_time: active_time,
					auto_active: auto_active,
					floor_attribute: floor_attribute,
					is_auto_learned: is_auto_learned,
					auto_learn_lvl: auto_learn_lvl,
					drop_weapon: drop_weapon,
					stamina_cost: stamina_cost,
					hits_with_weapon: hits_with_weapon,
					uses_item: uses_item,
					next_skill_id_auto_cast: next_skill_id_auto_cast,
					use_delay: use_delay,
					use_item_num: use_item_num,
					sender_action: sender_action,

					short_desc: remove_tildes_from(short_desc),
					desc: remove_tildes_from(desc),
					intone_effect: intone_effect,
					intone_sfx: intone_sfx,
					sender_effect: sender_effect,
					sender_sfx: sender_sfx,

					target_delay: target_delay,
					target_effect: target_effect,
					target_sfx: target_sfx,

					ground_effect: ground_effect,
					trace_effect: trace_effect,

					screen_represent: screen_represent,
					is_usable_in_market: is_usable_in_market,
					is_staggering: is_staggering
				}
			)
		)
	}

	pub fn parse_magic_type<'a>(bytes: &'a [u8]) -> Option<MagicType> {
		let mut entries: Vec<MagicTypeEntry> = Vec::new();
		let bytes_split = split_bytes_by_lines(Vec::from(bytes));
		let header = MagicTypeHeader { amount: 30 };

		match bytes_split.get(0) {
			Some(line) => {
				match magic_type_header(&line) {
					IResult::Done(_, magic_type_header_parsed) => {
						let header = magic_type_header_parsed;
					},

					e => {
						println!("Error while matching heaer {:?}", line);
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

			match magic_type_entry(&line) {
				IResult::Done(_, magic_type_entry_parsed) => {
					entries.push(magic_type_entry_parsed);
				},

				e => {
					println!("Error at line {}. Skipped.", idx);
				}
			}
		}

		Some(MagicType { header: header, entries: entries })
	}

	#[cfg(test)]
	mod tests {
		use super::{magic_type_header, magic_type_entry};
		use datfiles::parser::ParserSerializable;

		macro_rules! assert_header_amount_eq {
			( $str_to_parse:expr, $expected:expr ) => ({
	    		let header_bytes = String::from($str_to_parse).into_bytes();
				let (_, parsed_header) = magic_type_header(&header_bytes).unwrap();

				assert_eq!($expected, parsed_header.amount);
			})
		}

		macro_rules! assert_magic_type_field_eq {
			( $str_to_parse:expr, $field:ident, $expected:expr ) => ({
	    		let magic_type_bytes = String::from($str_to_parse).into_bytes();
				let (_, parsed_magic_entry) = magic_type_entry(&magic_type_bytes).unwrap();

				assert_eq!($expected, parsed_magic_entry.$field);
			})
		}

		const SAMPLE_MAGIC_ENTRY_1: &str = "2900 0 BladeStorm 1 0 1 19 0 0 5400 10 100 0 10 10 20 10 1000000 50 2 410 300 4 0 1 100 1 130 1 0 0 2000 0 38 Martial~skill Inflicts~extreme~damage~with~your~weapons~to~surrounding~enemies. superblades none rekt none 140 bladestruck chop none none 0 0 1";
		const SAMPLE_MAGIC_ENTRY_2: &str = "6000 3 Thunderstorm 1 1 1 15 3 2000 38000 200 30 1000 80 0 10 140 14000000 140 1 0 0 0 666 0 0 0 0 0 1 6001 1000 5 90 Spell Low~accuracy~heavy~damage~spell. superlightning none none none 280 incinerated zap burnt none 1 0 1";
		const SAMPLE_MAGIC_ENTRY_3: &str = "7200 10 Cure 0 0 0 25 5 10 850 0 100 0 60 10 0 0 8000 80 0 0 3000 0 0 1 80 0 0 0 0 0 0 0 0 Spell Heals~target. none none none none 0 none none none none 0 1 0";

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
			let (_, parsed_header) = magic_type_header(&header_bytes).unwrap();
			let reserialized_line = parsed_header.serialize_to_string();

			assert_eq!(SAMPLE_HEADER_ENTRY_1, reserialized_line);
		}

		#[test]
		fn parse_header_serialize_will_return_initial_header_2() {
			let header_bytes = String::from(SAMPLE_HEADER_ENTRY_2).into_bytes();
			let (_, parsed_header) = magic_type_header(&header_bytes).unwrap();
			let reserialized_line = parsed_header.serialize_to_string();

			assert_eq!(SAMPLE_HEADER_ENTRY_2, reserialized_line);
		}

		#[test]
		fn parse_header_serialize_will_return_initial_header_3() {
			let header_bytes = String::from(SAMPLE_HEADER_ENTRY_3).into_bytes();
			let (_, parsed_header) = magic_type_header(&header_bytes).unwrap();
			let reserialized_line = parsed_header.serialize_to_string();

			assert_eq!(SAMPLE_HEADER_ENTRY_3, reserialized_line);
		}

		#[test]
		fn parse_magic_entry_will_return_right_id() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, id, 2900);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, id, 6000);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, id, 7200);
		}

		#[test]
		fn parse_magic_entry_will_return_right_action_sort() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, action_sort, 0);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, action_sort, 3);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, action_sort, 10);
		}

		#[test]
		fn parse_magic_entry_will_return_right_skill_name() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, skill_name, "BladeStorm");
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, skill_name, "Thunderstorm");
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, skill_name, "Cure");
		}

		#[test]
		fn parse_magic_entry_will_return_right_is_offensive_on_use() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, is_offensive_on_use, true);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, is_offensive_on_use, true);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, is_offensive_on_use, false);
		}

		#[test]
		fn parse_magic_entry_will_return_right_is_ground_targeted() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, is_ground_targeted, false);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, is_ground_targeted, true);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, is_ground_targeted, false);
		}

		#[test]
		fn parse_magic_entry_will_return_right_is_multi_target() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, is_multi_target, true);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, is_multi_target, true);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, is_multi_target, false);
		}

		#[test]
		fn parse_magic_entry_will_return_right_is_body_target() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, is_body_target, true);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, is_body_target, false);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, is_body_target, true);
		}

		#[test]
		fn parse_magic_entry_will_return_right_is_passive_target() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, is_passive_target, false);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, is_passive_target, true);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, is_passive_target, true);
		}

		#[test]
		fn parse_magic_entry_will_return_right_is_terrain_target() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, is_terrain_target, false);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, is_terrain_target, true);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, is_terrain_target, false);
		}

		#[test]
		fn parse_magic_entry_will_return_right_is_none_target() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, is_none_target, true);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, is_none_target, true);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, is_none_target, false);
		}

		#[test]
		fn parse_magic_entry_will_return_right_is_self_target() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, is_self_target, true);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, is_self_target, true);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, is_self_target, true);
		}

		#[test]
		fn parse_magic_entry_will_return_right_skill_lvl() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, skill_lvl, 0);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, skill_lvl, 3);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, skill_lvl, 5);
		}

		#[test]
		fn parse_magic_entry_will_return_right_mp_cost() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, mp_cost, 0);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, mp_cost, 2000);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, mp_cost, 10);
		}

		#[test]
		fn parse_magic_entry_will_return_right_power() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, power, 5400);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, power, 38000);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, power, 850);
		}

		#[test]
		fn parse_magic_entry_will_return_right_intone_effect_duration() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, intone_effect_duration, 10);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, intone_effect_duration, 200);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, intone_effect_duration, 0);
		}

		#[test]
		fn parse_magic_entry_will_return_right_accuracy() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, accuracy, 100);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, accuracy, 30);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, accuracy, 100);
		}

		#[test]
		fn parse_magic_entry_will_return_right_time() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, time, 0);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, time, 1000);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, time, 0);
		}

		#[test]
		fn parse_magic_entry_will_return_right_range() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, range, 10);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, range, 80);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, range, 60);
		}

		#[test]
		fn parse_magic_entry_will_return_right_max_distance() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, max_distance, 10);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, max_distance, 0);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, max_distance, 10);
		}

		#[test]
		fn parse_magic_entry_will_return_right_status() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, status, 20);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, status, 10);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, status, 0);
		}

		#[test]
		fn parse_magic_entry_will_return_right_job_required() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, job_required, 10);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, job_required, 140);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, job_required, 0);
		}

		#[test]
		fn parse_magic_entry_will_return_right_xp_required() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, xp_required, 1000000);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, xp_required, 14000000);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, xp_required, 8000);
		}

		#[test]
		fn parse_magic_entry_will_return_right_lvl_required() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, lvl_required, 50);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, lvl_required, 140);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, lvl_required, 80);
		}

		#[test]
		fn parse_magic_entry_will_return_right_skill_type() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, skill_type, 2);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, skill_type, 1);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, skill_type, 0);
		}

		#[test]
		fn parse_magic_entry_will_return_right_weapon_required() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, weapon_required, 410);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, weapon_required, 0);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, weapon_required, 0);
		}

		#[test]
		fn parse_magic_entry_will_return_right_active_time() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, active_time, 300);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, active_time, 0);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, active_time, 3000);
		}

		#[test]
		fn parse_magic_entry_will_return_right_auto_active() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, auto_active, 4);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, auto_active, 0);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, auto_active, 0);
		}

		#[test]
		fn parse_magic_entry_will_return_right_is_auto_learned() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, is_auto_learned, true);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, is_auto_learned, false);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, is_auto_learned, true);
		}

		#[test]
		fn parse_magic_entry_will_return_right_auto_learn_lvl() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, auto_learn_lvl, 100);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, auto_learn_lvl, 0);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, auto_learn_lvl, 80);
		}

		#[test]
		fn parse_magic_entry_will_return_right_drop_weapon() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, drop_weapon, 1);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, drop_weapon, 0);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, drop_weapon, 0);
		}

		#[test]
		fn parse_magic_entry_will_return_right_stamina_cost() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, stamina_cost, 130);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, stamina_cost, 0);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, stamina_cost, 0);
		}

		#[test]
		fn parse_magic_entry_will_return_right_hits_with_weapon() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, hits_with_weapon, 1);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, hits_with_weapon, 0);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, hits_with_weapon, 0);
		}

		#[test]
		fn parse_magic_entry_will_return_right_uses_item() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, uses_item, 0);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, uses_item, 1);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, uses_item, 0);
		}

		#[test]
		fn parse_magic_entry_will_return_right_next_skill_id_auto_cast() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, next_skill_id_auto_cast, 0);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, next_skill_id_auto_cast, 6001);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, next_skill_id_auto_cast, 0);
		}

		#[test]
		fn parse_magic_entry_will_return_right_use_delay() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, use_delay, 2000);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, use_delay, 1000);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, use_delay, 0);
		}

		#[test]
		fn parse_magic_entry_will_return_right_use_item_num() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, use_item_num, 0);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, use_item_num, 5);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, use_item_num, 0);
		}

		#[test]
		fn parse_magic_entry_will_return_right_sender_action() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, sender_action, 38);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, sender_action, 90);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, sender_action, 0);
		}

		#[test]
		fn parse_magic_entry_will_return_right_short_desc() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, short_desc, "Martial skill");
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, short_desc, "Spell");
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, short_desc, "Spell");
		}

		#[test]
		fn parse_magic_entry_will_return_right_desc() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, desc, "Inflicts extreme damage with your weapons to surrounding enemies.");
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, desc, "Low accuracy heavy damage spell.");
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, desc, "Heals target.");
		}

		#[test]
		fn parse_magic_entry_will_return_right_intone_effect() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, intone_effect, "superblades");
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, intone_effect, "superlightning");
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, intone_effect, "none");
		}

		#[test]
		fn parse_magic_entry_will_return_right_intone_sfx() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, intone_sfx, "none");
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, intone_sfx, "none");
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, intone_sfx, "none");
		}

		#[test]
		fn parse_magic_entry_will_return_right_sender_effect() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, sender_effect, "rekt");
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, sender_effect, "none");
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, sender_effect, "none");
		}

		#[test]
		fn parse_magic_entry_will_return_right_sender_sfx() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, sender_sfx, "none");
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, sender_sfx, "none");
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, sender_sfx, "none");
		}

		#[test]
		fn parse_magic_entry_will_return_right_target_delay() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, target_delay, 140);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, target_delay, 280);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, target_delay, 0);
		}

		#[test]
		fn parse_magic_entry_will_return_right_target_effect() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, target_effect, "bladestruck");
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, target_effect, "incinerated");
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, target_effect, "none");
		}

		#[test]
		fn parse_magic_entry_will_return_right_target_sfx() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, target_sfx, "chop");
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, target_sfx, "zap");
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, target_sfx, "none");
		}

		#[test]
		fn parse_magic_entry_will_return_right_ground_effect() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, ground_effect, "none");
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, ground_effect, "burnt");
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, ground_effect, "none");
		}

		#[test]
		fn parse_magic_entry_will_return_right_trace_effect() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, trace_effect, "none");
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, trace_effect, "none");
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, trace_effect, "none");
		}

		#[test]
		fn parse_magic_entry_will_return_right_screen_represent() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, screen_represent, false);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, screen_represent, true);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, screen_represent, false);
		}

		#[test]
		fn parse_magic_entry_will_return_right_is_usable_in_market() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, is_usable_in_market, false);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, is_usable_in_market, false);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, is_usable_in_market, true);
		}

		#[test]
		fn parse_magic_entry_will_return_right_is_staggering() {
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_1, is_staggering, true);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_2, is_staggering, true);
			assert_magic_type_field_eq!(SAMPLE_MAGIC_ENTRY_3, is_staggering, false);
		}

		#[test]
		fn parse_magic_entry_serialize_will_return_initial_line_1() {
			let magic_type_bytes = String::from(SAMPLE_MAGIC_ENTRY_1).into_bytes();
			let (_, parsed_magic_entry) = magic_type_entry(&magic_type_bytes).unwrap();
			let reserialized_line = parsed_magic_entry.serialize_to_string();

			assert_eq!(SAMPLE_MAGIC_ENTRY_1, reserialized_line);
		}

		#[test]
		fn parse_magic_entry_serialize_will_return_initial_line_2() {
			let magic_type_bytes = String::from(SAMPLE_MAGIC_ENTRY_2).into_bytes();
			let (_, parsed_magic_entry) = magic_type_entry(&magic_type_bytes).unwrap();
			let reserialized_line = parsed_magic_entry.serialize_to_string();

			assert_eq!(SAMPLE_MAGIC_ENTRY_2, reserialized_line);
		}

		#[test]
		fn parse_magic_entry_serialize_will_return_initial_line_3() {
			let magic_type_bytes = String::from(SAMPLE_MAGIC_ENTRY_3).into_bytes();
			let (_, parsed_magic_entry) = magic_type_entry(&magic_type_bytes).unwrap();
			let reserialized_line = parsed_magic_entry.serialize_to_string();

			assert_eq!(SAMPLE_MAGIC_ENTRY_3, reserialized_line);
		}
	}
}

pub mod encoder {
	use super::{MagicType, MagicTypeEntry, MagicTypeHeader};
	use serde_json;

	pub fn decode_magic_type_to_json(magic_type: &MagicType) -> Vec<u8> {
		let json_magic_type = serde_json::to_string_pretty(&magic_type.entries).unwrap();
		println!("{}", magic_type.entries.len());

		json_magic_type.into_bytes()
	}

	pub fn encode_magic_type_from_json(magic_type: Vec<u8>) -> MagicType {
		let json_magic_type = String::from_utf8(magic_type).unwrap();
		let magic_type_entries: Vec<MagicTypeEntry> = serde_json::from_str(&json_magic_type).unwrap();

		MagicType { header: MagicTypeHeader { amount: magic_type_entries.len() as u32 }, entries: magic_type_entries }
	}
}