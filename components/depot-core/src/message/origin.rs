// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct Origin {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    members: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Origin {}

impl Origin {
    pub fn new() -> Origin {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Origin {
        static mut instance: ::protobuf::lazy::Lazy<Origin> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Origin,
        };
        unsafe {
            instance.get(|| {
                Origin {
                    name: ::protobuf::SingularField::none(),
                    members: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated string members = 2;

    pub fn clear_members(&mut self) {
        self.members.clear();
    }

    // Param is passed by value, moved
    pub fn set_members(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.members = v;
    }

    // Mutable pointer to the field.
    pub fn mut_members(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.members
    }

    // Take field
    pub fn take_members(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.members, ::protobuf::RepeatedField::new())
    }

    pub fn get_members(&self) -> &[::std::string::String] {
        &self.members
    }
}

impl ::protobuf::Message for Origin {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.members));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.members.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        for v in self.members.iter() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Origin>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Origin {
    fn new() -> Origin {
        Origin::new()
    }

    fn descriptor_static(_: ::std::option::Option<Origin>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    Origin::has_name,
                    Origin::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "members",
                    Origin::get_members,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Origin>(
                    "Origin",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Origin {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_members();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Origin {
    fn eq(&self, other: &Origin) -> bool {
        self.name == other.name &&
        self.members == other.members &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Origin {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x16, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x73, 0x2f, 0x6f, 0x72, 0x69, 0x67,
    0x69, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x06, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e,
    0x22, 0x27, 0x0a, 0x06, 0x4f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61,
    0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0f, 0x0a, 0x07, 0x6d, 0x65, 0x6d, 0x62,
    0x65, 0x72, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x4a, 0xb4, 0x01, 0x0a, 0x06, 0x12, 0x04,
    0x00, 0x00, 0x05, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x00, 0x08, 0x0e, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x02, 0x00, 0x05, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x02, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x03, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x03, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x03, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x03, 0x12, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x03, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x04, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x04, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x04, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x04,
    0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x04, 0x1c, 0x1d,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
