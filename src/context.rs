/// Recursively merge two tera values into one.
#[must_use]
pub fn merge_values(a: &tera::Value, b: &tera::Value) -> tera::Value {
    match (a, b) {
        // if both are objects, merge them
        _ if a.as_map().is_some() && b.as_map().is_some() => {
            let mut result = a.clone().into_map().expect("a is a map");
            for (k, v) in b.as_map().expect("b is a map") {
                result.insert(
                    k.clone(),
                    merge_values(
                        a.as_map()
                            .expect("a is a map")
                            .get(k)
                            .unwrap_or(&tera::Value::none()),
                        v,
                    ),
                );
            }
            result.into()
        }
        // otherwise, use the second value
        (_, b) => b.clone(),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_merge_values() {
        let a = tera::Value::from_serializable(&json!({
            "a": 1,
            "b": {
                "c": 2,
                "d": 3,
            },
        }));
        let b = tera::Value::from_serializable(&json!({
            "b": {
                "c": 4,
                "e": 5,
            },
            "f": 6,
        }));
        let result = merge_values(&a, &b);
        assert_eq!(
            result,
            tera::Value::from_serializable(&json!({
                "a": 1,
                "b": {
                    "c": 4,
                    "d": 3,
                    "e": 5,
                },
                "f": 6,
            }))
        );
    }
}
