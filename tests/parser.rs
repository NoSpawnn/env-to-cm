use env_to_cm::*;
use indoc::indoc;

macro_rules! expected {
    ( $( ($key:expr, $val:expr) ),* $(,)? ) => {
        {
            let array = [ $( (String::from($key), String::from($val)) ),* ];
            Vec::from(array)
        }
    };
}

//
// FIX THESE WITH THE NEW TOKEN ENUM! (im too tired rn x)
//

// #[test]
// fn parse_good<'a>() -> Result<(), ParseError<'a>> {
//     let config = ParseConfig::default();
//     let input = indoc! {"
//         value1=hello
//         value2=
//     "};
//     let expected = expected!(("value1", "hello"), ("value2", ""));
//     let actual = parse(input, config)?;
//     assert_eq!(actual, expected);
//     Ok(())
// }

// #[test]
// fn parse_bad() {
//     let config = ParseConfig::default();
//     let input = indoc! {"
//         key1
//         key2
//     "};
//     assert!(matches!(
//         parse(input, config).unwrap_err(),
//         ParseError::InvalidFormat(..)
//     ));
// }

// #[test]
// fn parse_empty() {
//     let config = ParseConfig::default();
//     let input = "";
//     assert_eq!(parse(input, config).unwrap_err(), ParseError::Empty);
// }
