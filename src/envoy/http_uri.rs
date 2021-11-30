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
//! Generated file from `envoy/config/core/v3/http_uri.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_25_2;

#[derive(PartialEq,Clone,Default)]
#[cfg_attr(feature = "with-serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct HttpUri {
    // message fields
    pub uri: ::std::string::String,
    pub timeout: ::protobuf::SingularPtrField<::protobuf::well_known_types::Duration>,
    // message oneof groups
    pub http_upstream_type: ::std::option::Option<HttpUri_oneof_http_upstream_type>,
    // special fields
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[cfg_attr(feature = "with-serde", serde(skip))]
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a HttpUri {
    fn default() -> &'a HttpUri {
        <HttpUri as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq,Debug)]
#[cfg_attr(feature = "with-serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub enum HttpUri_oneof_http_upstream_type {
    cluster(::std::string::String),
}

impl HttpUri {
    pub fn new() -> HttpUri {
        ::std::default::Default::default()
    }

    // string uri = 1;


    pub fn get_uri(&self) -> &str {
        &self.uri
    }
    pub fn clear_uri(&mut self) {
        self.uri.clear();
    }

    // Param is passed by value, moved
    pub fn set_uri(&mut self, v: ::std::string::String) {
        self.uri = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uri(&mut self) -> &mut ::std::string::String {
        &mut self.uri
    }

    // Take field
    pub fn take_uri(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.uri, ::std::string::String::new())
    }

    // string cluster = 2;


    pub fn get_cluster(&self) -> &str {
        match self.http_upstream_type {
            ::std::option::Option::Some(HttpUri_oneof_http_upstream_type::cluster(ref v)) => v,
            _ => "",
        }
    }
    pub fn clear_cluster(&mut self) {
        self.http_upstream_type = ::std::option::Option::None;
    }

    pub fn has_cluster(&self) -> bool {
        match self.http_upstream_type {
            ::std::option::Option::Some(HttpUri_oneof_http_upstream_type::cluster(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_cluster(&mut self, v: ::std::string::String) {
        self.http_upstream_type = ::std::option::Option::Some(HttpUri_oneof_http_upstream_type::cluster(v))
    }

    // Mutable pointer to the field.
    pub fn mut_cluster(&mut self) -> &mut ::std::string::String {
        if let ::std::option::Option::Some(HttpUri_oneof_http_upstream_type::cluster(_)) = self.http_upstream_type {
        } else {
            self.http_upstream_type = ::std::option::Option::Some(HttpUri_oneof_http_upstream_type::cluster(::std::string::String::new()));
        }
        match self.http_upstream_type {
            ::std::option::Option::Some(HttpUri_oneof_http_upstream_type::cluster(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_cluster(&mut self) -> ::std::string::String {
        if self.has_cluster() {
            match self.http_upstream_type.take() {
                ::std::option::Option::Some(HttpUri_oneof_http_upstream_type::cluster(v)) => v,
                _ => panic!(),
            }
        } else {
            ::std::string::String::new()
        }
    }

    // .google.protobuf.Duration timeout = 3;


    pub fn get_timeout(&self) -> &::protobuf::well_known_types::Duration {
        self.timeout.as_ref().unwrap_or_else(|| <::protobuf::well_known_types::Duration as ::protobuf::Message>::default_instance())
    }
    pub fn clear_timeout(&mut self) {
        self.timeout.clear();
    }

    pub fn has_timeout(&self) -> bool {
        self.timeout.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timeout(&mut self, v: ::protobuf::well_known_types::Duration) {
        self.timeout = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_timeout(&mut self) -> &mut ::protobuf::well_known_types::Duration {
        if self.timeout.is_none() {
            self.timeout.set_default();
        }
        self.timeout.as_mut().unwrap()
    }

    // Take field
    pub fn take_timeout(&mut self) -> ::protobuf::well_known_types::Duration {
        self.timeout.take().unwrap_or_else(|| ::protobuf::well_known_types::Duration::new())
    }
}

impl ::protobuf::Message for HttpUri {
    fn is_initialized(&self) -> bool {
        for v in &self.timeout {
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
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.uri)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.http_upstream_type = ::std::option::Option::Some(HttpUri_oneof_http_upstream_type::cluster(is.read_string()?));
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.timeout)?;
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
        if !self.uri.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.uri);
        }
        if let Some(ref v) = self.timeout.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let ::std::option::Option::Some(ref v) = self.http_upstream_type {
            match v {
                &HttpUri_oneof_http_upstream_type::cluster(ref v) => {
                    my_size += ::protobuf::rt::string_size(2, &v);
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.uri.is_empty() {
            os.write_string(1, &self.uri)?;
        }
        if let Some(ref v) = self.timeout.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let ::std::option::Option::Some(ref v) = self.http_upstream_type {
            match v {
                &HttpUri_oneof_http_upstream_type::cluster(ref v) => {
                    os.write_string(2, v)?;
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

    fn new() -> HttpUri {
        HttpUri::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "uri",
                |m: &HttpUri| { &m.uri },
                |m: &mut HttpUri| { &mut m.uri },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_string_accessor::<_>(
                "cluster",
                HttpUri::has_cluster,
                HttpUri::get_cluster,
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<::protobuf::well_known_types::Duration>>(
                "timeout",
                |m: &HttpUri| { &m.timeout },
                |m: &mut HttpUri| { &mut m.timeout },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<HttpUri>(
                "HttpUri",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static HttpUri {
        static instance: ::protobuf::rt::LazyV2<HttpUri> = ::protobuf::rt::LazyV2::INIT;
        instance.get(HttpUri::new)
    }
}

impl ::protobuf::Clear for HttpUri {
    fn clear(&mut self) {
        self.uri.clear();
        self.http_upstream_type = ::std::option::Option::None;
        self.timeout.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HttpUri {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HttpUri {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#envoy/config/core/v3/http_uri.proto\x12\x14envoy.config.core.v3\x1a\
    \x1egoogle/protobuf/duration.proto\x1a\x1dudpa/annotations/status.proto\
    \x1a!udpa/annotations/versioning.proto\x1a\x17validate/validate.proto\"\
    \xc7\x01\n\x07HttpUri\x12\x19\n\x03uri\x18\x01\x20\x01(\tR\x03uriB\x07\
    \xfaB\x04r\x02\x10\x01\x12#\n\x07cluster\x18\x02\x20\x01(\tH\0R\x07clust\
    erB\x07\xfaB\x04r\x02\x10\x01\x12?\n\x07timeout\x18\x03\x20\x01(\x0b2\
    \x19.google.protobuf.DurationR\x07timeoutB\n\xfaB\x07\xaa\x01\x04\x08\
    \x012\0B\x19\n\x12http_upstream_type\x12\x03\xf8B\x01:\x20\x9a\xc5\x88\
    \x1e\x1b\n\x19envoy.api.v2.core.HttpUriB<\n\"io.envoyproxy.envoy.config.\
    core.v3B\x0cHttpUriProtoP\x01\xba\x80\xc8\xd1\x06\x02\x10\x02b\x06proto3\
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
