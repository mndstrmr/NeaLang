pub(crate) fn reg_for_value_type(vt: &ir::ValueType, mode: x86::Mode, class: x86::RegClass) -> x86::Reg {
    match vt {
        ir::ValueType::U8 | ir::ValueType::I8 | ir::ValueType::Bool => class.u8(),
        ir::ValueType::U16 | ir::ValueType::I16 => class.u16(),
        ir::ValueType::U32 | ir::ValueType::I32 => class.u32(),
        ir::ValueType::U64 | ir::ValueType::I64 => class.u64(),
        ir::ValueType::UPtr | ir::ValueType::IPtr | ir::ValueType::Ref(_) | ir::ValueType::Index(_) => match mode {
            x86::Mode::X86 => class.u32(),
            x86::Mode::X8664 => class.u64(),
        },
    }
}

pub(crate) fn offset_of_compound_property(ct: &ir::CompoundTypeRef, idx: ir::PropertyIndex, mode: x86::Mode) -> usize {
    match ct.content() {
        ir::CompoundContent::Struct(struc) => {
            let mut offset = 0;

            for prop in &struc.props()[0..idx.idx()] {
                offset += size_for_storable_type(prop.prop_type(), mode);
            }

            offset
        },
    }
}

pub(crate) fn size_for_compound_type(ct: &ir::CompoundType, mode: x86::Mode) -> usize {
    match ct.content() {
        ir::CompoundContent::Struct(s) => {
            let mut size = 0;

            for s in s.props() {
                size += size_for_storable_type(s.prop_type(), mode);
            }

            size
        },
    }
}

pub(crate) fn size_for_value_type(vt: &ir::ValueType, mode: x86::Mode) -> usize {
    match vt {
        ir::ValueType::U8 | ir::ValueType::I8 | ir::ValueType::Bool => 1,
        ir::ValueType::U16 | ir::ValueType::I16 => 2,
        ir::ValueType::U32 | ir::ValueType::I32 => 4,
        ir::ValueType::U64 | ir::ValueType::I64 => 8,
        ir::ValueType::UPtr | ir::ValueType::IPtr | ir::ValueType::Ref(_) | ir::ValueType::Index(_) => mode.ptr_size(),
    }
}

pub(crate) fn size_for_storable_type(storable: &ir::StorableType, mode: x86::Mode) -> usize {
    match storable {
        ir::StorableType::Compound(ct) => size_for_compound_type(ct, mode),
        ir::StorableType::Value(vt) => size_for_value_type(vt, mode),
        ir::StorableType::Slice(_) => mode.ptr_size() * 2,
        ir::StorableType::SliceData(_) => panic!("Cannot compute raw size of SliceData type"),
    }
}