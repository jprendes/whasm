//! This module provides a derive to implement the `Grammar` trait on `struct`s and `enum`s.

/// The `Grammar` derive macro provides a mechanism to declaratively define `Grammar` elements.
/// The macro can be applied to `struct`s and `enum`s.
/// 
/// # Example
/// 
/// ## `struct`
/// 
/// A `struct` (with fields, as well as tuple, and unit `struct`s) can derive the `Grammar` trait:
/// 
/// ```
/// # use whasm::grammar::*;
/// # #[derive(PartialEq, Debug)]
/// #[derive(Grammar)]
/// struct MyStruct {
///     age: u32,
///     name: String,
/// }
/// ```
/// 
/// In this case the `struct` is obtained by deserializing each field in the order in which they
/// are declared in the `struct`. Unit `struct`s do not consume any bytes.
/// 
/// ```
/// # use whasm::grammar::*;
/// # #[derive(PartialEq, Debug)]
/// # #[derive(Grammar)]
/// # struct MyStruct {
/// #     age: u32,
/// #     name: String,
/// # }
/// let mut iter = [0x2A, 0x04, 0x74, 0x65, 0x73, 0x74].iter().copied();
/// let result: MyStruct = deserialize(&mut iter).unwrap();
/// assert_eq!(result, MyStruct { age: 42, name: String::from("test") });
/// ```
/// 
/// ## `enum`
/// 
/// Similarly, an `enum` can derive the `Grammar` trait. In this case, the `enum` variants must be
/// tagged with the `discriminant` attribute to indicate the byte that identifies the variant.
/// 
/// ```
/// # use whasm::grammar::*;
/// # #[derive(PartialEq, Debug)]
/// #[derive(Grammar)]
/// enum MyEnum {
///     #[discriminant(0x01)] UnitVariant,
///     #[discriminant(0x02)] TupleVariant(u32, String),
///     #[discriminant(0x03)] StructVariant {
///         age: u32,
///         name: String,
///     },
/// }
/// ```
/// 
/// In this case the `enum` variant is decided by consuming the first byte in the iterator and
/// matching its value to the different discriminants.
/// Then, the matching variant is deserialized as a `struct`.
/// ```
/// # use whasm::grammar::*;
/// # #[derive(PartialEq, Debug)]
/// # #[derive(Grammar)]
/// # enum MyEnum {
/// #     #[discriminant(0x01)] UnitVariant,
/// #     #[discriminant(0x02)] TupleVariant(u32, String),
/// #     #[discriminant(0x03)] StructVariant {
/// #         age: u32,
/// #         name: String,
/// #     },
/// # }
/// let mut iter = [0x03, 0x2A, 0x04, 0x74, 0x65, 0x73, 0x74].iter().copied();
/// let result: MyEnum = deserialize(&mut iter).unwrap();
/// assert_eq!(result, MyEnum::StructVariant { age: 42, name: String::from("test") });
/// ```
/// 
/// # Attributes
/// The macro provides the following attributes:
/// 
/// ## `sized`
/// This attribute indicates that the grammar object has its size encoded in it.
/// On `struct`s, the fist element of the struct is the size of its content in bytes.
/// On `enum`s, the variant discriminant is followed by the size of its content in bytes.
/// When tagged with this attribute, deserializing the grammar will verify that deserializing the
/// content of the element consumes exactly the expected size.
/// 
/// ### Example
/// 
/// A sized grammar is defined by tagging the `struct` or `enum` with the `sized` attribute.
/// 
/// ```
/// # use whasm::grammar::*;
/// # #[derive(PartialEq, Debug)]
/// #[derive(Grammar)]
/// #[sized]
/// struct SizedStruct {
///     name: String,
/// }
/// ```
/// 
/// On a `struct`, the byte stream must start with an encoded `u32` with the size of the structure.
/// 
/// ```
/// # use whasm::grammar::*;
/// # #[derive(PartialEq, Debug)]
/// # #[derive(Grammar)]
/// # #[sized]
/// # struct SizedStruct {
/// #     name: String,
/// # }
/// let mut iter = [0x05, 0x04, 0x74, 0x65, 0x73, 0x74].iter().copied();
/// let result: SizedStruct = deserialize(&mut iter).unwrap();
/// assert_eq!(result, SizedStruct { name: String::from("test") });
/// ```
/// 
/// Consuming fewer/more bytes than specified results in an deserialization error:
/// 
/// ```
/// # use whasm::grammar::*;
/// # #[derive(PartialEq, Debug)]
/// # #[derive(Grammar)]
/// # #[sized]
/// # struct SizedStruct {
/// #     name: String,
/// # }
/// for n in [4, 6].iter().copied() {
///     let bytes = [n, 0x04, 0x74, 0x65, 0x73, 0x74, 0x00];
///     let mut iter = bytes.iter().copied();
///     let result: Result<SizedStruct> = deserialize(&mut iter);
///     assert!(result.is_err());
/// }
/// ```
/// 
/// ## `matching`
/// This attribute can be applied to fields of a struct. With this attribute it is possible to
/// specify a pattern that the deserialized element must match. Failure to match the pattern
/// implies a deserialization error.
/// 
/// ### Example
/// 
/// A pattern constraint can be imposed on a field by tagging it with the `matching` attribute.
/// 
/// ```
/// # use whasm::grammar::*;
/// # #[derive(PartialEq, Debug)]
/// #[derive(Grammar)]
/// struct MatchingStruct {
///     #[matching(42)]
///     value: u8,
/// }
/// ```
/// 
/// The matching expression follow the same syntax as `match` arms patterns.
/// 
/// ```
/// # use whasm::grammar::*;
/// # #[derive(PartialEq, Debug)]
/// #[derive(Grammar)]
/// struct ComplexMatchingStruct {
///     #[matching(arr if arr == [0x01, 0x02, 0x03, 0x04])]
///     value: [u8; 4],
/// }
/// ```
/// 
/// Deserialization will succeed if the field value matches the specified pattern.
/// 
/// ```
/// # use whasm::grammar::*;
/// # #[derive(PartialEq, Debug)]
/// # #[derive(Grammar)]
/// # struct ComplexMatchingStruct {
/// #     #[matching(arr if arr == [0x01, 0x02, 0x03, 0x04])]
/// #     value: [u8; 4],
/// # }
/// let mut iter = [0x01, 0x02, 0x03, 0x04].iter().copied();
/// let result: ComplexMatchingStruct = deserialize(&mut iter).unwrap();
/// assert_eq!(result, ComplexMatchingStruct { value: [0x01, 0x02, 0x03, 0x04] });
/// ```
/// 
/// If the pattern does not match, the deserialization results in an error.
/// 
/// ```
/// # use whasm::grammar::*;
/// # #[derive(PartialEq, Debug)]
/// # #[derive(Grammar)]
/// # struct MatchingStruct {
/// #     #[matching(42)]
/// #     value: u8,
/// # }
/// let mut iter = [0x6F].iter().copied();
/// let result: Result<MatchingStruct> = deserialize(&mut iter);
/// assert!(result.is_err());
/// ```
/// 
/// ## `discriminant`
/// 
/// This attribute allows the deserializer to distinguish between variants of `enum`s.
/// All `enum` variants must be tagged with a `discriminant` attribute, a `forward` attribute,
/// or both.
/// The attribute takes a pattern that the first byte of the serialized enum must match.
/// 
/// ### Example
/// 
/// The pattern taken by the `discriminant` attribute follows the same syntax as `match` arms for
/// `u8` values (including irefutable patterns).
/// 
/// ```
/// # use whasm::grammar::*;
/// # #[derive(PartialEq, Debug)]
/// #[derive(Grammar)]
/// enum SomeEnum {
///     #[discriminant(0x00..=0x10|0x80)] Variant1,
///     #[discriminant(id if id > 0x60)] Variant2,
/// }
/// ```
/// 
/// The patterns are evaluated in order of occurrence (just like `match` arms).
/// 
/// ```
/// # use whasm::grammar::*;
/// # #[derive(PartialEq, Debug)]
/// # #[derive(Grammar)]
/// # enum SomeEnum {
/// #     #[discriminant(0x00..=0x10|0x80)] Variant1,
/// #     #[discriminant(id if id > 0x60)] Variant2,
/// # }
/// let mut iter = [0x80].iter().copied();
/// let result: SomeEnum = deserialize(&mut iter).unwrap();
/// assert_eq!(result, SomeEnum::Variant1);
/// ```
/// 
/// Deserializing a value that doesn't match any variant results in a deserialization error.
/// 
/// ```
/// # use whasm::grammar::*;
/// # #[derive(PartialEq, Debug)]
/// # #[derive(Grammar)]
/// # enum SomeEnum {
/// #     #[discriminant(0x00..=0x10|0x80)] Variant1,
/// #     #[discriminant(id if id > 0x60)] Variant2,
/// # }
/// let mut iter = [0x20].iter().copied();
/// let result: Result<SomeEnum> = deserialize(&mut iter);
/// assert!(result.is_err());
/// ```
/// 
/// ## `forward`
/// 
/// The forward attribute can be used with variants of `enum`s to indicate that the discriminant
/// byte should not be consumed, but forwarded to the variant.
/// 
/// ### Example
/// 
/// The `forward` attribute can be used alongside the `discriminant` attribute. 
/// 
/// ```
/// # use whasm::grammar::*;
/// # #[derive(PartialEq, Debug)]
/// #[derive(Grammar)]
/// enum ForwardingEnum {
///     #[discriminant(0x00)] Variant1,
///     #[discriminant(0x10..=0x7F)] #[forward] Variant2(Byte),
/// }
/// ```
/// 
/// In this case, the `discriminant` of `ForwardingEnum::Variant2` will not be consumed by the
/// `enum`, and will be captured by the `Byte` in the tuple.
/// 
/// ```
/// # use whasm::grammar::*;
/// # #[derive(PartialEq, Debug)]
/// # #[derive(Grammar)]
/// # enum ForwardingEnum {
/// #     #[discriminant(0x00)] Variant1,
/// #     #[discriminant(0x10..=0x7F)] #[forward] Variant2(Byte),
/// # }
/// let mut iter = [0x2A].iter().copied();
/// let result: ForwardingEnum = deserialize(&mut iter).unwrap();
/// assert_eq!(result, ForwardingEnum::Variant2(Byte(42)));
/// ```
/// 
/// When tagging a variant with the `forward` attribute, the `discriminant` attribute can be
/// omitted. This is the same as tagging the variant with the irrefutable pattern
/// `#[discriminant(_)]`.
/// 
/// ```
/// # use whasm::grammar::*;
/// # #[derive(PartialEq, Debug)]
/// #[derive(Grammar)]
/// enum OuterEnum {
///     #[discriminant(0x00)] Variant1,
///     #[forward] Variant2(InnerEnum),
/// }
/// 
/// # #[derive(PartialEq, Debug)]
/// #[derive(Grammar)]
/// enum InnerEnum {
///     #[discriminant(0x01)] Variant1,
///     #[discriminant(0x02)] Variant2,
/// }
/// ```
/// 
/// In this case, the `discriminant` of `OuterEnum::Variant2` will also be used to identify the
/// variant of `InnerEnum`.
/// 
/// ```
/// # use whasm::grammar::*;
/// # #[derive(PartialEq, Debug)]
/// # #[derive(Grammar)]
/// # enum OuterEnum {
/// #     #[discriminant(0x00)] Variant1,
/// #     #[forward] Variant2(InnerEnum),
/// # }
/// # 
/// # #[derive(PartialEq, Debug)]
/// # #[derive(Grammar)]
/// # enum InnerEnum {
/// #     #[discriminant(0x01)] Variant1,
/// #     #[discriminant(0x02)] Variant2,
/// # }
/// let mut iter = [0x01].iter().copied();
/// let result: OuterEnum = deserialize(&mut iter).unwrap();
/// assert_eq!(result, OuterEnum::Variant2(InnerEnum::Variant1));
/// ```
/// 
/// 
pub use whasm_grammar_derive::Grammar;

#[cfg(test)]
mod test {
    use crate as whasm;
    use whasm::grammar::*;

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
    fn fails_to_deserialize_matching_struct_that_does_not_match() {
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
