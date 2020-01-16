use std::convert::{TryFrom, TryInto};
use std::io;

use swf_types as ast;

use crate::basic_data_types::{emit_color_transform_with_alpha, emit_matrix};
use crate::display::{emit_blend_mode, emit_filter_list};
use crate::primitives::{emit_le_u16, emit_u8};

// TODO: Remove unused variants (`dead_code` should not be allowed)
#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum ButtonVersion {
  Button1,
  Button2,
}

pub(crate) fn emit_button_record_string<W: io::Write>(
  writer: &mut W,
  value: &[ast::ButtonRecord],
  version: ButtonVersion,
) -> io::Result<()> {
  for record in value {
    emit_button_record(writer, record, version)?;
  }
  emit_u8(writer, 0)
}

pub(crate) fn emit_button_record<W: io::Write>(
  writer: &mut W,
  value: &ast::ButtonRecord,
  version: ButtonVersion,
) -> io::Result<()> {
  let has_filters = value.filters.len() != 0;
  let has_blend_mode = value.blend_mode != ast::BlendMode::Normal;

  let flags: u8 = 0
    | (if value.state_up { 1 << 0 } else { 0 })
    | (if value.state_over { 1 << 1 } else { 0 })
    | (if value.state_down { 1 << 2 } else { 0 })
    | (if value.state_hit_test { 1 << 3 } else { 0 })
    | (if has_filters { 1 << 4 } else { 0 })
    | (if has_blend_mode { 1 << 5 } else { 0 });
  // Skip bits [6, 7]
  emit_u8(writer, flags)?;

  emit_le_u16(writer, value.character_id)?;
  emit_le_u16(writer, value.depth)?;
  emit_matrix(writer, &value.matrix)?;
  if version >= ButtonVersion::Button2 {
    emit_color_transform_with_alpha(writer, &value.color_transform.unwrap())?;
    if has_filters {
      emit_filter_list(writer, &value.filters)?;
    }
    if has_blend_mode {
      emit_blend_mode(writer, value.blend_mode)?;
    }
  }
  Ok(())
}

pub(crate) fn emit_button2_cond_action_string<W: io::Write>(
  writer: &mut W,
  value: &[ast::ButtonCondAction],
) -> io::Result<()> {
  for (index, action) in value.iter().enumerate() {
    let mut action_writer = Vec::new();
    emit_button2_cond_action(&mut action_writer, action)?;
    if index == value.len() - 1 {
      // !is_last
      emit_le_u16(writer, action_writer.len().try_into().unwrap())?;
    } else {
      // is_last
      emit_le_u16(writer, 0)?;
    }
    writer.write_all(&action_writer)?;
  }
  Ok(())
}

pub(crate) fn emit_button2_cond_action<W: io::Write>(writer: &mut W, value: &ast::ButtonCondAction) -> io::Result<()> {
  emit_button_cond(writer, &value.conditions.unwrap())?;
  writer.write_all(&value.actions)
}

pub(crate) fn emit_button_cond<W: io::Write>(writer: &mut W, value: &ast::ButtonCond) -> io::Result<()> {
  let key_code: u16 = match value.key_press {
    Some(key_code) => u16::try_from(key_code).unwrap() & 0x7f,
    None => 0,
  };
  let flags: u16 = 0
    | (if value.idle_to_over_up { 1 << 0 } else { 0 })
    | (if value.over_up_to_idle { 1 << 1 } else { 0 })
    | (if value.over_up_to_over_down { 1 << 2 } else { 0 })
    | (if value.over_down_to_over_up { 1 << 3 } else { 0 })
    | (if value.over_down_to_out_down { 1 << 4 } else { 0 })
    | (if value.out_down_to_over_down { 1 << 5 } else { 0 })
    | (if value.out_down_to_idle { 1 << 6 } else { 0 })
    | (if value.idle_to_over_down { 1 << 7 } else { 0 })
    | (if value.over_down_to_idle { 1 << 8 } else { 0 })
    | (key_code << 9);
  emit_le_u16(writer, flags)
}
