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
//! Generated file from `xds/core/v3/context_params.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(PartialEq,Clone,Default)]
pub struct ContextParams {
    // message fields
    pub params: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ContextParams {
    fn default() -> &'a ContextParams {
        <ContextParams as ::protobuf::Message>::default_instance()
    }
}

impl ContextParams {
    pub fn new() -> ContextParams {
        ::std::default::Default::default()
    }

    // repeated .xds.core.v3.ContextParams.ParamsEntry params = 1;


    pub fn get_params(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.params
    }
    pub fn clear_params(&mut self) {
        self.params.clear();
    }

    // Param is passed by value, moved
    pub fn set_params(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::string::String>) {
        self.params = v;
    }

    // Mutable pointer to the field.
    pub fn mut_params(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.params
    }

    // Take field
    pub fn take_params(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.params, ::std::collections::HashMap::new())
    }
}

impl ::protobuf::Message for ContextParams {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(wire_type, is, &mut self.params)?;
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
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(1, &self.params);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(1, &self.params, os)?;
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

    fn new() -> ContextParams {
        ContextParams::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(
                "params",
                |m: &ContextParams| { &m.params },
                |m: &mut ContextParams| { &mut m.params },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<ContextParams>(
                "ContextParams",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static ContextParams {
        static instance: ::protobuf::rt::LazyV2<ContextParams> = ::protobuf::rt::LazyV2::INIT;
        instance.get(ContextParams::new)
    }
}

impl ::protobuf::Clear for ContextParams {
    fn clear(&mut self) {
        self.params.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ContextParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ContextParams {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x20xds/core/v3/context_params.proto\x12\x0bxds.core.v3\x1a\x1dudpa/an\
    notations/status.proto\"\x8a\x01\n\rContextParams\x12>\n\x06params\x18\
    \x01\x20\x03(\x0b2&.xds.core.v3.ContextParams.ParamsEntryR\x06params\x1a\
    9\n\x0bParamsEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\n\
    \x05value\x18\x02\x20\x01(\tR\x05value:\x028\x01B;\n\x1bcom.github.udpa.\
    xds.core.v3B\x12ContextParamsProtoP\x01\xba\x80\xc8\xd1\x06\x02\x08\x01b\
    \x06proto3\
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