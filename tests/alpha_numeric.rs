extern crate strongly_typed;

use strongly_typed::*;

const ALPHA_NUMERIC_REGEX: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new("^[A-Za-z0-9]+$").expect("invalid regex"));
enum AlphaNumericContract {}

impl Contract for AlphaNumericContract {
    type Value = String;
    type Error = ();

    fn invariant(value: &Self::Value) -> Result<(), Self::Error> {
        if ALPHA_NUMERIC_REGEX.is_match(value) {
            Ok(())
        } else {
            Err(())
        }
    }
}

struct FixedLengthContract<T, const N: usize>(T);

impl<T: Contract<Value = String, Error = ()>, const N: usize> Contract
    for FixedLengthContract<T, N>
{
    type Value = String;
    type Error = ();

    fn invariant(value: &Self::Value) -> Result<(), Self::Error> {
        T::invariant(value)?;

        if value.chars().count() == N {
            Ok(())
        } else {
            Err(())
        }
    }
}

type DynamicLengthAlphaNumericString = TypedString<AlphaNumericContract>;

type FixedLengthAlphaNumericString<const N: usize> =
    TypedString<FixedLengthContract<AlphaNumericContract, N>>;

#[test]
fn ok_when_initialized_with_alpha_numeric_string() {
    assert!(DynamicLengthAlphaNumericString::new("0123456789ABCDEF".into()).is_ok());
    assert!(FixedLengthAlphaNumericString::<16>::new("0123456789ABCDEF".into()).is_ok());
}

#[test]
fn err_when_initialized_with_alpha_numeric_string() {
    assert!(DynamicLengthAlphaNumericString::new("☺️".into()).is_err());
    assert!(FixedLengthAlphaNumericString::<1>::new("☺".into()).is_err());
}

enum Base64Codec {}

impl Encode for Base64Codec {
    type Value = String;
    type Target = String;

    fn into(value: &Self::Value) -> Self::Target {
        base64::encode(value)
    }
}

impl Decode for Base64Codec {
    type Value = String;
    type Target = String;
    type Error = ();

    fn from(value: &Self::Target) -> Result<Self::Value, Self::Error> {
        let bytes = base64::decode(value).map_err(|_| ())?;
        String::from_utf8(bytes).map_err(|_| ())
    }
}

#[test]
fn encode_alpha_numeric_to_base64() {
    let an = FixedLengthAlphaNumericString::<5>::new("12345".into()).unwrap();
    let actual = an.encode::<Base64Codec>();
    let expected = "MTIzNDU=".to_owned();
    assert_eq!(actual, expected);
}

#[test]
fn decode_alpha_numeric_from_base64() {
    let base64 = "MTIzNDU=".to_owned();
    let actual = FixedLengthAlphaNumericString::<5>::decode::<Base64Codec>(&base64).unwrap();
    let expected = FixedLengthAlphaNumericString::<5>::new("12345".into()).unwrap();
    assert_eq!(actual, expected);
}