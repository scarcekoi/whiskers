use std::{collections::BTreeMap, fs, path::PathBuf};

use serde::Deserialize as _;
use tera::Kwargs;

use crate::models::Color;

pub fn if_fn(kwargs: Kwargs, _: &tera::State) -> Result<tera::Value, tera::Error> {
    let cond = kwargs
        .get::<bool>("cond")?
        .ok_or_else(|| tera::Error::message("cond is required"))?;
    let t = kwargs
        .get::<tera::Value>("t")?
        .ok_or_else(|| tera::Error::message("t is required"))?;
    let f = kwargs
        .get::<tera::Value>("f")?
        .ok_or_else(|| tera::Error::message("f is required"))?;

    Ok(if cond { t } else { f })
}

pub fn object(kwargs: Kwargs, _: &tera::State) -> Result<tera::Value, tera::Error> {
    // sorting the args gives us stable output
    let kwargs: BTreeMap<_, _> = kwargs.deserialize::<BTreeMap<String, serde_json::Value>>()?;
    Ok(tera::Value::from_serializable(&kwargs))
}

pub fn css_rgb(kwargs: Kwargs, _: &tera::State) -> Result<tera::Value, tera::Error> {
    let color: Color = Color::deserialize(
        kwargs
            .get::<&tera::Value>("color")?
            .ok_or_else(|| tera::Error::message("color is required"))?,
    )
    .map_err(|e| tera::Error::message(e.to_string()))?;

    let color: farver::RGB = (&color).into();
    Ok(tera::Value::from_serializable(&color.to_string()))
}

pub fn css_rgba(kwargs: Kwargs, _: &tera::State) -> Result<tera::Value, tera::Error> {
    let color: Color = Color::deserialize(
        kwargs
            .get::<&tera::Value>("color")?
            .ok_or_else(|| tera::Error::message("color is required"))?,
    )
    .map_err(|e| tera::Error::message(e.to_string()))?;
    let color: farver::RGBA = (&color).into();
    Ok(tera::Value::from_serializable(&color.to_string()))
}

pub fn css_hsl(kwargs: Kwargs, _: &tera::State) -> Result<tera::Value, tera::Error> {
    let color: Color = Color::deserialize(
        kwargs
            .get::<&tera::Value>("color")?
            .ok_or_else(|| tera::Error::message("color is required"))?,
    )
    .map_err(|e| tera::Error::message(e.to_string()))?;

    let color: farver::HSL = (&color).into();
    Ok(tera::Value::from_serializable(&color.to_string()))
}

pub fn css_hsla(kwargs: Kwargs, _: &tera::State) -> Result<tera::Value, tera::Error> {
    let color: Color = Color::deserialize(
        kwargs
            .get::<&tera::Value>("color")?
            .ok_or_else(|| tera::Error::message("color is required"))?,
    )
    .map_err(|e| tera::Error::message(e.to_string()))?;
    let color: farver::HSLA = (&color).into();
    Ok(tera::Value::from_serializable(&color.to_string()))
}

pub fn read_file_handler(
    template_directory: PathBuf,
) -> impl Fn(Kwargs, &tera::State) -> Result<tera::Value, tera::Error> {
    move |kwargs, _: &tera::State| -> Result<tera::Value, tera::Error> {
        let path: String = kwargs
            .get::<String>("path")?
            .ok_or_else(|| tera::Error::message("path is required"))?;
        let path = template_directory.join(path);
        let contents = fs::read_to_string(&path)
            .map_err(|_| format!("Failed to open file {}", path.display()));
        Ok(tera::Value::from_serializable(&contents))
    }
}
