use std::collections::HashMap;

use catppuccin::FlavorName;

pub type Matrix = HashMap<String, Vec<String>>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Unknown magic iterable: {name}")]
    UnknownIterable { name: String },

    #[error("Invalid matrix array object element: must have a single key and an array of strings as value")]
    InvalidObjectElement,

    #[error("Invalid matrix array element: must be a string or object")]
    InvalidElement,
}

// matrix in frontmatter is a list of strings or objects.
// objects must have a single key and an array of strings as the value.
// string array elements are substituted with the array from `iterables`.
//
// # Panics
//
// When a value that should be a string isn't (
pub fn from_values(
    values: Vec<tera::Value>,
    only_flavor: Option<FlavorName>,
) -> Result<Matrix, Error> {
    let iterables = magic_iterables(only_flavor);
    values
        .into_iter()
        .map(|v| match v {
            _ if v.as_str().is_some() => {
                let s = v.as_str().expect("v is a string");

                let iterable = iterables.get(s).ok_or_else(|| Error::UnknownIterable {
                    name: s.to_string(),
                })?;
                Ok((s.to_string(), iterable.clone()))
            }
            _ if v.as_map().is_some() => {
                let o = v.into_map().expect("v is a map");

                let (key, value) = o.into_iter().next().ok_or(Error::InvalidObjectElement)?;
                let value: Vec<String> = value
                    .as_array()
                    .ok_or(Error::InvalidObjectElement)?
                    .iter()
                    .map(|v| {
                        v.as_str()
                            .map(str::to_string)
                            .ok_or(Error::InvalidObjectElement)
                    })
                    .collect::<Result<Vec<String>, Error>>()?;
                Ok((
                    key.as_str().ok_or(Error::InvalidObjectElement)?.to_string(),
                    value,
                ))
            }
            _ => Err(Error::InvalidElement),
        })
        .collect::<Result<Matrix, Error>>()
}

fn magic_iterables(only_flavor: Option<FlavorName>) -> HashMap<&'static str, Vec<String>> {
    HashMap::from([
        (
            "flavor",
            only_flavor.map_or_else(
                || {
                    catppuccin::PALETTE
                        .into_iter()
                        .map(|flavor| flavor.identifier().to_string())
                        .collect::<Vec<String>>()
                },
                |flavor| vec![flavor.identifier().to_string()],
            ),
        ),
        ("accent", ctp_accents()),
    ])
}

fn ctp_accents() -> Vec<String> {
    catppuccin::PALETTE
        .latte
        .colors
        .iter()
        .filter(|c| c.accent)
        .map(|c| c.name.identifier().to_string())
        .collect()
}
