pub type SingleColor = u32;

/// Represent RGB color
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Color {
    pub red: SingleColor,
    pub green: SingleColor,
    pub blue: SingleColor,
}

#[cfg(test)]
mod tests {

    #[test]
    #[cfg(feature = "serde")]
    fn serde_serialization_deserialization() {
        use super::Color;

        let color = Color {
            red: 25,
            green: 220,
            blue: 100,
        };

        let serialized_string = serde_json::to_string(&color).unwrap();

        assert_eq!(serialized_string, "{\"red\":25,\"green\":220,\"blue\":100}");

        assert_eq!(
            serde_json::from_str::<Color>(&serialized_string).unwrap(),
            color
        );
    }
}
