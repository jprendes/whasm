pub use whasm_grammar_derive::Grammar;

#[cfg(test)]
mod test {
    use crate::grammar::*;

    #[derive(Grammar, Debug, PartialEq)]
    struct Struct {
        first: u32,
        second: String,
    }

    #[test]
    fn can_deserialize_struct() {
        let mut iter = [0x2A, 0x02, 0x34, 0x32, 0x03, 0x01, 0x02, 0x03].iter().copied();
        let result: Struct = deserialize(&mut iter).unwrap();
        assert_eq!(result, Struct {
            first: 42,
            second: "42".into(),
        });
    }

    #[derive(Grammar, Debug, PartialEq)]
    struct MatchingStruct {
        #[matching(42)]
        data: u32,
    }

    #[test]
    fn can_deserialize_matching_struct() {
        let mut iter = [0x2A].iter().copied();
        let result: MatchingStruct = deserialize(&mut iter).unwrap();
        assert_eq!(result, MatchingStruct {
            data: 42,
        });
    }

    #[test]
    #[should_panic]
    fn fails_to_deserialize_matching_struct_that_doesnt_match() {
        let mut iter = [0x2B].iter().copied();
        let _: MatchingStruct = deserialize(&mut iter).unwrap();
    }

    #[derive(Grammar, Debug, PartialEq)]
    #[sized]
    struct SizedStruct {
        data: u32,
    }

    #[test]
    fn can_deserialize_sized_struct() {
        let mut iter = [0x04, 0xAA, 0x80, 0x80, 0x00].iter().copied();
        let result: SizedStruct = deserialize(&mut iter).unwrap();
        assert_eq!(result, SizedStruct {
            data: 42,
        });
    }

    #[test]
    #[should_panic]
    fn fails_to_deserialize_sized_struct_with_smaller_size() {
        let mut iter = [0x05, 0xAA, 0x80, 0x80, 0x00, 0x00].iter().copied();
        let _: SizedStruct = deserialize(&mut iter).unwrap();
    }

    #[test]
    #[should_panic]
    fn fails_to_deserialize_sized_struct_with_bigger_size() {
        let mut iter = [0x04, 0xAA, 0x80, 0x80, 0x80, 0x00].iter().copied();
        let _: SizedStruct = deserialize(&mut iter).unwrap();
    }

    #[derive(Grammar, Debug, PartialEq)]
    struct TupleStruct (u32, String);

    #[test]
    fn can_deserialize_tuple_struct() {
        let mut iter = [0x2A, 0x02, 0x34, 0x32, 0x03, 0x01, 0x02, 0x03].iter().copied();
        let result = <TupleStruct as Grammar>::deserialize(&mut iter).unwrap();
        assert_eq!(result, TupleStruct (42, "42".into()));
    }

    #[derive(Grammar, Debug, PartialEq)]
    enum Enum {
        #[discriminant(0x11..=0x21)] V1(u8),
        #[discriminant(0x22)] V2(u8),
    }

    #[test]
    fn can_deserialize_enum() {
        let mut iter = [0x11, 0x03].iter().copied();
        let result: Enum = deserialize(&mut iter).unwrap();
        assert_eq!(result, Enum::V1(0x03));
    }

    #[test]
    #[should_panic]
    fn fails_to_deserialize_enum_with_invalid_discriminant() {
        let mut iter = [0x55, 0x03].iter().copied();
        let _: Enum = deserialize(&mut iter).unwrap();
    }

    #[derive(Grammar, Debug, PartialEq)]
    #[sized]
    enum SizedEnum {
        #[discriminant(0x11)] V1(u32),
        #[discriminant(0x22)] V2(u32),
    }

    #[test]
    fn can_deserialize_sized_enum() {
        let mut iter = [0x11, 0x04, 0xAA, 0x80, 0x80, 0x00].iter().copied();
        let result: SizedEnum = deserialize(&mut iter).unwrap();
        assert_eq!(result, SizedEnum::V1(42));
    }

    #[test]
    #[should_panic]
    fn fails_to_deserialize_sized_enum_with_smaller_size() {
        let mut iter = [0x11, 0x05, 0xAA, 0x80, 0x80, 0x00, 0x00].iter().copied();
        let _: SizedEnum = deserialize(&mut iter).unwrap();
    }

    #[test]
    #[should_panic]
    fn fails_to_deserialize_sized_enum_with_bigger_size() {
        let mut iter = [0x11, 0x04, 0xAA, 0x80, 0x80, 0x80, 0x00].iter().copied();
        let _: SizedEnum = deserialize(&mut iter).unwrap();
    }

    #[derive(Grammar, Debug, PartialEq)]
    enum ForwardingEnum {
        #[discriminant(0x33)] V1(u8),
        #[discriminant(0x44)] V2(u8),
        #[forward] V3(Enum),
    }

    #[test]
    fn can_deserialize_forwarding_enum() {
        let mut iter = [0x22, 0x03].iter().copied();
        let result: ForwardingEnum = deserialize(&mut iter).unwrap();
        assert_eq!(result, ForwardingEnum::V3(Enum::V2(0x03)));
    }

    #[test]
    #[should_panic]
    fn fails_to_deserialize_forwarding_enum_with_invalid_discriminant() {
        let mut iter = [0x55, 0x03].iter().copied();
        let _: ForwardingEnum = deserialize(&mut iter).unwrap();
    }
}