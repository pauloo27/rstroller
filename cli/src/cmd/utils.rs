use mpris::MetadataValue;

pub fn format_metadata_value(value: &MetadataValue) -> String {
    match value {
        MetadataValue::String(v) => v.into(),
        MetadataValue::I16(v) => v.to_string(),
        MetadataValue::I32(v) => v.to_string(),
        MetadataValue::I64(v) => v.to_string(),
        MetadataValue::U8(v) => v.to_string(),
        MetadataValue::U16(v) => v.to_string(),
        MetadataValue::U32(v) => v.to_string(),
        MetadataValue::U64(v) => v.to_string(),
        MetadataValue::F64(v) => v.to_string(),
        MetadataValue::Bool(v) => v.to_string(),
        MetadataValue::Array(v) => format!("{:?}", v),
        MetadataValue::Map(v) => format!("{:?}", v),
        MetadataValue::Unsupported => "unsupported".into(),
    }
}

pub fn parse_offset(arg: &str) -> Result<f64, String> {
    let arg = &arg[..arg.len() - 1];

    arg.parse::<f64>()
        .map_err(|e| format!("Failed to parse offset: {}", e))
}
