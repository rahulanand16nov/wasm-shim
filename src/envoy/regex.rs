// This file is generated by rust-protobuf 2.25.2. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `envoy/type/matcher/v3/regex.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct RegexMatcher {
    // message fields
    pub regex: ::std::string::String,
    // message oneof groups
    pub engine_type: ::std::option::Option<RegexMatcher_oneof_engine_type>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a RegexMatcher {
    fn default() -> &'a RegexMatcher {
        <RegexMatcher as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
#[cfg_attr(feature = "with-serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum RegexMatcher_oneof_engine_type {
    google_re2(RegexMatcher_GoogleRE2),
}

impl RegexMatcher {
    pub fn new() -> RegexMatcher {
        ::std::default::Default::default()
    }

    // .envoy.type.matcher.v3.RegexMatcher.GoogleRE2 google_re2 = 1;


    pub fn get_google_re2(&self) -> &RegexMatcher_GoogleRE2 {
        match self.engine_type {
            ::std::option::Option::Some(RegexMatcher_oneof_engine_type::google_re2(ref v)) => v,
            _ => <RegexMatcher_GoogleRE2 as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn clear_google_re2(&mut self) {
        self.engine_type = ::std::option::Option::None;
    }

    pub fn has_google_re2(&self) -> bool {
        match self.engine_type {
            ::std::option::Option::Some(RegexMatcher_oneof_engine_type::google_re2(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_google_re2(&mut self, v: RegexMatcher_GoogleRE2) {
        self.engine_type = ::std::option::Option::Some(RegexMatcher_oneof_engine_type::google_re2(v))
    }

    // Mutable pointer to the field.
    pub fn mut_google_re2(&mut self) -> &mut RegexMatcher_GoogleRE2 {
        if let ::std::option::Option::Some(RegexMatcher_oneof_engine_type::google_re2(_)) = self.engine_type {
        } else {
            self.engine_type = ::std::option::Option::Some(RegexMatcher_oneof_engine_type::google_re2(RegexMatcher_GoogleRE2::new()));
        }
        match self.engine_type {
            ::std::option::Option::Some(RegexMatcher_oneof_engine_type::google_re2(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_google_re2(&mut self) -> RegexMatcher_GoogleRE2 {
        if self.has_google_re2() {
            match self.engine_type.take() {
                ::std::option::Option::Some(RegexMatcher_oneof_engine_type::google_re2(v)) => v,
                _ => panic!(),
            }
        } else {
            RegexMatcher_GoogleRE2::new()
        }
    }

    // string regex = 2;


    pub fn get_regex(&self) -> &str {
        &self.regex
    }
    pub fn clear_regex(&mut self) {
        self.regex.clear();
    }

    // Param is passed by value, moved
    pub fn set_regex(&mut self, v: ::std::string::String) {
        self.regex = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_regex(&mut self) -> &mut ::std::string::String {
        &mut self.regex
    }

    // Take field
    pub fn take_regex(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.regex, ::std::string::String::new())
    }
}

impl ::protobuf::Message for RegexMatcher {
    fn is_initialized(&self) -> bool {
        if let Some(RegexMatcher_oneof_engine_type::google_re2(ref v)) = self.engine_type {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.engine_type = ::std::option::Option::Some(RegexMatcher_oneof_engine_type::google_re2(is.read_message()?));
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.regex)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.regex.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.regex);
        }
        if let ::std::option::Option::Some(ref v) = self.engine_type {
            match v {
                &RegexMatcher_oneof_engine_type::google_re2(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.regex.is_empty() {
            os.write_string(2, &self.regex)?;
        }
        if let ::std::option::Option::Some(ref v) = self.engine_type {
            match v {
                &RegexMatcher_oneof_engine_type::google_re2(ref v) => {
                    os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
                    os.write_raw_varint32(v.get_cached_size())?;
                    v.write_to_with_cached_sizes(os)?;
                },
            };
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> RegexMatcher {
        RegexMatcher::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_message_accessor::<_, RegexMatcher_GoogleRE2>(
                "google_re2",
                RegexMatcher::has_google_re2,
                RegexMatcher::get_google_re2,
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "regex",
                |m: &RegexMatcher| { &m.regex },
                |m: &mut RegexMatcher| { &mut m.regex },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<RegexMatcher>(
                "RegexMatcher",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static RegexMatcher {
        static instance: ::protobuf::rt::LazyV2<RegexMatcher> = ::protobuf::rt::LazyV2::INIT;
        instance.get(RegexMatcher::new)
    }
}

impl ::protobuf::Clear for RegexMatcher {
    fn clear(&mut self) {
        self.engine_type = ::std::option::Option::None;
        self.regex.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RegexMatcher {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RegexMatcher {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct RegexMatcher_GoogleRE2 {
    // message fields
    pub max_program_size: ::protobuf::SingularPtrField<::protobuf::well_known_types::UInt32Value>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a RegexMatcher_GoogleRE2 {
    fn default() -> &'a RegexMatcher_GoogleRE2 {
        <RegexMatcher_GoogleRE2 as ::protobuf::Message>::default_instance()
    }
}

impl RegexMatcher_GoogleRE2 {
    pub fn new() -> RegexMatcher_GoogleRE2 {
        ::std::default::Default::default()
    }

    // .google.protobuf.UInt32Value max_program_size = 1;


    pub fn get_max_program_size(&self) -> &::protobuf::well_known_types::UInt32Value {
        self.max_program_size.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::UInt32Value as ::protobuf::Message>::default_instance())
    }
    pub fn clear_max_program_size(&mut self) {
        self.max_program_size.clear();
    }

    pub fn has_max_program_size(&self) -> bool {
        self.max_program_size.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_program_size(&mut self, v: ::protobuf::well_known_types::UInt32Value) {
        self.max_program_size = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_max_program_size(&mut self) -> &mut ::protobuf::well_known_types::UInt32Value {
        if self.max_program_size.is_none() {
            self.max_program_size.set_default();
        }
        self.max_program_size.as_mut().unwrap()
    }

    // Take field
    pub fn take_max_program_size(&mut self) -> ::protobuf::well_known_types::UInt32Value {
        self.max_program_size.take().unwrap_or_else(|| ::protobuf::well_known_types::UInt32Value::new())
    }
}

impl ::protobuf::Message for RegexMatcher_GoogleRE2 {
    fn is_initialized(&self) -> bool {
        for v in &self.max_program_size {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.max_program_size)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.max_program_size.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.max_program_size.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> RegexMatcher_GoogleRE2 {
        RegexMatcher_GoogleRE2::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::UInt32Value>>(
                "max_program_size",
                |m: &RegexMatcher_GoogleRE2| { &m.max_program_size },
                |m: &mut RegexMatcher_GoogleRE2| { &mut m.max_program_size },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<RegexMatcher_GoogleRE2>(
                "RegexMatcher.GoogleRE2",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static RegexMatcher_GoogleRE2 {
        static instance: ::protobuf::rt::LazyV2<RegexMatcher_GoogleRE2> = ::protobuf::rt::LazyV2::INIT;
        instance.get(RegexMatcher_GoogleRE2::new)
    }
}

impl ::protobuf::Clear for RegexMatcher_GoogleRE2 {
    fn clear(&mut self) {
        self.max_program_size.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RegexMatcher_GoogleRE2 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RegexMatcher_GoogleRE2 {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct RegexMatchAndSubstitute {
    // message fields
    pub pattern: ::protobuf::SingularPtrField<RegexMatcher>,
    pub substitution: ::std::string::String,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a RegexMatchAndSubstitute {
    fn default() -> &'a RegexMatchAndSubstitute {
        <RegexMatchAndSubstitute as ::protobuf::Message>::default_instance()
    }
}

impl RegexMatchAndSubstitute {
    pub fn new() -> RegexMatchAndSubstitute {
        ::std::default::Default::default()
    }

    // .envoy.type.matcher.v3.RegexMatcher pattern = 1;


    pub fn get_pattern(&self) -> &RegexMatcher {
        self.pattern.as_ref().unwrap_or_else(|| <RegexMatcher as ::protobuf::Message>::default_instance())
    }
    pub fn clear_pattern(&mut self) {
        self.pattern.clear();
    }

    pub fn has_pattern(&self) -> bool {
        self.pattern.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pattern(&mut self, v: RegexMatcher) {
        self.pattern = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pattern(&mut self) -> &mut RegexMatcher {
        if self.pattern.is_none() {
            self.pattern.set_default();
        }
        self.pattern.as_mut().unwrap()
    }

    // Take field
    pub fn take_pattern(&mut self) -> RegexMatcher {
        self.pattern.take().unwrap_or_else(|| RegexMatcher::new())
    }

    // string substitution = 2;


    pub fn get_substitution(&self) -> &str {
        &self.substitution
    }
    pub fn clear_substitution(&mut self) {
        self.substitution.clear();
    }

    // Param is passed by value, moved
    pub fn set_substitution(&mut self, v: ::std::string::String) {
        self.substitution = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_substitution(&mut self) -> &mut ::std::string::String {
        &mut self.substitution
    }

    // Take field
    pub fn take_substitution(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.substitution, ::std::string::String::new())
    }
}

impl ::protobuf::Message for RegexMatchAndSubstitute {
    fn is_initialized(&self) -> bool {
        for v in &self.pattern {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pattern)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.substitution)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.pattern.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.substitution.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.substitution);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.pattern.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.substitution.is_empty() {
            os.write_string(2, &self.substitution)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> RegexMatchAndSubstitute {
        RegexMatchAndSubstitute::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RegexMatcher>>(
                "pattern",
                |m: &RegexMatchAndSubstitute| { &m.pattern },
                |m: &mut RegexMatchAndSubstitute| { &mut m.pattern },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "substitution",
                |m: &RegexMatchAndSubstitute| { &m.substitution },
                |m: &mut RegexMatchAndSubstitute| { &mut m.substitution },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<RegexMatchAndSubstitute>(
                "RegexMatchAndSubstitute",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static RegexMatchAndSubstitute {
        static instance: ::protobuf::rt::LazyV2<RegexMatchAndSubstitute> = ::protobuf::rt::LazyV2::INIT;
        instance.get(RegexMatchAndSubstitute::new)
    }
}

impl ::protobuf::Clear for RegexMatchAndSubstitute {
    fn clear(&mut self) {
        self.pattern.clear();
        self.substitution.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RegexMatchAndSubstitute {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RegexMatchAndSubstitute {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n!envoy/type/matcher/v3/regex.proto\x12\x15envoy.type.matcher.v3\x1a\
    \x1egoogle/protobuf/wrappers.proto\x1a#envoy/annotations/deprecation.pro\
    to\x1a\x1dudpa/annotations/status.proto\x1a!udpa/annotations/versioning.\
    proto\x1a\x17validate/validate.proto\"\xd8\x02\n\x0cRegexMatcher\x12X\n\
    \ngoogle_re2\x18\x01\x20\x01(\x0b2-.envoy.type.matcher.v3.RegexMatcher.G\
    oogleRE2H\0R\tgoogleRe2B\x08\xfaB\x05\x8a\x01\x02\x10\x01\x12\x1d\n\x05r\
    egex\x18\x02\x20\x01(\tR\x05regexB\x07\xfaB\x04r\x02\x10\x01\x1a\x92\x01\
    \n\tGoogleRE2\x12S\n\x10max_program_size\x18\x01\x20\x01(\x0b2\x1c.googl\
    e.protobuf.UInt32ValueR\x0emaxProgramSizeB\x0b\x18\x01\x92\xc7\x86\xd8\
    \x04\x033.0:0\x9a\xc5\x88\x1e+\n)envoy.type.matcher.RegexMatcher.GoogleR\
    E2B\x12\n\x0bengine_type\x12\x03\xf8B\x01:&\x9a\xc5\x88\x1e!\n\x1fenvoy.\
    type.matcher.RegexMatcher\"\xb9\x01\n\x17RegexMatchAndSubstitute\x12G\n\
    \x07pattern\x18\x01\x20\x01(\x0b2#.envoy.type.matcher.v3.RegexMatcherR\
    \x07patternB\x08\xfaB\x05\x8a\x01\x02\x10\x01\x12\"\n\x0csubstitution\
    \x18\x02\x20\x01(\tR\x0csubstitution:1\x9a\xc5\x88\x1e,\n*envoy.type.mat\
    cher.RegexMatchAndSubstituteB;\n#io.envoyproxy.envoy.type.matcher.v3B\nR\
    egexProtoP\x01\xba\x80\xc8\xd1\x06\x02\x10\x02b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
