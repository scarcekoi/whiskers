use std::{
    collections::{BTreeMap, HashMap},
    io::Write,
};

use base64::Engine as _;
use serde::Deserialize as _;

use crate::models::Color;

pub fn mix(
    value: &tera::Value,
    kwargs: tera::Kwargs,
    _: &tera::State,
) -> Result<tera::Value, tera::Error> {
    let base: Color = Color::deserialize(value).map_err(|e| tera::Error::message(e.to_string()))?;
    let blend: &tera::Value = kwargs.must_get::<&tera::Value>("color")?;
    let blend: Color =
        Color::deserialize(blend).map_err(|e| tera::Error::message(e.to_string()))?;
    let amount = kwargs
        .must_get::<&tera::Value>("amount")?
        .as_f64()
        .ok_or_else(|| tera::Error::message("blend amount must be a number"))?;

    let result = Color::mix(&base, &blend, amount)?;

    Ok(tera::Value::from_serializable(&result))
}

pub fn modify(
    value: &tera::Value,
    kwargs: tera::Kwargs,
    _: &tera::State,
) -> Result<tera::Value, tera::Error> {
    let color: Color =
        Color::deserialize(value).map_err(|e| tera::Error::message(e.to_string()))?;
    if let Some(hue) = kwargs.get::<i32>("hue")? {
        Ok(tera::Value::from_serializable(&color.mod_hue(hue)?))
    } else if let Some(saturation) = kwargs.get::<u8>("saturation")? {
        Ok(tera::Value::from_serializable(
            &color.mod_saturation(saturation)?,
        ))
    } else if let Some(lightness) = kwargs.get::<u8>("lightness")? {
        Ok(tera::Value::from_serializable(
            &color.mod_lightness(lightness)?,
        ))
    } else if let Some(opacity) = kwargs.get::<f32>("opacity")? {
        Ok(tera::Value::from_serializable(&color.mod_opacity(opacity)?))
    } else {
        Ok(value.clone())
    }
}

pub fn add(
    value: &tera::Value,
    kwargs: tera::Kwargs,
    _: &tera::State,
) -> Result<tera::Value, tera::Error> {
    let color: Color =
        Color::deserialize(value).map_err(|e| tera::Error::message(e.to_string()))?;
    if let Some(hue) = kwargs.get::<i32>("hue")? {
        Ok(tera::Value::from_serializable(&color.add_hue(hue)?))
    } else if let Some(saturation) = kwargs.get::<u8>("saturation")? {
        Ok(tera::Value::from_serializable(
            &color.add_saturation(saturation)?,
        ))
    } else if let Some(lightness) = kwargs.get::<u8>("lightness")? {
        Ok(tera::Value::from_serializable(
            &color.add_lightness(lightness)?,
        ))
    } else if let Some(opacity) = kwargs.get::<f32>("opacity")? {
        Ok(tera::Value::from_serializable(&color.add_opacity(opacity)?))
    } else {
        Ok(value.clone())
    }
}

pub fn sub(
    value: &tera::Value,
    kwargs: tera::Kwargs,
    _: &tera::State,
) -> Result<tera::Value, tera::Error> {
    let color: Color =
        Color::deserialize(value).map_err(|e| tera::Error::message(e.to_string()))?;
    if let Some(hue) = kwargs.get::<i32>("hue")? {
        Ok(tera::Value::from_serializable(&color.sub_hue(hue)?))
    } else if let Some(saturation) = kwargs.get::<u8>("saturation")? {
        Ok(tera::Value::from_serializable(
            &color.sub_saturation(saturation)?,
        ))
    } else if let Some(lightness) = kwargs.get::<u8>("lightness")? {
        Ok(tera::Value::from_serializable(
            &color.sub_lightness(lightness)?,
        ))
    } else if let Some(opacity) = kwargs.get::<f32>("opacity")? {
        Ok(tera::Value::from_serializable(&color.sub_opacity(opacity)?))
    } else {
        Ok(value.clone())
    }
}

pub fn urlencode_lzma(
    value: &tera::Value,
    _kwargs: tera::Kwargs,
    _: &tera::State,
) -> Result<tera::Value, tera::Error> {
    // encode the data with the following process:
    // 1. messagepack the data
    // 2. compress the messagepacked data with lzma (v1, preset 9)
    // 3. urlsafe base64 encode the compressed data
    let value: BTreeMap<String, serde_json::Value> =
        BTreeMap::<String, serde_json::Value>::deserialize(value)
            .map_err(|e| tera::Error::message(e.to_string()))?;
    let packed = rmp_serde::to_vec(&value).map_err(|e| tera::Error::message(e.to_string()))?;
    let mut options = lzma_rust::LZMA2Options::with_preset(9);
    options.dict_size = lzma_rust::LZMA2Options::DICT_SIZE_DEFAULT;
    let mut compressed = Vec::new();
    let mut writer = lzma_rust::LZMAWriter::new(
        lzma_rust::CountingWriter::new(&mut compressed),
        &options,
        true,
        false,
        Some(packed.len() as u64),
    )?;
    writer.write_all(&packed)?;
    let _ = writer.write(&[])?;
    let encoded = base64::engine::general_purpose::URL_SAFE.encode(compressed);
    Ok(tera::Value::from_serializable(&encoded))
}

pub fn trunc(
    value: f64,
    kwargs: tera::Kwargs,
    _: &tera::State,
) -> Result<tera::Value, tera::Error> {
    let places: usize = kwargs
        .get::<usize>("places")?
        .ok_or_else(|| tera::Error::message("number of places is required"))?;
    Ok(tera::Value::from_serializable(&format!("{value:.places$}")))
}

pub fn css_rgb(
    value: &tera::Value,
    _kwargs: tera::Kwargs,
    _: &tera::State,
) -> Result<tera::Value, tera::Error> {
    let color: Color =
        Color::deserialize(value).map_err(|e| tera::Error::message(e.to_string()))?;
    let color: farver::RGB = (&color).into();
    Ok(tera::Value::from_serializable(&color.to_string()))
}

pub fn hex(
    value: &tera::Value,
    _kwargs: tera::Kwargs,
    _: &tera::State,
) -> Result<tera::Value, tera::Error> {
    let color: Color =
        Color::deserialize(value).map_err(|e| tera::Error::message(e.to_string()))?;
    Ok(tera::Value::from_serializable(&color.hex))
}

pub fn css_rgba(
    value: &tera::Value,
    _kwargs: tera::Kwargs,
    _: &tera::State,
) -> Result<tera::Value, tera::Error> {
    let color: Color =
        Color::deserialize(value).map_err(|e| tera::Error::message(e.to_string()))?;
    let color: farver::RGBA = (&color).into();
    Ok(tera::Value::from_serializable(&color.to_string()))
}

pub fn css_hsl(
    value: &tera::Value,
    _kwargs: tera::Kwargs,
    _: &tera::State,
) -> Result<tera::Value, tera::Error> {
    let color: Color =
        Color::deserialize(value).map_err(|e| tera::Error::message(e.to_string()))?;
    let color: farver::HSL = (&color).into();
    Ok(tera::Value::from_serializable(&color.to_string()))
}

pub fn css_hsla(
    value: &tera::Value,
    _kwargs: tera::Kwargs,
    _: &tera::State,
) -> Result<tera::Value, tera::Error> {
    let color: Color =
        Color::deserialize(value).map_err(|e| tera::Error::message(e.to_string()))?;
    let color: farver::HSLA = (&color).into();
    Ok(tera::Value::from_serializable(&color.to_string()))
}
