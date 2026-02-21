use std::{io::Cursor, path::Path};

pub fn load_root_dictionary(config_path: &Path) -> Result<plist::Dictionary, String> {
    let value = plist::Value::from_file(config_path)
        .map_err(|err| format!("failed to read plist: {err}"))?;

    value
        .as_dictionary()
        .cloned()
        .ok_or_else(|| "terminal plist root is not a dictionary".to_string())
}

pub fn font_name_from_keyed_archive(value: &plist::Value) -> Option<String> {
    let bytes = value.as_data()?;
    let archive = plist::Value::from_reader(Cursor::new(bytes)).ok()?;
    let archive = archive.as_dictionary()?;

    let objects = archive.get("$objects")?.as_array()?;
    let root = archive
        .get("$top")?
        .as_dictionary()?
        .get("root")
        .and_then(uid_index)?;

    let root = objects.get(root)?.as_dictionary()?;
    let name_index = root.get("NSName").and_then(uid_index)?;
    objects
        .get(name_index)?
        .as_string()
        .map(ToString::to_string)
}

fn uid_index(value: &plist::Value) -> Option<usize> {
    match value {
        plist::Value::Uid(uid) => Some(uid.get() as usize),
        _ => None,
    }
}
