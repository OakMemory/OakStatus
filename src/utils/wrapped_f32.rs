use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize,Clone)]
pub struct WrappedF32(f32);

impl Serialize for WrappedF32 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if self.0 >= 0. {
            return serializer.serialize_f32(self.0);
        } else {
            return serializer.serialize_char('-');
        };
    }
}

impl From<f32> for WrappedF32 {
    fn from(v: f32) -> Self {
        WrappedF32(v)
    }
}

impl Into<f32> for WrappedF32 {
    fn into(self) -> f32 {
        self.0
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TestStruct {
        val: WrappedF32,
    }
    #[test]
    fn de_wrapped_f32() {
        let json0 = r#"{"val":-1.00000}"#;
        let json1 = r#"{"val":1.00000}"#;
        assert_eq!(
            serde_json::from_str::<TestStruct>(json0).unwrap(),
            TestStruct {
                val: WrappedF32(-1.00000)
            }
        );
        assert_eq!(
            serde_json::from_str::<TestStruct>(json1).unwrap(),
            TestStruct {
                val: WrappedF32(1.00000)
            }
        );
        assert_eq!(
            r#"{"val":"-"}"#,
            serde_json::to_string(&TestStruct {
                val: WrappedF32(-1.00000)
            })
            .unwrap()
        );
        assert_eq!(
            r#"{"val":1.0}"#,
            serde_json::to_string(&TestStruct {
                val: WrappedF32(1.00000)
            })
            .unwrap()
        );
    }
}
