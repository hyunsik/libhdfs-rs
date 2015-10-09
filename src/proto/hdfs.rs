// This file is generated. Do not edit
// @generated

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;
use super::Security::TokenProto;
use super::Security::GetDelegationTokenRequestProto;
use super::Security::GetDelegationTokenResponseProto;
use super::Security::RenewDelegationTokenRequestProto;
use super::Security::RenewDelegationTokenResponseProto;
use super::Security::CancelDelegationTokenRequestProto;
use super::Security::CancelDelegationTokenResponseProto;

#[derive(Clone,Default)]
pub struct ExtendedBlockProto {
    // message fields
    poolId: ::protobuf::SingularField<::std::string::String>,
    blockId: ::std::option::Option<u64>,
    generationStamp: ::std::option::Option<u64>,
    numBytes: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ExtendedBlockProto {
    pub fn new() -> ExtendedBlockProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExtendedBlockProto {
        static mut instance: ::protobuf::lazy::Lazy<ExtendedBlockProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExtendedBlockProto,
        };
        unsafe {
            instance.get(|| {
                ExtendedBlockProto {
                    poolId: ::protobuf::SingularField::none(),
                    blockId: ::std::option::Option::None,
                    generationStamp: ::std::option::Option::None,
                    numBytes: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string poolId = 1;

    pub fn clear_poolId(&mut self) {
        self.poolId.clear();
    }

    pub fn has_poolId(&self) -> bool {
        self.poolId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_poolId(&mut self, v: ::std::string::String) {
        self.poolId = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_poolId<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.poolId.is_none() {
            self.poolId.set_default();
        };
        self.poolId.as_mut().unwrap()
    }

    // Take field
    pub fn take_poolId(&mut self) -> ::std::string::String {
        self.poolId.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_poolId<'a>(&'a self) -> &'a str {
        match self.poolId.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required uint64 blockId = 2;

    pub fn clear_blockId(&mut self) {
        self.blockId = ::std::option::Option::None;
    }

    pub fn has_blockId(&self) -> bool {
        self.blockId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockId(&mut self, v: u64) {
        self.blockId = ::std::option::Option::Some(v);
    }

    pub fn get_blockId<'a>(&self) -> u64 {
        self.blockId.unwrap_or(0)
    }

    // required uint64 generationStamp = 3;

    pub fn clear_generationStamp(&mut self) {
        self.generationStamp = ::std::option::Option::None;
    }

    pub fn has_generationStamp(&self) -> bool {
        self.generationStamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_generationStamp(&mut self, v: u64) {
        self.generationStamp = ::std::option::Option::Some(v);
    }

    pub fn get_generationStamp<'a>(&self) -> u64 {
        self.generationStamp.unwrap_or(0)
    }

    // optional uint64 numBytes = 4;

    pub fn clear_numBytes(&mut self) {
        self.numBytes = ::std::option::Option::None;
    }

    pub fn has_numBytes(&self) -> bool {
        self.numBytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_numBytes(&mut self, v: u64) {
        self.numBytes = ::std::option::Option::Some(v);
    }

    pub fn get_numBytes<'a>(&self) -> u64 {
        self.numBytes.unwrap_or(0u64)
    }
}

impl ::protobuf::Message for ExtendedBlockProto {
    fn is_initialized(&self) -> bool {
        if self.poolId.is_none() {
            return false;
        };
        if self.blockId.is_none() {
            return false;
        };
        if self.generationStamp.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.poolId.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.blockId = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.generationStamp = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.numBytes = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.poolId.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.blockId.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.generationStamp.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.numBytes.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.poolId.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.blockId {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.generationStamp {
            try!(os.write_uint64(3, v));
        };
        if let Some(v) = self.numBytes {
            try!(os.write_uint64(4, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ExtendedBlockProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ExtendedBlockProto {
    fn new() -> ExtendedBlockProto {
        ExtendedBlockProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExtendedBlockProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "poolId",
                    ExtendedBlockProto::has_poolId,
                    ExtendedBlockProto::get_poolId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "blockId",
                    ExtendedBlockProto::has_blockId,
                    ExtendedBlockProto::get_blockId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "generationStamp",
                    ExtendedBlockProto::has_generationStamp,
                    ExtendedBlockProto::get_generationStamp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "numBytes",
                    ExtendedBlockProto::has_numBytes,
                    ExtendedBlockProto::get_numBytes,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExtendedBlockProto>(
                    "ExtendedBlockProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExtendedBlockProto {
    fn clear(&mut self) {
        self.clear_poolId();
        self.clear_blockId();
        self.clear_generationStamp();
        self.clear_numBytes();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ExtendedBlockProto {
    fn eq(&self, other: &ExtendedBlockProto) -> bool {
        self.poolId == other.poolId &&
        self.blockId == other.blockId &&
        self.generationStamp == other.generationStamp &&
        self.numBytes == other.numBytes &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ExtendedBlockProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DatanodeIDProto {
    // message fields
    ipAddr: ::protobuf::SingularField<::std::string::String>,
    hostName: ::protobuf::SingularField<::std::string::String>,
    datanodeUuid: ::protobuf::SingularField<::std::string::String>,
    xferPort: ::std::option::Option<u32>,
    infoPort: ::std::option::Option<u32>,
    ipcPort: ::std::option::Option<u32>,
    infoSecurePort: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl DatanodeIDProto {
    pub fn new() -> DatanodeIDProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DatanodeIDProto {
        static mut instance: ::protobuf::lazy::Lazy<DatanodeIDProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DatanodeIDProto,
        };
        unsafe {
            instance.get(|| {
                DatanodeIDProto {
                    ipAddr: ::protobuf::SingularField::none(),
                    hostName: ::protobuf::SingularField::none(),
                    datanodeUuid: ::protobuf::SingularField::none(),
                    xferPort: ::std::option::Option::None,
                    infoPort: ::std::option::Option::None,
                    ipcPort: ::std::option::Option::None,
                    infoSecurePort: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string ipAddr = 1;

    pub fn clear_ipAddr(&mut self) {
        self.ipAddr.clear();
    }

    pub fn has_ipAddr(&self) -> bool {
        self.ipAddr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ipAddr(&mut self, v: ::std::string::String) {
        self.ipAddr = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ipAddr<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.ipAddr.is_none() {
            self.ipAddr.set_default();
        };
        self.ipAddr.as_mut().unwrap()
    }

    // Take field
    pub fn take_ipAddr(&mut self) -> ::std::string::String {
        self.ipAddr.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ipAddr<'a>(&'a self) -> &'a str {
        match self.ipAddr.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string hostName = 2;

    pub fn clear_hostName(&mut self) {
        self.hostName.clear();
    }

    pub fn has_hostName(&self) -> bool {
        self.hostName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hostName(&mut self, v: ::std::string::String) {
        self.hostName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hostName<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.hostName.is_none() {
            self.hostName.set_default();
        };
        self.hostName.as_mut().unwrap()
    }

    // Take field
    pub fn take_hostName(&mut self) -> ::std::string::String {
        self.hostName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_hostName<'a>(&'a self) -> &'a str {
        match self.hostName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string datanodeUuid = 3;

    pub fn clear_datanodeUuid(&mut self) {
        self.datanodeUuid.clear();
    }

    pub fn has_datanodeUuid(&self) -> bool {
        self.datanodeUuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_datanodeUuid(&mut self, v: ::std::string::String) {
        self.datanodeUuid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_datanodeUuid<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.datanodeUuid.is_none() {
            self.datanodeUuid.set_default();
        };
        self.datanodeUuid.as_mut().unwrap()
    }

    // Take field
    pub fn take_datanodeUuid(&mut self) -> ::std::string::String {
        self.datanodeUuid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_datanodeUuid<'a>(&'a self) -> &'a str {
        match self.datanodeUuid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required uint32 xferPort = 4;

    pub fn clear_xferPort(&mut self) {
        self.xferPort = ::std::option::Option::None;
    }

    pub fn has_xferPort(&self) -> bool {
        self.xferPort.is_some()
    }

    // Param is passed by value, moved
    pub fn set_xferPort(&mut self, v: u32) {
        self.xferPort = ::std::option::Option::Some(v);
    }

    pub fn get_xferPort<'a>(&self) -> u32 {
        self.xferPort.unwrap_or(0)
    }

    // required uint32 infoPort = 5;

    pub fn clear_infoPort(&mut self) {
        self.infoPort = ::std::option::Option::None;
    }

    pub fn has_infoPort(&self) -> bool {
        self.infoPort.is_some()
    }

    // Param is passed by value, moved
    pub fn set_infoPort(&mut self, v: u32) {
        self.infoPort = ::std::option::Option::Some(v);
    }

    pub fn get_infoPort<'a>(&self) -> u32 {
        self.infoPort.unwrap_or(0)
    }

    // required uint32 ipcPort = 6;

    pub fn clear_ipcPort(&mut self) {
        self.ipcPort = ::std::option::Option::None;
    }

    pub fn has_ipcPort(&self) -> bool {
        self.ipcPort.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ipcPort(&mut self, v: u32) {
        self.ipcPort = ::std::option::Option::Some(v);
    }

    pub fn get_ipcPort<'a>(&self) -> u32 {
        self.ipcPort.unwrap_or(0)
    }

    // optional uint32 infoSecurePort = 7;

    pub fn clear_infoSecurePort(&mut self) {
        self.infoSecurePort = ::std::option::Option::None;
    }

    pub fn has_infoSecurePort(&self) -> bool {
        self.infoSecurePort.is_some()
    }

    // Param is passed by value, moved
    pub fn set_infoSecurePort(&mut self, v: u32) {
        self.infoSecurePort = ::std::option::Option::Some(v);
    }

    pub fn get_infoSecurePort<'a>(&self) -> u32 {
        self.infoSecurePort.unwrap_or(0u32)
    }
}

impl ::protobuf::Message for DatanodeIDProto {
    fn is_initialized(&self) -> bool {
        if self.ipAddr.is_none() {
            return false;
        };
        if self.hostName.is_none() {
            return false;
        };
        if self.datanodeUuid.is_none() {
            return false;
        };
        if self.xferPort.is_none() {
            return false;
        };
        if self.infoPort.is_none() {
            return false;
        };
        if self.ipcPort.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.ipAddr.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.hostName.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.datanodeUuid.set_default();
                    try!(is.read_string_into(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.xferPort = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.infoPort = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.ipcPort = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.infoSecurePort = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.ipAddr.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.hostName.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.datanodeUuid.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in self.xferPort.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.infoPort.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.ipcPort.iter() {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.infoSecurePort.iter() {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ipAddr.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.hostName.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.datanodeUuid.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.xferPort {
            try!(os.write_uint32(4, v));
        };
        if let Some(v) = self.infoPort {
            try!(os.write_uint32(5, v));
        };
        if let Some(v) = self.ipcPort {
            try!(os.write_uint32(6, v));
        };
        if let Some(v) = self.infoSecurePort {
            try!(os.write_uint32(7, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<DatanodeIDProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DatanodeIDProto {
    fn new() -> DatanodeIDProto {
        DatanodeIDProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DatanodeIDProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "ipAddr",
                    DatanodeIDProto::has_ipAddr,
                    DatanodeIDProto::get_ipAddr,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "hostName",
                    DatanodeIDProto::has_hostName,
                    DatanodeIDProto::get_hostName,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "datanodeUuid",
                    DatanodeIDProto::has_datanodeUuid,
                    DatanodeIDProto::get_datanodeUuid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "xferPort",
                    DatanodeIDProto::has_xferPort,
                    DatanodeIDProto::get_xferPort,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "infoPort",
                    DatanodeIDProto::has_infoPort,
                    DatanodeIDProto::get_infoPort,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "ipcPort",
                    DatanodeIDProto::has_ipcPort,
                    DatanodeIDProto::get_ipcPort,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "infoSecurePort",
                    DatanodeIDProto::has_infoSecurePort,
                    DatanodeIDProto::get_infoSecurePort,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DatanodeIDProto>(
                    "DatanodeIDProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DatanodeIDProto {
    fn clear(&mut self) {
        self.clear_ipAddr();
        self.clear_hostName();
        self.clear_datanodeUuid();
        self.clear_xferPort();
        self.clear_infoPort();
        self.clear_ipcPort();
        self.clear_infoSecurePort();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DatanodeIDProto {
    fn eq(&self, other: &DatanodeIDProto) -> bool {
        self.ipAddr == other.ipAddr &&
        self.hostName == other.hostName &&
        self.datanodeUuid == other.datanodeUuid &&
        self.xferPort == other.xferPort &&
        self.infoPort == other.infoPort &&
        self.ipcPort == other.ipcPort &&
        self.infoSecurePort == other.infoSecurePort &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DatanodeIDProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DatanodeLocalInfoProto {
    // message fields
    softwareVersion: ::protobuf::SingularField<::std::string::String>,
    configVersion: ::protobuf::SingularField<::std::string::String>,
    uptime: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl DatanodeLocalInfoProto {
    pub fn new() -> DatanodeLocalInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DatanodeLocalInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<DatanodeLocalInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DatanodeLocalInfoProto,
        };
        unsafe {
            instance.get(|| {
                DatanodeLocalInfoProto {
                    softwareVersion: ::protobuf::SingularField::none(),
                    configVersion: ::protobuf::SingularField::none(),
                    uptime: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string softwareVersion = 1;

    pub fn clear_softwareVersion(&mut self) {
        self.softwareVersion.clear();
    }

    pub fn has_softwareVersion(&self) -> bool {
        self.softwareVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_softwareVersion(&mut self, v: ::std::string::String) {
        self.softwareVersion = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_softwareVersion<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.softwareVersion.is_none() {
            self.softwareVersion.set_default();
        };
        self.softwareVersion.as_mut().unwrap()
    }

    // Take field
    pub fn take_softwareVersion(&mut self) -> ::std::string::String {
        self.softwareVersion.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_softwareVersion<'a>(&'a self) -> &'a str {
        match self.softwareVersion.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string configVersion = 2;

    pub fn clear_configVersion(&mut self) {
        self.configVersion.clear();
    }

    pub fn has_configVersion(&self) -> bool {
        self.configVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_configVersion(&mut self, v: ::std::string::String) {
        self.configVersion = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_configVersion<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.configVersion.is_none() {
            self.configVersion.set_default();
        };
        self.configVersion.as_mut().unwrap()
    }

    // Take field
    pub fn take_configVersion(&mut self) -> ::std::string::String {
        self.configVersion.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_configVersion<'a>(&'a self) -> &'a str {
        match self.configVersion.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required uint64 uptime = 3;

    pub fn clear_uptime(&mut self) {
        self.uptime = ::std::option::Option::None;
    }

    pub fn has_uptime(&self) -> bool {
        self.uptime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uptime(&mut self, v: u64) {
        self.uptime = ::std::option::Option::Some(v);
    }

    pub fn get_uptime<'a>(&self) -> u64 {
        self.uptime.unwrap_or(0)
    }
}

impl ::protobuf::Message for DatanodeLocalInfoProto {
    fn is_initialized(&self) -> bool {
        if self.softwareVersion.is_none() {
            return false;
        };
        if self.configVersion.is_none() {
            return false;
        };
        if self.uptime.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.softwareVersion.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.configVersion.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.uptime = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.softwareVersion.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.configVersion.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.uptime.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.softwareVersion.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.configVersion.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.uptime {
            try!(os.write_uint64(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<DatanodeLocalInfoProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DatanodeLocalInfoProto {
    fn new() -> DatanodeLocalInfoProto {
        DatanodeLocalInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DatanodeLocalInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "softwareVersion",
                    DatanodeLocalInfoProto::has_softwareVersion,
                    DatanodeLocalInfoProto::get_softwareVersion,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "configVersion",
                    DatanodeLocalInfoProto::has_configVersion,
                    DatanodeLocalInfoProto::get_configVersion,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "uptime",
                    DatanodeLocalInfoProto::has_uptime,
                    DatanodeLocalInfoProto::get_uptime,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DatanodeLocalInfoProto>(
                    "DatanodeLocalInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DatanodeLocalInfoProto {
    fn clear(&mut self) {
        self.clear_softwareVersion();
        self.clear_configVersion();
        self.clear_uptime();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DatanodeLocalInfoProto {
    fn eq(&self, other: &DatanodeLocalInfoProto) -> bool {
        self.softwareVersion == other.softwareVersion &&
        self.configVersion == other.configVersion &&
        self.uptime == other.uptime &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DatanodeLocalInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DatanodeInfosProto {
    // message fields
    datanodes: ::protobuf::RepeatedField<DatanodeInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl DatanodeInfosProto {
    pub fn new() -> DatanodeInfosProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DatanodeInfosProto {
        static mut instance: ::protobuf::lazy::Lazy<DatanodeInfosProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DatanodeInfosProto,
        };
        unsafe {
            instance.get(|| {
                DatanodeInfosProto {
                    datanodes: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .hadoop.hdfs.DatanodeInfoProto datanodes = 1;

    pub fn clear_datanodes(&mut self) {
        self.datanodes.clear();
    }

    // Param is passed by value, moved
    pub fn set_datanodes(&mut self, v: ::protobuf::RepeatedField<DatanodeInfoProto>) {
        self.datanodes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_datanodes<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<DatanodeInfoProto> {
        &mut self.datanodes
    }

    // Take field
    pub fn take_datanodes(&mut self) -> ::protobuf::RepeatedField<DatanodeInfoProto> {
        ::std::mem::replace(&mut self.datanodes, ::protobuf::RepeatedField::new())
    }

    pub fn get_datanodes<'a>(&'a self) -> &'a [DatanodeInfoProto] {
        &self.datanodes
    }
}

impl ::protobuf::Message for DatanodeInfosProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.datanodes));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.datanodes.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.datanodes.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<DatanodeInfosProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DatanodeInfosProto {
    fn new() -> DatanodeInfosProto {
        DatanodeInfosProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DatanodeInfosProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "datanodes",
                    DatanodeInfosProto::get_datanodes,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DatanodeInfosProto>(
                    "DatanodeInfosProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DatanodeInfosProto {
    fn clear(&mut self) {
        self.clear_datanodes();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DatanodeInfosProto {
    fn eq(&self, other: &DatanodeInfosProto) -> bool {
        self.datanodes == other.datanodes &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DatanodeInfosProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DatanodeInfoProto {
    // message fields
    id: ::protobuf::SingularPtrField<DatanodeIDProto>,
    capacity: ::std::option::Option<u64>,
    dfsUsed: ::std::option::Option<u64>,
    remaining: ::std::option::Option<u64>,
    blockPoolUsed: ::std::option::Option<u64>,
    lastUpdate: ::std::option::Option<u64>,
    xceiverCount: ::std::option::Option<u32>,
    location: ::protobuf::SingularField<::std::string::String>,
    adminState: ::std::option::Option<DatanodeInfoProto_AdminState>,
    cacheCapacity: ::std::option::Option<u64>,
    cacheUsed: ::std::option::Option<u64>,
    lastUpdateMonotonic: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl DatanodeInfoProto {
    pub fn new() -> DatanodeInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DatanodeInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<DatanodeInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DatanodeInfoProto,
        };
        unsafe {
            instance.get(|| {
                DatanodeInfoProto {
                    id: ::protobuf::SingularPtrField::none(),
                    capacity: ::std::option::Option::None,
                    dfsUsed: ::std::option::Option::None,
                    remaining: ::std::option::Option::None,
                    blockPoolUsed: ::std::option::Option::None,
                    lastUpdate: ::std::option::Option::None,
                    xceiverCount: ::std::option::Option::None,
                    location: ::protobuf::SingularField::none(),
                    adminState: ::std::option::Option::None,
                    cacheCapacity: ::std::option::Option::None,
                    cacheUsed: ::std::option::Option::None,
                    lastUpdateMonotonic: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.DatanodeIDProto id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: DatanodeIDProto) {
        self.id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id<'a>(&'a mut self) -> &'a mut DatanodeIDProto {
        if self.id.is_none() {
            self.id.set_default();
        };
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> DatanodeIDProto {
        self.id.take().unwrap_or_else(|| DatanodeIDProto::new())
    }

    pub fn get_id<'a>(&'a self) -> &'a DatanodeIDProto {
        self.id.as_ref().unwrap_or_else(|| DatanodeIDProto::default_instance())
    }

    // optional uint64 capacity = 2;

    pub fn clear_capacity(&mut self) {
        self.capacity = ::std::option::Option::None;
    }

    pub fn has_capacity(&self) -> bool {
        self.capacity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_capacity(&mut self, v: u64) {
        self.capacity = ::std::option::Option::Some(v);
    }

    pub fn get_capacity<'a>(&self) -> u64 {
        self.capacity.unwrap_or(0u64)
    }

    // optional uint64 dfsUsed = 3;

    pub fn clear_dfsUsed(&mut self) {
        self.dfsUsed = ::std::option::Option::None;
    }

    pub fn has_dfsUsed(&self) -> bool {
        self.dfsUsed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dfsUsed(&mut self, v: u64) {
        self.dfsUsed = ::std::option::Option::Some(v);
    }

    pub fn get_dfsUsed<'a>(&self) -> u64 {
        self.dfsUsed.unwrap_or(0u64)
    }

    // optional uint64 remaining = 4;

    pub fn clear_remaining(&mut self) {
        self.remaining = ::std::option::Option::None;
    }

    pub fn has_remaining(&self) -> bool {
        self.remaining.is_some()
    }

    // Param is passed by value, moved
    pub fn set_remaining(&mut self, v: u64) {
        self.remaining = ::std::option::Option::Some(v);
    }

    pub fn get_remaining<'a>(&self) -> u64 {
        self.remaining.unwrap_or(0u64)
    }

    // optional uint64 blockPoolUsed = 5;

    pub fn clear_blockPoolUsed(&mut self) {
        self.blockPoolUsed = ::std::option::Option::None;
    }

    pub fn has_blockPoolUsed(&self) -> bool {
        self.blockPoolUsed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockPoolUsed(&mut self, v: u64) {
        self.blockPoolUsed = ::std::option::Option::Some(v);
    }

    pub fn get_blockPoolUsed<'a>(&self) -> u64 {
        self.blockPoolUsed.unwrap_or(0u64)
    }

    // optional uint64 lastUpdate = 6;

    pub fn clear_lastUpdate(&mut self) {
        self.lastUpdate = ::std::option::Option::None;
    }

    pub fn has_lastUpdate(&self) -> bool {
        self.lastUpdate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastUpdate(&mut self, v: u64) {
        self.lastUpdate = ::std::option::Option::Some(v);
    }

    pub fn get_lastUpdate<'a>(&self) -> u64 {
        self.lastUpdate.unwrap_or(0u64)
    }

    // optional uint32 xceiverCount = 7;

    pub fn clear_xceiverCount(&mut self) {
        self.xceiverCount = ::std::option::Option::None;
    }

    pub fn has_xceiverCount(&self) -> bool {
        self.xceiverCount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_xceiverCount(&mut self, v: u32) {
        self.xceiverCount = ::std::option::Option::Some(v);
    }

    pub fn get_xceiverCount<'a>(&self) -> u32 {
        self.xceiverCount.unwrap_or(0u32)
    }

    // optional string location = 8;

    pub fn clear_location(&mut self) {
        self.location.clear();
    }

    pub fn has_location(&self) -> bool {
        self.location.is_some()
    }

    // Param is passed by value, moved
    pub fn set_location(&mut self, v: ::std::string::String) {
        self.location = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_location<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.location.is_none() {
            self.location.set_default();
        };
        self.location.as_mut().unwrap()
    }

    // Take field
    pub fn take_location(&mut self) -> ::std::string::String {
        self.location.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_location<'a>(&'a self) -> &'a str {
        match self.location.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .hadoop.hdfs.DatanodeInfoProto.AdminState adminState = 10;

    pub fn clear_adminState(&mut self) {
        self.adminState = ::std::option::Option::None;
    }

    pub fn has_adminState(&self) -> bool {
        self.adminState.is_some()
    }

    // Param is passed by value, moved
    pub fn set_adminState(&mut self, v: DatanodeInfoProto_AdminState) {
        self.adminState = ::std::option::Option::Some(v);
    }

    pub fn get_adminState<'a>(&self) -> DatanodeInfoProto_AdminState {
        self.adminState.unwrap_or(DatanodeInfoProto_AdminState::NORMAL)
    }

    // optional uint64 cacheCapacity = 11;

    pub fn clear_cacheCapacity(&mut self) {
        self.cacheCapacity = ::std::option::Option::None;
    }

    pub fn has_cacheCapacity(&self) -> bool {
        self.cacheCapacity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cacheCapacity(&mut self, v: u64) {
        self.cacheCapacity = ::std::option::Option::Some(v);
    }

    pub fn get_cacheCapacity<'a>(&self) -> u64 {
        self.cacheCapacity.unwrap_or(0u64)
    }

    // optional uint64 cacheUsed = 12;

    pub fn clear_cacheUsed(&mut self) {
        self.cacheUsed = ::std::option::Option::None;
    }

    pub fn has_cacheUsed(&self) -> bool {
        self.cacheUsed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cacheUsed(&mut self, v: u64) {
        self.cacheUsed = ::std::option::Option::Some(v);
    }

    pub fn get_cacheUsed<'a>(&self) -> u64 {
        self.cacheUsed.unwrap_or(0u64)
    }

    // optional uint64 lastUpdateMonotonic = 13;

    pub fn clear_lastUpdateMonotonic(&mut self) {
        self.lastUpdateMonotonic = ::std::option::Option::None;
    }

    pub fn has_lastUpdateMonotonic(&self) -> bool {
        self.lastUpdateMonotonic.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastUpdateMonotonic(&mut self, v: u64) {
        self.lastUpdateMonotonic = ::std::option::Option::Some(v);
    }

    pub fn get_lastUpdateMonotonic<'a>(&self) -> u64 {
        self.lastUpdateMonotonic.unwrap_or(0u64)
    }
}

impl ::protobuf::Message for DatanodeInfoProto {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.id.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.capacity = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.dfsUsed = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.remaining = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.blockPoolUsed = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.lastUpdate = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.xceiverCount = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.location.set_default();
                    try!(is.read_string_into(tmp))
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.adminState = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.cacheCapacity = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.cacheUsed = ::std::option::Option::Some(tmp);
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.lastUpdateMonotonic = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.capacity.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.dfsUsed.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.remaining.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.blockPoolUsed.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.lastUpdate.iter() {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.xceiverCount.iter() {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.location.iter() {
            my_size += ::protobuf::rt::string_size(8, &value);
        };
        for value in self.adminState.iter() {
            my_size += ::protobuf::rt::enum_size(10, *value);
        };
        for value in self.cacheCapacity.iter() {
            my_size += ::protobuf::rt::value_size(11, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.cacheUsed.iter() {
            my_size += ::protobuf::rt::value_size(12, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.lastUpdateMonotonic.iter() {
            my_size += ::protobuf::rt::value_size(13, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.capacity {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.dfsUsed {
            try!(os.write_uint64(3, v));
        };
        if let Some(v) = self.remaining {
            try!(os.write_uint64(4, v));
        };
        if let Some(v) = self.blockPoolUsed {
            try!(os.write_uint64(5, v));
        };
        if let Some(v) = self.lastUpdate {
            try!(os.write_uint64(6, v));
        };
        if let Some(v) = self.xceiverCount {
            try!(os.write_uint32(7, v));
        };
        if let Some(v) = self.location.as_ref() {
            try!(os.write_string(8, &v));
        };
        if let Some(v) = self.adminState {
            try!(os.write_enum(10, v as i32));
        };
        if let Some(v) = self.cacheCapacity {
            try!(os.write_uint64(11, v));
        };
        if let Some(v) = self.cacheUsed {
            try!(os.write_uint64(12, v));
        };
        if let Some(v) = self.lastUpdateMonotonic {
            try!(os.write_uint64(13, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<DatanodeInfoProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DatanodeInfoProto {
    fn new() -> DatanodeInfoProto {
        DatanodeInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DatanodeInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "id",
                    DatanodeInfoProto::has_id,
                    DatanodeInfoProto::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "capacity",
                    DatanodeInfoProto::has_capacity,
                    DatanodeInfoProto::get_capacity,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "dfsUsed",
                    DatanodeInfoProto::has_dfsUsed,
                    DatanodeInfoProto::get_dfsUsed,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "remaining",
                    DatanodeInfoProto::has_remaining,
                    DatanodeInfoProto::get_remaining,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "blockPoolUsed",
                    DatanodeInfoProto::has_blockPoolUsed,
                    DatanodeInfoProto::get_blockPoolUsed,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "lastUpdate",
                    DatanodeInfoProto::has_lastUpdate,
                    DatanodeInfoProto::get_lastUpdate,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "xceiverCount",
                    DatanodeInfoProto::has_xceiverCount,
                    DatanodeInfoProto::get_xceiverCount,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "location",
                    DatanodeInfoProto::has_location,
                    DatanodeInfoProto::get_location,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "adminState",
                    DatanodeInfoProto::has_adminState,
                    DatanodeInfoProto::get_adminState,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "cacheCapacity",
                    DatanodeInfoProto::has_cacheCapacity,
                    DatanodeInfoProto::get_cacheCapacity,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "cacheUsed",
                    DatanodeInfoProto::has_cacheUsed,
                    DatanodeInfoProto::get_cacheUsed,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "lastUpdateMonotonic",
                    DatanodeInfoProto::has_lastUpdateMonotonic,
                    DatanodeInfoProto::get_lastUpdateMonotonic,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DatanodeInfoProto>(
                    "DatanodeInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DatanodeInfoProto {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_capacity();
        self.clear_dfsUsed();
        self.clear_remaining();
        self.clear_blockPoolUsed();
        self.clear_lastUpdate();
        self.clear_xceiverCount();
        self.clear_location();
        self.clear_adminState();
        self.clear_cacheCapacity();
        self.clear_cacheUsed();
        self.clear_lastUpdateMonotonic();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DatanodeInfoProto {
    fn eq(&self, other: &DatanodeInfoProto) -> bool {
        self.id == other.id &&
        self.capacity == other.capacity &&
        self.dfsUsed == other.dfsUsed &&
        self.remaining == other.remaining &&
        self.blockPoolUsed == other.blockPoolUsed &&
        self.lastUpdate == other.lastUpdate &&
        self.xceiverCount == other.xceiverCount &&
        self.location == other.location &&
        self.adminState == other.adminState &&
        self.cacheCapacity == other.cacheCapacity &&
        self.cacheUsed == other.cacheUsed &&
        self.lastUpdateMonotonic == other.lastUpdateMonotonic &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DatanodeInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DatanodeInfoProto_AdminState {
    NORMAL = 0,
    DECOMMISSION_INPROGRESS = 1,
    DECOMMISSIONED = 2,
}

impl ::protobuf::ProtobufEnum for DatanodeInfoProto_AdminState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DatanodeInfoProto_AdminState> {
        match value {
            0 => ::std::option::Option::Some(DatanodeInfoProto_AdminState::NORMAL),
            1 => ::std::option::Option::Some(DatanodeInfoProto_AdminState::DECOMMISSION_INPROGRESS),
            2 => ::std::option::Option::Some(DatanodeInfoProto_AdminState::DECOMMISSIONED),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<DatanodeInfoProto_AdminState>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DatanodeInfoProto_AdminState", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DatanodeInfoProto_AdminState {
}

#[derive(Clone,Default)]
pub struct DatanodeStorageProto {
    // message fields
    storageUuid: ::protobuf::SingularField<::std::string::String>,
    state: ::std::option::Option<DatanodeStorageProto_StorageState>,
    storageType: ::std::option::Option<StorageTypeProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl DatanodeStorageProto {
    pub fn new() -> DatanodeStorageProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DatanodeStorageProto {
        static mut instance: ::protobuf::lazy::Lazy<DatanodeStorageProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DatanodeStorageProto,
        };
        unsafe {
            instance.get(|| {
                DatanodeStorageProto {
                    storageUuid: ::protobuf::SingularField::none(),
                    state: ::std::option::Option::None,
                    storageType: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string storageUuid = 1;

    pub fn clear_storageUuid(&mut self) {
        self.storageUuid.clear();
    }

    pub fn has_storageUuid(&self) -> bool {
        self.storageUuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storageUuid(&mut self, v: ::std::string::String) {
        self.storageUuid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_storageUuid<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.storageUuid.is_none() {
            self.storageUuid.set_default();
        };
        self.storageUuid.as_mut().unwrap()
    }

    // Take field
    pub fn take_storageUuid(&mut self) -> ::std::string::String {
        self.storageUuid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_storageUuid<'a>(&'a self) -> &'a str {
        match self.storageUuid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .hadoop.hdfs.DatanodeStorageProto.StorageState state = 2;

    pub fn clear_state(&mut self) {
        self.state = ::std::option::Option::None;
    }

    pub fn has_state(&self) -> bool {
        self.state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: DatanodeStorageProto_StorageState) {
        self.state = ::std::option::Option::Some(v);
    }

    pub fn get_state<'a>(&self) -> DatanodeStorageProto_StorageState {
        self.state.unwrap_or(DatanodeStorageProto_StorageState::NORMAL)
    }

    // optional .hadoop.hdfs.StorageTypeProto storageType = 3;

    pub fn clear_storageType(&mut self) {
        self.storageType = ::std::option::Option::None;
    }

    pub fn has_storageType(&self) -> bool {
        self.storageType.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storageType(&mut self, v: StorageTypeProto) {
        self.storageType = ::std::option::Option::Some(v);
    }

    pub fn get_storageType<'a>(&self) -> StorageTypeProto {
        self.storageType.unwrap_or(StorageTypeProto::DISK)
    }
}

impl ::protobuf::Message for DatanodeStorageProto {
    fn is_initialized(&self) -> bool {
        if self.storageUuid.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.storageUuid.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.state = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.storageType = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.storageUuid.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.state.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in self.storageType.iter() {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.storageUuid.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.state {
            try!(os.write_enum(2, v as i32));
        };
        if let Some(v) = self.storageType {
            try!(os.write_enum(3, v as i32));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<DatanodeStorageProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DatanodeStorageProto {
    fn new() -> DatanodeStorageProto {
        DatanodeStorageProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DatanodeStorageProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "storageUuid",
                    DatanodeStorageProto::has_storageUuid,
                    DatanodeStorageProto::get_storageUuid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "state",
                    DatanodeStorageProto::has_state,
                    DatanodeStorageProto::get_state,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "storageType",
                    DatanodeStorageProto::has_storageType,
                    DatanodeStorageProto::get_storageType,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DatanodeStorageProto>(
                    "DatanodeStorageProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DatanodeStorageProto {
    fn clear(&mut self) {
        self.clear_storageUuid();
        self.clear_state();
        self.clear_storageType();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DatanodeStorageProto {
    fn eq(&self, other: &DatanodeStorageProto) -> bool {
        self.storageUuid == other.storageUuid &&
        self.state == other.state &&
        self.storageType == other.storageType &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DatanodeStorageProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DatanodeStorageProto_StorageState {
    NORMAL = 0,
    READ_ONLY_SHARED = 1,
}

impl ::protobuf::ProtobufEnum for DatanodeStorageProto_StorageState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DatanodeStorageProto_StorageState> {
        match value {
            0 => ::std::option::Option::Some(DatanodeStorageProto_StorageState::NORMAL),
            1 => ::std::option::Option::Some(DatanodeStorageProto_StorageState::READ_ONLY_SHARED),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<DatanodeStorageProto_StorageState>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DatanodeStorageProto_StorageState", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DatanodeStorageProto_StorageState {
}

#[derive(Clone,Default)]
pub struct StorageReportProto {
    // message fields
    storageUuid: ::protobuf::SingularField<::std::string::String>,
    failed: ::std::option::Option<bool>,
    capacity: ::std::option::Option<u64>,
    dfsUsed: ::std::option::Option<u64>,
    remaining: ::std::option::Option<u64>,
    blockPoolUsed: ::std::option::Option<u64>,
    storage: ::protobuf::SingularPtrField<DatanodeStorageProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl StorageReportProto {
    pub fn new() -> StorageReportProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StorageReportProto {
        static mut instance: ::protobuf::lazy::Lazy<StorageReportProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StorageReportProto,
        };
        unsafe {
            instance.get(|| {
                StorageReportProto {
                    storageUuid: ::protobuf::SingularField::none(),
                    failed: ::std::option::Option::None,
                    capacity: ::std::option::Option::None,
                    dfsUsed: ::std::option::Option::None,
                    remaining: ::std::option::Option::None,
                    blockPoolUsed: ::std::option::Option::None,
                    storage: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string storageUuid = 1;

    pub fn clear_storageUuid(&mut self) {
        self.storageUuid.clear();
    }

    pub fn has_storageUuid(&self) -> bool {
        self.storageUuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storageUuid(&mut self, v: ::std::string::String) {
        self.storageUuid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_storageUuid<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.storageUuid.is_none() {
            self.storageUuid.set_default();
        };
        self.storageUuid.as_mut().unwrap()
    }

    // Take field
    pub fn take_storageUuid(&mut self) -> ::std::string::String {
        self.storageUuid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_storageUuid<'a>(&'a self) -> &'a str {
        match self.storageUuid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bool failed = 2;

    pub fn clear_failed(&mut self) {
        self.failed = ::std::option::Option::None;
    }

    pub fn has_failed(&self) -> bool {
        self.failed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_failed(&mut self, v: bool) {
        self.failed = ::std::option::Option::Some(v);
    }

    pub fn get_failed<'a>(&self) -> bool {
        self.failed.unwrap_or(false)
    }

    // optional uint64 capacity = 3;

    pub fn clear_capacity(&mut self) {
        self.capacity = ::std::option::Option::None;
    }

    pub fn has_capacity(&self) -> bool {
        self.capacity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_capacity(&mut self, v: u64) {
        self.capacity = ::std::option::Option::Some(v);
    }

    pub fn get_capacity<'a>(&self) -> u64 {
        self.capacity.unwrap_or(0u64)
    }

    // optional uint64 dfsUsed = 4;

    pub fn clear_dfsUsed(&mut self) {
        self.dfsUsed = ::std::option::Option::None;
    }

    pub fn has_dfsUsed(&self) -> bool {
        self.dfsUsed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dfsUsed(&mut self, v: u64) {
        self.dfsUsed = ::std::option::Option::Some(v);
    }

    pub fn get_dfsUsed<'a>(&self) -> u64 {
        self.dfsUsed.unwrap_or(0u64)
    }

    // optional uint64 remaining = 5;

    pub fn clear_remaining(&mut self) {
        self.remaining = ::std::option::Option::None;
    }

    pub fn has_remaining(&self) -> bool {
        self.remaining.is_some()
    }

    // Param is passed by value, moved
    pub fn set_remaining(&mut self, v: u64) {
        self.remaining = ::std::option::Option::Some(v);
    }

    pub fn get_remaining<'a>(&self) -> u64 {
        self.remaining.unwrap_or(0u64)
    }

    // optional uint64 blockPoolUsed = 6;

    pub fn clear_blockPoolUsed(&mut self) {
        self.blockPoolUsed = ::std::option::Option::None;
    }

    pub fn has_blockPoolUsed(&self) -> bool {
        self.blockPoolUsed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockPoolUsed(&mut self, v: u64) {
        self.blockPoolUsed = ::std::option::Option::Some(v);
    }

    pub fn get_blockPoolUsed<'a>(&self) -> u64 {
        self.blockPoolUsed.unwrap_or(0u64)
    }

    // optional .hadoop.hdfs.DatanodeStorageProto storage = 7;

    pub fn clear_storage(&mut self) {
        self.storage.clear();
    }

    pub fn has_storage(&self) -> bool {
        self.storage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storage(&mut self, v: DatanodeStorageProto) {
        self.storage = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_storage<'a>(&'a mut self) -> &'a mut DatanodeStorageProto {
        if self.storage.is_none() {
            self.storage.set_default();
        };
        self.storage.as_mut().unwrap()
    }

    // Take field
    pub fn take_storage(&mut self) -> DatanodeStorageProto {
        self.storage.take().unwrap_or_else(|| DatanodeStorageProto::new())
    }

    pub fn get_storage<'a>(&'a self) -> &'a DatanodeStorageProto {
        self.storage.as_ref().unwrap_or_else(|| DatanodeStorageProto::default_instance())
    }
}

impl ::protobuf::Message for StorageReportProto {
    fn is_initialized(&self) -> bool {
        if self.storageUuid.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.storageUuid.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.failed = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.capacity = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.dfsUsed = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.remaining = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.blockPoolUsed = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.storage.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.storageUuid.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if self.failed.is_some() {
            my_size += 2;
        };
        for value in self.capacity.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.dfsUsed.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.remaining.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.blockPoolUsed.iter() {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.storage.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.storageUuid.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.failed {
            try!(os.write_bool(2, v));
        };
        if let Some(v) = self.capacity {
            try!(os.write_uint64(3, v));
        };
        if let Some(v) = self.dfsUsed {
            try!(os.write_uint64(4, v));
        };
        if let Some(v) = self.remaining {
            try!(os.write_uint64(5, v));
        };
        if let Some(v) = self.blockPoolUsed {
            try!(os.write_uint64(6, v));
        };
        if let Some(v) = self.storage.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StorageReportProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StorageReportProto {
    fn new() -> StorageReportProto {
        StorageReportProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<StorageReportProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "storageUuid",
                    StorageReportProto::has_storageUuid,
                    StorageReportProto::get_storageUuid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "failed",
                    StorageReportProto::has_failed,
                    StorageReportProto::get_failed,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "capacity",
                    StorageReportProto::has_capacity,
                    StorageReportProto::get_capacity,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "dfsUsed",
                    StorageReportProto::has_dfsUsed,
                    StorageReportProto::get_dfsUsed,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "remaining",
                    StorageReportProto::has_remaining,
                    StorageReportProto::get_remaining,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "blockPoolUsed",
                    StorageReportProto::has_blockPoolUsed,
                    StorageReportProto::get_blockPoolUsed,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "storage",
                    StorageReportProto::has_storage,
                    StorageReportProto::get_storage,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StorageReportProto>(
                    "StorageReportProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StorageReportProto {
    fn clear(&mut self) {
        self.clear_storageUuid();
        self.clear_failed();
        self.clear_capacity();
        self.clear_dfsUsed();
        self.clear_remaining();
        self.clear_blockPoolUsed();
        self.clear_storage();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StorageReportProto {
    fn eq(&self, other: &StorageReportProto) -> bool {
        self.storageUuid == other.storageUuid &&
        self.failed == other.failed &&
        self.capacity == other.capacity &&
        self.dfsUsed == other.dfsUsed &&
        self.remaining == other.remaining &&
        self.blockPoolUsed == other.blockPoolUsed &&
        self.storage == other.storage &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StorageReportProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ContentSummaryProto {
    // message fields
    length: ::std::option::Option<u64>,
    fileCount: ::std::option::Option<u64>,
    directoryCount: ::std::option::Option<u64>,
    quota: ::std::option::Option<u64>,
    spaceConsumed: ::std::option::Option<u64>,
    spaceQuota: ::std::option::Option<u64>,
    typeQuotaInfos: ::protobuf::SingularPtrField<StorageTypeQuotaInfosProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ContentSummaryProto {
    pub fn new() -> ContentSummaryProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ContentSummaryProto {
        static mut instance: ::protobuf::lazy::Lazy<ContentSummaryProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ContentSummaryProto,
        };
        unsafe {
            instance.get(|| {
                ContentSummaryProto {
                    length: ::std::option::Option::None,
                    fileCount: ::std::option::Option::None,
                    directoryCount: ::std::option::Option::None,
                    quota: ::std::option::Option::None,
                    spaceConsumed: ::std::option::Option::None,
                    spaceQuota: ::std::option::Option::None,
                    typeQuotaInfos: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint64 length = 1;

    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: u64) {
        self.length = ::std::option::Option::Some(v);
    }

    pub fn get_length<'a>(&self) -> u64 {
        self.length.unwrap_or(0)
    }

    // required uint64 fileCount = 2;

    pub fn clear_fileCount(&mut self) {
        self.fileCount = ::std::option::Option::None;
    }

    pub fn has_fileCount(&self) -> bool {
        self.fileCount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileCount(&mut self, v: u64) {
        self.fileCount = ::std::option::Option::Some(v);
    }

    pub fn get_fileCount<'a>(&self) -> u64 {
        self.fileCount.unwrap_or(0)
    }

    // required uint64 directoryCount = 3;

    pub fn clear_directoryCount(&mut self) {
        self.directoryCount = ::std::option::Option::None;
    }

    pub fn has_directoryCount(&self) -> bool {
        self.directoryCount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_directoryCount(&mut self, v: u64) {
        self.directoryCount = ::std::option::Option::Some(v);
    }

    pub fn get_directoryCount<'a>(&self) -> u64 {
        self.directoryCount.unwrap_or(0)
    }

    // required uint64 quota = 4;

    pub fn clear_quota(&mut self) {
        self.quota = ::std::option::Option::None;
    }

    pub fn has_quota(&self) -> bool {
        self.quota.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quota(&mut self, v: u64) {
        self.quota = ::std::option::Option::Some(v);
    }

    pub fn get_quota<'a>(&self) -> u64 {
        self.quota.unwrap_or(0)
    }

    // required uint64 spaceConsumed = 5;

    pub fn clear_spaceConsumed(&mut self) {
        self.spaceConsumed = ::std::option::Option::None;
    }

    pub fn has_spaceConsumed(&self) -> bool {
        self.spaceConsumed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spaceConsumed(&mut self, v: u64) {
        self.spaceConsumed = ::std::option::Option::Some(v);
    }

    pub fn get_spaceConsumed<'a>(&self) -> u64 {
        self.spaceConsumed.unwrap_or(0)
    }

    // required uint64 spaceQuota = 6;

    pub fn clear_spaceQuota(&mut self) {
        self.spaceQuota = ::std::option::Option::None;
    }

    pub fn has_spaceQuota(&self) -> bool {
        self.spaceQuota.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spaceQuota(&mut self, v: u64) {
        self.spaceQuota = ::std::option::Option::Some(v);
    }

    pub fn get_spaceQuota<'a>(&self) -> u64 {
        self.spaceQuota.unwrap_or(0)
    }

    // optional .hadoop.hdfs.StorageTypeQuotaInfosProto typeQuotaInfos = 7;

    pub fn clear_typeQuotaInfos(&mut self) {
        self.typeQuotaInfos.clear();
    }

    pub fn has_typeQuotaInfos(&self) -> bool {
        self.typeQuotaInfos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_typeQuotaInfos(&mut self, v: StorageTypeQuotaInfosProto) {
        self.typeQuotaInfos = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_typeQuotaInfos<'a>(&'a mut self) -> &'a mut StorageTypeQuotaInfosProto {
        if self.typeQuotaInfos.is_none() {
            self.typeQuotaInfos.set_default();
        };
        self.typeQuotaInfos.as_mut().unwrap()
    }

    // Take field
    pub fn take_typeQuotaInfos(&mut self) -> StorageTypeQuotaInfosProto {
        self.typeQuotaInfos.take().unwrap_or_else(|| StorageTypeQuotaInfosProto::new())
    }

    pub fn get_typeQuotaInfos<'a>(&'a self) -> &'a StorageTypeQuotaInfosProto {
        self.typeQuotaInfos.as_ref().unwrap_or_else(|| StorageTypeQuotaInfosProto::default_instance())
    }
}

impl ::protobuf::Message for ContentSummaryProto {
    fn is_initialized(&self) -> bool {
        if self.length.is_none() {
            return false;
        };
        if self.fileCount.is_none() {
            return false;
        };
        if self.directoryCount.is_none() {
            return false;
        };
        if self.quota.is_none() {
            return false;
        };
        if self.spaceConsumed.is_none() {
            return false;
        };
        if self.spaceQuota.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.length = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.fileCount = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.directoryCount = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.quota = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.spaceConsumed = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.spaceQuota = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.typeQuotaInfos.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.length.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.fileCount.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.directoryCount.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.quota.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.spaceConsumed.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.spaceQuota.iter() {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.typeQuotaInfos.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.length {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.fileCount {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.directoryCount {
            try!(os.write_uint64(3, v));
        };
        if let Some(v) = self.quota {
            try!(os.write_uint64(4, v));
        };
        if let Some(v) = self.spaceConsumed {
            try!(os.write_uint64(5, v));
        };
        if let Some(v) = self.spaceQuota {
            try!(os.write_uint64(6, v));
        };
        if let Some(v) = self.typeQuotaInfos.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ContentSummaryProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ContentSummaryProto {
    fn new() -> ContentSummaryProto {
        ContentSummaryProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ContentSummaryProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "length",
                    ContentSummaryProto::has_length,
                    ContentSummaryProto::get_length,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "fileCount",
                    ContentSummaryProto::has_fileCount,
                    ContentSummaryProto::get_fileCount,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "directoryCount",
                    ContentSummaryProto::has_directoryCount,
                    ContentSummaryProto::get_directoryCount,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "quota",
                    ContentSummaryProto::has_quota,
                    ContentSummaryProto::get_quota,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "spaceConsumed",
                    ContentSummaryProto::has_spaceConsumed,
                    ContentSummaryProto::get_spaceConsumed,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "spaceQuota",
                    ContentSummaryProto::has_spaceQuota,
                    ContentSummaryProto::get_spaceQuota,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "typeQuotaInfos",
                    ContentSummaryProto::has_typeQuotaInfos,
                    ContentSummaryProto::get_typeQuotaInfos,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ContentSummaryProto>(
                    "ContentSummaryProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ContentSummaryProto {
    fn clear(&mut self) {
        self.clear_length();
        self.clear_fileCount();
        self.clear_directoryCount();
        self.clear_quota();
        self.clear_spaceConsumed();
        self.clear_spaceQuota();
        self.clear_typeQuotaInfos();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ContentSummaryProto {
    fn eq(&self, other: &ContentSummaryProto) -> bool {
        self.length == other.length &&
        self.fileCount == other.fileCount &&
        self.directoryCount == other.directoryCount &&
        self.quota == other.quota &&
        self.spaceConsumed == other.spaceConsumed &&
        self.spaceQuota == other.spaceQuota &&
        self.typeQuotaInfos == other.typeQuotaInfos &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ContentSummaryProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StorageTypeQuotaInfosProto {
    // message fields
    typeQuotaInfo: ::protobuf::RepeatedField<StorageTypeQuotaInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl StorageTypeQuotaInfosProto {
    pub fn new() -> StorageTypeQuotaInfosProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StorageTypeQuotaInfosProto {
        static mut instance: ::protobuf::lazy::Lazy<StorageTypeQuotaInfosProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StorageTypeQuotaInfosProto,
        };
        unsafe {
            instance.get(|| {
                StorageTypeQuotaInfosProto {
                    typeQuotaInfo: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .hadoop.hdfs.StorageTypeQuotaInfoProto typeQuotaInfo = 1;

    pub fn clear_typeQuotaInfo(&mut self) {
        self.typeQuotaInfo.clear();
    }

    // Param is passed by value, moved
    pub fn set_typeQuotaInfo(&mut self, v: ::protobuf::RepeatedField<StorageTypeQuotaInfoProto>) {
        self.typeQuotaInfo = v;
    }

    // Mutable pointer to the field.
    pub fn mut_typeQuotaInfo<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<StorageTypeQuotaInfoProto> {
        &mut self.typeQuotaInfo
    }

    // Take field
    pub fn take_typeQuotaInfo(&mut self) -> ::protobuf::RepeatedField<StorageTypeQuotaInfoProto> {
        ::std::mem::replace(&mut self.typeQuotaInfo, ::protobuf::RepeatedField::new())
    }

    pub fn get_typeQuotaInfo<'a>(&'a self) -> &'a [StorageTypeQuotaInfoProto] {
        &self.typeQuotaInfo
    }
}

impl ::protobuf::Message for StorageTypeQuotaInfosProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.typeQuotaInfo));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.typeQuotaInfo.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.typeQuotaInfo.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StorageTypeQuotaInfosProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StorageTypeQuotaInfosProto {
    fn new() -> StorageTypeQuotaInfosProto {
        StorageTypeQuotaInfosProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<StorageTypeQuotaInfosProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "typeQuotaInfo",
                    StorageTypeQuotaInfosProto::get_typeQuotaInfo,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StorageTypeQuotaInfosProto>(
                    "StorageTypeQuotaInfosProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StorageTypeQuotaInfosProto {
    fn clear(&mut self) {
        self.clear_typeQuotaInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StorageTypeQuotaInfosProto {
    fn eq(&self, other: &StorageTypeQuotaInfosProto) -> bool {
        self.typeQuotaInfo == other.typeQuotaInfo &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StorageTypeQuotaInfosProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StorageTypeQuotaInfoProto {
    // message fields
    field_type: ::std::option::Option<StorageTypeProto>,
    quota: ::std::option::Option<u64>,
    consumed: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl StorageTypeQuotaInfoProto {
    pub fn new() -> StorageTypeQuotaInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StorageTypeQuotaInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<StorageTypeQuotaInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StorageTypeQuotaInfoProto,
        };
        unsafe {
            instance.get(|| {
                StorageTypeQuotaInfoProto {
                    field_type: ::std::option::Option::None,
                    quota: ::std::option::Option::None,
                    consumed: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.StorageTypeProto type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: StorageTypeProto) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type<'a>(&self) -> StorageTypeProto {
        self.field_type.unwrap_or(StorageTypeProto::DISK)
    }

    // required uint64 quota = 2;

    pub fn clear_quota(&mut self) {
        self.quota = ::std::option::Option::None;
    }

    pub fn has_quota(&self) -> bool {
        self.quota.is_some()
    }

    // Param is passed by value, moved
    pub fn set_quota(&mut self, v: u64) {
        self.quota = ::std::option::Option::Some(v);
    }

    pub fn get_quota<'a>(&self) -> u64 {
        self.quota.unwrap_or(0)
    }

    // required uint64 consumed = 3;

    pub fn clear_consumed(&mut self) {
        self.consumed = ::std::option::Option::None;
    }

    pub fn has_consumed(&self) -> bool {
        self.consumed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_consumed(&mut self, v: u64) {
        self.consumed = ::std::option::Option::Some(v);
    }

    pub fn get_consumed<'a>(&self) -> u64 {
        self.consumed.unwrap_or(0)
    }
}

impl ::protobuf::Message for StorageTypeQuotaInfoProto {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        };
        if self.quota.is_none() {
            return false;
        };
        if self.consumed.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.quota = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.consumed = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.field_type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.quota.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.consumed.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.quota {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.consumed {
            try!(os.write_uint64(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StorageTypeQuotaInfoProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StorageTypeQuotaInfoProto {
    fn new() -> StorageTypeQuotaInfoProto {
        StorageTypeQuotaInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<StorageTypeQuotaInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "field_type",
                    StorageTypeQuotaInfoProto::has_field_type,
                    StorageTypeQuotaInfoProto::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "quota",
                    StorageTypeQuotaInfoProto::has_quota,
                    StorageTypeQuotaInfoProto::get_quota,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "consumed",
                    StorageTypeQuotaInfoProto::has_consumed,
                    StorageTypeQuotaInfoProto::get_consumed,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StorageTypeQuotaInfoProto>(
                    "StorageTypeQuotaInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StorageTypeQuotaInfoProto {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_quota();
        self.clear_consumed();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StorageTypeQuotaInfoProto {
    fn eq(&self, other: &StorageTypeQuotaInfoProto) -> bool {
        self.field_type == other.field_type &&
        self.quota == other.quota &&
        self.consumed == other.consumed &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StorageTypeQuotaInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CorruptFileBlocksProto {
    // message fields
    files: ::protobuf::RepeatedField<::std::string::String>,
    cookie: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CorruptFileBlocksProto {
    pub fn new() -> CorruptFileBlocksProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CorruptFileBlocksProto {
        static mut instance: ::protobuf::lazy::Lazy<CorruptFileBlocksProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CorruptFileBlocksProto,
        };
        unsafe {
            instance.get(|| {
                CorruptFileBlocksProto {
                    files: ::protobuf::RepeatedField::new(),
                    cookie: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated string files = 1;

    pub fn clear_files(&mut self) {
        self.files.clear();
    }

    // Param is passed by value, moved
    pub fn set_files(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.files = v;
    }

    // Mutable pointer to the field.
    pub fn mut_files<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.files
    }

    // Take field
    pub fn take_files(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.files, ::protobuf::RepeatedField::new())
    }

    pub fn get_files<'a>(&'a self) -> &'a [::std::string::String] {
        &self.files
    }

    // required string cookie = 2;

    pub fn clear_cookie(&mut self) {
        self.cookie.clear();
    }

    pub fn has_cookie(&self) -> bool {
        self.cookie.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cookie(&mut self, v: ::std::string::String) {
        self.cookie = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cookie<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.cookie.is_none() {
            self.cookie.set_default();
        };
        self.cookie.as_mut().unwrap()
    }

    // Take field
    pub fn take_cookie(&mut self) -> ::std::string::String {
        self.cookie.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_cookie<'a>(&'a self) -> &'a str {
        match self.cookie.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for CorruptFileBlocksProto {
    fn is_initialized(&self) -> bool {
        if self.cookie.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.files));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.cookie.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.files.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.cookie.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.files.iter() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.cookie.as_ref() {
            try!(os.write_string(2, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<CorruptFileBlocksProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CorruptFileBlocksProto {
    fn new() -> CorruptFileBlocksProto {
        CorruptFileBlocksProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CorruptFileBlocksProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "files",
                    CorruptFileBlocksProto::get_files,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "cookie",
                    CorruptFileBlocksProto::has_cookie,
                    CorruptFileBlocksProto::get_cookie,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CorruptFileBlocksProto>(
                    "CorruptFileBlocksProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CorruptFileBlocksProto {
    fn clear(&mut self) {
        self.clear_files();
        self.clear_cookie();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CorruptFileBlocksProto {
    fn eq(&self, other: &CorruptFileBlocksProto) -> bool {
        self.files == other.files &&
        self.cookie == other.cookie &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CorruptFileBlocksProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FsPermissionProto {
    // message fields
    perm: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl FsPermissionProto {
    pub fn new() -> FsPermissionProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FsPermissionProto {
        static mut instance: ::protobuf::lazy::Lazy<FsPermissionProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FsPermissionProto,
        };
        unsafe {
            instance.get(|| {
                FsPermissionProto {
                    perm: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 perm = 1;

    pub fn clear_perm(&mut self) {
        self.perm = ::std::option::Option::None;
    }

    pub fn has_perm(&self) -> bool {
        self.perm.is_some()
    }

    // Param is passed by value, moved
    pub fn set_perm(&mut self, v: u32) {
        self.perm = ::std::option::Option::Some(v);
    }

    pub fn get_perm<'a>(&self) -> u32 {
        self.perm.unwrap_or(0)
    }
}

impl ::protobuf::Message for FsPermissionProto {
    fn is_initialized(&self) -> bool {
        if self.perm.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.perm = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.perm.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.perm {
            try!(os.write_uint32(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<FsPermissionProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FsPermissionProto {
    fn new() -> FsPermissionProto {
        FsPermissionProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<FsPermissionProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "perm",
                    FsPermissionProto::has_perm,
                    FsPermissionProto::get_perm,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FsPermissionProto>(
                    "FsPermissionProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FsPermissionProto {
    fn clear(&mut self) {
        self.clear_perm();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FsPermissionProto {
    fn eq(&self, other: &FsPermissionProto) -> bool {
        self.perm == other.perm &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FsPermissionProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StorageTypesProto {
    // message fields
    storageTypes: ::std::vec::Vec<StorageTypeProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl StorageTypesProto {
    pub fn new() -> StorageTypesProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StorageTypesProto {
        static mut instance: ::protobuf::lazy::Lazy<StorageTypesProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StorageTypesProto,
        };
        unsafe {
            instance.get(|| {
                StorageTypesProto {
                    storageTypes: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .hadoop.hdfs.StorageTypeProto storageTypes = 1;

    pub fn clear_storageTypes(&mut self) {
        self.storageTypes.clear();
    }

    // Param is passed by value, moved
    pub fn set_storageTypes(&mut self, v: ::std::vec::Vec<StorageTypeProto>) {
        self.storageTypes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_storageTypes<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<StorageTypeProto> {
        &mut self.storageTypes
    }

    // Take field
    pub fn take_storageTypes(&mut self) -> ::std::vec::Vec<StorageTypeProto> {
        ::std::mem::replace(&mut self.storageTypes, ::std::vec::Vec::new())
    }

    pub fn get_storageTypes<'a>(&'a self) -> &'a [StorageTypeProto] {
        &self.storageTypes
    }
}

impl ::protobuf::Message for StorageTypesProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.storageTypes));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.storageTypes.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.storageTypes.iter() {
            try!(os.write_enum(1, *v as i32));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StorageTypesProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StorageTypesProto {
    fn new() -> StorageTypesProto {
        StorageTypesProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<StorageTypesProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_enum_accessor(
                    "storageTypes",
                    StorageTypesProto::get_storageTypes,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StorageTypesProto>(
                    "StorageTypesProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StorageTypesProto {
    fn clear(&mut self) {
        self.clear_storageTypes();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StorageTypesProto {
    fn eq(&self, other: &StorageTypesProto) -> bool {
        self.storageTypes == other.storageTypes &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StorageTypesProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BlockStoragePolicyProto {
    // message fields
    policyId: ::std::option::Option<u32>,
    name: ::protobuf::SingularField<::std::string::String>,
    creationPolicy: ::protobuf::SingularPtrField<StorageTypesProto>,
    creationFallbackPolicy: ::protobuf::SingularPtrField<StorageTypesProto>,
    replicationFallbackPolicy: ::protobuf::SingularPtrField<StorageTypesProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl BlockStoragePolicyProto {
    pub fn new() -> BlockStoragePolicyProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockStoragePolicyProto {
        static mut instance: ::protobuf::lazy::Lazy<BlockStoragePolicyProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockStoragePolicyProto,
        };
        unsafe {
            instance.get(|| {
                BlockStoragePolicyProto {
                    policyId: ::std::option::Option::None,
                    name: ::protobuf::SingularField::none(),
                    creationPolicy: ::protobuf::SingularPtrField::none(),
                    creationFallbackPolicy: ::protobuf::SingularPtrField::none(),
                    replicationFallbackPolicy: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 policyId = 1;

    pub fn clear_policyId(&mut self) {
        self.policyId = ::std::option::Option::None;
    }

    pub fn has_policyId(&self) -> bool {
        self.policyId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_policyId(&mut self, v: u32) {
        self.policyId = ::std::option::Option::Some(v);
    }

    pub fn get_policyId<'a>(&self) -> u32 {
        self.policyId.unwrap_or(0)
    }

    // required string name = 2;

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
    pub fn mut_name<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name<'a>(&'a self) -> &'a str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required .hadoop.hdfs.StorageTypesProto creationPolicy = 3;

    pub fn clear_creationPolicy(&mut self) {
        self.creationPolicy.clear();
    }

    pub fn has_creationPolicy(&self) -> bool {
        self.creationPolicy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_creationPolicy(&mut self, v: StorageTypesProto) {
        self.creationPolicy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_creationPolicy<'a>(&'a mut self) -> &'a mut StorageTypesProto {
        if self.creationPolicy.is_none() {
            self.creationPolicy.set_default();
        };
        self.creationPolicy.as_mut().unwrap()
    }

    // Take field
    pub fn take_creationPolicy(&mut self) -> StorageTypesProto {
        self.creationPolicy.take().unwrap_or_else(|| StorageTypesProto::new())
    }

    pub fn get_creationPolicy<'a>(&'a self) -> &'a StorageTypesProto {
        self.creationPolicy.as_ref().unwrap_or_else(|| StorageTypesProto::default_instance())
    }

    // optional .hadoop.hdfs.StorageTypesProto creationFallbackPolicy = 4;

    pub fn clear_creationFallbackPolicy(&mut self) {
        self.creationFallbackPolicy.clear();
    }

    pub fn has_creationFallbackPolicy(&self) -> bool {
        self.creationFallbackPolicy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_creationFallbackPolicy(&mut self, v: StorageTypesProto) {
        self.creationFallbackPolicy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_creationFallbackPolicy<'a>(&'a mut self) -> &'a mut StorageTypesProto {
        if self.creationFallbackPolicy.is_none() {
            self.creationFallbackPolicy.set_default();
        };
        self.creationFallbackPolicy.as_mut().unwrap()
    }

    // Take field
    pub fn take_creationFallbackPolicy(&mut self) -> StorageTypesProto {
        self.creationFallbackPolicy.take().unwrap_or_else(|| StorageTypesProto::new())
    }

    pub fn get_creationFallbackPolicy<'a>(&'a self) -> &'a StorageTypesProto {
        self.creationFallbackPolicy.as_ref().unwrap_or_else(|| StorageTypesProto::default_instance())
    }

    // optional .hadoop.hdfs.StorageTypesProto replicationFallbackPolicy = 5;

    pub fn clear_replicationFallbackPolicy(&mut self) {
        self.replicationFallbackPolicy.clear();
    }

    pub fn has_replicationFallbackPolicy(&self) -> bool {
        self.replicationFallbackPolicy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_replicationFallbackPolicy(&mut self, v: StorageTypesProto) {
        self.replicationFallbackPolicy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_replicationFallbackPolicy<'a>(&'a mut self) -> &'a mut StorageTypesProto {
        if self.replicationFallbackPolicy.is_none() {
            self.replicationFallbackPolicy.set_default();
        };
        self.replicationFallbackPolicy.as_mut().unwrap()
    }

    // Take field
    pub fn take_replicationFallbackPolicy(&mut self) -> StorageTypesProto {
        self.replicationFallbackPolicy.take().unwrap_or_else(|| StorageTypesProto::new())
    }

    pub fn get_replicationFallbackPolicy<'a>(&'a self) -> &'a StorageTypesProto {
        self.replicationFallbackPolicy.as_ref().unwrap_or_else(|| StorageTypesProto::default_instance())
    }
}

impl ::protobuf::Message for BlockStoragePolicyProto {
    fn is_initialized(&self) -> bool {
        if self.policyId.is_none() {
            return false;
        };
        if self.name.is_none() {
            return false;
        };
        if self.creationPolicy.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.policyId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.name.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.creationPolicy.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.creationFallbackPolicy.set_default();
                    try!(is.merge_message(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.replicationFallbackPolicy.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.policyId.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.creationPolicy.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.creationFallbackPolicy.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.replicationFallbackPolicy.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.policyId {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.creationPolicy.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.creationFallbackPolicy.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.replicationFallbackPolicy.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<BlockStoragePolicyProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BlockStoragePolicyProto {
    fn new() -> BlockStoragePolicyProto {
        BlockStoragePolicyProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockStoragePolicyProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "policyId",
                    BlockStoragePolicyProto::has_policyId,
                    BlockStoragePolicyProto::get_policyId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    BlockStoragePolicyProto::has_name,
                    BlockStoragePolicyProto::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "creationPolicy",
                    BlockStoragePolicyProto::has_creationPolicy,
                    BlockStoragePolicyProto::get_creationPolicy,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "creationFallbackPolicy",
                    BlockStoragePolicyProto::has_creationFallbackPolicy,
                    BlockStoragePolicyProto::get_creationFallbackPolicy,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "replicationFallbackPolicy",
                    BlockStoragePolicyProto::has_replicationFallbackPolicy,
                    BlockStoragePolicyProto::get_replicationFallbackPolicy,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockStoragePolicyProto>(
                    "BlockStoragePolicyProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockStoragePolicyProto {
    fn clear(&mut self) {
        self.clear_policyId();
        self.clear_name();
        self.clear_creationPolicy();
        self.clear_creationFallbackPolicy();
        self.clear_replicationFallbackPolicy();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BlockStoragePolicyProto {
    fn eq(&self, other: &BlockStoragePolicyProto) -> bool {
        self.policyId == other.policyId &&
        self.name == other.name &&
        self.creationPolicy == other.creationPolicy &&
        self.creationFallbackPolicy == other.creationFallbackPolicy &&
        self.replicationFallbackPolicy == other.replicationFallbackPolicy &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BlockStoragePolicyProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StorageUuidsProto {
    // message fields
    storageUuids: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl StorageUuidsProto {
    pub fn new() -> StorageUuidsProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StorageUuidsProto {
        static mut instance: ::protobuf::lazy::Lazy<StorageUuidsProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StorageUuidsProto,
        };
        unsafe {
            instance.get(|| {
                StorageUuidsProto {
                    storageUuids: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated string storageUuids = 1;

    pub fn clear_storageUuids(&mut self) {
        self.storageUuids.clear();
    }

    // Param is passed by value, moved
    pub fn set_storageUuids(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.storageUuids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_storageUuids<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.storageUuids
    }

    // Take field
    pub fn take_storageUuids(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.storageUuids, ::protobuf::RepeatedField::new())
    }

    pub fn get_storageUuids<'a>(&'a self) -> &'a [::std::string::String] {
        &self.storageUuids
    }
}

impl ::protobuf::Message for StorageUuidsProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.storageUuids));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.storageUuids.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.storageUuids.iter() {
            try!(os.write_string(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StorageUuidsProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StorageUuidsProto {
    fn new() -> StorageUuidsProto {
        StorageUuidsProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<StorageUuidsProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "storageUuids",
                    StorageUuidsProto::get_storageUuids,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StorageUuidsProto>(
                    "StorageUuidsProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StorageUuidsProto {
    fn clear(&mut self) {
        self.clear_storageUuids();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StorageUuidsProto {
    fn eq(&self, other: &StorageUuidsProto) -> bool {
        self.storageUuids == other.storageUuids &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StorageUuidsProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct LocatedBlockProto {
    // message fields
    b: ::protobuf::SingularPtrField<ExtendedBlockProto>,
    offset: ::std::option::Option<u64>,
    locs: ::protobuf::RepeatedField<DatanodeInfoProto>,
    corrupt: ::std::option::Option<bool>,
    blockToken: ::protobuf::SingularPtrField<TokenProto>,
    isCached: ::std::vec::Vec<bool>,
    storageTypes: ::std::vec::Vec<StorageTypeProto>,
    storageIDs: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl LocatedBlockProto {
    pub fn new() -> LocatedBlockProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LocatedBlockProto {
        static mut instance: ::protobuf::lazy::Lazy<LocatedBlockProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LocatedBlockProto,
        };
        unsafe {
            instance.get(|| {
                LocatedBlockProto {
                    b: ::protobuf::SingularPtrField::none(),
                    offset: ::std::option::Option::None,
                    locs: ::protobuf::RepeatedField::new(),
                    corrupt: ::std::option::Option::None,
                    blockToken: ::protobuf::SingularPtrField::none(),
                    isCached: ::std::vec::Vec::new(),
                    storageTypes: ::std::vec::Vec::new(),
                    storageIDs: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.ExtendedBlockProto b = 1;

    pub fn clear_b(&mut self) {
        self.b.clear();
    }

    pub fn has_b(&self) -> bool {
        self.b.is_some()
    }

    // Param is passed by value, moved
    pub fn set_b(&mut self, v: ExtendedBlockProto) {
        self.b = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_b<'a>(&'a mut self) -> &'a mut ExtendedBlockProto {
        if self.b.is_none() {
            self.b.set_default();
        };
        self.b.as_mut().unwrap()
    }

    // Take field
    pub fn take_b(&mut self) -> ExtendedBlockProto {
        self.b.take().unwrap_or_else(|| ExtendedBlockProto::new())
    }

    pub fn get_b<'a>(&'a self) -> &'a ExtendedBlockProto {
        self.b.as_ref().unwrap_or_else(|| ExtendedBlockProto::default_instance())
    }

    // required uint64 offset = 2;

    pub fn clear_offset(&mut self) {
        self.offset = ::std::option::Option::None;
    }

    pub fn has_offset(&self) -> bool {
        self.offset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_offset(&mut self, v: u64) {
        self.offset = ::std::option::Option::Some(v);
    }

    pub fn get_offset<'a>(&self) -> u64 {
        self.offset.unwrap_or(0)
    }

    // repeated .hadoop.hdfs.DatanodeInfoProto locs = 3;

    pub fn clear_locs(&mut self) {
        self.locs.clear();
    }

    // Param is passed by value, moved
    pub fn set_locs(&mut self, v: ::protobuf::RepeatedField<DatanodeInfoProto>) {
        self.locs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_locs<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<DatanodeInfoProto> {
        &mut self.locs
    }

    // Take field
    pub fn take_locs(&mut self) -> ::protobuf::RepeatedField<DatanodeInfoProto> {
        ::std::mem::replace(&mut self.locs, ::protobuf::RepeatedField::new())
    }

    pub fn get_locs<'a>(&'a self) -> &'a [DatanodeInfoProto] {
        &self.locs
    }

    // required bool corrupt = 4;

    pub fn clear_corrupt(&mut self) {
        self.corrupt = ::std::option::Option::None;
    }

    pub fn has_corrupt(&self) -> bool {
        self.corrupt.is_some()
    }

    // Param is passed by value, moved
    pub fn set_corrupt(&mut self, v: bool) {
        self.corrupt = ::std::option::Option::Some(v);
    }

    pub fn get_corrupt<'a>(&self) -> bool {
        self.corrupt.unwrap_or(false)
    }

    // required .hadoop.common.TokenProto blockToken = 5;

    pub fn clear_blockToken(&mut self) {
        self.blockToken.clear();
    }

    pub fn has_blockToken(&self) -> bool {
        self.blockToken.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockToken(&mut self, v: TokenProto) {
        self.blockToken = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blockToken<'a>(&'a mut self) -> &'a mut TokenProto {
        if self.blockToken.is_none() {
            self.blockToken.set_default();
        };
        self.blockToken.as_mut().unwrap()
    }

    // Take field
    pub fn take_blockToken(&mut self) -> TokenProto {
        self.blockToken.take().unwrap_or_else(|| TokenProto::new())
    }

    pub fn get_blockToken<'a>(&'a self) -> &'a TokenProto {
        self.blockToken.as_ref().unwrap_or_else(|| TokenProto::default_instance())
    }

    // repeated bool isCached = 6;

    pub fn clear_isCached(&mut self) {
        self.isCached.clear();
    }

    // Param is passed by value, moved
    pub fn set_isCached(&mut self, v: ::std::vec::Vec<bool>) {
        self.isCached = v;
    }

    // Mutable pointer to the field.
    pub fn mut_isCached<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<bool> {
        &mut self.isCached
    }

    // Take field
    pub fn take_isCached(&mut self) -> ::std::vec::Vec<bool> {
        ::std::mem::replace(&mut self.isCached, ::std::vec::Vec::new())
    }

    pub fn get_isCached<'a>(&'a self) -> &'a [bool] {
        &self.isCached
    }

    // repeated .hadoop.hdfs.StorageTypeProto storageTypes = 7;

    pub fn clear_storageTypes(&mut self) {
        self.storageTypes.clear();
    }

    // Param is passed by value, moved
    pub fn set_storageTypes(&mut self, v: ::std::vec::Vec<StorageTypeProto>) {
        self.storageTypes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_storageTypes<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<StorageTypeProto> {
        &mut self.storageTypes
    }

    // Take field
    pub fn take_storageTypes(&mut self) -> ::std::vec::Vec<StorageTypeProto> {
        ::std::mem::replace(&mut self.storageTypes, ::std::vec::Vec::new())
    }

    pub fn get_storageTypes<'a>(&'a self) -> &'a [StorageTypeProto] {
        &self.storageTypes
    }

    // repeated string storageIDs = 8;

    pub fn clear_storageIDs(&mut self) {
        self.storageIDs.clear();
    }

    // Param is passed by value, moved
    pub fn set_storageIDs(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.storageIDs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_storageIDs<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.storageIDs
    }

    // Take field
    pub fn take_storageIDs(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.storageIDs, ::protobuf::RepeatedField::new())
    }

    pub fn get_storageIDs<'a>(&'a self) -> &'a [::std::string::String] {
        &self.storageIDs
    }
}

impl ::protobuf::Message for LocatedBlockProto {
    fn is_initialized(&self) -> bool {
        if self.b.is_none() {
            return false;
        };
        if self.offset.is_none() {
            return false;
        };
        if self.corrupt.is_none() {
            return false;
        };
        if self.blockToken.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.b.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.offset = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.locs));
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.corrupt = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.blockToken.set_default();
                    try!(is.merge_message(tmp))
                },
                6 => {
                    try!(::protobuf::rt::read_repeated_bool_into(wire_type, is, &mut self.isCached));
                },
                7 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.storageTypes));
                },
                8 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.storageIDs));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.b.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.offset.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.locs.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.corrupt.is_some() {
            my_size += 2;
        };
        for value in self.blockToken.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.isCached.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.isCached.len() as u32) + (self.isCached.len() * 1) as u32;
        };
        for value in self.storageTypes.iter() {
            my_size += ::protobuf::rt::enum_size(7, *value);
        };
        for value in self.storageIDs.iter() {
            my_size += ::protobuf::rt::string_size(8, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.b.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.offset {
            try!(os.write_uint64(2, v));
        };
        for v in self.locs.iter() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.corrupt {
            try!(os.write_bool(4, v));
        };
        if let Some(v) = self.blockToken.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if !self.isCached.is_empty() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32((self.isCached.len() * 1) as u32));
            for v in self.isCached.iter() {
                try!(os.write_bool_no_tag(*v));
            };
        };
        for v in self.storageTypes.iter() {
            try!(os.write_enum(7, *v as i32));
        };
        for v in self.storageIDs.iter() {
            try!(os.write_string(8, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<LocatedBlockProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LocatedBlockProto {
    fn new() -> LocatedBlockProto {
        LocatedBlockProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<LocatedBlockProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "b",
                    LocatedBlockProto::has_b,
                    LocatedBlockProto::get_b,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "offset",
                    LocatedBlockProto::has_offset,
                    LocatedBlockProto::get_offset,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "locs",
                    LocatedBlockProto::get_locs,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "corrupt",
                    LocatedBlockProto::has_corrupt,
                    LocatedBlockProto::get_corrupt,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "blockToken",
                    LocatedBlockProto::has_blockToken,
                    LocatedBlockProto::get_blockToken,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bool_accessor(
                    "isCached",
                    LocatedBlockProto::get_isCached,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_enum_accessor(
                    "storageTypes",
                    LocatedBlockProto::get_storageTypes,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "storageIDs",
                    LocatedBlockProto::get_storageIDs,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LocatedBlockProto>(
                    "LocatedBlockProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LocatedBlockProto {
    fn clear(&mut self) {
        self.clear_b();
        self.clear_offset();
        self.clear_locs();
        self.clear_corrupt();
        self.clear_blockToken();
        self.clear_isCached();
        self.clear_storageTypes();
        self.clear_storageIDs();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for LocatedBlockProto {
    fn eq(&self, other: &LocatedBlockProto) -> bool {
        self.b == other.b &&
        self.offset == other.offset &&
        self.locs == other.locs &&
        self.corrupt == other.corrupt &&
        self.blockToken == other.blockToken &&
        self.isCached == other.isCached &&
        self.storageTypes == other.storageTypes &&
        self.storageIDs == other.storageIDs &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for LocatedBlockProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DataEncryptionKeyProto {
    // message fields
    keyId: ::std::option::Option<u32>,
    blockPoolId: ::protobuf::SingularField<::std::string::String>,
    nonce: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    encryptionKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    expiryDate: ::std::option::Option<u64>,
    encryptionAlgorithm: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl DataEncryptionKeyProto {
    pub fn new() -> DataEncryptionKeyProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataEncryptionKeyProto {
        static mut instance: ::protobuf::lazy::Lazy<DataEncryptionKeyProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataEncryptionKeyProto,
        };
        unsafe {
            instance.get(|| {
                DataEncryptionKeyProto {
                    keyId: ::std::option::Option::None,
                    blockPoolId: ::protobuf::SingularField::none(),
                    nonce: ::protobuf::SingularField::none(),
                    encryptionKey: ::protobuf::SingularField::none(),
                    expiryDate: ::std::option::Option::None,
                    encryptionAlgorithm: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 keyId = 1;

    pub fn clear_keyId(&mut self) {
        self.keyId = ::std::option::Option::None;
    }

    pub fn has_keyId(&self) -> bool {
        self.keyId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_keyId(&mut self, v: u32) {
        self.keyId = ::std::option::Option::Some(v);
    }

    pub fn get_keyId<'a>(&self) -> u32 {
        self.keyId.unwrap_or(0)
    }

    // required string blockPoolId = 2;

    pub fn clear_blockPoolId(&mut self) {
        self.blockPoolId.clear();
    }

    pub fn has_blockPoolId(&self) -> bool {
        self.blockPoolId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockPoolId(&mut self, v: ::std::string::String) {
        self.blockPoolId = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blockPoolId<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.blockPoolId.is_none() {
            self.blockPoolId.set_default();
        };
        self.blockPoolId.as_mut().unwrap()
    }

    // Take field
    pub fn take_blockPoolId(&mut self) -> ::std::string::String {
        self.blockPoolId.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_blockPoolId<'a>(&'a self) -> &'a str {
        match self.blockPoolId.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required bytes nonce = 3;

    pub fn clear_nonce(&mut self) {
        self.nonce.clear();
    }

    pub fn has_nonce(&self) -> bool {
        self.nonce.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nonce(&mut self, v: ::std::vec::Vec<u8>) {
        self.nonce = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nonce<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.nonce.is_none() {
            self.nonce.set_default();
        };
        self.nonce.as_mut().unwrap()
    }

    // Take field
    pub fn take_nonce(&mut self) -> ::std::vec::Vec<u8> {
        self.nonce.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_nonce<'a>(&'a self) -> &'a [u8] {
        match self.nonce.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required bytes encryptionKey = 4;

    pub fn clear_encryptionKey(&mut self) {
        self.encryptionKey.clear();
    }

    pub fn has_encryptionKey(&self) -> bool {
        self.encryptionKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encryptionKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.encryptionKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encryptionKey<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.encryptionKey.is_none() {
            self.encryptionKey.set_default();
        };
        self.encryptionKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_encryptionKey(&mut self) -> ::std::vec::Vec<u8> {
        self.encryptionKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_encryptionKey<'a>(&'a self) -> &'a [u8] {
        match self.encryptionKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required uint64 expiryDate = 5;

    pub fn clear_expiryDate(&mut self) {
        self.expiryDate = ::std::option::Option::None;
    }

    pub fn has_expiryDate(&self) -> bool {
        self.expiryDate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expiryDate(&mut self, v: u64) {
        self.expiryDate = ::std::option::Option::Some(v);
    }

    pub fn get_expiryDate<'a>(&self) -> u64 {
        self.expiryDate.unwrap_or(0)
    }

    // optional string encryptionAlgorithm = 6;

    pub fn clear_encryptionAlgorithm(&mut self) {
        self.encryptionAlgorithm.clear();
    }

    pub fn has_encryptionAlgorithm(&self) -> bool {
        self.encryptionAlgorithm.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encryptionAlgorithm(&mut self, v: ::std::string::String) {
        self.encryptionAlgorithm = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_encryptionAlgorithm<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.encryptionAlgorithm.is_none() {
            self.encryptionAlgorithm.set_default();
        };
        self.encryptionAlgorithm.as_mut().unwrap()
    }

    // Take field
    pub fn take_encryptionAlgorithm(&mut self) -> ::std::string::String {
        self.encryptionAlgorithm.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_encryptionAlgorithm<'a>(&'a self) -> &'a str {
        match self.encryptionAlgorithm.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for DataEncryptionKeyProto {
    fn is_initialized(&self) -> bool {
        if self.keyId.is_none() {
            return false;
        };
        if self.blockPoolId.is_none() {
            return false;
        };
        if self.nonce.is_none() {
            return false;
        };
        if self.encryptionKey.is_none() {
            return false;
        };
        if self.expiryDate.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.keyId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.blockPoolId.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.nonce.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.encryptionKey.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.expiryDate = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.encryptionAlgorithm.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.keyId.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.blockPoolId.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.nonce.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in self.encryptionKey.iter() {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        for value in self.expiryDate.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.encryptionAlgorithm.iter() {
            my_size += ::protobuf::rt::string_size(6, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.keyId {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.blockPoolId.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.nonce.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        if let Some(v) = self.encryptionKey.as_ref() {
            try!(os.write_bytes(4, &v));
        };
        if let Some(v) = self.expiryDate {
            try!(os.write_uint64(5, v));
        };
        if let Some(v) = self.encryptionAlgorithm.as_ref() {
            try!(os.write_string(6, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<DataEncryptionKeyProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DataEncryptionKeyProto {
    fn new() -> DataEncryptionKeyProto {
        DataEncryptionKeyProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataEncryptionKeyProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "keyId",
                    DataEncryptionKeyProto::has_keyId,
                    DataEncryptionKeyProto::get_keyId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "blockPoolId",
                    DataEncryptionKeyProto::has_blockPoolId,
                    DataEncryptionKeyProto::get_blockPoolId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "nonce",
                    DataEncryptionKeyProto::has_nonce,
                    DataEncryptionKeyProto::get_nonce,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "encryptionKey",
                    DataEncryptionKeyProto::has_encryptionKey,
                    DataEncryptionKeyProto::get_encryptionKey,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "expiryDate",
                    DataEncryptionKeyProto::has_expiryDate,
                    DataEncryptionKeyProto::get_expiryDate,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "encryptionAlgorithm",
                    DataEncryptionKeyProto::has_encryptionAlgorithm,
                    DataEncryptionKeyProto::get_encryptionAlgorithm,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DataEncryptionKeyProto>(
                    "DataEncryptionKeyProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataEncryptionKeyProto {
    fn clear(&mut self) {
        self.clear_keyId();
        self.clear_blockPoolId();
        self.clear_nonce();
        self.clear_encryptionKey();
        self.clear_expiryDate();
        self.clear_encryptionAlgorithm();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DataEncryptionKeyProto {
    fn eq(&self, other: &DataEncryptionKeyProto) -> bool {
        self.keyId == other.keyId &&
        self.blockPoolId == other.blockPoolId &&
        self.nonce == other.nonce &&
        self.encryptionKey == other.encryptionKey &&
        self.expiryDate == other.expiryDate &&
        self.encryptionAlgorithm == other.encryptionAlgorithm &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DataEncryptionKeyProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct FileEncryptionInfoProto {
    // message fields
    suite: ::std::option::Option<CipherSuiteProto>,
    cryptoProtocolVersion: ::std::option::Option<CryptoProtocolVersionProto>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    iv: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    keyName: ::protobuf::SingularField<::std::string::String>,
    ezKeyVersionName: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl FileEncryptionInfoProto {
    pub fn new() -> FileEncryptionInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FileEncryptionInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<FileEncryptionInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FileEncryptionInfoProto,
        };
        unsafe {
            instance.get(|| {
                FileEncryptionInfoProto {
                    suite: ::std::option::Option::None,
                    cryptoProtocolVersion: ::std::option::Option::None,
                    key: ::protobuf::SingularField::none(),
                    iv: ::protobuf::SingularField::none(),
                    keyName: ::protobuf::SingularField::none(),
                    ezKeyVersionName: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.CipherSuiteProto suite = 1;

    pub fn clear_suite(&mut self) {
        self.suite = ::std::option::Option::None;
    }

    pub fn has_suite(&self) -> bool {
        self.suite.is_some()
    }

    // Param is passed by value, moved
    pub fn set_suite(&mut self, v: CipherSuiteProto) {
        self.suite = ::std::option::Option::Some(v);
    }

    pub fn get_suite<'a>(&self) -> CipherSuiteProto {
        self.suite.unwrap_or(CipherSuiteProto::UNKNOWN)
    }

    // required .hadoop.hdfs.CryptoProtocolVersionProto cryptoProtocolVersion = 2;

    pub fn clear_cryptoProtocolVersion(&mut self) {
        self.cryptoProtocolVersion = ::std::option::Option::None;
    }

    pub fn has_cryptoProtocolVersion(&self) -> bool {
        self.cryptoProtocolVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cryptoProtocolVersion(&mut self, v: CryptoProtocolVersionProto) {
        self.cryptoProtocolVersion = ::std::option::Option::Some(v);
    }

    pub fn get_cryptoProtocolVersion<'a>(&self) -> CryptoProtocolVersionProto {
        self.cryptoProtocolVersion.unwrap_or(CryptoProtocolVersionProto::UNKNOWN_PROTOCOL_VERSION)
    }

    // required bytes key = 3;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key<'a>(&'a self) -> &'a [u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required bytes iv = 4;

    pub fn clear_iv(&mut self) {
        self.iv.clear();
    }

    pub fn has_iv(&self) -> bool {
        self.iv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_iv(&mut self, v: ::std::vec::Vec<u8>) {
        self.iv = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_iv<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.iv.is_none() {
            self.iv.set_default();
        };
        self.iv.as_mut().unwrap()
    }

    // Take field
    pub fn take_iv(&mut self) -> ::std::vec::Vec<u8> {
        self.iv.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_iv<'a>(&'a self) -> &'a [u8] {
        match self.iv.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required string keyName = 5;

    pub fn clear_keyName(&mut self) {
        self.keyName.clear();
    }

    pub fn has_keyName(&self) -> bool {
        self.keyName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_keyName(&mut self, v: ::std::string::String) {
        self.keyName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_keyName<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.keyName.is_none() {
            self.keyName.set_default();
        };
        self.keyName.as_mut().unwrap()
    }

    // Take field
    pub fn take_keyName(&mut self) -> ::std::string::String {
        self.keyName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_keyName<'a>(&'a self) -> &'a str {
        match self.keyName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string ezKeyVersionName = 6;

    pub fn clear_ezKeyVersionName(&mut self) {
        self.ezKeyVersionName.clear();
    }

    pub fn has_ezKeyVersionName(&self) -> bool {
        self.ezKeyVersionName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ezKeyVersionName(&mut self, v: ::std::string::String) {
        self.ezKeyVersionName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ezKeyVersionName<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.ezKeyVersionName.is_none() {
            self.ezKeyVersionName.set_default();
        };
        self.ezKeyVersionName.as_mut().unwrap()
    }

    // Take field
    pub fn take_ezKeyVersionName(&mut self) -> ::std::string::String {
        self.ezKeyVersionName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ezKeyVersionName<'a>(&'a self) -> &'a str {
        match self.ezKeyVersionName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for FileEncryptionInfoProto {
    fn is_initialized(&self) -> bool {
        if self.suite.is_none() {
            return false;
        };
        if self.cryptoProtocolVersion.is_none() {
            return false;
        };
        if self.key.is_none() {
            return false;
        };
        if self.iv.is_none() {
            return false;
        };
        if self.keyName.is_none() {
            return false;
        };
        if self.ezKeyVersionName.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.suite = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.cryptoProtocolVersion = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.key.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.iv.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.keyName.set_default();
                    try!(is.read_string_into(tmp))
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.ezKeyVersionName.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.suite.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.cryptoProtocolVersion.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in self.key.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in self.iv.iter() {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        for value in self.keyName.iter() {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        for value in self.ezKeyVersionName.iter() {
            my_size += ::protobuf::rt::string_size(6, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.suite {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.cryptoProtocolVersion {
            try!(os.write_enum(2, v as i32));
        };
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        if let Some(v) = self.iv.as_ref() {
            try!(os.write_bytes(4, &v));
        };
        if let Some(v) = self.keyName.as_ref() {
            try!(os.write_string(5, &v));
        };
        if let Some(v) = self.ezKeyVersionName.as_ref() {
            try!(os.write_string(6, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<FileEncryptionInfoProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FileEncryptionInfoProto {
    fn new() -> FileEncryptionInfoProto {
        FileEncryptionInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<FileEncryptionInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "suite",
                    FileEncryptionInfoProto::has_suite,
                    FileEncryptionInfoProto::get_suite,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "cryptoProtocolVersion",
                    FileEncryptionInfoProto::has_cryptoProtocolVersion,
                    FileEncryptionInfoProto::get_cryptoProtocolVersion,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    FileEncryptionInfoProto::has_key,
                    FileEncryptionInfoProto::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "iv",
                    FileEncryptionInfoProto::has_iv,
                    FileEncryptionInfoProto::get_iv,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "keyName",
                    FileEncryptionInfoProto::has_keyName,
                    FileEncryptionInfoProto::get_keyName,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "ezKeyVersionName",
                    FileEncryptionInfoProto::has_ezKeyVersionName,
                    FileEncryptionInfoProto::get_ezKeyVersionName,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FileEncryptionInfoProto>(
                    "FileEncryptionInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FileEncryptionInfoProto {
    fn clear(&mut self) {
        self.clear_suite();
        self.clear_cryptoProtocolVersion();
        self.clear_key();
        self.clear_iv();
        self.clear_keyName();
        self.clear_ezKeyVersionName();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FileEncryptionInfoProto {
    fn eq(&self, other: &FileEncryptionInfoProto) -> bool {
        self.suite == other.suite &&
        self.cryptoProtocolVersion == other.cryptoProtocolVersion &&
        self.key == other.key &&
        self.iv == other.iv &&
        self.keyName == other.keyName &&
        self.ezKeyVersionName == other.ezKeyVersionName &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FileEncryptionInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PerFileEncryptionInfoProto {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    iv: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    ezKeyVersionName: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl PerFileEncryptionInfoProto {
    pub fn new() -> PerFileEncryptionInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PerFileEncryptionInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<PerFileEncryptionInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PerFileEncryptionInfoProto,
        };
        unsafe {
            instance.get(|| {
                PerFileEncryptionInfoProto {
                    key: ::protobuf::SingularField::none(),
                    iv: ::protobuf::SingularField::none(),
                    ezKeyVersionName: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key<'a>(&'a self) -> &'a [u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required bytes iv = 2;

    pub fn clear_iv(&mut self) {
        self.iv.clear();
    }

    pub fn has_iv(&self) -> bool {
        self.iv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_iv(&mut self, v: ::std::vec::Vec<u8>) {
        self.iv = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_iv<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.iv.is_none() {
            self.iv.set_default();
        };
        self.iv.as_mut().unwrap()
    }

    // Take field
    pub fn take_iv(&mut self) -> ::std::vec::Vec<u8> {
        self.iv.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_iv<'a>(&'a self) -> &'a [u8] {
        match self.iv.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required string ezKeyVersionName = 3;

    pub fn clear_ezKeyVersionName(&mut self) {
        self.ezKeyVersionName.clear();
    }

    pub fn has_ezKeyVersionName(&self) -> bool {
        self.ezKeyVersionName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ezKeyVersionName(&mut self, v: ::std::string::String) {
        self.ezKeyVersionName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ezKeyVersionName<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.ezKeyVersionName.is_none() {
            self.ezKeyVersionName.set_default();
        };
        self.ezKeyVersionName.as_mut().unwrap()
    }

    // Take field
    pub fn take_ezKeyVersionName(&mut self) -> ::std::string::String {
        self.ezKeyVersionName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ezKeyVersionName<'a>(&'a self) -> &'a str {
        match self.ezKeyVersionName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for PerFileEncryptionInfoProto {
    fn is_initialized(&self) -> bool {
        if self.key.is_none() {
            return false;
        };
        if self.iv.is_none() {
            return false;
        };
        if self.ezKeyVersionName.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.key.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.iv.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.ezKeyVersionName.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.key.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.iv.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in self.ezKeyVersionName.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.iv.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.ezKeyVersionName.as_ref() {
            try!(os.write_string(3, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PerFileEncryptionInfoProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PerFileEncryptionInfoProto {
    fn new() -> PerFileEncryptionInfoProto {
        PerFileEncryptionInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<PerFileEncryptionInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    PerFileEncryptionInfoProto::has_key,
                    PerFileEncryptionInfoProto::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "iv",
                    PerFileEncryptionInfoProto::has_iv,
                    PerFileEncryptionInfoProto::get_iv,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "ezKeyVersionName",
                    PerFileEncryptionInfoProto::has_ezKeyVersionName,
                    PerFileEncryptionInfoProto::get_ezKeyVersionName,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PerFileEncryptionInfoProto>(
                    "PerFileEncryptionInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PerFileEncryptionInfoProto {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_iv();
        self.clear_ezKeyVersionName();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PerFileEncryptionInfoProto {
    fn eq(&self, other: &PerFileEncryptionInfoProto) -> bool {
        self.key == other.key &&
        self.iv == other.iv &&
        self.ezKeyVersionName == other.ezKeyVersionName &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PerFileEncryptionInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ZoneEncryptionInfoProto {
    // message fields
    suite: ::std::option::Option<CipherSuiteProto>,
    cryptoProtocolVersion: ::std::option::Option<CryptoProtocolVersionProto>,
    keyName: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ZoneEncryptionInfoProto {
    pub fn new() -> ZoneEncryptionInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ZoneEncryptionInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<ZoneEncryptionInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ZoneEncryptionInfoProto,
        };
        unsafe {
            instance.get(|| {
                ZoneEncryptionInfoProto {
                    suite: ::std::option::Option::None,
                    cryptoProtocolVersion: ::std::option::Option::None,
                    keyName: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.CipherSuiteProto suite = 1;

    pub fn clear_suite(&mut self) {
        self.suite = ::std::option::Option::None;
    }

    pub fn has_suite(&self) -> bool {
        self.suite.is_some()
    }

    // Param is passed by value, moved
    pub fn set_suite(&mut self, v: CipherSuiteProto) {
        self.suite = ::std::option::Option::Some(v);
    }

    pub fn get_suite<'a>(&self) -> CipherSuiteProto {
        self.suite.unwrap_or(CipherSuiteProto::UNKNOWN)
    }

    // required .hadoop.hdfs.CryptoProtocolVersionProto cryptoProtocolVersion = 2;

    pub fn clear_cryptoProtocolVersion(&mut self) {
        self.cryptoProtocolVersion = ::std::option::Option::None;
    }

    pub fn has_cryptoProtocolVersion(&self) -> bool {
        self.cryptoProtocolVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cryptoProtocolVersion(&mut self, v: CryptoProtocolVersionProto) {
        self.cryptoProtocolVersion = ::std::option::Option::Some(v);
    }

    pub fn get_cryptoProtocolVersion<'a>(&self) -> CryptoProtocolVersionProto {
        self.cryptoProtocolVersion.unwrap_or(CryptoProtocolVersionProto::UNKNOWN_PROTOCOL_VERSION)
    }

    // required string keyName = 3;

    pub fn clear_keyName(&mut self) {
        self.keyName.clear();
    }

    pub fn has_keyName(&self) -> bool {
        self.keyName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_keyName(&mut self, v: ::std::string::String) {
        self.keyName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_keyName<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.keyName.is_none() {
            self.keyName.set_default();
        };
        self.keyName.as_mut().unwrap()
    }

    // Take field
    pub fn take_keyName(&mut self) -> ::std::string::String {
        self.keyName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_keyName<'a>(&'a self) -> &'a str {
        match self.keyName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for ZoneEncryptionInfoProto {
    fn is_initialized(&self) -> bool {
        if self.suite.is_none() {
            return false;
        };
        if self.cryptoProtocolVersion.is_none() {
            return false;
        };
        if self.keyName.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.suite = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.cryptoProtocolVersion = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.keyName.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.suite.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.cryptoProtocolVersion.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in self.keyName.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.suite {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.cryptoProtocolVersion {
            try!(os.write_enum(2, v as i32));
        };
        if let Some(v) = self.keyName.as_ref() {
            try!(os.write_string(3, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ZoneEncryptionInfoProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ZoneEncryptionInfoProto {
    fn new() -> ZoneEncryptionInfoProto {
        ZoneEncryptionInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ZoneEncryptionInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "suite",
                    ZoneEncryptionInfoProto::has_suite,
                    ZoneEncryptionInfoProto::get_suite,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "cryptoProtocolVersion",
                    ZoneEncryptionInfoProto::has_cryptoProtocolVersion,
                    ZoneEncryptionInfoProto::get_cryptoProtocolVersion,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "keyName",
                    ZoneEncryptionInfoProto::has_keyName,
                    ZoneEncryptionInfoProto::get_keyName,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ZoneEncryptionInfoProto>(
                    "ZoneEncryptionInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ZoneEncryptionInfoProto {
    fn clear(&mut self) {
        self.clear_suite();
        self.clear_cryptoProtocolVersion();
        self.clear_keyName();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ZoneEncryptionInfoProto {
    fn eq(&self, other: &ZoneEncryptionInfoProto) -> bool {
        self.suite == other.suite &&
        self.cryptoProtocolVersion == other.cryptoProtocolVersion &&
        self.keyName == other.keyName &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ZoneEncryptionInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CipherOptionProto {
    // message fields
    suite: ::std::option::Option<CipherSuiteProto>,
    inKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    inIv: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    outKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    outIv: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CipherOptionProto {
    pub fn new() -> CipherOptionProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CipherOptionProto {
        static mut instance: ::protobuf::lazy::Lazy<CipherOptionProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CipherOptionProto,
        };
        unsafe {
            instance.get(|| {
                CipherOptionProto {
                    suite: ::std::option::Option::None,
                    inKey: ::protobuf::SingularField::none(),
                    inIv: ::protobuf::SingularField::none(),
                    outKey: ::protobuf::SingularField::none(),
                    outIv: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.CipherSuiteProto suite = 1;

    pub fn clear_suite(&mut self) {
        self.suite = ::std::option::Option::None;
    }

    pub fn has_suite(&self) -> bool {
        self.suite.is_some()
    }

    // Param is passed by value, moved
    pub fn set_suite(&mut self, v: CipherSuiteProto) {
        self.suite = ::std::option::Option::Some(v);
    }

    pub fn get_suite<'a>(&self) -> CipherSuiteProto {
        self.suite.unwrap_or(CipherSuiteProto::UNKNOWN)
    }

    // optional bytes inKey = 2;

    pub fn clear_inKey(&mut self) {
        self.inKey.clear();
    }

    pub fn has_inKey(&self) -> bool {
        self.inKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.inKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_inKey<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.inKey.is_none() {
            self.inKey.set_default();
        };
        self.inKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_inKey(&mut self) -> ::std::vec::Vec<u8> {
        self.inKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_inKey<'a>(&'a self) -> &'a [u8] {
        match self.inKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes inIv = 3;

    pub fn clear_inIv(&mut self) {
        self.inIv.clear();
    }

    pub fn has_inIv(&self) -> bool {
        self.inIv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_inIv(&mut self, v: ::std::vec::Vec<u8>) {
        self.inIv = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_inIv<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.inIv.is_none() {
            self.inIv.set_default();
        };
        self.inIv.as_mut().unwrap()
    }

    // Take field
    pub fn take_inIv(&mut self) -> ::std::vec::Vec<u8> {
        self.inIv.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_inIv<'a>(&'a self) -> &'a [u8] {
        match self.inIv.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes outKey = 4;

    pub fn clear_outKey(&mut self) {
        self.outKey.clear();
    }

    pub fn has_outKey(&self) -> bool {
        self.outKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_outKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.outKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_outKey<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.outKey.is_none() {
            self.outKey.set_default();
        };
        self.outKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_outKey(&mut self) -> ::std::vec::Vec<u8> {
        self.outKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_outKey<'a>(&'a self) -> &'a [u8] {
        match self.outKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes outIv = 5;

    pub fn clear_outIv(&mut self) {
        self.outIv.clear();
    }

    pub fn has_outIv(&self) -> bool {
        self.outIv.is_some()
    }

    // Param is passed by value, moved
    pub fn set_outIv(&mut self, v: ::std::vec::Vec<u8>) {
        self.outIv = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_outIv<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.outIv.is_none() {
            self.outIv.set_default();
        };
        self.outIv.as_mut().unwrap()
    }

    // Take field
    pub fn take_outIv(&mut self) -> ::std::vec::Vec<u8> {
        self.outIv.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_outIv<'a>(&'a self) -> &'a [u8] {
        match self.outIv.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for CipherOptionProto {
    fn is_initialized(&self) -> bool {
        if self.suite.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.suite = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.inKey.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.inIv.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.outKey.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.outIv.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.suite.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.inKey.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in self.inIv.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in self.outKey.iter() {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        for value in self.outIv.iter() {
            my_size += ::protobuf::rt::bytes_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.suite {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.inKey.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.inIv.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        if let Some(v) = self.outKey.as_ref() {
            try!(os.write_bytes(4, &v));
        };
        if let Some(v) = self.outIv.as_ref() {
            try!(os.write_bytes(5, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<CipherOptionProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CipherOptionProto {
    fn new() -> CipherOptionProto {
        CipherOptionProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CipherOptionProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "suite",
                    CipherOptionProto::has_suite,
                    CipherOptionProto::get_suite,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "inKey",
                    CipherOptionProto::has_inKey,
                    CipherOptionProto::get_inKey,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "inIv",
                    CipherOptionProto::has_inIv,
                    CipherOptionProto::get_inIv,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "outKey",
                    CipherOptionProto::has_outKey,
                    CipherOptionProto::get_outKey,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "outIv",
                    CipherOptionProto::has_outIv,
                    CipherOptionProto::get_outIv,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CipherOptionProto>(
                    "CipherOptionProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CipherOptionProto {
    fn clear(&mut self) {
        self.clear_suite();
        self.clear_inKey();
        self.clear_inIv();
        self.clear_outKey();
        self.clear_outIv();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CipherOptionProto {
    fn eq(&self, other: &CipherOptionProto) -> bool {
        self.suite == other.suite &&
        self.inKey == other.inKey &&
        self.inIv == other.inIv &&
        self.outKey == other.outKey &&
        self.outIv == other.outIv &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CipherOptionProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct LocatedBlocksProto {
    // message fields
    fileLength: ::std::option::Option<u64>,
    blocks: ::protobuf::RepeatedField<LocatedBlockProto>,
    underConstruction: ::std::option::Option<bool>,
    lastBlock: ::protobuf::SingularPtrField<LocatedBlockProto>,
    isLastBlockComplete: ::std::option::Option<bool>,
    fileEncryptionInfo: ::protobuf::SingularPtrField<FileEncryptionInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl LocatedBlocksProto {
    pub fn new() -> LocatedBlocksProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LocatedBlocksProto {
        static mut instance: ::protobuf::lazy::Lazy<LocatedBlocksProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LocatedBlocksProto,
        };
        unsafe {
            instance.get(|| {
                LocatedBlocksProto {
                    fileLength: ::std::option::Option::None,
                    blocks: ::protobuf::RepeatedField::new(),
                    underConstruction: ::std::option::Option::None,
                    lastBlock: ::protobuf::SingularPtrField::none(),
                    isLastBlockComplete: ::std::option::Option::None,
                    fileEncryptionInfo: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint64 fileLength = 1;

    pub fn clear_fileLength(&mut self) {
        self.fileLength = ::std::option::Option::None;
    }

    pub fn has_fileLength(&self) -> bool {
        self.fileLength.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileLength(&mut self, v: u64) {
        self.fileLength = ::std::option::Option::Some(v);
    }

    pub fn get_fileLength<'a>(&self) -> u64 {
        self.fileLength.unwrap_or(0)
    }

    // repeated .hadoop.hdfs.LocatedBlockProto blocks = 2;

    pub fn clear_blocks(&mut self) {
        self.blocks.clear();
    }

    // Param is passed by value, moved
    pub fn set_blocks(&mut self, v: ::protobuf::RepeatedField<LocatedBlockProto>) {
        self.blocks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_blocks<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<LocatedBlockProto> {
        &mut self.blocks
    }

    // Take field
    pub fn take_blocks(&mut self) -> ::protobuf::RepeatedField<LocatedBlockProto> {
        ::std::mem::replace(&mut self.blocks, ::protobuf::RepeatedField::new())
    }

    pub fn get_blocks<'a>(&'a self) -> &'a [LocatedBlockProto] {
        &self.blocks
    }

    // required bool underConstruction = 3;

    pub fn clear_underConstruction(&mut self) {
        self.underConstruction = ::std::option::Option::None;
    }

    pub fn has_underConstruction(&self) -> bool {
        self.underConstruction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_underConstruction(&mut self, v: bool) {
        self.underConstruction = ::std::option::Option::Some(v);
    }

    pub fn get_underConstruction<'a>(&self) -> bool {
        self.underConstruction.unwrap_or(false)
    }

    // optional .hadoop.hdfs.LocatedBlockProto lastBlock = 4;

    pub fn clear_lastBlock(&mut self) {
        self.lastBlock.clear();
    }

    pub fn has_lastBlock(&self) -> bool {
        self.lastBlock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastBlock(&mut self, v: LocatedBlockProto) {
        self.lastBlock = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lastBlock<'a>(&'a mut self) -> &'a mut LocatedBlockProto {
        if self.lastBlock.is_none() {
            self.lastBlock.set_default();
        };
        self.lastBlock.as_mut().unwrap()
    }

    // Take field
    pub fn take_lastBlock(&mut self) -> LocatedBlockProto {
        self.lastBlock.take().unwrap_or_else(|| LocatedBlockProto::new())
    }

    pub fn get_lastBlock<'a>(&'a self) -> &'a LocatedBlockProto {
        self.lastBlock.as_ref().unwrap_or_else(|| LocatedBlockProto::default_instance())
    }

    // required bool isLastBlockComplete = 5;

    pub fn clear_isLastBlockComplete(&mut self) {
        self.isLastBlockComplete = ::std::option::Option::None;
    }

    pub fn has_isLastBlockComplete(&self) -> bool {
        self.isLastBlockComplete.is_some()
    }

    // Param is passed by value, moved
    pub fn set_isLastBlockComplete(&mut self, v: bool) {
        self.isLastBlockComplete = ::std::option::Option::Some(v);
    }

    pub fn get_isLastBlockComplete<'a>(&self) -> bool {
        self.isLastBlockComplete.unwrap_or(false)
    }

    // optional .hadoop.hdfs.FileEncryptionInfoProto fileEncryptionInfo = 6;

    pub fn clear_fileEncryptionInfo(&mut self) {
        self.fileEncryptionInfo.clear();
    }

    pub fn has_fileEncryptionInfo(&self) -> bool {
        self.fileEncryptionInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileEncryptionInfo(&mut self, v: FileEncryptionInfoProto) {
        self.fileEncryptionInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fileEncryptionInfo<'a>(&'a mut self) -> &'a mut FileEncryptionInfoProto {
        if self.fileEncryptionInfo.is_none() {
            self.fileEncryptionInfo.set_default();
        };
        self.fileEncryptionInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_fileEncryptionInfo(&mut self) -> FileEncryptionInfoProto {
        self.fileEncryptionInfo.take().unwrap_or_else(|| FileEncryptionInfoProto::new())
    }

    pub fn get_fileEncryptionInfo<'a>(&'a self) -> &'a FileEncryptionInfoProto {
        self.fileEncryptionInfo.as_ref().unwrap_or_else(|| FileEncryptionInfoProto::default_instance())
    }
}

impl ::protobuf::Message for LocatedBlocksProto {
    fn is_initialized(&self) -> bool {
        if self.fileLength.is_none() {
            return false;
        };
        if self.underConstruction.is_none() {
            return false;
        };
        if self.isLastBlockComplete.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.fileLength = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.blocks));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.underConstruction = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.lastBlock.set_default();
                    try!(is.merge_message(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.isLastBlockComplete = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.fileEncryptionInfo.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.fileLength.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.blocks.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.underConstruction.is_some() {
            my_size += 2;
        };
        for value in self.lastBlock.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.isLastBlockComplete.is_some() {
            my_size += 2;
        };
        for value in self.fileEncryptionInfo.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.fileLength {
            try!(os.write_uint64(1, v));
        };
        for v in self.blocks.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.underConstruction {
            try!(os.write_bool(3, v));
        };
        if let Some(v) = self.lastBlock.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.isLastBlockComplete {
            try!(os.write_bool(5, v));
        };
        if let Some(v) = self.fileEncryptionInfo.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<LocatedBlocksProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LocatedBlocksProto {
    fn new() -> LocatedBlocksProto {
        LocatedBlocksProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<LocatedBlocksProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "fileLength",
                    LocatedBlocksProto::has_fileLength,
                    LocatedBlocksProto::get_fileLength,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "blocks",
                    LocatedBlocksProto::get_blocks,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "underConstruction",
                    LocatedBlocksProto::has_underConstruction,
                    LocatedBlocksProto::get_underConstruction,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "lastBlock",
                    LocatedBlocksProto::has_lastBlock,
                    LocatedBlocksProto::get_lastBlock,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "isLastBlockComplete",
                    LocatedBlocksProto::has_isLastBlockComplete,
                    LocatedBlocksProto::get_isLastBlockComplete,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "fileEncryptionInfo",
                    LocatedBlocksProto::has_fileEncryptionInfo,
                    LocatedBlocksProto::get_fileEncryptionInfo,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LocatedBlocksProto>(
                    "LocatedBlocksProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LocatedBlocksProto {
    fn clear(&mut self) {
        self.clear_fileLength();
        self.clear_blocks();
        self.clear_underConstruction();
        self.clear_lastBlock();
        self.clear_isLastBlockComplete();
        self.clear_fileEncryptionInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for LocatedBlocksProto {
    fn eq(&self, other: &LocatedBlocksProto) -> bool {
        self.fileLength == other.fileLength &&
        self.blocks == other.blocks &&
        self.underConstruction == other.underConstruction &&
        self.lastBlock == other.lastBlock &&
        self.isLastBlockComplete == other.isLastBlockComplete &&
        self.fileEncryptionInfo == other.fileEncryptionInfo &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for LocatedBlocksProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct HdfsFileStatusProto {
    // message fields
    fileType: ::std::option::Option<HdfsFileStatusProto_FileType>,
    path: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    length: ::std::option::Option<u64>,
    permission: ::protobuf::SingularPtrField<FsPermissionProto>,
    owner: ::protobuf::SingularField<::std::string::String>,
    group: ::protobuf::SingularField<::std::string::String>,
    modification_time: ::std::option::Option<u64>,
    access_time: ::std::option::Option<u64>,
    symlink: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    block_replication: ::std::option::Option<u32>,
    blocksize: ::std::option::Option<u64>,
    locations: ::protobuf::SingularPtrField<LocatedBlocksProto>,
    fileId: ::std::option::Option<u64>,
    childrenNum: ::std::option::Option<i32>,
    fileEncryptionInfo: ::protobuf::SingularPtrField<FileEncryptionInfoProto>,
    storagePolicy: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl HdfsFileStatusProto {
    pub fn new() -> HdfsFileStatusProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HdfsFileStatusProto {
        static mut instance: ::protobuf::lazy::Lazy<HdfsFileStatusProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HdfsFileStatusProto,
        };
        unsafe {
            instance.get(|| {
                HdfsFileStatusProto {
                    fileType: ::std::option::Option::None,
                    path: ::protobuf::SingularField::none(),
                    length: ::std::option::Option::None,
                    permission: ::protobuf::SingularPtrField::none(),
                    owner: ::protobuf::SingularField::none(),
                    group: ::protobuf::SingularField::none(),
                    modification_time: ::std::option::Option::None,
                    access_time: ::std::option::Option::None,
                    symlink: ::protobuf::SingularField::none(),
                    block_replication: ::std::option::Option::None,
                    blocksize: ::std::option::Option::None,
                    locations: ::protobuf::SingularPtrField::none(),
                    fileId: ::std::option::Option::None,
                    childrenNum: ::std::option::Option::None,
                    fileEncryptionInfo: ::protobuf::SingularPtrField::none(),
                    storagePolicy: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.HdfsFileStatusProto.FileType fileType = 1;

    pub fn clear_fileType(&mut self) {
        self.fileType = ::std::option::Option::None;
    }

    pub fn has_fileType(&self) -> bool {
        self.fileType.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileType(&mut self, v: HdfsFileStatusProto_FileType) {
        self.fileType = ::std::option::Option::Some(v);
    }

    pub fn get_fileType<'a>(&self) -> HdfsFileStatusProto_FileType {
        self.fileType.unwrap_or(HdfsFileStatusProto_FileType::IS_DIR)
    }

    // required bytes path = 2;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::vec::Vec<u8>) {
        self.path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.path.is_none() {
            self.path.set_default();
        };
        self.path.as_mut().unwrap()
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::vec::Vec<u8> {
        self.path.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_path<'a>(&'a self) -> &'a [u8] {
        match self.path.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required uint64 length = 3;

    pub fn clear_length(&mut self) {
        self.length = ::std::option::Option::None;
    }

    pub fn has_length(&self) -> bool {
        self.length.is_some()
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: u64) {
        self.length = ::std::option::Option::Some(v);
    }

    pub fn get_length<'a>(&self) -> u64 {
        self.length.unwrap_or(0)
    }

    // required .hadoop.hdfs.FsPermissionProto permission = 4;

    pub fn clear_permission(&mut self) {
        self.permission.clear();
    }

    pub fn has_permission(&self) -> bool {
        self.permission.is_some()
    }

    // Param is passed by value, moved
    pub fn set_permission(&mut self, v: FsPermissionProto) {
        self.permission = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_permission<'a>(&'a mut self) -> &'a mut FsPermissionProto {
        if self.permission.is_none() {
            self.permission.set_default();
        };
        self.permission.as_mut().unwrap()
    }

    // Take field
    pub fn take_permission(&mut self) -> FsPermissionProto {
        self.permission.take().unwrap_or_else(|| FsPermissionProto::new())
    }

    pub fn get_permission<'a>(&'a self) -> &'a FsPermissionProto {
        self.permission.as_ref().unwrap_or_else(|| FsPermissionProto::default_instance())
    }

    // required string owner = 5;

    pub fn clear_owner(&mut self) {
        self.owner.clear();
    }

    pub fn has_owner(&self) -> bool {
        self.owner.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner(&mut self, v: ::std::string::String) {
        self.owner = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_owner<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.owner.is_none() {
            self.owner.set_default();
        };
        self.owner.as_mut().unwrap()
    }

    // Take field
    pub fn take_owner(&mut self) -> ::std::string::String {
        self.owner.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_owner<'a>(&'a self) -> &'a str {
        match self.owner.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string group = 6;

    pub fn clear_group(&mut self) {
        self.group.clear();
    }

    pub fn has_group(&self) -> bool {
        self.group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_group(&mut self, v: ::std::string::String) {
        self.group = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_group<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.group.is_none() {
            self.group.set_default();
        };
        self.group.as_mut().unwrap()
    }

    // Take field
    pub fn take_group(&mut self) -> ::std::string::String {
        self.group.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_group<'a>(&'a self) -> &'a str {
        match self.group.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required uint64 modification_time = 7;

    pub fn clear_modification_time(&mut self) {
        self.modification_time = ::std::option::Option::None;
    }

    pub fn has_modification_time(&self) -> bool {
        self.modification_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_modification_time(&mut self, v: u64) {
        self.modification_time = ::std::option::Option::Some(v);
    }

    pub fn get_modification_time<'a>(&self) -> u64 {
        self.modification_time.unwrap_or(0)
    }

    // required uint64 access_time = 8;

    pub fn clear_access_time(&mut self) {
        self.access_time = ::std::option::Option::None;
    }

    pub fn has_access_time(&self) -> bool {
        self.access_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_access_time(&mut self, v: u64) {
        self.access_time = ::std::option::Option::Some(v);
    }

    pub fn get_access_time<'a>(&self) -> u64 {
        self.access_time.unwrap_or(0)
    }

    // optional bytes symlink = 9;

    pub fn clear_symlink(&mut self) {
        self.symlink.clear();
    }

    pub fn has_symlink(&self) -> bool {
        self.symlink.is_some()
    }

    // Param is passed by value, moved
    pub fn set_symlink(&mut self, v: ::std::vec::Vec<u8>) {
        self.symlink = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_symlink<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.symlink.is_none() {
            self.symlink.set_default();
        };
        self.symlink.as_mut().unwrap()
    }

    // Take field
    pub fn take_symlink(&mut self) -> ::std::vec::Vec<u8> {
        self.symlink.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_symlink<'a>(&'a self) -> &'a [u8] {
        match self.symlink.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional uint32 block_replication = 10;

    pub fn clear_block_replication(&mut self) {
        self.block_replication = ::std::option::Option::None;
    }

    pub fn has_block_replication(&self) -> bool {
        self.block_replication.is_some()
    }

    // Param is passed by value, moved
    pub fn set_block_replication(&mut self, v: u32) {
        self.block_replication = ::std::option::Option::Some(v);
    }

    pub fn get_block_replication<'a>(&self) -> u32 {
        self.block_replication.unwrap_or(0u32)
    }

    // optional uint64 blocksize = 11;

    pub fn clear_blocksize(&mut self) {
        self.blocksize = ::std::option::Option::None;
    }

    pub fn has_blocksize(&self) -> bool {
        self.blocksize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blocksize(&mut self, v: u64) {
        self.blocksize = ::std::option::Option::Some(v);
    }

    pub fn get_blocksize<'a>(&self) -> u64 {
        self.blocksize.unwrap_or(0u64)
    }

    // optional .hadoop.hdfs.LocatedBlocksProto locations = 12;

    pub fn clear_locations(&mut self) {
        self.locations.clear();
    }

    pub fn has_locations(&self) -> bool {
        self.locations.is_some()
    }

    // Param is passed by value, moved
    pub fn set_locations(&mut self, v: LocatedBlocksProto) {
        self.locations = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_locations<'a>(&'a mut self) -> &'a mut LocatedBlocksProto {
        if self.locations.is_none() {
            self.locations.set_default();
        };
        self.locations.as_mut().unwrap()
    }

    // Take field
    pub fn take_locations(&mut self) -> LocatedBlocksProto {
        self.locations.take().unwrap_or_else(|| LocatedBlocksProto::new())
    }

    pub fn get_locations<'a>(&'a self) -> &'a LocatedBlocksProto {
        self.locations.as_ref().unwrap_or_else(|| LocatedBlocksProto::default_instance())
    }

    // optional uint64 fileId = 13;

    pub fn clear_fileId(&mut self) {
        self.fileId = ::std::option::Option::None;
    }

    pub fn has_fileId(&self) -> bool {
        self.fileId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileId(&mut self, v: u64) {
        self.fileId = ::std::option::Option::Some(v);
    }

    pub fn get_fileId<'a>(&self) -> u64 {
        self.fileId.unwrap_or(0u64)
    }

    // optional int32 childrenNum = 14;

    pub fn clear_childrenNum(&mut self) {
        self.childrenNum = ::std::option::Option::None;
    }

    pub fn has_childrenNum(&self) -> bool {
        self.childrenNum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_childrenNum(&mut self, v: i32) {
        self.childrenNum = ::std::option::Option::Some(v);
    }

    pub fn get_childrenNum<'a>(&self) -> i32 {
        self.childrenNum.unwrap_or(-1i32)
    }

    // optional .hadoop.hdfs.FileEncryptionInfoProto fileEncryptionInfo = 15;

    pub fn clear_fileEncryptionInfo(&mut self) {
        self.fileEncryptionInfo.clear();
    }

    pub fn has_fileEncryptionInfo(&self) -> bool {
        self.fileEncryptionInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileEncryptionInfo(&mut self, v: FileEncryptionInfoProto) {
        self.fileEncryptionInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fileEncryptionInfo<'a>(&'a mut self) -> &'a mut FileEncryptionInfoProto {
        if self.fileEncryptionInfo.is_none() {
            self.fileEncryptionInfo.set_default();
        };
        self.fileEncryptionInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_fileEncryptionInfo(&mut self) -> FileEncryptionInfoProto {
        self.fileEncryptionInfo.take().unwrap_or_else(|| FileEncryptionInfoProto::new())
    }

    pub fn get_fileEncryptionInfo<'a>(&'a self) -> &'a FileEncryptionInfoProto {
        self.fileEncryptionInfo.as_ref().unwrap_or_else(|| FileEncryptionInfoProto::default_instance())
    }

    // optional uint32 storagePolicy = 16;

    pub fn clear_storagePolicy(&mut self) {
        self.storagePolicy = ::std::option::Option::None;
    }

    pub fn has_storagePolicy(&self) -> bool {
        self.storagePolicy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storagePolicy(&mut self, v: u32) {
        self.storagePolicy = ::std::option::Option::Some(v);
    }

    pub fn get_storagePolicy<'a>(&self) -> u32 {
        self.storagePolicy.unwrap_or(0u32)
    }
}

impl ::protobuf::Message for HdfsFileStatusProto {
    fn is_initialized(&self) -> bool {
        if self.fileType.is_none() {
            return false;
        };
        if self.path.is_none() {
            return false;
        };
        if self.length.is_none() {
            return false;
        };
        if self.permission.is_none() {
            return false;
        };
        if self.owner.is_none() {
            return false;
        };
        if self.group.is_none() {
            return false;
        };
        if self.modification_time.is_none() {
            return false;
        };
        if self.access_time.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.fileType = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.path.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.length = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.permission.set_default();
                    try!(is.merge_message(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.owner.set_default();
                    try!(is.read_string_into(tmp))
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.group.set_default();
                    try!(is.read_string_into(tmp))
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.modification_time = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.access_time = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.symlink.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.block_replication = ::std::option::Option::Some(tmp);
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.blocksize = ::std::option::Option::Some(tmp);
                },
                12 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.locations.set_default();
                    try!(is.merge_message(tmp))
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.fileId = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.childrenNum = ::std::option::Option::Some(tmp);
                },
                15 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.fileEncryptionInfo.set_default();
                    try!(is.merge_message(tmp))
                },
                16 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.storagePolicy = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.fileType.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.path.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in self.length.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.permission.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.owner.iter() {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        for value in self.group.iter() {
            my_size += ::protobuf::rt::string_size(6, &value);
        };
        for value in self.modification_time.iter() {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.access_time.iter() {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.symlink.iter() {
            my_size += ::protobuf::rt::bytes_size(9, &value);
        };
        for value in self.block_replication.iter() {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.blocksize.iter() {
            my_size += ::protobuf::rt::value_size(11, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.locations.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.fileId.iter() {
            my_size += ::protobuf::rt::value_size(13, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.childrenNum.iter() {
            my_size += ::protobuf::rt::value_size(14, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.fileEncryptionInfo.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.storagePolicy.iter() {
            my_size += ::protobuf::rt::value_size(16, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.fileType {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.path.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.length {
            try!(os.write_uint64(3, v));
        };
        if let Some(v) = self.permission.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.owner.as_ref() {
            try!(os.write_string(5, &v));
        };
        if let Some(v) = self.group.as_ref() {
            try!(os.write_string(6, &v));
        };
        if let Some(v) = self.modification_time {
            try!(os.write_uint64(7, v));
        };
        if let Some(v) = self.access_time {
            try!(os.write_uint64(8, v));
        };
        if let Some(v) = self.symlink.as_ref() {
            try!(os.write_bytes(9, &v));
        };
        if let Some(v) = self.block_replication {
            try!(os.write_uint32(10, v));
        };
        if let Some(v) = self.blocksize {
            try!(os.write_uint64(11, v));
        };
        if let Some(v) = self.locations.as_ref() {
            try!(os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.fileId {
            try!(os.write_uint64(13, v));
        };
        if let Some(v) = self.childrenNum {
            try!(os.write_int32(14, v));
        };
        if let Some(v) = self.fileEncryptionInfo.as_ref() {
            try!(os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.storagePolicy {
            try!(os.write_uint32(16, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<HdfsFileStatusProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for HdfsFileStatusProto {
    fn new() -> HdfsFileStatusProto {
        HdfsFileStatusProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<HdfsFileStatusProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "fileType",
                    HdfsFileStatusProto::has_fileType,
                    HdfsFileStatusProto::get_fileType,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "path",
                    HdfsFileStatusProto::has_path,
                    HdfsFileStatusProto::get_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "length",
                    HdfsFileStatusProto::has_length,
                    HdfsFileStatusProto::get_length,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "permission",
                    HdfsFileStatusProto::has_permission,
                    HdfsFileStatusProto::get_permission,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "owner",
                    HdfsFileStatusProto::has_owner,
                    HdfsFileStatusProto::get_owner,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "group",
                    HdfsFileStatusProto::has_group,
                    HdfsFileStatusProto::get_group,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "modification_time",
                    HdfsFileStatusProto::has_modification_time,
                    HdfsFileStatusProto::get_modification_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "access_time",
                    HdfsFileStatusProto::has_access_time,
                    HdfsFileStatusProto::get_access_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "symlink",
                    HdfsFileStatusProto::has_symlink,
                    HdfsFileStatusProto::get_symlink,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "block_replication",
                    HdfsFileStatusProto::has_block_replication,
                    HdfsFileStatusProto::get_block_replication,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "blocksize",
                    HdfsFileStatusProto::has_blocksize,
                    HdfsFileStatusProto::get_blocksize,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "locations",
                    HdfsFileStatusProto::has_locations,
                    HdfsFileStatusProto::get_locations,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "fileId",
                    HdfsFileStatusProto::has_fileId,
                    HdfsFileStatusProto::get_fileId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "childrenNum",
                    HdfsFileStatusProto::has_childrenNum,
                    HdfsFileStatusProto::get_childrenNum,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "fileEncryptionInfo",
                    HdfsFileStatusProto::has_fileEncryptionInfo,
                    HdfsFileStatusProto::get_fileEncryptionInfo,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "storagePolicy",
                    HdfsFileStatusProto::has_storagePolicy,
                    HdfsFileStatusProto::get_storagePolicy,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HdfsFileStatusProto>(
                    "HdfsFileStatusProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HdfsFileStatusProto {
    fn clear(&mut self) {
        self.clear_fileType();
        self.clear_path();
        self.clear_length();
        self.clear_permission();
        self.clear_owner();
        self.clear_group();
        self.clear_modification_time();
        self.clear_access_time();
        self.clear_symlink();
        self.clear_block_replication();
        self.clear_blocksize();
        self.clear_locations();
        self.clear_fileId();
        self.clear_childrenNum();
        self.clear_fileEncryptionInfo();
        self.clear_storagePolicy();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for HdfsFileStatusProto {
    fn eq(&self, other: &HdfsFileStatusProto) -> bool {
        self.fileType == other.fileType &&
        self.path == other.path &&
        self.length == other.length &&
        self.permission == other.permission &&
        self.owner == other.owner &&
        self.group == other.group &&
        self.modification_time == other.modification_time &&
        self.access_time == other.access_time &&
        self.symlink == other.symlink &&
        self.block_replication == other.block_replication &&
        self.blocksize == other.blocksize &&
        self.locations == other.locations &&
        self.fileId == other.fileId &&
        self.childrenNum == other.childrenNum &&
        self.fileEncryptionInfo == other.fileEncryptionInfo &&
        self.storagePolicy == other.storagePolicy &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for HdfsFileStatusProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum HdfsFileStatusProto_FileType {
    IS_DIR = 1,
    IS_FILE = 2,
    IS_SYMLINK = 3,
}

impl ::protobuf::ProtobufEnum for HdfsFileStatusProto_FileType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<HdfsFileStatusProto_FileType> {
        match value {
            1 => ::std::option::Option::Some(HdfsFileStatusProto_FileType::IS_DIR),
            2 => ::std::option::Option::Some(HdfsFileStatusProto_FileType::IS_FILE),
            3 => ::std::option::Option::Some(HdfsFileStatusProto_FileType::IS_SYMLINK),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<HdfsFileStatusProto_FileType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("HdfsFileStatusProto_FileType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for HdfsFileStatusProto_FileType {
}

#[derive(Clone,Default)]
pub struct FsServerDefaultsProto {
    // message fields
    blockSize: ::std::option::Option<u64>,
    bytesPerChecksum: ::std::option::Option<u32>,
    writePacketSize: ::std::option::Option<u32>,
    replication: ::std::option::Option<u32>,
    fileBufferSize: ::std::option::Option<u32>,
    encryptDataTransfer: ::std::option::Option<bool>,
    trashInterval: ::std::option::Option<u64>,
    checksumType: ::std::option::Option<ChecksumTypeProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl FsServerDefaultsProto {
    pub fn new() -> FsServerDefaultsProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FsServerDefaultsProto {
        static mut instance: ::protobuf::lazy::Lazy<FsServerDefaultsProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FsServerDefaultsProto,
        };
        unsafe {
            instance.get(|| {
                FsServerDefaultsProto {
                    blockSize: ::std::option::Option::None,
                    bytesPerChecksum: ::std::option::Option::None,
                    writePacketSize: ::std::option::Option::None,
                    replication: ::std::option::Option::None,
                    fileBufferSize: ::std::option::Option::None,
                    encryptDataTransfer: ::std::option::Option::None,
                    trashInterval: ::std::option::Option::None,
                    checksumType: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint64 blockSize = 1;

    pub fn clear_blockSize(&mut self) {
        self.blockSize = ::std::option::Option::None;
    }

    pub fn has_blockSize(&self) -> bool {
        self.blockSize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockSize(&mut self, v: u64) {
        self.blockSize = ::std::option::Option::Some(v);
    }

    pub fn get_blockSize<'a>(&self) -> u64 {
        self.blockSize.unwrap_or(0)
    }

    // required uint32 bytesPerChecksum = 2;

    pub fn clear_bytesPerChecksum(&mut self) {
        self.bytesPerChecksum = ::std::option::Option::None;
    }

    pub fn has_bytesPerChecksum(&self) -> bool {
        self.bytesPerChecksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bytesPerChecksum(&mut self, v: u32) {
        self.bytesPerChecksum = ::std::option::Option::Some(v);
    }

    pub fn get_bytesPerChecksum<'a>(&self) -> u32 {
        self.bytesPerChecksum.unwrap_or(0)
    }

    // required uint32 writePacketSize = 3;

    pub fn clear_writePacketSize(&mut self) {
        self.writePacketSize = ::std::option::Option::None;
    }

    pub fn has_writePacketSize(&self) -> bool {
        self.writePacketSize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_writePacketSize(&mut self, v: u32) {
        self.writePacketSize = ::std::option::Option::Some(v);
    }

    pub fn get_writePacketSize<'a>(&self) -> u32 {
        self.writePacketSize.unwrap_or(0)
    }

    // required uint32 replication = 4;

    pub fn clear_replication(&mut self) {
        self.replication = ::std::option::Option::None;
    }

    pub fn has_replication(&self) -> bool {
        self.replication.is_some()
    }

    // Param is passed by value, moved
    pub fn set_replication(&mut self, v: u32) {
        self.replication = ::std::option::Option::Some(v);
    }

    pub fn get_replication<'a>(&self) -> u32 {
        self.replication.unwrap_or(0)
    }

    // required uint32 fileBufferSize = 5;

    pub fn clear_fileBufferSize(&mut self) {
        self.fileBufferSize = ::std::option::Option::None;
    }

    pub fn has_fileBufferSize(&self) -> bool {
        self.fileBufferSize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileBufferSize(&mut self, v: u32) {
        self.fileBufferSize = ::std::option::Option::Some(v);
    }

    pub fn get_fileBufferSize<'a>(&self) -> u32 {
        self.fileBufferSize.unwrap_or(0)
    }

    // optional bool encryptDataTransfer = 6;

    pub fn clear_encryptDataTransfer(&mut self) {
        self.encryptDataTransfer = ::std::option::Option::None;
    }

    pub fn has_encryptDataTransfer(&self) -> bool {
        self.encryptDataTransfer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_encryptDataTransfer(&mut self, v: bool) {
        self.encryptDataTransfer = ::std::option::Option::Some(v);
    }

    pub fn get_encryptDataTransfer<'a>(&self) -> bool {
        self.encryptDataTransfer.unwrap_or(false)
    }

    // optional uint64 trashInterval = 7;

    pub fn clear_trashInterval(&mut self) {
        self.trashInterval = ::std::option::Option::None;
    }

    pub fn has_trashInterval(&self) -> bool {
        self.trashInterval.is_some()
    }

    // Param is passed by value, moved
    pub fn set_trashInterval(&mut self, v: u64) {
        self.trashInterval = ::std::option::Option::Some(v);
    }

    pub fn get_trashInterval<'a>(&self) -> u64 {
        self.trashInterval.unwrap_or(0u64)
    }

    // optional .hadoop.hdfs.ChecksumTypeProto checksumType = 8;

    pub fn clear_checksumType(&mut self) {
        self.checksumType = ::std::option::Option::None;
    }

    pub fn has_checksumType(&self) -> bool {
        self.checksumType.is_some()
    }

    // Param is passed by value, moved
    pub fn set_checksumType(&mut self, v: ChecksumTypeProto) {
        self.checksumType = ::std::option::Option::Some(v);
    }

    pub fn get_checksumType<'a>(&self) -> ChecksumTypeProto {
        self.checksumType.unwrap_or(ChecksumTypeProto::CHECKSUM_CRC32)
    }
}

impl ::protobuf::Message for FsServerDefaultsProto {
    fn is_initialized(&self) -> bool {
        if self.blockSize.is_none() {
            return false;
        };
        if self.bytesPerChecksum.is_none() {
            return false;
        };
        if self.writePacketSize.is_none() {
            return false;
        };
        if self.replication.is_none() {
            return false;
        };
        if self.fileBufferSize.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.blockSize = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.bytesPerChecksum = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.writePacketSize = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.replication = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.fileBufferSize = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.encryptDataTransfer = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.trashInterval = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.checksumType = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.blockSize.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.bytesPerChecksum.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.writePacketSize.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.replication.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.fileBufferSize.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.encryptDataTransfer.is_some() {
            my_size += 2;
        };
        for value in self.trashInterval.iter() {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.checksumType.iter() {
            my_size += ::protobuf::rt::enum_size(8, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.blockSize {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.bytesPerChecksum {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.writePacketSize {
            try!(os.write_uint32(3, v));
        };
        if let Some(v) = self.replication {
            try!(os.write_uint32(4, v));
        };
        if let Some(v) = self.fileBufferSize {
            try!(os.write_uint32(5, v));
        };
        if let Some(v) = self.encryptDataTransfer {
            try!(os.write_bool(6, v));
        };
        if let Some(v) = self.trashInterval {
            try!(os.write_uint64(7, v));
        };
        if let Some(v) = self.checksumType {
            try!(os.write_enum(8, v as i32));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<FsServerDefaultsProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for FsServerDefaultsProto {
    fn new() -> FsServerDefaultsProto {
        FsServerDefaultsProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<FsServerDefaultsProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "blockSize",
                    FsServerDefaultsProto::has_blockSize,
                    FsServerDefaultsProto::get_blockSize,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "bytesPerChecksum",
                    FsServerDefaultsProto::has_bytesPerChecksum,
                    FsServerDefaultsProto::get_bytesPerChecksum,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "writePacketSize",
                    FsServerDefaultsProto::has_writePacketSize,
                    FsServerDefaultsProto::get_writePacketSize,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "replication",
                    FsServerDefaultsProto::has_replication,
                    FsServerDefaultsProto::get_replication,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "fileBufferSize",
                    FsServerDefaultsProto::has_fileBufferSize,
                    FsServerDefaultsProto::get_fileBufferSize,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "encryptDataTransfer",
                    FsServerDefaultsProto::has_encryptDataTransfer,
                    FsServerDefaultsProto::get_encryptDataTransfer,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "trashInterval",
                    FsServerDefaultsProto::has_trashInterval,
                    FsServerDefaultsProto::get_trashInterval,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "checksumType",
                    FsServerDefaultsProto::has_checksumType,
                    FsServerDefaultsProto::get_checksumType,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FsServerDefaultsProto>(
                    "FsServerDefaultsProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FsServerDefaultsProto {
    fn clear(&mut self) {
        self.clear_blockSize();
        self.clear_bytesPerChecksum();
        self.clear_writePacketSize();
        self.clear_replication();
        self.clear_fileBufferSize();
        self.clear_encryptDataTransfer();
        self.clear_trashInterval();
        self.clear_checksumType();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for FsServerDefaultsProto {
    fn eq(&self, other: &FsServerDefaultsProto) -> bool {
        self.blockSize == other.blockSize &&
        self.bytesPerChecksum == other.bytesPerChecksum &&
        self.writePacketSize == other.writePacketSize &&
        self.replication == other.replication &&
        self.fileBufferSize == other.fileBufferSize &&
        self.encryptDataTransfer == other.encryptDataTransfer &&
        self.trashInterval == other.trashInterval &&
        self.checksumType == other.checksumType &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for FsServerDefaultsProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DirectoryListingProto {
    // message fields
    partialListing: ::protobuf::RepeatedField<HdfsFileStatusProto>,
    remainingEntries: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl DirectoryListingProto {
    pub fn new() -> DirectoryListingProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DirectoryListingProto {
        static mut instance: ::protobuf::lazy::Lazy<DirectoryListingProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DirectoryListingProto,
        };
        unsafe {
            instance.get(|| {
                DirectoryListingProto {
                    partialListing: ::protobuf::RepeatedField::new(),
                    remainingEntries: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .hadoop.hdfs.HdfsFileStatusProto partialListing = 1;

    pub fn clear_partialListing(&mut self) {
        self.partialListing.clear();
    }

    // Param is passed by value, moved
    pub fn set_partialListing(&mut self, v: ::protobuf::RepeatedField<HdfsFileStatusProto>) {
        self.partialListing = v;
    }

    // Mutable pointer to the field.
    pub fn mut_partialListing<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<HdfsFileStatusProto> {
        &mut self.partialListing
    }

    // Take field
    pub fn take_partialListing(&mut self) -> ::protobuf::RepeatedField<HdfsFileStatusProto> {
        ::std::mem::replace(&mut self.partialListing, ::protobuf::RepeatedField::new())
    }

    pub fn get_partialListing<'a>(&'a self) -> &'a [HdfsFileStatusProto] {
        &self.partialListing
    }

    // required uint32 remainingEntries = 2;

    pub fn clear_remainingEntries(&mut self) {
        self.remainingEntries = ::std::option::Option::None;
    }

    pub fn has_remainingEntries(&self) -> bool {
        self.remainingEntries.is_some()
    }

    // Param is passed by value, moved
    pub fn set_remainingEntries(&mut self, v: u32) {
        self.remainingEntries = ::std::option::Option::Some(v);
    }

    pub fn get_remainingEntries<'a>(&self) -> u32 {
        self.remainingEntries.unwrap_or(0)
    }
}

impl ::protobuf::Message for DirectoryListingProto {
    fn is_initialized(&self) -> bool {
        if self.remainingEntries.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.partialListing));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.remainingEntries = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.partialListing.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.remainingEntries.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.partialListing.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.remainingEntries {
            try!(os.write_uint32(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<DirectoryListingProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DirectoryListingProto {
    fn new() -> DirectoryListingProto {
        DirectoryListingProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DirectoryListingProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "partialListing",
                    DirectoryListingProto::get_partialListing,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "remainingEntries",
                    DirectoryListingProto::has_remainingEntries,
                    DirectoryListingProto::get_remainingEntries,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DirectoryListingProto>(
                    "DirectoryListingProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DirectoryListingProto {
    fn clear(&mut self) {
        self.clear_partialListing();
        self.clear_remainingEntries();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DirectoryListingProto {
    fn eq(&self, other: &DirectoryListingProto) -> bool {
        self.partialListing == other.partialListing &&
        self.remainingEntries == other.remainingEntries &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DirectoryListingProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SnapshottableDirectoryStatusProto {
    // message fields
    dirStatus: ::protobuf::SingularPtrField<HdfsFileStatusProto>,
    snapshot_quota: ::std::option::Option<u32>,
    snapshot_number: ::std::option::Option<u32>,
    parent_fullpath: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl SnapshottableDirectoryStatusProto {
    pub fn new() -> SnapshottableDirectoryStatusProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshottableDirectoryStatusProto {
        static mut instance: ::protobuf::lazy::Lazy<SnapshottableDirectoryStatusProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshottableDirectoryStatusProto,
        };
        unsafe {
            instance.get(|| {
                SnapshottableDirectoryStatusProto {
                    dirStatus: ::protobuf::SingularPtrField::none(),
                    snapshot_quota: ::std::option::Option::None,
                    snapshot_number: ::std::option::Option::None,
                    parent_fullpath: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.HdfsFileStatusProto dirStatus = 1;

    pub fn clear_dirStatus(&mut self) {
        self.dirStatus.clear();
    }

    pub fn has_dirStatus(&self) -> bool {
        self.dirStatus.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dirStatus(&mut self, v: HdfsFileStatusProto) {
        self.dirStatus = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_dirStatus<'a>(&'a mut self) -> &'a mut HdfsFileStatusProto {
        if self.dirStatus.is_none() {
            self.dirStatus.set_default();
        };
        self.dirStatus.as_mut().unwrap()
    }

    // Take field
    pub fn take_dirStatus(&mut self) -> HdfsFileStatusProto {
        self.dirStatus.take().unwrap_or_else(|| HdfsFileStatusProto::new())
    }

    pub fn get_dirStatus<'a>(&'a self) -> &'a HdfsFileStatusProto {
        self.dirStatus.as_ref().unwrap_or_else(|| HdfsFileStatusProto::default_instance())
    }

    // required uint32 snapshot_quota = 2;

    pub fn clear_snapshot_quota(&mut self) {
        self.snapshot_quota = ::std::option::Option::None;
    }

    pub fn has_snapshot_quota(&self) -> bool {
        self.snapshot_quota.is_some()
    }

    // Param is passed by value, moved
    pub fn set_snapshot_quota(&mut self, v: u32) {
        self.snapshot_quota = ::std::option::Option::Some(v);
    }

    pub fn get_snapshot_quota<'a>(&self) -> u32 {
        self.snapshot_quota.unwrap_or(0)
    }

    // required uint32 snapshot_number = 3;

    pub fn clear_snapshot_number(&mut self) {
        self.snapshot_number = ::std::option::Option::None;
    }

    pub fn has_snapshot_number(&self) -> bool {
        self.snapshot_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_snapshot_number(&mut self, v: u32) {
        self.snapshot_number = ::std::option::Option::Some(v);
    }

    pub fn get_snapshot_number<'a>(&self) -> u32 {
        self.snapshot_number.unwrap_or(0)
    }

    // required bytes parent_fullpath = 4;

    pub fn clear_parent_fullpath(&mut self) {
        self.parent_fullpath.clear();
    }

    pub fn has_parent_fullpath(&self) -> bool {
        self.parent_fullpath.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parent_fullpath(&mut self, v: ::std::vec::Vec<u8>) {
        self.parent_fullpath = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_parent_fullpath<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.parent_fullpath.is_none() {
            self.parent_fullpath.set_default();
        };
        self.parent_fullpath.as_mut().unwrap()
    }

    // Take field
    pub fn take_parent_fullpath(&mut self) -> ::std::vec::Vec<u8> {
        self.parent_fullpath.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_parent_fullpath<'a>(&'a self) -> &'a [u8] {
        match self.parent_fullpath.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for SnapshottableDirectoryStatusProto {
    fn is_initialized(&self) -> bool {
        if self.dirStatus.is_none() {
            return false;
        };
        if self.snapshot_quota.is_none() {
            return false;
        };
        if self.snapshot_number.is_none() {
            return false;
        };
        if self.parent_fullpath.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.dirStatus.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.snapshot_quota = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.snapshot_number = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.parent_fullpath.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.dirStatus.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.snapshot_quota.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.snapshot_number.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.parent_fullpath.iter() {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dirStatus.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.snapshot_quota {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.snapshot_number {
            try!(os.write_uint32(3, v));
        };
        if let Some(v) = self.parent_fullpath.as_ref() {
            try!(os.write_bytes(4, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SnapshottableDirectoryStatusProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SnapshottableDirectoryStatusProto {
    fn new() -> SnapshottableDirectoryStatusProto {
        SnapshottableDirectoryStatusProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshottableDirectoryStatusProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "dirStatus",
                    SnapshottableDirectoryStatusProto::has_dirStatus,
                    SnapshottableDirectoryStatusProto::get_dirStatus,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "snapshot_quota",
                    SnapshottableDirectoryStatusProto::has_snapshot_quota,
                    SnapshottableDirectoryStatusProto::get_snapshot_quota,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "snapshot_number",
                    SnapshottableDirectoryStatusProto::has_snapshot_number,
                    SnapshottableDirectoryStatusProto::get_snapshot_number,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "parent_fullpath",
                    SnapshottableDirectoryStatusProto::has_parent_fullpath,
                    SnapshottableDirectoryStatusProto::get_parent_fullpath,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapshottableDirectoryStatusProto>(
                    "SnapshottableDirectoryStatusProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshottableDirectoryStatusProto {
    fn clear(&mut self) {
        self.clear_dirStatus();
        self.clear_snapshot_quota();
        self.clear_snapshot_number();
        self.clear_parent_fullpath();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SnapshottableDirectoryStatusProto {
    fn eq(&self, other: &SnapshottableDirectoryStatusProto) -> bool {
        self.dirStatus == other.dirStatus &&
        self.snapshot_quota == other.snapshot_quota &&
        self.snapshot_number == other.snapshot_number &&
        self.parent_fullpath == other.parent_fullpath &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SnapshottableDirectoryStatusProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SnapshottableDirectoryListingProto {
    // message fields
    snapshottableDirListing: ::protobuf::RepeatedField<SnapshottableDirectoryStatusProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl SnapshottableDirectoryListingProto {
    pub fn new() -> SnapshottableDirectoryListingProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshottableDirectoryListingProto {
        static mut instance: ::protobuf::lazy::Lazy<SnapshottableDirectoryListingProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshottableDirectoryListingProto,
        };
        unsafe {
            instance.get(|| {
                SnapshottableDirectoryListingProto {
                    snapshottableDirListing: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .hadoop.hdfs.SnapshottableDirectoryStatusProto snapshottableDirListing = 1;

    pub fn clear_snapshottableDirListing(&mut self) {
        self.snapshottableDirListing.clear();
    }

    // Param is passed by value, moved
    pub fn set_snapshottableDirListing(&mut self, v: ::protobuf::RepeatedField<SnapshottableDirectoryStatusProto>) {
        self.snapshottableDirListing = v;
    }

    // Mutable pointer to the field.
    pub fn mut_snapshottableDirListing<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<SnapshottableDirectoryStatusProto> {
        &mut self.snapshottableDirListing
    }

    // Take field
    pub fn take_snapshottableDirListing(&mut self) -> ::protobuf::RepeatedField<SnapshottableDirectoryStatusProto> {
        ::std::mem::replace(&mut self.snapshottableDirListing, ::protobuf::RepeatedField::new())
    }

    pub fn get_snapshottableDirListing<'a>(&'a self) -> &'a [SnapshottableDirectoryStatusProto] {
        &self.snapshottableDirListing
    }
}

impl ::protobuf::Message for SnapshottableDirectoryListingProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.snapshottableDirListing));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.snapshottableDirListing.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.snapshottableDirListing.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SnapshottableDirectoryListingProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SnapshottableDirectoryListingProto {
    fn new() -> SnapshottableDirectoryListingProto {
        SnapshottableDirectoryListingProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshottableDirectoryListingProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "snapshottableDirListing",
                    SnapshottableDirectoryListingProto::get_snapshottableDirListing,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapshottableDirectoryListingProto>(
                    "SnapshottableDirectoryListingProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshottableDirectoryListingProto {
    fn clear(&mut self) {
        self.clear_snapshottableDirListing();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SnapshottableDirectoryListingProto {
    fn eq(&self, other: &SnapshottableDirectoryListingProto) -> bool {
        self.snapshottableDirListing == other.snapshottableDirListing &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SnapshottableDirectoryListingProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SnapshotDiffReportEntryProto {
    // message fields
    fullpath: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    modificationLabel: ::protobuf::SingularField<::std::string::String>,
    targetPath: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl SnapshotDiffReportEntryProto {
    pub fn new() -> SnapshotDiffReportEntryProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshotDiffReportEntryProto {
        static mut instance: ::protobuf::lazy::Lazy<SnapshotDiffReportEntryProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshotDiffReportEntryProto,
        };
        unsafe {
            instance.get(|| {
                SnapshotDiffReportEntryProto {
                    fullpath: ::protobuf::SingularField::none(),
                    modificationLabel: ::protobuf::SingularField::none(),
                    targetPath: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bytes fullpath = 1;

    pub fn clear_fullpath(&mut self) {
        self.fullpath.clear();
    }

    pub fn has_fullpath(&self) -> bool {
        self.fullpath.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fullpath(&mut self, v: ::std::vec::Vec<u8>) {
        self.fullpath = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fullpath<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.fullpath.is_none() {
            self.fullpath.set_default();
        };
        self.fullpath.as_mut().unwrap()
    }

    // Take field
    pub fn take_fullpath(&mut self) -> ::std::vec::Vec<u8> {
        self.fullpath.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_fullpath<'a>(&'a self) -> &'a [u8] {
        match self.fullpath.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // required string modificationLabel = 2;

    pub fn clear_modificationLabel(&mut self) {
        self.modificationLabel.clear();
    }

    pub fn has_modificationLabel(&self) -> bool {
        self.modificationLabel.is_some()
    }

    // Param is passed by value, moved
    pub fn set_modificationLabel(&mut self, v: ::std::string::String) {
        self.modificationLabel = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_modificationLabel<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.modificationLabel.is_none() {
            self.modificationLabel.set_default();
        };
        self.modificationLabel.as_mut().unwrap()
    }

    // Take field
    pub fn take_modificationLabel(&mut self) -> ::std::string::String {
        self.modificationLabel.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_modificationLabel<'a>(&'a self) -> &'a str {
        match self.modificationLabel.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bytes targetPath = 3;

    pub fn clear_targetPath(&mut self) {
        self.targetPath.clear();
    }

    pub fn has_targetPath(&self) -> bool {
        self.targetPath.is_some()
    }

    // Param is passed by value, moved
    pub fn set_targetPath(&mut self, v: ::std::vec::Vec<u8>) {
        self.targetPath = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_targetPath<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.targetPath.is_none() {
            self.targetPath.set_default();
        };
        self.targetPath.as_mut().unwrap()
    }

    // Take field
    pub fn take_targetPath(&mut self) -> ::std::vec::Vec<u8> {
        self.targetPath.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_targetPath<'a>(&'a self) -> &'a [u8] {
        match self.targetPath.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for SnapshotDiffReportEntryProto {
    fn is_initialized(&self) -> bool {
        if self.fullpath.is_none() {
            return false;
        };
        if self.modificationLabel.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.fullpath.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.modificationLabel.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.targetPath.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.fullpath.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.modificationLabel.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.targetPath.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.fullpath.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.modificationLabel.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.targetPath.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SnapshotDiffReportEntryProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SnapshotDiffReportEntryProto {
    fn new() -> SnapshotDiffReportEntryProto {
        SnapshotDiffReportEntryProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshotDiffReportEntryProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "fullpath",
                    SnapshotDiffReportEntryProto::has_fullpath,
                    SnapshotDiffReportEntryProto::get_fullpath,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "modificationLabel",
                    SnapshotDiffReportEntryProto::has_modificationLabel,
                    SnapshotDiffReportEntryProto::get_modificationLabel,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "targetPath",
                    SnapshotDiffReportEntryProto::has_targetPath,
                    SnapshotDiffReportEntryProto::get_targetPath,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapshotDiffReportEntryProto>(
                    "SnapshotDiffReportEntryProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshotDiffReportEntryProto {
    fn clear(&mut self) {
        self.clear_fullpath();
        self.clear_modificationLabel();
        self.clear_targetPath();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SnapshotDiffReportEntryProto {
    fn eq(&self, other: &SnapshotDiffReportEntryProto) -> bool {
        self.fullpath == other.fullpath &&
        self.modificationLabel == other.modificationLabel &&
        self.targetPath == other.targetPath &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SnapshotDiffReportEntryProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SnapshotDiffReportProto {
    // message fields
    snapshotRoot: ::protobuf::SingularField<::std::string::String>,
    fromSnapshot: ::protobuf::SingularField<::std::string::String>,
    toSnapshot: ::protobuf::SingularField<::std::string::String>,
    diffReportEntries: ::protobuf::RepeatedField<SnapshotDiffReportEntryProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl SnapshotDiffReportProto {
    pub fn new() -> SnapshotDiffReportProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshotDiffReportProto {
        static mut instance: ::protobuf::lazy::Lazy<SnapshotDiffReportProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshotDiffReportProto,
        };
        unsafe {
            instance.get(|| {
                SnapshotDiffReportProto {
                    snapshotRoot: ::protobuf::SingularField::none(),
                    fromSnapshot: ::protobuf::SingularField::none(),
                    toSnapshot: ::protobuf::SingularField::none(),
                    diffReportEntries: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string snapshotRoot = 1;

    pub fn clear_snapshotRoot(&mut self) {
        self.snapshotRoot.clear();
    }

    pub fn has_snapshotRoot(&self) -> bool {
        self.snapshotRoot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_snapshotRoot(&mut self, v: ::std::string::String) {
        self.snapshotRoot = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_snapshotRoot<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.snapshotRoot.is_none() {
            self.snapshotRoot.set_default();
        };
        self.snapshotRoot.as_mut().unwrap()
    }

    // Take field
    pub fn take_snapshotRoot(&mut self) -> ::std::string::String {
        self.snapshotRoot.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_snapshotRoot<'a>(&'a self) -> &'a str {
        match self.snapshotRoot.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string fromSnapshot = 2;

    pub fn clear_fromSnapshot(&mut self) {
        self.fromSnapshot.clear();
    }

    pub fn has_fromSnapshot(&self) -> bool {
        self.fromSnapshot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fromSnapshot(&mut self, v: ::std::string::String) {
        self.fromSnapshot = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fromSnapshot<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.fromSnapshot.is_none() {
            self.fromSnapshot.set_default();
        };
        self.fromSnapshot.as_mut().unwrap()
    }

    // Take field
    pub fn take_fromSnapshot(&mut self) -> ::std::string::String {
        self.fromSnapshot.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_fromSnapshot<'a>(&'a self) -> &'a str {
        match self.fromSnapshot.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string toSnapshot = 3;

    pub fn clear_toSnapshot(&mut self) {
        self.toSnapshot.clear();
    }

    pub fn has_toSnapshot(&self) -> bool {
        self.toSnapshot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_toSnapshot(&mut self, v: ::std::string::String) {
        self.toSnapshot = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_toSnapshot<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.toSnapshot.is_none() {
            self.toSnapshot.set_default();
        };
        self.toSnapshot.as_mut().unwrap()
    }

    // Take field
    pub fn take_toSnapshot(&mut self) -> ::std::string::String {
        self.toSnapshot.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_toSnapshot<'a>(&'a self) -> &'a str {
        match self.toSnapshot.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated .hadoop.hdfs.SnapshotDiffReportEntryProto diffReportEntries = 4;

    pub fn clear_diffReportEntries(&mut self) {
        self.diffReportEntries.clear();
    }

    // Param is passed by value, moved
    pub fn set_diffReportEntries(&mut self, v: ::protobuf::RepeatedField<SnapshotDiffReportEntryProto>) {
        self.diffReportEntries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_diffReportEntries<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<SnapshotDiffReportEntryProto> {
        &mut self.diffReportEntries
    }

    // Take field
    pub fn take_diffReportEntries(&mut self) -> ::protobuf::RepeatedField<SnapshotDiffReportEntryProto> {
        ::std::mem::replace(&mut self.diffReportEntries, ::protobuf::RepeatedField::new())
    }

    pub fn get_diffReportEntries<'a>(&'a self) -> &'a [SnapshotDiffReportEntryProto] {
        &self.diffReportEntries
    }
}

impl ::protobuf::Message for SnapshotDiffReportProto {
    fn is_initialized(&self) -> bool {
        if self.snapshotRoot.is_none() {
            return false;
        };
        if self.fromSnapshot.is_none() {
            return false;
        };
        if self.toSnapshot.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.snapshotRoot.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.fromSnapshot.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.toSnapshot.set_default();
                    try!(is.read_string_into(tmp))
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.diffReportEntries));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.snapshotRoot.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.fromSnapshot.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.toSnapshot.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in self.diffReportEntries.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.snapshotRoot.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.fromSnapshot.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.toSnapshot.as_ref() {
            try!(os.write_string(3, &v));
        };
        for v in self.diffReportEntries.iter() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SnapshotDiffReportProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SnapshotDiffReportProto {
    fn new() -> SnapshotDiffReportProto {
        SnapshotDiffReportProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshotDiffReportProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "snapshotRoot",
                    SnapshotDiffReportProto::has_snapshotRoot,
                    SnapshotDiffReportProto::get_snapshotRoot,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "fromSnapshot",
                    SnapshotDiffReportProto::has_fromSnapshot,
                    SnapshotDiffReportProto::get_fromSnapshot,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "toSnapshot",
                    SnapshotDiffReportProto::has_toSnapshot,
                    SnapshotDiffReportProto::get_toSnapshot,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "diffReportEntries",
                    SnapshotDiffReportProto::get_diffReportEntries,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapshotDiffReportProto>(
                    "SnapshotDiffReportProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshotDiffReportProto {
    fn clear(&mut self) {
        self.clear_snapshotRoot();
        self.clear_fromSnapshot();
        self.clear_toSnapshot();
        self.clear_diffReportEntries();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SnapshotDiffReportProto {
    fn eq(&self, other: &SnapshotDiffReportProto) -> bool {
        self.snapshotRoot == other.snapshotRoot &&
        self.fromSnapshot == other.fromSnapshot &&
        self.toSnapshot == other.toSnapshot &&
        self.diffReportEntries == other.diffReportEntries &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SnapshotDiffReportProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StorageInfoProto {
    // message fields
    layoutVersion: ::std::option::Option<u32>,
    namespceID: ::std::option::Option<u32>,
    clusterID: ::protobuf::SingularField<::std::string::String>,
    cTime: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl StorageInfoProto {
    pub fn new() -> StorageInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StorageInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<StorageInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StorageInfoProto,
        };
        unsafe {
            instance.get(|| {
                StorageInfoProto {
                    layoutVersion: ::std::option::Option::None,
                    namespceID: ::std::option::Option::None,
                    clusterID: ::protobuf::SingularField::none(),
                    cTime: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 layoutVersion = 1;

    pub fn clear_layoutVersion(&mut self) {
        self.layoutVersion = ::std::option::Option::None;
    }

    pub fn has_layoutVersion(&self) -> bool {
        self.layoutVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_layoutVersion(&mut self, v: u32) {
        self.layoutVersion = ::std::option::Option::Some(v);
    }

    pub fn get_layoutVersion<'a>(&self) -> u32 {
        self.layoutVersion.unwrap_or(0)
    }

    // required uint32 namespceID = 2;

    pub fn clear_namespceID(&mut self) {
        self.namespceID = ::std::option::Option::None;
    }

    pub fn has_namespceID(&self) -> bool {
        self.namespceID.is_some()
    }

    // Param is passed by value, moved
    pub fn set_namespceID(&mut self, v: u32) {
        self.namespceID = ::std::option::Option::Some(v);
    }

    pub fn get_namespceID<'a>(&self) -> u32 {
        self.namespceID.unwrap_or(0)
    }

    // required string clusterID = 3;

    pub fn clear_clusterID(&mut self) {
        self.clusterID.clear();
    }

    pub fn has_clusterID(&self) -> bool {
        self.clusterID.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clusterID(&mut self, v: ::std::string::String) {
        self.clusterID = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clusterID<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.clusterID.is_none() {
            self.clusterID.set_default();
        };
        self.clusterID.as_mut().unwrap()
    }

    // Take field
    pub fn take_clusterID(&mut self) -> ::std::string::String {
        self.clusterID.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_clusterID<'a>(&'a self) -> &'a str {
        match self.clusterID.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required uint64 cTime = 4;

    pub fn clear_cTime(&mut self) {
        self.cTime = ::std::option::Option::None;
    }

    pub fn has_cTime(&self) -> bool {
        self.cTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cTime(&mut self, v: u64) {
        self.cTime = ::std::option::Option::Some(v);
    }

    pub fn get_cTime<'a>(&self) -> u64 {
        self.cTime.unwrap_or(0)
    }
}

impl ::protobuf::Message for StorageInfoProto {
    fn is_initialized(&self) -> bool {
        if self.layoutVersion.is_none() {
            return false;
        };
        if self.namespceID.is_none() {
            return false;
        };
        if self.clusterID.is_none() {
            return false;
        };
        if self.cTime.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.layoutVersion = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.namespceID = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.clusterID.set_default();
                    try!(is.read_string_into(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.cTime = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.layoutVersion.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.namespceID.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.clusterID.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in self.cTime.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.layoutVersion {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.namespceID {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.clusterID.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.cTime {
            try!(os.write_uint64(4, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StorageInfoProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StorageInfoProto {
    fn new() -> StorageInfoProto {
        StorageInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<StorageInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "layoutVersion",
                    StorageInfoProto::has_layoutVersion,
                    StorageInfoProto::get_layoutVersion,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "namespceID",
                    StorageInfoProto::has_namespceID,
                    StorageInfoProto::get_namespceID,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "clusterID",
                    StorageInfoProto::has_clusterID,
                    StorageInfoProto::get_clusterID,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "cTime",
                    StorageInfoProto::has_cTime,
                    StorageInfoProto::get_cTime,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StorageInfoProto>(
                    "StorageInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StorageInfoProto {
    fn clear(&mut self) {
        self.clear_layoutVersion();
        self.clear_namespceID();
        self.clear_clusterID();
        self.clear_cTime();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StorageInfoProto {
    fn eq(&self, other: &StorageInfoProto) -> bool {
        self.layoutVersion == other.layoutVersion &&
        self.namespceID == other.namespceID &&
        self.clusterID == other.clusterID &&
        self.cTime == other.cTime &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StorageInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct NamenodeRegistrationProto {
    // message fields
    rpcAddress: ::protobuf::SingularField<::std::string::String>,
    httpAddress: ::protobuf::SingularField<::std::string::String>,
    storageInfo: ::protobuf::SingularPtrField<StorageInfoProto>,
    role: ::std::option::Option<NamenodeRegistrationProto_NamenodeRoleProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl NamenodeRegistrationProto {
    pub fn new() -> NamenodeRegistrationProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NamenodeRegistrationProto {
        static mut instance: ::protobuf::lazy::Lazy<NamenodeRegistrationProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NamenodeRegistrationProto,
        };
        unsafe {
            instance.get(|| {
                NamenodeRegistrationProto {
                    rpcAddress: ::protobuf::SingularField::none(),
                    httpAddress: ::protobuf::SingularField::none(),
                    storageInfo: ::protobuf::SingularPtrField::none(),
                    role: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string rpcAddress = 1;

    pub fn clear_rpcAddress(&mut self) {
        self.rpcAddress.clear();
    }

    pub fn has_rpcAddress(&self) -> bool {
        self.rpcAddress.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rpcAddress(&mut self, v: ::std::string::String) {
        self.rpcAddress = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rpcAddress<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.rpcAddress.is_none() {
            self.rpcAddress.set_default();
        };
        self.rpcAddress.as_mut().unwrap()
    }

    // Take field
    pub fn take_rpcAddress(&mut self) -> ::std::string::String {
        self.rpcAddress.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_rpcAddress<'a>(&'a self) -> &'a str {
        match self.rpcAddress.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string httpAddress = 2;

    pub fn clear_httpAddress(&mut self) {
        self.httpAddress.clear();
    }

    pub fn has_httpAddress(&self) -> bool {
        self.httpAddress.is_some()
    }

    // Param is passed by value, moved
    pub fn set_httpAddress(&mut self, v: ::std::string::String) {
        self.httpAddress = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_httpAddress<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.httpAddress.is_none() {
            self.httpAddress.set_default();
        };
        self.httpAddress.as_mut().unwrap()
    }

    // Take field
    pub fn take_httpAddress(&mut self) -> ::std::string::String {
        self.httpAddress.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_httpAddress<'a>(&'a self) -> &'a str {
        match self.httpAddress.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required .hadoop.hdfs.StorageInfoProto storageInfo = 3;

    pub fn clear_storageInfo(&mut self) {
        self.storageInfo.clear();
    }

    pub fn has_storageInfo(&self) -> bool {
        self.storageInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storageInfo(&mut self, v: StorageInfoProto) {
        self.storageInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_storageInfo<'a>(&'a mut self) -> &'a mut StorageInfoProto {
        if self.storageInfo.is_none() {
            self.storageInfo.set_default();
        };
        self.storageInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_storageInfo(&mut self) -> StorageInfoProto {
        self.storageInfo.take().unwrap_or_else(|| StorageInfoProto::new())
    }

    pub fn get_storageInfo<'a>(&'a self) -> &'a StorageInfoProto {
        self.storageInfo.as_ref().unwrap_or_else(|| StorageInfoProto::default_instance())
    }

    // optional .hadoop.hdfs.NamenodeRegistrationProto.NamenodeRoleProto role = 4;

    pub fn clear_role(&mut self) {
        self.role = ::std::option::Option::None;
    }

    pub fn has_role(&self) -> bool {
        self.role.is_some()
    }

    // Param is passed by value, moved
    pub fn set_role(&mut self, v: NamenodeRegistrationProto_NamenodeRoleProto) {
        self.role = ::std::option::Option::Some(v);
    }

    pub fn get_role<'a>(&self) -> NamenodeRegistrationProto_NamenodeRoleProto {
        self.role.unwrap_or(NamenodeRegistrationProto_NamenodeRoleProto::NAMENODE)
    }
}

impl ::protobuf::Message for NamenodeRegistrationProto {
    fn is_initialized(&self) -> bool {
        if self.rpcAddress.is_none() {
            return false;
        };
        if self.httpAddress.is_none() {
            return false;
        };
        if self.storageInfo.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.rpcAddress.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.httpAddress.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.storageInfo.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.role = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.rpcAddress.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.httpAddress.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.storageInfo.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.role.iter() {
            my_size += ::protobuf::rt::enum_size(4, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.rpcAddress.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.httpAddress.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.storageInfo.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.role {
            try!(os.write_enum(4, v as i32));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<NamenodeRegistrationProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for NamenodeRegistrationProto {
    fn new() -> NamenodeRegistrationProto {
        NamenodeRegistrationProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<NamenodeRegistrationProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "rpcAddress",
                    NamenodeRegistrationProto::has_rpcAddress,
                    NamenodeRegistrationProto::get_rpcAddress,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "httpAddress",
                    NamenodeRegistrationProto::has_httpAddress,
                    NamenodeRegistrationProto::get_httpAddress,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "storageInfo",
                    NamenodeRegistrationProto::has_storageInfo,
                    NamenodeRegistrationProto::get_storageInfo,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "role",
                    NamenodeRegistrationProto::has_role,
                    NamenodeRegistrationProto::get_role,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NamenodeRegistrationProto>(
                    "NamenodeRegistrationProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NamenodeRegistrationProto {
    fn clear(&mut self) {
        self.clear_rpcAddress();
        self.clear_httpAddress();
        self.clear_storageInfo();
        self.clear_role();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for NamenodeRegistrationProto {
    fn eq(&self, other: &NamenodeRegistrationProto) -> bool {
        self.rpcAddress == other.rpcAddress &&
        self.httpAddress == other.httpAddress &&
        self.storageInfo == other.storageInfo &&
        self.role == other.role &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for NamenodeRegistrationProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum NamenodeRegistrationProto_NamenodeRoleProto {
    NAMENODE = 1,
    BACKUP = 2,
    CHECKPOINT = 3,
}

impl ::protobuf::ProtobufEnum for NamenodeRegistrationProto_NamenodeRoleProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<NamenodeRegistrationProto_NamenodeRoleProto> {
        match value {
            1 => ::std::option::Option::Some(NamenodeRegistrationProto_NamenodeRoleProto::NAMENODE),
            2 => ::std::option::Option::Some(NamenodeRegistrationProto_NamenodeRoleProto::BACKUP),
            3 => ::std::option::Option::Some(NamenodeRegistrationProto_NamenodeRoleProto::CHECKPOINT),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<NamenodeRegistrationProto_NamenodeRoleProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("NamenodeRegistrationProto_NamenodeRoleProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for NamenodeRegistrationProto_NamenodeRoleProto {
}

#[derive(Clone,Default)]
pub struct CheckpointSignatureProto {
    // message fields
    blockPoolId: ::protobuf::SingularField<::std::string::String>,
    mostRecentCheckpointTxId: ::std::option::Option<u64>,
    curSegmentTxId: ::std::option::Option<u64>,
    storageInfo: ::protobuf::SingularPtrField<StorageInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CheckpointSignatureProto {
    pub fn new() -> CheckpointSignatureProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CheckpointSignatureProto {
        static mut instance: ::protobuf::lazy::Lazy<CheckpointSignatureProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CheckpointSignatureProto,
        };
        unsafe {
            instance.get(|| {
                CheckpointSignatureProto {
                    blockPoolId: ::protobuf::SingularField::none(),
                    mostRecentCheckpointTxId: ::std::option::Option::None,
                    curSegmentTxId: ::std::option::Option::None,
                    storageInfo: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string blockPoolId = 1;

    pub fn clear_blockPoolId(&mut self) {
        self.blockPoolId.clear();
    }

    pub fn has_blockPoolId(&self) -> bool {
        self.blockPoolId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockPoolId(&mut self, v: ::std::string::String) {
        self.blockPoolId = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blockPoolId<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.blockPoolId.is_none() {
            self.blockPoolId.set_default();
        };
        self.blockPoolId.as_mut().unwrap()
    }

    // Take field
    pub fn take_blockPoolId(&mut self) -> ::std::string::String {
        self.blockPoolId.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_blockPoolId<'a>(&'a self) -> &'a str {
        match self.blockPoolId.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required uint64 mostRecentCheckpointTxId = 2;

    pub fn clear_mostRecentCheckpointTxId(&mut self) {
        self.mostRecentCheckpointTxId = ::std::option::Option::None;
    }

    pub fn has_mostRecentCheckpointTxId(&self) -> bool {
        self.mostRecentCheckpointTxId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mostRecentCheckpointTxId(&mut self, v: u64) {
        self.mostRecentCheckpointTxId = ::std::option::Option::Some(v);
    }

    pub fn get_mostRecentCheckpointTxId<'a>(&self) -> u64 {
        self.mostRecentCheckpointTxId.unwrap_or(0)
    }

    // required uint64 curSegmentTxId = 3;

    pub fn clear_curSegmentTxId(&mut self) {
        self.curSegmentTxId = ::std::option::Option::None;
    }

    pub fn has_curSegmentTxId(&self) -> bool {
        self.curSegmentTxId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_curSegmentTxId(&mut self, v: u64) {
        self.curSegmentTxId = ::std::option::Option::Some(v);
    }

    pub fn get_curSegmentTxId<'a>(&self) -> u64 {
        self.curSegmentTxId.unwrap_or(0)
    }

    // required .hadoop.hdfs.StorageInfoProto storageInfo = 4;

    pub fn clear_storageInfo(&mut self) {
        self.storageInfo.clear();
    }

    pub fn has_storageInfo(&self) -> bool {
        self.storageInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storageInfo(&mut self, v: StorageInfoProto) {
        self.storageInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_storageInfo<'a>(&'a mut self) -> &'a mut StorageInfoProto {
        if self.storageInfo.is_none() {
            self.storageInfo.set_default();
        };
        self.storageInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_storageInfo(&mut self) -> StorageInfoProto {
        self.storageInfo.take().unwrap_or_else(|| StorageInfoProto::new())
    }

    pub fn get_storageInfo<'a>(&'a self) -> &'a StorageInfoProto {
        self.storageInfo.as_ref().unwrap_or_else(|| StorageInfoProto::default_instance())
    }
}

impl ::protobuf::Message for CheckpointSignatureProto {
    fn is_initialized(&self) -> bool {
        if self.blockPoolId.is_none() {
            return false;
        };
        if self.mostRecentCheckpointTxId.is_none() {
            return false;
        };
        if self.curSegmentTxId.is_none() {
            return false;
        };
        if self.storageInfo.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.blockPoolId.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.mostRecentCheckpointTxId = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.curSegmentTxId = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.storageInfo.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.blockPoolId.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.mostRecentCheckpointTxId.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.curSegmentTxId.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.storageInfo.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.blockPoolId.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.mostRecentCheckpointTxId {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.curSegmentTxId {
            try!(os.write_uint64(3, v));
        };
        if let Some(v) = self.storageInfo.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<CheckpointSignatureProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CheckpointSignatureProto {
    fn new() -> CheckpointSignatureProto {
        CheckpointSignatureProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CheckpointSignatureProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "blockPoolId",
                    CheckpointSignatureProto::has_blockPoolId,
                    CheckpointSignatureProto::get_blockPoolId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "mostRecentCheckpointTxId",
                    CheckpointSignatureProto::has_mostRecentCheckpointTxId,
                    CheckpointSignatureProto::get_mostRecentCheckpointTxId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "curSegmentTxId",
                    CheckpointSignatureProto::has_curSegmentTxId,
                    CheckpointSignatureProto::get_curSegmentTxId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "storageInfo",
                    CheckpointSignatureProto::has_storageInfo,
                    CheckpointSignatureProto::get_storageInfo,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CheckpointSignatureProto>(
                    "CheckpointSignatureProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CheckpointSignatureProto {
    fn clear(&mut self) {
        self.clear_blockPoolId();
        self.clear_mostRecentCheckpointTxId();
        self.clear_curSegmentTxId();
        self.clear_storageInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CheckpointSignatureProto {
    fn eq(&self, other: &CheckpointSignatureProto) -> bool {
        self.blockPoolId == other.blockPoolId &&
        self.mostRecentCheckpointTxId == other.mostRecentCheckpointTxId &&
        self.curSegmentTxId == other.curSegmentTxId &&
        self.storageInfo == other.storageInfo &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CheckpointSignatureProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct NamenodeCommandProto {
    // message fields
    action: ::std::option::Option<u32>,
    field_type: ::std::option::Option<NamenodeCommandProto_Type>,
    checkpointCmd: ::protobuf::SingularPtrField<CheckpointCommandProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl NamenodeCommandProto {
    pub fn new() -> NamenodeCommandProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NamenodeCommandProto {
        static mut instance: ::protobuf::lazy::Lazy<NamenodeCommandProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NamenodeCommandProto,
        };
        unsafe {
            instance.get(|| {
                NamenodeCommandProto {
                    action: ::std::option::Option::None,
                    field_type: ::std::option::Option::None,
                    checkpointCmd: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 action = 1;

    pub fn clear_action(&mut self) {
        self.action = ::std::option::Option::None;
    }

    pub fn has_action(&self) -> bool {
        self.action.is_some()
    }

    // Param is passed by value, moved
    pub fn set_action(&mut self, v: u32) {
        self.action = ::std::option::Option::Some(v);
    }

    pub fn get_action<'a>(&self) -> u32 {
        self.action.unwrap_or(0)
    }

    // required .hadoop.hdfs.NamenodeCommandProto.Type type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: NamenodeCommandProto_Type) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type<'a>(&self) -> NamenodeCommandProto_Type {
        self.field_type.unwrap_or(NamenodeCommandProto_Type::NamenodeCommand)
    }

    // optional .hadoop.hdfs.CheckpointCommandProto checkpointCmd = 3;

    pub fn clear_checkpointCmd(&mut self) {
        self.checkpointCmd.clear();
    }

    pub fn has_checkpointCmd(&self) -> bool {
        self.checkpointCmd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_checkpointCmd(&mut self, v: CheckpointCommandProto) {
        self.checkpointCmd = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_checkpointCmd<'a>(&'a mut self) -> &'a mut CheckpointCommandProto {
        if self.checkpointCmd.is_none() {
            self.checkpointCmd.set_default();
        };
        self.checkpointCmd.as_mut().unwrap()
    }

    // Take field
    pub fn take_checkpointCmd(&mut self) -> CheckpointCommandProto {
        self.checkpointCmd.take().unwrap_or_else(|| CheckpointCommandProto::new())
    }

    pub fn get_checkpointCmd<'a>(&'a self) -> &'a CheckpointCommandProto {
        self.checkpointCmd.as_ref().unwrap_or_else(|| CheckpointCommandProto::default_instance())
    }
}

impl ::protobuf::Message for NamenodeCommandProto {
    fn is_initialized(&self) -> bool {
        if self.action.is_none() {
            return false;
        };
        if self.field_type.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.action = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.checkpointCmd.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.action.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.field_type.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in self.checkpointCmd.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.action {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.field_type {
            try!(os.write_enum(2, v as i32));
        };
        if let Some(v) = self.checkpointCmd.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<NamenodeCommandProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for NamenodeCommandProto {
    fn new() -> NamenodeCommandProto {
        NamenodeCommandProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<NamenodeCommandProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "action",
                    NamenodeCommandProto::has_action,
                    NamenodeCommandProto::get_action,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "field_type",
                    NamenodeCommandProto::has_field_type,
                    NamenodeCommandProto::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "checkpointCmd",
                    NamenodeCommandProto::has_checkpointCmd,
                    NamenodeCommandProto::get_checkpointCmd,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NamenodeCommandProto>(
                    "NamenodeCommandProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NamenodeCommandProto {
    fn clear(&mut self) {
        self.clear_action();
        self.clear_field_type();
        self.clear_checkpointCmd();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for NamenodeCommandProto {
    fn eq(&self, other: &NamenodeCommandProto) -> bool {
        self.action == other.action &&
        self.field_type == other.field_type &&
        self.checkpointCmd == other.checkpointCmd &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for NamenodeCommandProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum NamenodeCommandProto_Type {
    NamenodeCommand = 0,
    CheckPointCommand = 1,
}

impl ::protobuf::ProtobufEnum for NamenodeCommandProto_Type {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<NamenodeCommandProto_Type> {
        match value {
            0 => ::std::option::Option::Some(NamenodeCommandProto_Type::NamenodeCommand),
            1 => ::std::option::Option::Some(NamenodeCommandProto_Type::CheckPointCommand),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<NamenodeCommandProto_Type>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("NamenodeCommandProto_Type", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for NamenodeCommandProto_Type {
}

#[derive(Clone,Default)]
pub struct CheckpointCommandProto {
    // message fields
    signature: ::protobuf::SingularPtrField<CheckpointSignatureProto>,
    needToReturnImage: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CheckpointCommandProto {
    pub fn new() -> CheckpointCommandProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CheckpointCommandProto {
        static mut instance: ::protobuf::lazy::Lazy<CheckpointCommandProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CheckpointCommandProto,
        };
        unsafe {
            instance.get(|| {
                CheckpointCommandProto {
                    signature: ::protobuf::SingularPtrField::none(),
                    needToReturnImage: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.CheckpointSignatureProto signature = 1;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    pub fn has_signature(&self) -> bool {
        self.signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: CheckpointSignatureProto) {
        self.signature = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature<'a>(&'a mut self) -> &'a mut CheckpointSignatureProto {
        if self.signature.is_none() {
            self.signature.set_default();
        };
        self.signature.as_mut().unwrap()
    }

    // Take field
    pub fn take_signature(&mut self) -> CheckpointSignatureProto {
        self.signature.take().unwrap_or_else(|| CheckpointSignatureProto::new())
    }

    pub fn get_signature<'a>(&'a self) -> &'a CheckpointSignatureProto {
        self.signature.as_ref().unwrap_or_else(|| CheckpointSignatureProto::default_instance())
    }

    // required bool needToReturnImage = 2;

    pub fn clear_needToReturnImage(&mut self) {
        self.needToReturnImage = ::std::option::Option::None;
    }

    pub fn has_needToReturnImage(&self) -> bool {
        self.needToReturnImage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_needToReturnImage(&mut self, v: bool) {
        self.needToReturnImage = ::std::option::Option::Some(v);
    }

    pub fn get_needToReturnImage<'a>(&self) -> bool {
        self.needToReturnImage.unwrap_or(false)
    }
}

impl ::protobuf::Message for CheckpointCommandProto {
    fn is_initialized(&self) -> bool {
        if self.signature.is_none() {
            return false;
        };
        if self.needToReturnImage.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.signature.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.needToReturnImage = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.signature.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.needToReturnImage.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.signature.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.needToReturnImage {
            try!(os.write_bool(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<CheckpointCommandProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CheckpointCommandProto {
    fn new() -> CheckpointCommandProto {
        CheckpointCommandProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CheckpointCommandProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "signature",
                    CheckpointCommandProto::has_signature,
                    CheckpointCommandProto::get_signature,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "needToReturnImage",
                    CheckpointCommandProto::has_needToReturnImage,
                    CheckpointCommandProto::get_needToReturnImage,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CheckpointCommandProto>(
                    "CheckpointCommandProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CheckpointCommandProto {
    fn clear(&mut self) {
        self.clear_signature();
        self.clear_needToReturnImage();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CheckpointCommandProto {
    fn eq(&self, other: &CheckpointCommandProto) -> bool {
        self.signature == other.signature &&
        self.needToReturnImage == other.needToReturnImage &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CheckpointCommandProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BlockProto {
    // message fields
    blockId: ::std::option::Option<u64>,
    genStamp: ::std::option::Option<u64>,
    numBytes: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl BlockProto {
    pub fn new() -> BlockProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockProto {
        static mut instance: ::protobuf::lazy::Lazy<BlockProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockProto,
        };
        unsafe {
            instance.get(|| {
                BlockProto {
                    blockId: ::std::option::Option::None,
                    genStamp: ::std::option::Option::None,
                    numBytes: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint64 blockId = 1;

    pub fn clear_blockId(&mut self) {
        self.blockId = ::std::option::Option::None;
    }

    pub fn has_blockId(&self) -> bool {
        self.blockId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockId(&mut self, v: u64) {
        self.blockId = ::std::option::Option::Some(v);
    }

    pub fn get_blockId<'a>(&self) -> u64 {
        self.blockId.unwrap_or(0)
    }

    // required uint64 genStamp = 2;

    pub fn clear_genStamp(&mut self) {
        self.genStamp = ::std::option::Option::None;
    }

    pub fn has_genStamp(&self) -> bool {
        self.genStamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_genStamp(&mut self, v: u64) {
        self.genStamp = ::std::option::Option::Some(v);
    }

    pub fn get_genStamp<'a>(&self) -> u64 {
        self.genStamp.unwrap_or(0)
    }

    // optional uint64 numBytes = 3;

    pub fn clear_numBytes(&mut self) {
        self.numBytes = ::std::option::Option::None;
    }

    pub fn has_numBytes(&self) -> bool {
        self.numBytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_numBytes(&mut self, v: u64) {
        self.numBytes = ::std::option::Option::Some(v);
    }

    pub fn get_numBytes<'a>(&self) -> u64 {
        self.numBytes.unwrap_or(0u64)
    }
}

impl ::protobuf::Message for BlockProto {
    fn is_initialized(&self) -> bool {
        if self.blockId.is_none() {
            return false;
        };
        if self.genStamp.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.blockId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.genStamp = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.numBytes = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.blockId.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.genStamp.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.numBytes.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.blockId {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.genStamp {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.numBytes {
            try!(os.write_uint64(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<BlockProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BlockProto {
    fn new() -> BlockProto {
        BlockProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "blockId",
                    BlockProto::has_blockId,
                    BlockProto::get_blockId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "genStamp",
                    BlockProto::has_genStamp,
                    BlockProto::get_genStamp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "numBytes",
                    BlockProto::has_numBytes,
                    BlockProto::get_numBytes,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockProto>(
                    "BlockProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockProto {
    fn clear(&mut self) {
        self.clear_blockId();
        self.clear_genStamp();
        self.clear_numBytes();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BlockProto {
    fn eq(&self, other: &BlockProto) -> bool {
        self.blockId == other.blockId &&
        self.genStamp == other.genStamp &&
        self.numBytes == other.numBytes &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BlockProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BlockWithLocationsProto {
    // message fields
    block: ::protobuf::SingularPtrField<BlockProto>,
    datanodeUuids: ::protobuf::RepeatedField<::std::string::String>,
    storageUuids: ::protobuf::RepeatedField<::std::string::String>,
    storageTypes: ::std::vec::Vec<StorageTypeProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl BlockWithLocationsProto {
    pub fn new() -> BlockWithLocationsProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockWithLocationsProto {
        static mut instance: ::protobuf::lazy::Lazy<BlockWithLocationsProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockWithLocationsProto,
        };
        unsafe {
            instance.get(|| {
                BlockWithLocationsProto {
                    block: ::protobuf::SingularPtrField::none(),
                    datanodeUuids: ::protobuf::RepeatedField::new(),
                    storageUuids: ::protobuf::RepeatedField::new(),
                    storageTypes: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.BlockProto block = 1;

    pub fn clear_block(&mut self) {
        self.block.clear();
    }

    pub fn has_block(&self) -> bool {
        self.block.is_some()
    }

    // Param is passed by value, moved
    pub fn set_block(&mut self, v: BlockProto) {
        self.block = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block<'a>(&'a mut self) -> &'a mut BlockProto {
        if self.block.is_none() {
            self.block.set_default();
        };
        self.block.as_mut().unwrap()
    }

    // Take field
    pub fn take_block(&mut self) -> BlockProto {
        self.block.take().unwrap_or_else(|| BlockProto::new())
    }

    pub fn get_block<'a>(&'a self) -> &'a BlockProto {
        self.block.as_ref().unwrap_or_else(|| BlockProto::default_instance())
    }

    // repeated string datanodeUuids = 2;

    pub fn clear_datanodeUuids(&mut self) {
        self.datanodeUuids.clear();
    }

    // Param is passed by value, moved
    pub fn set_datanodeUuids(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.datanodeUuids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_datanodeUuids<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.datanodeUuids
    }

    // Take field
    pub fn take_datanodeUuids(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.datanodeUuids, ::protobuf::RepeatedField::new())
    }

    pub fn get_datanodeUuids<'a>(&'a self) -> &'a [::std::string::String] {
        &self.datanodeUuids
    }

    // repeated string storageUuids = 3;

    pub fn clear_storageUuids(&mut self) {
        self.storageUuids.clear();
    }

    // Param is passed by value, moved
    pub fn set_storageUuids(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.storageUuids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_storageUuids<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.storageUuids
    }

    // Take field
    pub fn take_storageUuids(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.storageUuids, ::protobuf::RepeatedField::new())
    }

    pub fn get_storageUuids<'a>(&'a self) -> &'a [::std::string::String] {
        &self.storageUuids
    }

    // repeated .hadoop.hdfs.StorageTypeProto storageTypes = 4;

    pub fn clear_storageTypes(&mut self) {
        self.storageTypes.clear();
    }

    // Param is passed by value, moved
    pub fn set_storageTypes(&mut self, v: ::std::vec::Vec<StorageTypeProto>) {
        self.storageTypes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_storageTypes<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<StorageTypeProto> {
        &mut self.storageTypes
    }

    // Take field
    pub fn take_storageTypes(&mut self) -> ::std::vec::Vec<StorageTypeProto> {
        ::std::mem::replace(&mut self.storageTypes, ::std::vec::Vec::new())
    }

    pub fn get_storageTypes<'a>(&'a self) -> &'a [StorageTypeProto] {
        &self.storageTypes
    }
}

impl ::protobuf::Message for BlockWithLocationsProto {
    fn is_initialized(&self) -> bool {
        if self.block.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.block.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.datanodeUuids));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.storageUuids));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.storageTypes));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.block.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.datanodeUuids.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.storageUuids.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in self.storageTypes.iter() {
            my_size += ::protobuf::rt::enum_size(4, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.block.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.datanodeUuids.iter() {
            try!(os.write_string(2, &v));
        };
        for v in self.storageUuids.iter() {
            try!(os.write_string(3, &v));
        };
        for v in self.storageTypes.iter() {
            try!(os.write_enum(4, *v as i32));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<BlockWithLocationsProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BlockWithLocationsProto {
    fn new() -> BlockWithLocationsProto {
        BlockWithLocationsProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockWithLocationsProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "block",
                    BlockWithLocationsProto::has_block,
                    BlockWithLocationsProto::get_block,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "datanodeUuids",
                    BlockWithLocationsProto::get_datanodeUuids,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "storageUuids",
                    BlockWithLocationsProto::get_storageUuids,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_enum_accessor(
                    "storageTypes",
                    BlockWithLocationsProto::get_storageTypes,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockWithLocationsProto>(
                    "BlockWithLocationsProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockWithLocationsProto {
    fn clear(&mut self) {
        self.clear_block();
        self.clear_datanodeUuids();
        self.clear_storageUuids();
        self.clear_storageTypes();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BlockWithLocationsProto {
    fn eq(&self, other: &BlockWithLocationsProto) -> bool {
        self.block == other.block &&
        self.datanodeUuids == other.datanodeUuids &&
        self.storageUuids == other.storageUuids &&
        self.storageTypes == other.storageTypes &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BlockWithLocationsProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BlocksWithLocationsProto {
    // message fields
    blocks: ::protobuf::RepeatedField<BlockWithLocationsProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl BlocksWithLocationsProto {
    pub fn new() -> BlocksWithLocationsProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlocksWithLocationsProto {
        static mut instance: ::protobuf::lazy::Lazy<BlocksWithLocationsProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlocksWithLocationsProto,
        };
        unsafe {
            instance.get(|| {
                BlocksWithLocationsProto {
                    blocks: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .hadoop.hdfs.BlockWithLocationsProto blocks = 1;

    pub fn clear_blocks(&mut self) {
        self.blocks.clear();
    }

    // Param is passed by value, moved
    pub fn set_blocks(&mut self, v: ::protobuf::RepeatedField<BlockWithLocationsProto>) {
        self.blocks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_blocks<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<BlockWithLocationsProto> {
        &mut self.blocks
    }

    // Take field
    pub fn take_blocks(&mut self) -> ::protobuf::RepeatedField<BlockWithLocationsProto> {
        ::std::mem::replace(&mut self.blocks, ::protobuf::RepeatedField::new())
    }

    pub fn get_blocks<'a>(&'a self) -> &'a [BlockWithLocationsProto] {
        &self.blocks
    }
}

impl ::protobuf::Message for BlocksWithLocationsProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.blocks));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.blocks.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.blocks.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<BlocksWithLocationsProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BlocksWithLocationsProto {
    fn new() -> BlocksWithLocationsProto {
        BlocksWithLocationsProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlocksWithLocationsProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "blocks",
                    BlocksWithLocationsProto::get_blocks,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlocksWithLocationsProto>(
                    "BlocksWithLocationsProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlocksWithLocationsProto {
    fn clear(&mut self) {
        self.clear_blocks();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BlocksWithLocationsProto {
    fn eq(&self, other: &BlocksWithLocationsProto) -> bool {
        self.blocks == other.blocks &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BlocksWithLocationsProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RemoteEditLogProto {
    // message fields
    startTxId: ::std::option::Option<u64>,
    endTxId: ::std::option::Option<u64>,
    isInProgress: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RemoteEditLogProto {
    pub fn new() -> RemoteEditLogProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RemoteEditLogProto {
        static mut instance: ::protobuf::lazy::Lazy<RemoteEditLogProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RemoteEditLogProto,
        };
        unsafe {
            instance.get(|| {
                RemoteEditLogProto {
                    startTxId: ::std::option::Option::None,
                    endTxId: ::std::option::Option::None,
                    isInProgress: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint64 startTxId = 1;

    pub fn clear_startTxId(&mut self) {
        self.startTxId = ::std::option::Option::None;
    }

    pub fn has_startTxId(&self) -> bool {
        self.startTxId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_startTxId(&mut self, v: u64) {
        self.startTxId = ::std::option::Option::Some(v);
    }

    pub fn get_startTxId<'a>(&self) -> u64 {
        self.startTxId.unwrap_or(0)
    }

    // required uint64 endTxId = 2;

    pub fn clear_endTxId(&mut self) {
        self.endTxId = ::std::option::Option::None;
    }

    pub fn has_endTxId(&self) -> bool {
        self.endTxId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_endTxId(&mut self, v: u64) {
        self.endTxId = ::std::option::Option::Some(v);
    }

    pub fn get_endTxId<'a>(&self) -> u64 {
        self.endTxId.unwrap_or(0)
    }

    // optional bool isInProgress = 3;

    pub fn clear_isInProgress(&mut self) {
        self.isInProgress = ::std::option::Option::None;
    }

    pub fn has_isInProgress(&self) -> bool {
        self.isInProgress.is_some()
    }

    // Param is passed by value, moved
    pub fn set_isInProgress(&mut self, v: bool) {
        self.isInProgress = ::std::option::Option::Some(v);
    }

    pub fn get_isInProgress<'a>(&self) -> bool {
        self.isInProgress.unwrap_or(false)
    }
}

impl ::protobuf::Message for RemoteEditLogProto {
    fn is_initialized(&self) -> bool {
        if self.startTxId.is_none() {
            return false;
        };
        if self.endTxId.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.startTxId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.endTxId = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.isInProgress = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.startTxId.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.endTxId.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.isInProgress.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.startTxId {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.endTxId {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.isInProgress {
            try!(os.write_bool(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<RemoteEditLogProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RemoteEditLogProto {
    fn new() -> RemoteEditLogProto {
        RemoteEditLogProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RemoteEditLogProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "startTxId",
                    RemoteEditLogProto::has_startTxId,
                    RemoteEditLogProto::get_startTxId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "endTxId",
                    RemoteEditLogProto::has_endTxId,
                    RemoteEditLogProto::get_endTxId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "isInProgress",
                    RemoteEditLogProto::has_isInProgress,
                    RemoteEditLogProto::get_isInProgress,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RemoteEditLogProto>(
                    "RemoteEditLogProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RemoteEditLogProto {
    fn clear(&mut self) {
        self.clear_startTxId();
        self.clear_endTxId();
        self.clear_isInProgress();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RemoteEditLogProto {
    fn eq(&self, other: &RemoteEditLogProto) -> bool {
        self.startTxId == other.startTxId &&
        self.endTxId == other.endTxId &&
        self.isInProgress == other.isInProgress &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RemoteEditLogProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RemoteEditLogManifestProto {
    // message fields
    logs: ::protobuf::RepeatedField<RemoteEditLogProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RemoteEditLogManifestProto {
    pub fn new() -> RemoteEditLogManifestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RemoteEditLogManifestProto {
        static mut instance: ::protobuf::lazy::Lazy<RemoteEditLogManifestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RemoteEditLogManifestProto,
        };
        unsafe {
            instance.get(|| {
                RemoteEditLogManifestProto {
                    logs: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .hadoop.hdfs.RemoteEditLogProto logs = 1;

    pub fn clear_logs(&mut self) {
        self.logs.clear();
    }

    // Param is passed by value, moved
    pub fn set_logs(&mut self, v: ::protobuf::RepeatedField<RemoteEditLogProto>) {
        self.logs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_logs<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<RemoteEditLogProto> {
        &mut self.logs
    }

    // Take field
    pub fn take_logs(&mut self) -> ::protobuf::RepeatedField<RemoteEditLogProto> {
        ::std::mem::replace(&mut self.logs, ::protobuf::RepeatedField::new())
    }

    pub fn get_logs<'a>(&'a self) -> &'a [RemoteEditLogProto] {
        &self.logs
    }
}

impl ::protobuf::Message for RemoteEditLogManifestProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.logs));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.logs.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.logs.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<RemoteEditLogManifestProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RemoteEditLogManifestProto {
    fn new() -> RemoteEditLogManifestProto {
        RemoteEditLogManifestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RemoteEditLogManifestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "logs",
                    RemoteEditLogManifestProto::get_logs,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RemoteEditLogManifestProto>(
                    "RemoteEditLogManifestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RemoteEditLogManifestProto {
    fn clear(&mut self) {
        self.clear_logs();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RemoteEditLogManifestProto {
    fn eq(&self, other: &RemoteEditLogManifestProto) -> bool {
        self.logs == other.logs &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RemoteEditLogManifestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct NamespaceInfoProto {
    // message fields
    buildVersion: ::protobuf::SingularField<::std::string::String>,
    unused: ::std::option::Option<u32>,
    blockPoolID: ::protobuf::SingularField<::std::string::String>,
    storageInfo: ::protobuf::SingularPtrField<StorageInfoProto>,
    softwareVersion: ::protobuf::SingularField<::std::string::String>,
    capabilities: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl NamespaceInfoProto {
    pub fn new() -> NamespaceInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NamespaceInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<NamespaceInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NamespaceInfoProto,
        };
        unsafe {
            instance.get(|| {
                NamespaceInfoProto {
                    buildVersion: ::protobuf::SingularField::none(),
                    unused: ::std::option::Option::None,
                    blockPoolID: ::protobuf::SingularField::none(),
                    storageInfo: ::protobuf::SingularPtrField::none(),
                    softwareVersion: ::protobuf::SingularField::none(),
                    capabilities: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string buildVersion = 1;

    pub fn clear_buildVersion(&mut self) {
        self.buildVersion.clear();
    }

    pub fn has_buildVersion(&self) -> bool {
        self.buildVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_buildVersion(&mut self, v: ::std::string::String) {
        self.buildVersion = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_buildVersion<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.buildVersion.is_none() {
            self.buildVersion.set_default();
        };
        self.buildVersion.as_mut().unwrap()
    }

    // Take field
    pub fn take_buildVersion(&mut self) -> ::std::string::String {
        self.buildVersion.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_buildVersion<'a>(&'a self) -> &'a str {
        match self.buildVersion.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required uint32 unused = 2;

    pub fn clear_unused(&mut self) {
        self.unused = ::std::option::Option::None;
    }

    pub fn has_unused(&self) -> bool {
        self.unused.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unused(&mut self, v: u32) {
        self.unused = ::std::option::Option::Some(v);
    }

    pub fn get_unused<'a>(&self) -> u32 {
        self.unused.unwrap_or(0)
    }

    // required string blockPoolID = 3;

    pub fn clear_blockPoolID(&mut self) {
        self.blockPoolID.clear();
    }

    pub fn has_blockPoolID(&self) -> bool {
        self.blockPoolID.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockPoolID(&mut self, v: ::std::string::String) {
        self.blockPoolID = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blockPoolID<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.blockPoolID.is_none() {
            self.blockPoolID.set_default();
        };
        self.blockPoolID.as_mut().unwrap()
    }

    // Take field
    pub fn take_blockPoolID(&mut self) -> ::std::string::String {
        self.blockPoolID.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_blockPoolID<'a>(&'a self) -> &'a str {
        match self.blockPoolID.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required .hadoop.hdfs.StorageInfoProto storageInfo = 4;

    pub fn clear_storageInfo(&mut self) {
        self.storageInfo.clear();
    }

    pub fn has_storageInfo(&self) -> bool {
        self.storageInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_storageInfo(&mut self, v: StorageInfoProto) {
        self.storageInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_storageInfo<'a>(&'a mut self) -> &'a mut StorageInfoProto {
        if self.storageInfo.is_none() {
            self.storageInfo.set_default();
        };
        self.storageInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_storageInfo(&mut self) -> StorageInfoProto {
        self.storageInfo.take().unwrap_or_else(|| StorageInfoProto::new())
    }

    pub fn get_storageInfo<'a>(&'a self) -> &'a StorageInfoProto {
        self.storageInfo.as_ref().unwrap_or_else(|| StorageInfoProto::default_instance())
    }

    // required string softwareVersion = 5;

    pub fn clear_softwareVersion(&mut self) {
        self.softwareVersion.clear();
    }

    pub fn has_softwareVersion(&self) -> bool {
        self.softwareVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_softwareVersion(&mut self, v: ::std::string::String) {
        self.softwareVersion = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_softwareVersion<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.softwareVersion.is_none() {
            self.softwareVersion.set_default();
        };
        self.softwareVersion.as_mut().unwrap()
    }

    // Take field
    pub fn take_softwareVersion(&mut self) -> ::std::string::String {
        self.softwareVersion.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_softwareVersion<'a>(&'a self) -> &'a str {
        match self.softwareVersion.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional uint64 capabilities = 6;

    pub fn clear_capabilities(&mut self) {
        self.capabilities = ::std::option::Option::None;
    }

    pub fn has_capabilities(&self) -> bool {
        self.capabilities.is_some()
    }

    // Param is passed by value, moved
    pub fn set_capabilities(&mut self, v: u64) {
        self.capabilities = ::std::option::Option::Some(v);
    }

    pub fn get_capabilities<'a>(&self) -> u64 {
        self.capabilities.unwrap_or(0u64)
    }
}

impl ::protobuf::Message for NamespaceInfoProto {
    fn is_initialized(&self) -> bool {
        if self.buildVersion.is_none() {
            return false;
        };
        if self.unused.is_none() {
            return false;
        };
        if self.blockPoolID.is_none() {
            return false;
        };
        if self.storageInfo.is_none() {
            return false;
        };
        if self.softwareVersion.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.buildVersion.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.unused = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.blockPoolID.set_default();
                    try!(is.read_string_into(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.storageInfo.set_default();
                    try!(is.merge_message(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.softwareVersion.set_default();
                    try!(is.read_string_into(tmp))
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.capabilities = ::std::option::Option::Some(tmp);
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.buildVersion.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.unused.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.blockPoolID.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in self.storageInfo.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.softwareVersion.iter() {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        for value in self.capabilities.iter() {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.buildVersion.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.unused {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.blockPoolID.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.storageInfo.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.softwareVersion.as_ref() {
            try!(os.write_string(5, &v));
        };
        if let Some(v) = self.capabilities {
            try!(os.write_uint64(6, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<NamespaceInfoProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for NamespaceInfoProto {
    fn new() -> NamespaceInfoProto {
        NamespaceInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<NamespaceInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "buildVersion",
                    NamespaceInfoProto::has_buildVersion,
                    NamespaceInfoProto::get_buildVersion,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "unused",
                    NamespaceInfoProto::has_unused,
                    NamespaceInfoProto::get_unused,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "blockPoolID",
                    NamespaceInfoProto::has_blockPoolID,
                    NamespaceInfoProto::get_blockPoolID,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "storageInfo",
                    NamespaceInfoProto::has_storageInfo,
                    NamespaceInfoProto::get_storageInfo,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "softwareVersion",
                    NamespaceInfoProto::has_softwareVersion,
                    NamespaceInfoProto::get_softwareVersion,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "capabilities",
                    NamespaceInfoProto::has_capabilities,
                    NamespaceInfoProto::get_capabilities,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NamespaceInfoProto>(
                    "NamespaceInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NamespaceInfoProto {
    fn clear(&mut self) {
        self.clear_buildVersion();
        self.clear_unused();
        self.clear_blockPoolID();
        self.clear_storageInfo();
        self.clear_softwareVersion();
        self.clear_capabilities();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for NamespaceInfoProto {
    fn eq(&self, other: &NamespaceInfoProto) -> bool {
        self.buildVersion == other.buildVersion &&
        self.unused == other.unused &&
        self.blockPoolID == other.blockPoolID &&
        self.storageInfo == other.storageInfo &&
        self.softwareVersion == other.softwareVersion &&
        self.capabilities == other.capabilities &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for NamespaceInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BlockKeyProto {
    // message fields
    keyId: ::std::option::Option<u32>,
    expiryDate: ::std::option::Option<u64>,
    keyBytes: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl BlockKeyProto {
    pub fn new() -> BlockKeyProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockKeyProto {
        static mut instance: ::protobuf::lazy::Lazy<BlockKeyProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockKeyProto,
        };
        unsafe {
            instance.get(|| {
                BlockKeyProto {
                    keyId: ::std::option::Option::None,
                    expiryDate: ::std::option::Option::None,
                    keyBytes: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 keyId = 1;

    pub fn clear_keyId(&mut self) {
        self.keyId = ::std::option::Option::None;
    }

    pub fn has_keyId(&self) -> bool {
        self.keyId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_keyId(&mut self, v: u32) {
        self.keyId = ::std::option::Option::Some(v);
    }

    pub fn get_keyId<'a>(&self) -> u32 {
        self.keyId.unwrap_or(0)
    }

    // required uint64 expiryDate = 2;

    pub fn clear_expiryDate(&mut self) {
        self.expiryDate = ::std::option::Option::None;
    }

    pub fn has_expiryDate(&self) -> bool {
        self.expiryDate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expiryDate(&mut self, v: u64) {
        self.expiryDate = ::std::option::Option::Some(v);
    }

    pub fn get_expiryDate<'a>(&self) -> u64 {
        self.expiryDate.unwrap_or(0)
    }

    // optional bytes keyBytes = 3;

    pub fn clear_keyBytes(&mut self) {
        self.keyBytes.clear();
    }

    pub fn has_keyBytes(&self) -> bool {
        self.keyBytes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_keyBytes(&mut self, v: ::std::vec::Vec<u8>) {
        self.keyBytes = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_keyBytes<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.keyBytes.is_none() {
            self.keyBytes.set_default();
        };
        self.keyBytes.as_mut().unwrap()
    }

    // Take field
    pub fn take_keyBytes(&mut self) -> ::std::vec::Vec<u8> {
        self.keyBytes.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_keyBytes<'a>(&'a self) -> &'a [u8] {
        match self.keyBytes.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for BlockKeyProto {
    fn is_initialized(&self) -> bool {
        if self.keyId.is_none() {
            return false;
        };
        if self.expiryDate.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.keyId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.expiryDate = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.keyBytes.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.keyId.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.expiryDate.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.keyBytes.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.keyId {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.expiryDate {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.keyBytes.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<BlockKeyProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BlockKeyProto {
    fn new() -> BlockKeyProto {
        BlockKeyProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockKeyProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "keyId",
                    BlockKeyProto::has_keyId,
                    BlockKeyProto::get_keyId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "expiryDate",
                    BlockKeyProto::has_expiryDate,
                    BlockKeyProto::get_expiryDate,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "keyBytes",
                    BlockKeyProto::has_keyBytes,
                    BlockKeyProto::get_keyBytes,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockKeyProto>(
                    "BlockKeyProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockKeyProto {
    fn clear(&mut self) {
        self.clear_keyId();
        self.clear_expiryDate();
        self.clear_keyBytes();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BlockKeyProto {
    fn eq(&self, other: &BlockKeyProto) -> bool {
        self.keyId == other.keyId &&
        self.expiryDate == other.expiryDate &&
        self.keyBytes == other.keyBytes &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BlockKeyProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ExportedBlockKeysProto {
    // message fields
    isBlockTokenEnabled: ::std::option::Option<bool>,
    keyUpdateInterval: ::std::option::Option<u64>,
    tokenLifeTime: ::std::option::Option<u64>,
    currentKey: ::protobuf::SingularPtrField<BlockKeyProto>,
    allKeys: ::protobuf::RepeatedField<BlockKeyProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ExportedBlockKeysProto {
    pub fn new() -> ExportedBlockKeysProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExportedBlockKeysProto {
        static mut instance: ::protobuf::lazy::Lazy<ExportedBlockKeysProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExportedBlockKeysProto,
        };
        unsafe {
            instance.get(|| {
                ExportedBlockKeysProto {
                    isBlockTokenEnabled: ::std::option::Option::None,
                    keyUpdateInterval: ::std::option::Option::None,
                    tokenLifeTime: ::std::option::Option::None,
                    currentKey: ::protobuf::SingularPtrField::none(),
                    allKeys: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bool isBlockTokenEnabled = 1;

    pub fn clear_isBlockTokenEnabled(&mut self) {
        self.isBlockTokenEnabled = ::std::option::Option::None;
    }

    pub fn has_isBlockTokenEnabled(&self) -> bool {
        self.isBlockTokenEnabled.is_some()
    }

    // Param is passed by value, moved
    pub fn set_isBlockTokenEnabled(&mut self, v: bool) {
        self.isBlockTokenEnabled = ::std::option::Option::Some(v);
    }

    pub fn get_isBlockTokenEnabled<'a>(&self) -> bool {
        self.isBlockTokenEnabled.unwrap_or(false)
    }

    // required uint64 keyUpdateInterval = 2;

    pub fn clear_keyUpdateInterval(&mut self) {
        self.keyUpdateInterval = ::std::option::Option::None;
    }

    pub fn has_keyUpdateInterval(&self) -> bool {
        self.keyUpdateInterval.is_some()
    }

    // Param is passed by value, moved
    pub fn set_keyUpdateInterval(&mut self, v: u64) {
        self.keyUpdateInterval = ::std::option::Option::Some(v);
    }

    pub fn get_keyUpdateInterval<'a>(&self) -> u64 {
        self.keyUpdateInterval.unwrap_or(0)
    }

    // required uint64 tokenLifeTime = 3;

    pub fn clear_tokenLifeTime(&mut self) {
        self.tokenLifeTime = ::std::option::Option::None;
    }

    pub fn has_tokenLifeTime(&self) -> bool {
        self.tokenLifeTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tokenLifeTime(&mut self, v: u64) {
        self.tokenLifeTime = ::std::option::Option::Some(v);
    }

    pub fn get_tokenLifeTime<'a>(&self) -> u64 {
        self.tokenLifeTime.unwrap_or(0)
    }

    // required .hadoop.hdfs.BlockKeyProto currentKey = 4;

    pub fn clear_currentKey(&mut self) {
        self.currentKey.clear();
    }

    pub fn has_currentKey(&self) -> bool {
        self.currentKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currentKey(&mut self, v: BlockKeyProto) {
        self.currentKey = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currentKey<'a>(&'a mut self) -> &'a mut BlockKeyProto {
        if self.currentKey.is_none() {
            self.currentKey.set_default();
        };
        self.currentKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_currentKey(&mut self) -> BlockKeyProto {
        self.currentKey.take().unwrap_or_else(|| BlockKeyProto::new())
    }

    pub fn get_currentKey<'a>(&'a self) -> &'a BlockKeyProto {
        self.currentKey.as_ref().unwrap_or_else(|| BlockKeyProto::default_instance())
    }

    // repeated .hadoop.hdfs.BlockKeyProto allKeys = 5;

    pub fn clear_allKeys(&mut self) {
        self.allKeys.clear();
    }

    // Param is passed by value, moved
    pub fn set_allKeys(&mut self, v: ::protobuf::RepeatedField<BlockKeyProto>) {
        self.allKeys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_allKeys<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<BlockKeyProto> {
        &mut self.allKeys
    }

    // Take field
    pub fn take_allKeys(&mut self) -> ::protobuf::RepeatedField<BlockKeyProto> {
        ::std::mem::replace(&mut self.allKeys, ::protobuf::RepeatedField::new())
    }

    pub fn get_allKeys<'a>(&'a self) -> &'a [BlockKeyProto] {
        &self.allKeys
    }
}

impl ::protobuf::Message for ExportedBlockKeysProto {
    fn is_initialized(&self) -> bool {
        if self.isBlockTokenEnabled.is_none() {
            return false;
        };
        if self.keyUpdateInterval.is_none() {
            return false;
        };
        if self.tokenLifeTime.is_none() {
            return false;
        };
        if self.currentKey.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.isBlockTokenEnabled = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.keyUpdateInterval = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.tokenLifeTime = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.currentKey.set_default();
                    try!(is.merge_message(tmp))
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.allKeys));
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.isBlockTokenEnabled.is_some() {
            my_size += 2;
        };
        for value in self.keyUpdateInterval.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.tokenLifeTime.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.currentKey.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.allKeys.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.isBlockTokenEnabled {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.keyUpdateInterval {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.tokenLifeTime {
            try!(os.write_uint64(3, v));
        };
        if let Some(v) = self.currentKey.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.allKeys.iter() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ExportedBlockKeysProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ExportedBlockKeysProto {
    fn new() -> ExportedBlockKeysProto {
        ExportedBlockKeysProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExportedBlockKeysProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "isBlockTokenEnabled",
                    ExportedBlockKeysProto::has_isBlockTokenEnabled,
                    ExportedBlockKeysProto::get_isBlockTokenEnabled,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "keyUpdateInterval",
                    ExportedBlockKeysProto::has_keyUpdateInterval,
                    ExportedBlockKeysProto::get_keyUpdateInterval,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "tokenLifeTime",
                    ExportedBlockKeysProto::has_tokenLifeTime,
                    ExportedBlockKeysProto::get_tokenLifeTime,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "currentKey",
                    ExportedBlockKeysProto::has_currentKey,
                    ExportedBlockKeysProto::get_currentKey,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "allKeys",
                    ExportedBlockKeysProto::get_allKeys,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExportedBlockKeysProto>(
                    "ExportedBlockKeysProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExportedBlockKeysProto {
    fn clear(&mut self) {
        self.clear_isBlockTokenEnabled();
        self.clear_keyUpdateInterval();
        self.clear_tokenLifeTime();
        self.clear_currentKey();
        self.clear_allKeys();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ExportedBlockKeysProto {
    fn eq(&self, other: &ExportedBlockKeysProto) -> bool {
        self.isBlockTokenEnabled == other.isBlockTokenEnabled &&
        self.keyUpdateInterval == other.keyUpdateInterval &&
        self.tokenLifeTime == other.tokenLifeTime &&
        self.currentKey == other.currentKey &&
        self.allKeys == other.allKeys &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ExportedBlockKeysProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RecoveringBlockProto {
    // message fields
    newGenStamp: ::std::option::Option<u64>,
    block: ::protobuf::SingularPtrField<LocatedBlockProto>,
    truncateBlock: ::protobuf::SingularPtrField<BlockProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RecoveringBlockProto {
    pub fn new() -> RecoveringBlockProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RecoveringBlockProto {
        static mut instance: ::protobuf::lazy::Lazy<RecoveringBlockProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RecoveringBlockProto,
        };
        unsafe {
            instance.get(|| {
                RecoveringBlockProto {
                    newGenStamp: ::std::option::Option::None,
                    block: ::protobuf::SingularPtrField::none(),
                    truncateBlock: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint64 newGenStamp = 1;

    pub fn clear_newGenStamp(&mut self) {
        self.newGenStamp = ::std::option::Option::None;
    }

    pub fn has_newGenStamp(&self) -> bool {
        self.newGenStamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_newGenStamp(&mut self, v: u64) {
        self.newGenStamp = ::std::option::Option::Some(v);
    }

    pub fn get_newGenStamp<'a>(&self) -> u64 {
        self.newGenStamp.unwrap_or(0)
    }

    // required .hadoop.hdfs.LocatedBlockProto block = 2;

    pub fn clear_block(&mut self) {
        self.block.clear();
    }

    pub fn has_block(&self) -> bool {
        self.block.is_some()
    }

    // Param is passed by value, moved
    pub fn set_block(&mut self, v: LocatedBlockProto) {
        self.block = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block<'a>(&'a mut self) -> &'a mut LocatedBlockProto {
        if self.block.is_none() {
            self.block.set_default();
        };
        self.block.as_mut().unwrap()
    }

    // Take field
    pub fn take_block(&mut self) -> LocatedBlockProto {
        self.block.take().unwrap_or_else(|| LocatedBlockProto::new())
    }

    pub fn get_block<'a>(&'a self) -> &'a LocatedBlockProto {
        self.block.as_ref().unwrap_or_else(|| LocatedBlockProto::default_instance())
    }

    // optional .hadoop.hdfs.BlockProto truncateBlock = 3;

    pub fn clear_truncateBlock(&mut self) {
        self.truncateBlock.clear();
    }

    pub fn has_truncateBlock(&self) -> bool {
        self.truncateBlock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_truncateBlock(&mut self, v: BlockProto) {
        self.truncateBlock = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_truncateBlock<'a>(&'a mut self) -> &'a mut BlockProto {
        if self.truncateBlock.is_none() {
            self.truncateBlock.set_default();
        };
        self.truncateBlock.as_mut().unwrap()
    }

    // Take field
    pub fn take_truncateBlock(&mut self) -> BlockProto {
        self.truncateBlock.take().unwrap_or_else(|| BlockProto::new())
    }

    pub fn get_truncateBlock<'a>(&'a self) -> &'a BlockProto {
        self.truncateBlock.as_ref().unwrap_or_else(|| BlockProto::default_instance())
    }
}

impl ::protobuf::Message for RecoveringBlockProto {
    fn is_initialized(&self) -> bool {
        if self.newGenStamp.is_none() {
            return false;
        };
        if self.block.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.newGenStamp = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.block.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.truncateBlock.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.newGenStamp.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.block.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.truncateBlock.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.newGenStamp {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.block.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.truncateBlock.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<RecoveringBlockProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RecoveringBlockProto {
    fn new() -> RecoveringBlockProto {
        RecoveringBlockProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RecoveringBlockProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "newGenStamp",
                    RecoveringBlockProto::has_newGenStamp,
                    RecoveringBlockProto::get_newGenStamp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "block",
                    RecoveringBlockProto::has_block,
                    RecoveringBlockProto::get_block,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "truncateBlock",
                    RecoveringBlockProto::has_truncateBlock,
                    RecoveringBlockProto::get_truncateBlock,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RecoveringBlockProto>(
                    "RecoveringBlockProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RecoveringBlockProto {
    fn clear(&mut self) {
        self.clear_newGenStamp();
        self.clear_block();
        self.clear_truncateBlock();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RecoveringBlockProto {
    fn eq(&self, other: &RecoveringBlockProto) -> bool {
        self.newGenStamp == other.newGenStamp &&
        self.block == other.block &&
        self.truncateBlock == other.truncateBlock &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RecoveringBlockProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct VersionRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl VersionRequestProto {
    pub fn new() -> VersionRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VersionRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<VersionRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VersionRequestProto,
        };
        unsafe {
            instance.get(|| {
                VersionRequestProto {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for VersionRequestProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<VersionRequestProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for VersionRequestProto {
    fn new() -> VersionRequestProto {
        VersionRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<VersionRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<VersionRequestProto>(
                    "VersionRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VersionRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for VersionRequestProto {
    fn eq(&self, other: &VersionRequestProto) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for VersionRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct VersionResponseProto {
    // message fields
    info: ::protobuf::SingularPtrField<NamespaceInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl VersionResponseProto {
    pub fn new() -> VersionResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VersionResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<VersionResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VersionResponseProto,
        };
        unsafe {
            instance.get(|| {
                VersionResponseProto {
                    info: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.NamespaceInfoProto info = 1;

    pub fn clear_info(&mut self) {
        self.info.clear();
    }

    pub fn has_info(&self) -> bool {
        self.info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_info(&mut self, v: NamespaceInfoProto) {
        self.info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_info<'a>(&'a mut self) -> &'a mut NamespaceInfoProto {
        if self.info.is_none() {
            self.info.set_default();
        };
        self.info.as_mut().unwrap()
    }

    // Take field
    pub fn take_info(&mut self) -> NamespaceInfoProto {
        self.info.take().unwrap_or_else(|| NamespaceInfoProto::new())
    }

    pub fn get_info<'a>(&'a self) -> &'a NamespaceInfoProto {
        self.info.as_ref().unwrap_or_else(|| NamespaceInfoProto::default_instance())
    }
}

impl ::protobuf::Message for VersionResponseProto {
    fn is_initialized(&self) -> bool {
        if self.info.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.info.set_default();
                    try!(is.merge_message(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.info.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.info.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<VersionResponseProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for VersionResponseProto {
    fn new() -> VersionResponseProto {
        VersionResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<VersionResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "info",
                    VersionResponseProto::has_info,
                    VersionResponseProto::get_info,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VersionResponseProto>(
                    "VersionResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VersionResponseProto {
    fn clear(&mut self) {
        self.clear_info();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for VersionResponseProto {
    fn eq(&self, other: &VersionResponseProto) -> bool {
        self.info == other.info &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for VersionResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SnapshotInfoProto {
    // message fields
    snapshotName: ::protobuf::SingularField<::std::string::String>,
    snapshotRoot: ::protobuf::SingularField<::std::string::String>,
    permission: ::protobuf::SingularPtrField<FsPermissionProto>,
    owner: ::protobuf::SingularField<::std::string::String>,
    group: ::protobuf::SingularField<::std::string::String>,
    createTime: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl SnapshotInfoProto {
    pub fn new() -> SnapshotInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshotInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<SnapshotInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshotInfoProto,
        };
        unsafe {
            instance.get(|| {
                SnapshotInfoProto {
                    snapshotName: ::protobuf::SingularField::none(),
                    snapshotRoot: ::protobuf::SingularField::none(),
                    permission: ::protobuf::SingularPtrField::none(),
                    owner: ::protobuf::SingularField::none(),
                    group: ::protobuf::SingularField::none(),
                    createTime: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string snapshotName = 1;

    pub fn clear_snapshotName(&mut self) {
        self.snapshotName.clear();
    }

    pub fn has_snapshotName(&self) -> bool {
        self.snapshotName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_snapshotName(&mut self, v: ::std::string::String) {
        self.snapshotName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_snapshotName<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.snapshotName.is_none() {
            self.snapshotName.set_default();
        };
        self.snapshotName.as_mut().unwrap()
    }

    // Take field
    pub fn take_snapshotName(&mut self) -> ::std::string::String {
        self.snapshotName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_snapshotName<'a>(&'a self) -> &'a str {
        match self.snapshotName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string snapshotRoot = 2;

    pub fn clear_snapshotRoot(&mut self) {
        self.snapshotRoot.clear();
    }

    pub fn has_snapshotRoot(&self) -> bool {
        self.snapshotRoot.is_some()
    }

    // Param is passed by value, moved
    pub fn set_snapshotRoot(&mut self, v: ::std::string::String) {
        self.snapshotRoot = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_snapshotRoot<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.snapshotRoot.is_none() {
            self.snapshotRoot.set_default();
        };
        self.snapshotRoot.as_mut().unwrap()
    }

    // Take field
    pub fn take_snapshotRoot(&mut self) -> ::std::string::String {
        self.snapshotRoot.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_snapshotRoot<'a>(&'a self) -> &'a str {
        match self.snapshotRoot.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required .hadoop.hdfs.FsPermissionProto permission = 3;

    pub fn clear_permission(&mut self) {
        self.permission.clear();
    }

    pub fn has_permission(&self) -> bool {
        self.permission.is_some()
    }

    // Param is passed by value, moved
    pub fn set_permission(&mut self, v: FsPermissionProto) {
        self.permission = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_permission<'a>(&'a mut self) -> &'a mut FsPermissionProto {
        if self.permission.is_none() {
            self.permission.set_default();
        };
        self.permission.as_mut().unwrap()
    }

    // Take field
    pub fn take_permission(&mut self) -> FsPermissionProto {
        self.permission.take().unwrap_or_else(|| FsPermissionProto::new())
    }

    pub fn get_permission<'a>(&'a self) -> &'a FsPermissionProto {
        self.permission.as_ref().unwrap_or_else(|| FsPermissionProto::default_instance())
    }

    // required string owner = 4;

    pub fn clear_owner(&mut self) {
        self.owner.clear();
    }

    pub fn has_owner(&self) -> bool {
        self.owner.is_some()
    }

    // Param is passed by value, moved
    pub fn set_owner(&mut self, v: ::std::string::String) {
        self.owner = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_owner<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.owner.is_none() {
            self.owner.set_default();
        };
        self.owner.as_mut().unwrap()
    }

    // Take field
    pub fn take_owner(&mut self) -> ::std::string::String {
        self.owner.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_owner<'a>(&'a self) -> &'a str {
        match self.owner.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string group = 5;

    pub fn clear_group(&mut self) {
        self.group.clear();
    }

    pub fn has_group(&self) -> bool {
        self.group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_group(&mut self, v: ::std::string::String) {
        self.group = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_group<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.group.is_none() {
            self.group.set_default();
        };
        self.group.as_mut().unwrap()
    }

    // Take field
    pub fn take_group(&mut self) -> ::std::string::String {
        self.group.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_group<'a>(&'a self) -> &'a str {
        match self.group.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string createTime = 6;

    pub fn clear_createTime(&mut self) {
        self.createTime.clear();
    }

    pub fn has_createTime(&self) -> bool {
        self.createTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_createTime(&mut self, v: ::std::string::String) {
        self.createTime = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_createTime<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.createTime.is_none() {
            self.createTime.set_default();
        };
        self.createTime.as_mut().unwrap()
    }

    // Take field
    pub fn take_createTime(&mut self) -> ::std::string::String {
        self.createTime.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_createTime<'a>(&'a self) -> &'a str {
        match self.createTime.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for SnapshotInfoProto {
    fn is_initialized(&self) -> bool {
        if self.snapshotName.is_none() {
            return false;
        };
        if self.snapshotRoot.is_none() {
            return false;
        };
        if self.permission.is_none() {
            return false;
        };
        if self.owner.is_none() {
            return false;
        };
        if self.group.is_none() {
            return false;
        };
        if self.createTime.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.snapshotName.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.snapshotRoot.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.permission.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.owner.set_default();
                    try!(is.read_string_into(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.group.set_default();
                    try!(is.read_string_into(tmp))
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.createTime.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.snapshotName.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.snapshotRoot.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.permission.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.owner.iter() {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in self.group.iter() {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        for value in self.createTime.iter() {
            my_size += ::protobuf::rt::string_size(6, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.snapshotName.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.snapshotRoot.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.permission.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.owner.as_ref() {
            try!(os.write_string(4, &v));
        };
        if let Some(v) = self.group.as_ref() {
            try!(os.write_string(5, &v));
        };
        if let Some(v) = self.createTime.as_ref() {
            try!(os.write_string(6, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<SnapshotInfoProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SnapshotInfoProto {
    fn new() -> SnapshotInfoProto {
        SnapshotInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshotInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "snapshotName",
                    SnapshotInfoProto::has_snapshotName,
                    SnapshotInfoProto::get_snapshotName,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "snapshotRoot",
                    SnapshotInfoProto::has_snapshotRoot,
                    SnapshotInfoProto::get_snapshotRoot,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "permission",
                    SnapshotInfoProto::has_permission,
                    SnapshotInfoProto::get_permission,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "owner",
                    SnapshotInfoProto::has_owner,
                    SnapshotInfoProto::get_owner,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "group",
                    SnapshotInfoProto::has_group,
                    SnapshotInfoProto::get_group,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "createTime",
                    SnapshotInfoProto::has_createTime,
                    SnapshotInfoProto::get_createTime,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapshotInfoProto>(
                    "SnapshotInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshotInfoProto {
    fn clear(&mut self) {
        self.clear_snapshotName();
        self.clear_snapshotRoot();
        self.clear_permission();
        self.clear_owner();
        self.clear_group();
        self.clear_createTime();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SnapshotInfoProto {
    fn eq(&self, other: &SnapshotInfoProto) -> bool {
        self.snapshotName == other.snapshotName &&
        self.snapshotRoot == other.snapshotRoot &&
        self.permission == other.permission &&
        self.owner == other.owner &&
        self.group == other.group &&
        self.createTime == other.createTime &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SnapshotInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RollingUpgradeStatusProto {
    // message fields
    blockPoolId: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RollingUpgradeStatusProto {
    pub fn new() -> RollingUpgradeStatusProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RollingUpgradeStatusProto {
        static mut instance: ::protobuf::lazy::Lazy<RollingUpgradeStatusProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RollingUpgradeStatusProto,
        };
        unsafe {
            instance.get(|| {
                RollingUpgradeStatusProto {
                    blockPoolId: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string blockPoolId = 1;

    pub fn clear_blockPoolId(&mut self) {
        self.blockPoolId.clear();
    }

    pub fn has_blockPoolId(&self) -> bool {
        self.blockPoolId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockPoolId(&mut self, v: ::std::string::String) {
        self.blockPoolId = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blockPoolId<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.blockPoolId.is_none() {
            self.blockPoolId.set_default();
        };
        self.blockPoolId.as_mut().unwrap()
    }

    // Take field
    pub fn take_blockPoolId(&mut self) -> ::std::string::String {
        self.blockPoolId.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_blockPoolId<'a>(&'a self) -> &'a str {
        match self.blockPoolId.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for RollingUpgradeStatusProto {
    fn is_initialized(&self) -> bool {
        if self.blockPoolId.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.blockPoolId.set_default();
                    try!(is.read_string_into(tmp))
                },
                _ => {
                    let unknown = try!(is.read_unknown(wire_type));
                    self.mut_unknown_fields().add_value(field_number, unknown);
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.blockPoolId.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.blockPoolId.as_ref() {
            try!(os.write_string(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields<'s>(&'s self) -> &'s ::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields<'s>(&'s mut self) -> &'s mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<RollingUpgradeStatusProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RollingUpgradeStatusProto {
    fn new() -> RollingUpgradeStatusProto {
        RollingUpgradeStatusProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RollingUpgradeStatusProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "blockPoolId",
                    RollingUpgradeStatusProto::has_blockPoolId,
                    RollingUpgradeStatusProto::get_blockPoolId,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RollingUpgradeStatusProto>(
                    "RollingUpgradeStatusProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RollingUpgradeStatusProto {
    fn clear(&mut self) {
        self.clear_blockPoolId();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RollingUpgradeStatusProto {
    fn eq(&self, other: &RollingUpgradeStatusProto) -> bool {
        self.blockPoolId == other.blockPoolId &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RollingUpgradeStatusProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum StorageTypeProto {
    DISK = 1,
    SSD = 2,
    ARCHIVE = 3,
    RAM_DISK = 4,
}

impl ::protobuf::ProtobufEnum for StorageTypeProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<StorageTypeProto> {
        match value {
            1 => ::std::option::Option::Some(StorageTypeProto::DISK),
            2 => ::std::option::Option::Some(StorageTypeProto::SSD),
            3 => ::std::option::Option::Some(StorageTypeProto::ARCHIVE),
            4 => ::std::option::Option::Some(StorageTypeProto::RAM_DISK),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<StorageTypeProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("StorageTypeProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for StorageTypeProto {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CipherSuiteProto {
    UNKNOWN = 1,
    AES_CTR_NOPADDING = 2,
}

impl ::protobuf::ProtobufEnum for CipherSuiteProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CipherSuiteProto> {
        match value {
            1 => ::std::option::Option::Some(CipherSuiteProto::UNKNOWN),
            2 => ::std::option::Option::Some(CipherSuiteProto::AES_CTR_NOPADDING),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<CipherSuiteProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CipherSuiteProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CipherSuiteProto {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CryptoProtocolVersionProto {
    UNKNOWN_PROTOCOL_VERSION = 1,
    ENCRYPTION_ZONES = 2,
}

impl ::protobuf::ProtobufEnum for CryptoProtocolVersionProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CryptoProtocolVersionProto> {
        match value {
            1 => ::std::option::Option::Some(CryptoProtocolVersionProto::UNKNOWN_PROTOCOL_VERSION),
            2 => ::std::option::Option::Some(CryptoProtocolVersionProto::ENCRYPTION_ZONES),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<CryptoProtocolVersionProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CryptoProtocolVersionProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CryptoProtocolVersionProto {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ChecksumTypeProto {
    CHECKSUM_NULL = 0,
    CHECKSUM_CRC32 = 1,
    CHECKSUM_CRC32C = 2,
}

impl ::protobuf::ProtobufEnum for ChecksumTypeProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ChecksumTypeProto> {
        match value {
            0 => ::std::option::Option::Some(ChecksumTypeProto::CHECKSUM_NULL),
            1 => ::std::option::Option::Some(ChecksumTypeProto::CHECKSUM_CRC32),
            2 => ::std::option::Option::Some(ChecksumTypeProto::CHECKSUM_CRC32C),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<ChecksumTypeProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ChecksumTypeProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ChecksumTypeProto {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ReplicaStateProto {
    FINALIZED = 0,
    RBW = 1,
    RWR = 2,
    RUR = 3,
    TEMPORARY = 4,
}

impl ::protobuf::ProtobufEnum for ReplicaStateProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ReplicaStateProto> {
        match value {
            0 => ::std::option::Option::Some(ReplicaStateProto::FINALIZED),
            1 => ::std::option::Option::Some(ReplicaStateProto::RBW),
            2 => ::std::option::Option::Some(ReplicaStateProto::RWR),
            3 => ::std::option::Option::Some(ReplicaStateProto::RUR),
            4 => ::std::option::Option::Some(ReplicaStateProto::TEMPORARY),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<ReplicaStateProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ReplicaStateProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ReplicaStateProto {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0a, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x68, 0x61,
    0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x1a, 0x0e, 0x53, 0x65, 0x63, 0x75, 0x72,
    0x69, 0x74, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x63, 0x0a, 0x12, 0x45, 0x78, 0x74,
    0x65, 0x6e, 0x64, 0x65, 0x64, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x0e, 0x0a, 0x06, 0x70, 0x6f, 0x6f, 0x6c, 0x49, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12,
    0x0f, 0x0a, 0x07, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x49, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04,
    0x12, 0x17, 0x0a, 0x0f, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74,
    0x61, 0x6d, 0x70, 0x18, 0x03, 0x20, 0x02, 0x28, 0x04, 0x12, 0x13, 0x0a, 0x08, 0x6e, 0x75, 0x6d,
    0x42, 0x79, 0x74, 0x65, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04, 0x3a, 0x01, 0x30, 0x22, 0x99,
    0x01, 0x0a, 0x0f, 0x44, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x49, 0x44, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x0e, 0x0a, 0x06, 0x69, 0x70, 0x41, 0x64, 0x64, 0x72, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x09, 0x12, 0x10, 0x0a, 0x08, 0x68, 0x6f, 0x73, 0x74, 0x4e, 0x61, 0x6d, 0x65, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x09, 0x12, 0x14, 0x0a, 0x0c, 0x64, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65,
    0x55, 0x75, 0x69, 0x64, 0x18, 0x03, 0x20, 0x02, 0x28, 0x09, 0x12, 0x10, 0x0a, 0x08, 0x78, 0x66,
    0x65, 0x72, 0x50, 0x6f, 0x72, 0x74, 0x18, 0x04, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x10, 0x0a, 0x08,
    0x69, 0x6e, 0x66, 0x6f, 0x50, 0x6f, 0x72, 0x74, 0x18, 0x05, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x0f,
    0x0a, 0x07, 0x69, 0x70, 0x63, 0x50, 0x6f, 0x72, 0x74, 0x18, 0x06, 0x20, 0x02, 0x28, 0x0d, 0x12,
    0x19, 0x0a, 0x0e, 0x69, 0x6e, 0x66, 0x6f, 0x53, 0x65, 0x63, 0x75, 0x72, 0x65, 0x50, 0x6f, 0x72,
    0x74, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0d, 0x3a, 0x01, 0x30, 0x22, 0x58, 0x0a, 0x16, 0x44, 0x61,
    0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x4c, 0x6f, 0x63, 0x61, 0x6c, 0x49, 0x6e, 0x66, 0x6f, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x17, 0x0a, 0x0f, 0x73, 0x6f, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65,
    0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x15, 0x0a,
    0x0d, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x09, 0x12, 0x0e, 0x0a, 0x06, 0x75, 0x70, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x03,
    0x20, 0x02, 0x28, 0x04, 0x22, 0x47, 0x0a, 0x12, 0x44, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65,
    0x49, 0x6e, 0x66, 0x6f, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x31, 0x0a, 0x09, 0x64, 0x61,
    0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1e, 0x2e,
    0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61,
    0x6e, 0x6f, 0x64, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xba, 0x03,
    0x0a, 0x11, 0x44, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x28, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32,
    0x1c, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x44, 0x61,
    0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x49, 0x44, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x13, 0x0a,
    0x08, 0x63, 0x61, 0x70, 0x61, 0x63, 0x69, 0x74, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x3a,
    0x01, 0x30, 0x12, 0x12, 0x0a, 0x07, 0x64, 0x66, 0x73, 0x55, 0x73, 0x65, 0x64, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x04, 0x3a, 0x01, 0x30, 0x12, 0x14, 0x0a, 0x09, 0x72, 0x65, 0x6d, 0x61, 0x69, 0x6e,
    0x69, 0x6e, 0x67, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04, 0x3a, 0x01, 0x30, 0x12, 0x18, 0x0a, 0x0d,
    0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x50, 0x6f, 0x6f, 0x6c, 0x55, 0x73, 0x65, 0x64, 0x18, 0x05, 0x20,
    0x01, 0x28, 0x04, 0x3a, 0x01, 0x30, 0x12, 0x15, 0x0a, 0x0a, 0x6c, 0x61, 0x73, 0x74, 0x55, 0x70,
    0x64, 0x61, 0x74, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x04, 0x3a, 0x01, 0x30, 0x12, 0x17, 0x0a,
    0x0c, 0x78, 0x63, 0x65, 0x69, 0x76, 0x65, 0x72, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x07, 0x20,
    0x01, 0x28, 0x0d, 0x3a, 0x01, 0x30, 0x12, 0x10, 0x0a, 0x08, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x18, 0x08, 0x20, 0x01, 0x28, 0x09, 0x12, 0x45, 0x0a, 0x0a, 0x61, 0x64, 0x6d, 0x69,
    0x6e, 0x53, 0x74, 0x61, 0x74, 0x65, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x29, 0x2e, 0x68,
    0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x6e,
    0x6f, 0x64, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x41, 0x64, 0x6d,
    0x69, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x65, 0x3a, 0x06, 0x4e, 0x4f, 0x52, 0x4d, 0x41, 0x4c, 0x12,
    0x18, 0x0a, 0x0d, 0x63, 0x61, 0x63, 0x68, 0x65, 0x43, 0x61, 0x70, 0x61, 0x63, 0x69, 0x74, 0x79,
    0x18, 0x0b, 0x20, 0x01, 0x28, 0x04, 0x3a, 0x01, 0x30, 0x12, 0x14, 0x0a, 0x09, 0x63, 0x61, 0x63,
    0x68, 0x65, 0x55, 0x73, 0x65, 0x64, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x04, 0x3a, 0x01, 0x30, 0x12,
    0x1e, 0x0a, 0x13, 0x6c, 0x61, 0x73, 0x74, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x4d, 0x6f, 0x6e,
    0x6f, 0x74, 0x6f, 0x6e, 0x69, 0x63, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x04, 0x3a, 0x01, 0x30, 0x22,
    0x49, 0x0a, 0x0a, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x0a, 0x0a,
    0x06, 0x4e, 0x4f, 0x52, 0x4d, 0x41, 0x4c, 0x10, 0x00, 0x12, 0x1b, 0x0a, 0x17, 0x44, 0x45, 0x43,
    0x4f, 0x4d, 0x4d, 0x49, 0x53, 0x53, 0x49, 0x4f, 0x4e, 0x5f, 0x49, 0x4e, 0x50, 0x52, 0x4f, 0x47,
    0x52, 0x45, 0x53, 0x53, 0x10, 0x01, 0x12, 0x12, 0x0a, 0x0e, 0x44, 0x45, 0x43, 0x4f, 0x4d, 0x4d,
    0x49, 0x53, 0x53, 0x49, 0x4f, 0x4e, 0x45, 0x44, 0x10, 0x02, 0x22, 0xde, 0x01, 0x0a, 0x14, 0x44,
    0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x13, 0x0a, 0x0b, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x55, 0x75,
    0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x45, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x74,
    0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x2e, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70,
    0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x53, 0x74,
    0x6f, 0x72, 0x61, 0x67, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x53, 0x74, 0x6f, 0x72, 0x61,
    0x67, 0x65, 0x53, 0x74, 0x61, 0x74, 0x65, 0x3a, 0x06, 0x4e, 0x4f, 0x52, 0x4d, 0x41, 0x4c, 0x12,
    0x38, 0x0a, 0x0b, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0e, 0x32, 0x1d, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64,
    0x66, 0x73, 0x2e, 0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x3a, 0x04, 0x44, 0x49, 0x53, 0x4b, 0x22, 0x30, 0x0a, 0x0c, 0x53, 0x74, 0x6f,
    0x72, 0x61, 0x67, 0x65, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x0a, 0x0a, 0x06, 0x4e, 0x4f, 0x52,
    0x4d, 0x41, 0x4c, 0x10, 0x00, 0x12, 0x14, 0x0a, 0x10, 0x52, 0x45, 0x41, 0x44, 0x5f, 0x4f, 0x4e,
    0x4c, 0x59, 0x5f, 0x53, 0x48, 0x41, 0x52, 0x45, 0x44, 0x10, 0x01, 0x22, 0xd1, 0x01, 0x0a, 0x12,
    0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x52, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x17, 0x0a, 0x0b, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x55, 0x75, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x42, 0x02, 0x18, 0x01, 0x12, 0x15, 0x0a, 0x06, 0x66,
    0x61, 0x69, 0x6c, 0x65, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x05, 0x66, 0x61, 0x6c,
    0x73, 0x65, 0x12, 0x13, 0x0a, 0x08, 0x63, 0x61, 0x70, 0x61, 0x63, 0x69, 0x74, 0x79, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x04, 0x3a, 0x01, 0x30, 0x12, 0x12, 0x0a, 0x07, 0x64, 0x66, 0x73, 0x55, 0x73,
    0x65, 0x64, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04, 0x3a, 0x01, 0x30, 0x12, 0x14, 0x0a, 0x09, 0x72,
    0x65, 0x6d, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x18, 0x05, 0x20, 0x01, 0x28, 0x04, 0x3a, 0x01,
    0x30, 0x12, 0x18, 0x0a, 0x0d, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x50, 0x6f, 0x6f, 0x6c, 0x55, 0x73,
    0x65, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x04, 0x3a, 0x01, 0x30, 0x12, 0x32, 0x0a, 0x07, 0x73,
    0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x68,
    0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x6e,
    0x6f, 0x64, 0x65, 0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22,
    0xcb, 0x01, 0x0a, 0x13, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x53, 0x75, 0x6d, 0x6d, 0x61,
    0x72, 0x79, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0e, 0x0a, 0x06, 0x6c, 0x65, 0x6e, 0x67, 0x74,
    0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x12, 0x11, 0x0a, 0x09, 0x66, 0x69, 0x6c, 0x65, 0x43,
    0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x12, 0x16, 0x0a, 0x0e, 0x64, 0x69,
    0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x03, 0x20, 0x02,
    0x28, 0x04, 0x12, 0x0d, 0x0a, 0x05, 0x71, 0x75, 0x6f, 0x74, 0x61, 0x18, 0x04, 0x20, 0x02, 0x28,
    0x04, 0x12, 0x15, 0x0a, 0x0d, 0x73, 0x70, 0x61, 0x63, 0x65, 0x43, 0x6f, 0x6e, 0x73, 0x75, 0x6d,
    0x65, 0x64, 0x18, 0x05, 0x20, 0x02, 0x28, 0x04, 0x12, 0x12, 0x0a, 0x0a, 0x73, 0x70, 0x61, 0x63,
    0x65, 0x51, 0x75, 0x6f, 0x74, 0x61, 0x18, 0x06, 0x20, 0x02, 0x28, 0x04, 0x12, 0x3f, 0x0a, 0x0e,
    0x74, 0x79, 0x70, 0x65, 0x51, 0x75, 0x6f, 0x74, 0x61, 0x49, 0x6e, 0x66, 0x6f, 0x73, 0x18, 0x07,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x27, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64,
    0x66, 0x73, 0x2e, 0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x51, 0x75,
    0x6f, 0x74, 0x61, 0x49, 0x6e, 0x66, 0x6f, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x5b, 0x0a,
    0x1a, 0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x51, 0x75, 0x6f, 0x74,
    0x61, 0x49, 0x6e, 0x66, 0x6f, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x3d, 0x0a, 0x0d, 0x74,
    0x79, 0x70, 0x65, 0x51, 0x75, 0x6f, 0x74, 0x61, 0x49, 0x6e, 0x66, 0x6f, 0x18, 0x01, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x26, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73,
    0x2e, 0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x51, 0x75, 0x6f, 0x74,
    0x61, 0x49, 0x6e, 0x66, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x69, 0x0a, 0x19, 0x53, 0x74,
    0x6f, 0x72, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x51, 0x75, 0x6f, 0x74, 0x61, 0x49, 0x6e,
    0x66, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2b, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x1d, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68,
    0x64, 0x66, 0x73, 0x2e, 0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0d, 0x0a, 0x05, 0x71, 0x75, 0x6f, 0x74, 0x61, 0x18, 0x02, 0x20,
    0x02, 0x28, 0x04, 0x12, 0x10, 0x0a, 0x08, 0x63, 0x6f, 0x6e, 0x73, 0x75, 0x6d, 0x65, 0x64, 0x18,
    0x03, 0x20, 0x02, 0x28, 0x04, 0x22, 0x37, 0x0a, 0x16, 0x43, 0x6f, 0x72, 0x72, 0x75, 0x70, 0x74,
    0x46, 0x69, 0x6c, 0x65, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x0d, 0x0a, 0x05, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x09, 0x12, 0x0e,
    0x0a, 0x06, 0x63, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x22, 0x21,
    0x0a, 0x11, 0x46, 0x73, 0x50, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x0c, 0x0a, 0x04, 0x70, 0x65, 0x72, 0x6d, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x0d, 0x22, 0x48, 0x0a, 0x11, 0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65,
    0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x33, 0x0a, 0x0c, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67,
    0x65, 0x54, 0x79, 0x70, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0e, 0x32, 0x1d, 0x2e, 0x68,
    0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x53, 0x74, 0x6f, 0x72, 0x61,
    0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xf4, 0x01, 0x0a, 0x17,
    0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x50, 0x6f, 0x6c, 0x69,
    0x63, 0x79, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x10, 0x0a, 0x08, 0x70, 0x6f, 0x6c, 0x69, 0x63,
    0x79, 0x49, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x12, 0x36, 0x0a, 0x0e, 0x63, 0x72, 0x65, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x50, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0b, 0x32,
    0x1e, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x53, 0x74,
    0x6f, 0x72, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x3e, 0x0a, 0x16, 0x63, 0x72, 0x65, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x46, 0x61, 0x6c, 0x6c, 0x62,
    0x61, 0x63, 0x6b, 0x50, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1e, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x53, 0x74,
    0x6f, 0x72, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x41, 0x0a, 0x19, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x46, 0x61,
    0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x50, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73,
    0x2e, 0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x73, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x22, 0x29, 0x0a, 0x11, 0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x55, 0x75, 0x69,
    0x64, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x14, 0x0a, 0x0c, 0x73, 0x74, 0x6f, 0x72, 0x61,
    0x67, 0x65, 0x55, 0x75, 0x69, 0x64, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x09, 0x22, 0x9c, 0x02,
    0x0a, 0x11, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x65, 0x64, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x2a, 0x0a, 0x01, 0x62, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x1f,
    0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x45, 0x78, 0x74,
    0x65, 0x6e, 0x64, 0x65, 0x64, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x0e, 0x0a, 0x06, 0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x12,
    0x2c, 0x0a, 0x04, 0x6c, 0x6f, 0x63, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1e, 0x2e,
    0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61,
    0x6e, 0x6f, 0x64, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0f, 0x0a,
    0x07, 0x63, 0x6f, 0x72, 0x72, 0x75, 0x70, 0x74, 0x18, 0x04, 0x20, 0x02, 0x28, 0x08, 0x12, 0x2d,
    0x0a, 0x0a, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x05, 0x20, 0x02,
    0x28, 0x0b, 0x32, 0x19, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x63, 0x6f, 0x6d, 0x6d,
    0x6f, 0x6e, 0x2e, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x14, 0x0a,
    0x08, 0x69, 0x73, 0x43, 0x61, 0x63, 0x68, 0x65, 0x64, 0x18, 0x06, 0x20, 0x03, 0x28, 0x08, 0x42,
    0x02, 0x10, 0x01, 0x12, 0x33, 0x0a, 0x0c, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x54, 0x79,
    0x70, 0x65, 0x73, 0x18, 0x07, 0x20, 0x03, 0x28, 0x0e, 0x32, 0x1d, 0x2e, 0x68, 0x61, 0x64, 0x6f,
    0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x54,
    0x79, 0x70, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x12, 0x0a, 0x0a, 0x73, 0x74, 0x6f, 0x72,
    0x61, 0x67, 0x65, 0x49, 0x44, 0x73, 0x18, 0x08, 0x20, 0x03, 0x28, 0x09, 0x22, 0x93, 0x01, 0x0a,
    0x16, 0x44, 0x61, 0x74, 0x61, 0x45, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x4b,
    0x65, 0x79, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0d, 0x0a, 0x05, 0x6b, 0x65, 0x79, 0x49, 0x64,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x13, 0x0a, 0x0b, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x50,
    0x6f, 0x6f, 0x6c, 0x49, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0d, 0x0a, 0x05, 0x6e,
    0x6f, 0x6e, 0x63, 0x65, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x15, 0x0a, 0x0d, 0x65, 0x6e,
    0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x4b, 0x65, 0x79, 0x18, 0x04, 0x20, 0x02, 0x28,
    0x0c, 0x12, 0x12, 0x0a, 0x0a, 0x65, 0x78, 0x70, 0x69, 0x72, 0x79, 0x44, 0x61, 0x74, 0x65, 0x18,
    0x05, 0x20, 0x02, 0x28, 0x04, 0x12, 0x1b, 0x0a, 0x13, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74,
    0x69, 0x6f, 0x6e, 0x41, 0x6c, 0x67, 0x6f, 0x72, 0x69, 0x74, 0x68, 0x6d, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x09, 0x22, 0xd3, 0x01, 0x0a, 0x17, 0x46, 0x69, 0x6c, 0x65, 0x45, 0x6e, 0x63, 0x72, 0x79,
    0x70, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2c,
    0x0a, 0x05, 0x73, 0x75, 0x69, 0x74, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x1d, 0x2e,
    0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x43, 0x69, 0x70, 0x68,
    0x65, 0x72, 0x53, 0x75, 0x69, 0x74, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x46, 0x0a, 0x15,
    0x63, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x56, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x27, 0x2e, 0x68, 0x61,
    0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x43, 0x72, 0x79, 0x70, 0x74, 0x6f,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x02, 0x28,
    0x0c, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x76, 0x18, 0x04, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0f, 0x0a,
    0x07, 0x6b, 0x65, 0x79, 0x4e, 0x61, 0x6d, 0x65, 0x18, 0x05, 0x20, 0x02, 0x28, 0x09, 0x12, 0x18,
    0x0a, 0x10, 0x65, 0x7a, 0x4b, 0x65, 0x79, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x4e, 0x61,
    0x6d, 0x65, 0x18, 0x06, 0x20, 0x02, 0x28, 0x09, 0x22, 0x4f, 0x0a, 0x1a, 0x50, 0x65, 0x72, 0x46,
    0x69, 0x6c, 0x65, 0x45, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x66,
    0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x0c, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x76, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x12,
    0x18, 0x0a, 0x10, 0x65, 0x7a, 0x4b, 0x65, 0x79, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x4e,
    0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x02, 0x28, 0x09, 0x22, 0xa0, 0x01, 0x0a, 0x17, 0x5a, 0x6f,
    0x6e, 0x65, 0x45, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2c, 0x0a, 0x05, 0x73, 0x75, 0x69, 0x74, 0x65, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x0e, 0x32, 0x1d, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64,
    0x66, 0x73, 0x2e, 0x43, 0x69, 0x70, 0x68, 0x65, 0x72, 0x53, 0x75, 0x69, 0x74, 0x65, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x46, 0x0a, 0x15, 0x63, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x02,
    0x28, 0x0e, 0x32, 0x27, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73,
    0x2e, 0x43, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x56,
    0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0f, 0x0a, 0x07, 0x6b,
    0x65, 0x79, 0x4e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x02, 0x28, 0x09, 0x22, 0x7d, 0x0a, 0x11,
    0x43, 0x69, 0x70, 0x68, 0x65, 0x72, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x2c, 0x0a, 0x05, 0x73, 0x75, 0x69, 0x74, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e,
    0x32, 0x1d, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x43,
    0x69, 0x70, 0x68, 0x65, 0x72, 0x53, 0x75, 0x69, 0x74, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x0d, 0x0a, 0x05, 0x69, 0x6e, 0x4b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0c,
    0x0a, 0x04, 0x69, 0x6e, 0x49, 0x76, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0e, 0x0a, 0x06,
    0x6f, 0x75, 0x74, 0x4b, 0x65, 0x79, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0d, 0x0a, 0x05,
    0x6f, 0x75, 0x74, 0x49, 0x76, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x85, 0x02, 0x0a, 0x12,
    0x4c, 0x6f, 0x63, 0x61, 0x74, 0x65, 0x64, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x73, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x12, 0x0a, 0x0a, 0x66, 0x69, 0x6c, 0x65, 0x4c, 0x65, 0x6e, 0x67, 0x74, 0x68,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x12, 0x2e, 0x0a, 0x06, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x73,
    0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e,
    0x68, 0x64, 0x66, 0x73, 0x2e, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x65, 0x64, 0x42, 0x6c, 0x6f, 0x63,
    0x6b, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x19, 0x0a, 0x11, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x43,
    0x6f, 0x6e, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x02, 0x28,
    0x08, 0x12, 0x31, 0x0a, 0x09, 0x6c, 0x61, 0x73, 0x74, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64,
    0x66, 0x73, 0x2e, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x65, 0x64, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1b, 0x0a, 0x13, 0x69, 0x73, 0x4c, 0x61, 0x73, 0x74, 0x42, 0x6c,
    0x6f, 0x63, 0x6b, 0x43, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x65, 0x18, 0x05, 0x20, 0x02, 0x28,
    0x08, 0x12, 0x40, 0x0a, 0x12, 0x66, 0x69, 0x6c, 0x65, 0x45, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74,
    0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x24, 0x2e,
    0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x46, 0x69, 0x6c, 0x65,
    0x45, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x22, 0xa8, 0x04, 0x0a, 0x13, 0x48, 0x64, 0x66, 0x73, 0x46, 0x69, 0x6c, 0x65,
    0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x3b, 0x0a, 0x08, 0x66,
    0x69, 0x6c, 0x65, 0x54, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x29, 0x2e,
    0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x48, 0x64, 0x66, 0x73,
    0x46, 0x69, 0x6c, 0x65, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x2e,
    0x46, 0x69, 0x6c, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0c, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68,
    0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x0e, 0x0a, 0x06, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68,
    0x18, 0x03, 0x20, 0x02, 0x28, 0x04, 0x12, 0x32, 0x0a, 0x0a, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x73,
    0x73, 0x69, 0x6f, 0x6e, 0x18, 0x04, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x68, 0x61, 0x64,
    0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x46, 0x73, 0x50, 0x65, 0x72, 0x6d, 0x69,
    0x73, 0x73, 0x69, 0x6f, 0x6e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0d, 0x0a, 0x05, 0x6f, 0x77,
    0x6e, 0x65, 0x72, 0x18, 0x05, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0d, 0x0a, 0x05, 0x67, 0x72, 0x6f,
    0x75, 0x70, 0x18, 0x06, 0x20, 0x02, 0x28, 0x09, 0x12, 0x19, 0x0a, 0x11, 0x6d, 0x6f, 0x64, 0x69,
    0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x07, 0x20,
    0x02, 0x28, 0x04, 0x12, 0x13, 0x0a, 0x0b, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x5f, 0x74, 0x69,
    0x6d, 0x65, 0x18, 0x08, 0x20, 0x02, 0x28, 0x04, 0x12, 0x0f, 0x0a, 0x07, 0x73, 0x79, 0x6d, 0x6c,
    0x69, 0x6e, 0x6b, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x1c, 0x0a, 0x11, 0x62, 0x6c, 0x6f,
    0x63, 0x6b, 0x5f, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x0a,
    0x20, 0x01, 0x28, 0x0d, 0x3a, 0x01, 0x30, 0x12, 0x14, 0x0a, 0x09, 0x62, 0x6c, 0x6f, 0x63, 0x6b,
    0x73, 0x69, 0x7a, 0x65, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x04, 0x3a, 0x01, 0x30, 0x12, 0x32, 0x0a,
    0x09, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x1f, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x4c,
    0x6f, 0x63, 0x61, 0x74, 0x65, 0x64, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x73, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x11, 0x0a, 0x06, 0x66, 0x69, 0x6c, 0x65, 0x49, 0x64, 0x18, 0x0d, 0x20, 0x01, 0x28,
    0x04, 0x3a, 0x01, 0x30, 0x12, 0x17, 0x0a, 0x0b, 0x63, 0x68, 0x69, 0x6c, 0x64, 0x72, 0x65, 0x6e,
    0x4e, 0x75, 0x6d, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x05, 0x3a, 0x02, 0x2d, 0x31, 0x12, 0x40, 0x0a,
    0x12, 0x66, 0x69, 0x6c, 0x65, 0x45, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x49,
    0x6e, 0x66, 0x6f, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x24, 0x2e, 0x68, 0x61, 0x64, 0x6f,
    0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x46, 0x69, 0x6c, 0x65, 0x45, 0x6e, 0x63, 0x72,
    0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x6e, 0x66, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x18, 0x0a, 0x0d, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x50, 0x6f, 0x6c, 0x69, 0x63, 0x79,
    0x18, 0x10, 0x20, 0x01, 0x28, 0x0d, 0x3a, 0x01, 0x30, 0x22, 0x33, 0x0a, 0x08, 0x46, 0x69, 0x6c,
    0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0a, 0x0a, 0x06, 0x49, 0x53, 0x5f, 0x44, 0x49, 0x52, 0x10,
    0x01, 0x12, 0x0b, 0x0a, 0x07, 0x49, 0x53, 0x5f, 0x46, 0x49, 0x4c, 0x45, 0x10, 0x02, 0x12, 0x0e,
    0x0a, 0x0a, 0x49, 0x53, 0x5f, 0x53, 0x59, 0x4d, 0x4c, 0x49, 0x4e, 0x4b, 0x10, 0x03, 0x22, 0x8e,
    0x02, 0x0a, 0x15, 0x46, 0x73, 0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x44, 0x65, 0x66, 0x61, 0x75,
    0x6c, 0x74, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x11, 0x0a, 0x09, 0x62, 0x6c, 0x6f, 0x63,
    0x6b, 0x53, 0x69, 0x7a, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x12, 0x18, 0x0a, 0x10, 0x62,
    0x79, 0x74, 0x65, 0x73, 0x50, 0x65, 0x72, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x17, 0x0a, 0x0f, 0x77, 0x72, 0x69, 0x74, 0x65, 0x50, 0x61,
    0x63, 0x6b, 0x65, 0x74, 0x53, 0x69, 0x7a, 0x65, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x13,
    0x0a, 0x0b, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x04, 0x20,
    0x02, 0x28, 0x0d, 0x12, 0x16, 0x0a, 0x0e, 0x66, 0x69, 0x6c, 0x65, 0x42, 0x75, 0x66, 0x66, 0x65,
    0x72, 0x53, 0x69, 0x7a, 0x65, 0x18, 0x05, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x22, 0x0a, 0x13, 0x65,
    0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x44, 0x61, 0x74, 0x61, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66,
    0x65, 0x72, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x05, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x12,
    0x18, 0x0a, 0x0d, 0x74, 0x72, 0x61, 0x73, 0x68, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c,
    0x18, 0x07, 0x20, 0x01, 0x28, 0x04, 0x3a, 0x01, 0x30, 0x12, 0x44, 0x0a, 0x0c, 0x63, 0x68, 0x65,
    0x63, 0x6b, 0x73, 0x75, 0x6d, 0x54, 0x79, 0x70, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x1e, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x43, 0x68,
    0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x54, 0x79, 0x70, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x3a,
    0x0e, 0x43, 0x48, 0x45, 0x43, 0x4b, 0x53, 0x55, 0x4d, 0x5f, 0x43, 0x52, 0x43, 0x33, 0x32, 0x22,
    0x6b, 0x0a, 0x15, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x4c, 0x69, 0x73, 0x74,
    0x69, 0x6e, 0x67, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x38, 0x0a, 0x0e, 0x70, 0x61, 0x72, 0x74,
    0x69, 0x61, 0x6c, 0x4c, 0x69, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x20, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x48,
    0x64, 0x66, 0x73, 0x46, 0x69, 0x6c, 0x65, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x18, 0x0a, 0x10, 0x72, 0x65, 0x6d, 0x61, 0x69, 0x6e, 0x69, 0x6e, 0x67, 0x45,
    0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0d, 0x22, 0xa2, 0x01, 0x0a,
    0x21, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x44, 0x69,
    0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x33, 0x0a, 0x09, 0x64, 0x69, 0x72, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68,
    0x64, 0x66, 0x73, 0x2e, 0x48, 0x64, 0x66, 0x73, 0x46, 0x69, 0x6c, 0x65, 0x53, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x16, 0x0a, 0x0e, 0x73, 0x6e, 0x61, 0x70, 0x73,
    0x68, 0x6f, 0x74, 0x5f, 0x71, 0x75, 0x6f, 0x74, 0x61, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0d, 0x12,
    0x17, 0x0a, 0x0f, 0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x5f, 0x6e, 0x75, 0x6d, 0x62,
    0x65, 0x72, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x17, 0x0a, 0x0f, 0x70, 0x61, 0x72, 0x65,
    0x6e, 0x74, 0x5f, 0x66, 0x75, 0x6c, 0x6c, 0x70, 0x61, 0x74, 0x68, 0x18, 0x04, 0x20, 0x02, 0x28,
    0x0c, 0x22, 0x75, 0x0a, 0x22, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x74, 0x61, 0x62,
    0x6c, 0x65, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x4c, 0x69, 0x73, 0x74, 0x69,
    0x6e, 0x67, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x4f, 0x0a, 0x17, 0x73, 0x6e, 0x61, 0x70, 0x73,
    0x68, 0x6f, 0x74, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x44, 0x69, 0x72, 0x4c, 0x69, 0x73, 0x74, 0x69,
    0x6e, 0x67, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x2e, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f,
    0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x74,
    0x61, 0x62, 0x6c, 0x65, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x53, 0x74, 0x61,
    0x74, 0x75, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x5f, 0x0a, 0x1c, 0x53, 0x6e, 0x61, 0x70,
    0x73, 0x68, 0x6f, 0x74, 0x44, 0x69, 0x66, 0x66, 0x52, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x45, 0x6e,
    0x74, 0x72, 0x79, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x10, 0x0a, 0x08, 0x66, 0x75, 0x6c, 0x6c,
    0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x19, 0x0a, 0x11, 0x6d, 0x6f,
    0x64, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4c, 0x61, 0x62, 0x65, 0x6c, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x09, 0x12, 0x12, 0x0a, 0x0a, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x50,
    0x61, 0x74, 0x68, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x9f, 0x01, 0x0a, 0x17, 0x53, 0x6e,
    0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x44, 0x69, 0x66, 0x66, 0x52, 0x65, 0x70, 0x6f, 0x72, 0x74,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x14, 0x0a, 0x0c, 0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f,
    0x74, 0x52, 0x6f, 0x6f, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x14, 0x0a, 0x0c, 0x66,
    0x72, 0x6f, 0x6d, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x18, 0x02, 0x20, 0x02, 0x28,
    0x09, 0x12, 0x12, 0x0a, 0x0a, 0x74, 0x6f, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x18,
    0x03, 0x20, 0x02, 0x28, 0x09, 0x12, 0x44, 0x0a, 0x11, 0x64, 0x69, 0x66, 0x66, 0x52, 0x65, 0x70,
    0x6f, 0x72, 0x74, 0x45, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x29, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x53,
    0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x44, 0x69, 0x66, 0x66, 0x52, 0x65, 0x70, 0x6f, 0x72,
    0x74, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x5f, 0x0a, 0x10, 0x53,
    0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x15, 0x0a, 0x0d, 0x6c, 0x61, 0x79, 0x6f, 0x75, 0x74, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x12, 0x0a, 0x0a, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x70,
    0x63, 0x65, 0x49, 0x44, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x11, 0x0a, 0x09, 0x63, 0x6c,
    0x75, 0x73, 0x74, 0x65, 0x72, 0x49, 0x44, 0x18, 0x03, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0d, 0x0a,
    0x05, 0x63, 0x54, 0x69, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x02, 0x28, 0x04, 0x22, 0x89, 0x02, 0x0a,
    0x19, 0x4e, 0x61, 0x6d, 0x65, 0x6e, 0x6f, 0x64, 0x65, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x12, 0x0a, 0x0a, 0x72, 0x70,
    0x63, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x13,
    0x0a, 0x0b, 0x68, 0x74, 0x74, 0x70, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x02, 0x20,
    0x02, 0x28, 0x09, 0x12, 0x32, 0x0a, 0x0b, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x49, 0x6e,
    0x66, 0x6f, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f,
    0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x49, 0x6e,
    0x66, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x50, 0x0a, 0x04, 0x72, 0x6f, 0x6c, 0x65, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x38, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68,
    0x64, 0x66, 0x73, 0x2e, 0x4e, 0x61, 0x6d, 0x65, 0x6e, 0x6f, 0x64, 0x65, 0x52, 0x65, 0x67, 0x69,
    0x73, 0x74, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x4e, 0x61,
    0x6d, 0x65, 0x6e, 0x6f, 0x64, 0x65, 0x52, 0x6f, 0x6c, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x3a,
    0x08, 0x4e, 0x41, 0x4d, 0x45, 0x4e, 0x4f, 0x44, 0x45, 0x22, 0x3d, 0x0a, 0x11, 0x4e, 0x61, 0x6d,
    0x65, 0x6e, 0x6f, 0x64, 0x65, 0x52, 0x6f, 0x6c, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0c,
    0x0a, 0x08, 0x4e, 0x41, 0x4d, 0x45, 0x4e, 0x4f, 0x44, 0x45, 0x10, 0x01, 0x12, 0x0a, 0x0a, 0x06,
    0x42, 0x41, 0x43, 0x4b, 0x55, 0x50, 0x10, 0x02, 0x12, 0x0e, 0x0a, 0x0a, 0x43, 0x48, 0x45, 0x43,
    0x4b, 0x50, 0x4f, 0x49, 0x4e, 0x54, 0x10, 0x03, 0x22, 0x9d, 0x01, 0x0a, 0x18, 0x43, 0x68, 0x65,
    0x63, 0x6b, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x13, 0x0a, 0x0b, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x50, 0x6f,
    0x6f, 0x6c, 0x49, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x20, 0x0a, 0x18, 0x6d, 0x6f,
    0x73, 0x74, 0x52, 0x65, 0x63, 0x65, 0x6e, 0x74, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x70, 0x6f, 0x69,
    0x6e, 0x74, 0x54, 0x78, 0x49, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x12, 0x16, 0x0a, 0x0e,
    0x63, 0x75, 0x72, 0x53, 0x65, 0x67, 0x6d, 0x65, 0x6e, 0x74, 0x54, 0x78, 0x49, 0x64, 0x18, 0x03,
    0x20, 0x02, 0x28, 0x04, 0x12, 0x32, 0x0a, 0x0b, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x49,
    0x6e, 0x66, 0x6f, 0x18, 0x04, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x68, 0x61, 0x64, 0x6f,
    0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x49,
    0x6e, 0x66, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xcc, 0x01, 0x0a, 0x14, 0x4e, 0x61, 0x6d,
    0x65, 0x6e, 0x6f, 0x64, 0x65, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x0e, 0x0a, 0x06, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x0d, 0x12, 0x34, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0e, 0x32,
    0x26, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x4e, 0x61,
    0x6d, 0x65, 0x6e, 0x6f, 0x64, 0x65, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x12, 0x3a, 0x0a, 0x0d, 0x63, 0x68, 0x65, 0x63, 0x6b,
    0x70, 0x6f, 0x69, 0x6e, 0x74, 0x43, 0x6d, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x23,
    0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x43, 0x68, 0x65,
    0x63, 0x6b, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x22, 0x32, 0x0a, 0x04, 0x54, 0x79, 0x70, 0x65, 0x12, 0x13, 0x0a, 0x0f, 0x4e,
    0x61, 0x6d, 0x65, 0x6e, 0x6f, 0x64, 0x65, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x10, 0x00,
    0x12, 0x15, 0x0a, 0x11, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x50, 0x6f, 0x69, 0x6e, 0x74, 0x43, 0x6f,
    0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x10, 0x01, 0x22, 0x6d, 0x0a, 0x16, 0x43, 0x68, 0x65, 0x63, 0x6b,
    0x70, 0x6f, 0x69, 0x6e, 0x74, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x38, 0x0a, 0x09, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64,
    0x66, 0x73, 0x2e, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x53, 0x69, 0x67,
    0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x19, 0x0a, 0x11, 0x6e,
    0x65, 0x65, 0x64, 0x54, 0x6f, 0x52, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x49, 0x6d, 0x61, 0x67, 0x65,
    0x18, 0x02, 0x20, 0x02, 0x28, 0x08, 0x22, 0x44, 0x0a, 0x0a, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0f, 0x0a, 0x07, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x49, 0x64, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x04, 0x12, 0x10, 0x0a, 0x08, 0x67, 0x65, 0x6e, 0x53, 0x74, 0x61, 0x6d,
    0x70, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x12, 0x13, 0x0a, 0x08, 0x6e, 0x75, 0x6d, 0x42, 0x79,
    0x74, 0x65, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x3a, 0x01, 0x30, 0x22, 0xa3, 0x01, 0x0a,
    0x17, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x57, 0x69, 0x74, 0x68, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x26, 0x0a, 0x05, 0x62, 0x6c, 0x6f, 0x63,
    0x6b, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70,
    0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x15, 0x0a, 0x0d, 0x64, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x55, 0x75, 0x69, 0x64,
    0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x09, 0x12, 0x14, 0x0a, 0x0c, 0x73, 0x74, 0x6f, 0x72, 0x61,
    0x67, 0x65, 0x55, 0x75, 0x69, 0x64, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x09, 0x12, 0x33, 0x0a,
    0x0c, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x73, 0x18, 0x04, 0x20,
    0x03, 0x28, 0x0e, 0x32, 0x1d, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66,
    0x73, 0x2e, 0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x22, 0x50, 0x0a, 0x18, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x73, 0x57, 0x69, 0x74, 0x68,
    0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x34,
    0x0a, 0x06, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x24,
    0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x42, 0x6c, 0x6f,
    0x63, 0x6b, 0x57, 0x69, 0x74, 0x68, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x22, 0x55, 0x0a, 0x12, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x45, 0x64,
    0x69, 0x74, 0x4c, 0x6f, 0x67, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x11, 0x0a, 0x09, 0x73, 0x74,
    0x61, 0x72, 0x74, 0x54, 0x78, 0x49, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x12, 0x0f, 0x0a,
    0x07, 0x65, 0x6e, 0x64, 0x54, 0x78, 0x49, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x12, 0x1b,
    0x0a, 0x0c, 0x69, 0x73, 0x49, 0x6e, 0x50, 0x72, 0x6f, 0x67, 0x72, 0x65, 0x73, 0x73, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x08, 0x3a, 0x05, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x22, 0x4b, 0x0a, 0x1a, 0x52,
    0x65, 0x6d, 0x6f, 0x74, 0x65, 0x45, 0x64, 0x69, 0x74, 0x4c, 0x6f, 0x67, 0x4d, 0x61, 0x6e, 0x69,
    0x66, 0x65, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2d, 0x0a, 0x04, 0x6c, 0x6f, 0x67,
    0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70,
    0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x45, 0x64, 0x69, 0x74,
    0x4c, 0x6f, 0x67, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xb5, 0x01, 0x0a, 0x12, 0x4e, 0x61, 0x6d,
    0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x14, 0x0a, 0x0c, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0e, 0x0a, 0x06, 0x75, 0x6e, 0x75, 0x73, 0x65, 0x64, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x13, 0x0a, 0x0b, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x50, 0x6f,
    0x6f, 0x6c, 0x49, 0x44, 0x18, 0x03, 0x20, 0x02, 0x28, 0x09, 0x12, 0x32, 0x0a, 0x0b, 0x73, 0x74,
    0x6f, 0x72, 0x61, 0x67, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x18, 0x04, 0x20, 0x02, 0x28, 0x0b, 0x32,
    0x1d, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x53, 0x74,
    0x6f, 0x72, 0x61, 0x67, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x17,
    0x0a, 0x0f, 0x73, 0x6f, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f,
    0x6e, 0x18, 0x05, 0x20, 0x02, 0x28, 0x09, 0x12, 0x17, 0x0a, 0x0c, 0x63, 0x61, 0x70, 0x61, 0x62,
    0x69, 0x6c, 0x69, 0x74, 0x69, 0x65, 0x73, 0x18, 0x06, 0x20, 0x01, 0x28, 0x04, 0x3a, 0x01, 0x30,
    0x22, 0x44, 0x0a, 0x0d, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x4b, 0x65, 0x79, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x0d, 0x0a, 0x05, 0x6b, 0x65, 0x79, 0x49, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0d,
    0x12, 0x12, 0x0a, 0x0a, 0x65, 0x78, 0x70, 0x69, 0x72, 0x79, 0x44, 0x61, 0x74, 0x65, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x04, 0x12, 0x10, 0x0a, 0x08, 0x6b, 0x65, 0x79, 0x42, 0x79, 0x74, 0x65, 0x73,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x22, 0xc4, 0x01, 0x0a, 0x16, 0x45, 0x78, 0x70, 0x6f, 0x72,
    0x74, 0x65, 0x64, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x4b, 0x65, 0x79, 0x73, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x1b, 0x0a, 0x13, 0x69, 0x73, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x54, 0x6f, 0x6b, 0x65,
    0x6e, 0x45, 0x6e, 0x61, 0x62, 0x6c, 0x65, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x08, 0x12, 0x19,
    0x0a, 0x11, 0x6b, 0x65, 0x79, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x49, 0x6e, 0x74, 0x65, 0x72,
    0x76, 0x61, 0x6c, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x12, 0x15, 0x0a, 0x0d, 0x74, 0x6f, 0x6b,
    0x65, 0x6e, 0x4c, 0x69, 0x66, 0x65, 0x54, 0x69, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x02, 0x28, 0x04,
    0x12, 0x2e, 0x0a, 0x0a, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x4b, 0x65, 0x79, 0x18, 0x04,
    0x20, 0x02, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64,
    0x66, 0x73, 0x2e, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x4b, 0x65, 0x79, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x2b, 0x0a, 0x07, 0x61, 0x6c, 0x6c, 0x4b, 0x65, 0x79, 0x73, 0x18, 0x05, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x1a, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e,
    0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x4b, 0x65, 0x79, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x8a, 0x01,
    0x0a, 0x14, 0x52, 0x65, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x69, 0x6e, 0x67, 0x42, 0x6c, 0x6f, 0x63,
    0x6b, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x13, 0x0a, 0x0b, 0x6e, 0x65, 0x77, 0x47, 0x65, 0x6e,
    0x53, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x12, 0x2d, 0x0a, 0x05, 0x62,
    0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x68, 0x61, 0x64,
    0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x65, 0x64,
    0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2e, 0x0a, 0x0d, 0x74, 0x72,
    0x75, 0x6e, 0x63, 0x61, 0x74, 0x65, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x17, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e,
    0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x15, 0x0a, 0x13, 0x56, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x22, 0x45, 0x0a, 0x14, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2d, 0x0a, 0x04, 0x69, 0x6e, 0x66,
    0x6f, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70,
    0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x4e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x49,
    0x6e, 0x66, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xa5, 0x01, 0x0a, 0x11, 0x53, 0x6e, 0x61,
    0x70, 0x73, 0x68, 0x6f, 0x74, 0x49, 0x6e, 0x66, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x14,
    0x0a, 0x0c, 0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x4e, 0x61, 0x6d, 0x65, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x09, 0x12, 0x14, 0x0a, 0x0c, 0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74,
    0x52, 0x6f, 0x6f, 0x74, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x12, 0x32, 0x0a, 0x0a, 0x70, 0x65,
    0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x1e,
    0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x46, 0x73, 0x50,
    0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0d,
    0x0a, 0x05, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x18, 0x04, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0d, 0x0a,
    0x05, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x18, 0x05, 0x20, 0x02, 0x28, 0x09, 0x12, 0x12, 0x0a, 0x0a,
    0x63, 0x72, 0x65, 0x61, 0x74, 0x65, 0x54, 0x69, 0x6d, 0x65, 0x18, 0x06, 0x20, 0x02, 0x28, 0x09,
    0x22, 0x30, 0x0a, 0x19, 0x52, 0x6f, 0x6c, 0x6c, 0x69, 0x6e, 0x67, 0x55, 0x70, 0x67, 0x72, 0x61,
    0x64, 0x65, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x13, 0x0a,
    0x0b, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x50, 0x6f, 0x6f, 0x6c, 0x49, 0x64, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x09, 0x2a, 0x40, 0x0a, 0x10, 0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70,
    0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x08, 0x0a, 0x04, 0x44, 0x49, 0x53, 0x4b, 0x10, 0x01,
    0x12, 0x07, 0x0a, 0x03, 0x53, 0x53, 0x44, 0x10, 0x02, 0x12, 0x0b, 0x0a, 0x07, 0x41, 0x52, 0x43,
    0x48, 0x49, 0x56, 0x45, 0x10, 0x03, 0x12, 0x0c, 0x0a, 0x08, 0x52, 0x41, 0x4d, 0x5f, 0x44, 0x49,
    0x53, 0x4b, 0x10, 0x04, 0x2a, 0x36, 0x0a, 0x10, 0x43, 0x69, 0x70, 0x68, 0x65, 0x72, 0x53, 0x75,
    0x69, 0x74, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x0a, 0x07, 0x55, 0x4e, 0x4b, 0x4e,
    0x4f, 0x57, 0x4e, 0x10, 0x01, 0x12, 0x15, 0x0a, 0x11, 0x41, 0x45, 0x53, 0x5f, 0x43, 0x54, 0x52,
    0x5f, 0x4e, 0x4f, 0x50, 0x41, 0x44, 0x44, 0x49, 0x4e, 0x47, 0x10, 0x02, 0x2a, 0x50, 0x0a, 0x1a,
    0x43, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x56, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1c, 0x0a, 0x18, 0x55, 0x4e,
    0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x5f, 0x50, 0x52, 0x4f, 0x54, 0x4f, 0x43, 0x4f, 0x4c, 0x5f, 0x56,
    0x45, 0x52, 0x53, 0x49, 0x4f, 0x4e, 0x10, 0x01, 0x12, 0x14, 0x0a, 0x10, 0x45, 0x4e, 0x43, 0x52,
    0x59, 0x50, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x5a, 0x4f, 0x4e, 0x45, 0x53, 0x10, 0x02, 0x2a, 0x4f,
    0x0a, 0x11, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x54, 0x79, 0x70, 0x65, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x11, 0x0a, 0x0d, 0x43, 0x48, 0x45, 0x43, 0x4b, 0x53, 0x55, 0x4d, 0x5f,
    0x4e, 0x55, 0x4c, 0x4c, 0x10, 0x00, 0x12, 0x12, 0x0a, 0x0e, 0x43, 0x48, 0x45, 0x43, 0x4b, 0x53,
    0x55, 0x4d, 0x5f, 0x43, 0x52, 0x43, 0x33, 0x32, 0x10, 0x01, 0x12, 0x13, 0x0a, 0x0f, 0x43, 0x48,
    0x45, 0x43, 0x4b, 0x53, 0x55, 0x4d, 0x5f, 0x43, 0x52, 0x43, 0x33, 0x32, 0x43, 0x10, 0x02, 0x2a,
    0x4c, 0x0a, 0x11, 0x52, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x53, 0x74, 0x61, 0x74, 0x65, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0d, 0x0a, 0x09, 0x46, 0x49, 0x4e, 0x41, 0x4c, 0x49, 0x5a, 0x45,
    0x44, 0x10, 0x00, 0x12, 0x07, 0x0a, 0x03, 0x52, 0x42, 0x57, 0x10, 0x01, 0x12, 0x07, 0x0a, 0x03,
    0x52, 0x57, 0x52, 0x10, 0x02, 0x12, 0x07, 0x0a, 0x03, 0x52, 0x55, 0x52, 0x10, 0x03, 0x12, 0x0d,
    0x0a, 0x09, 0x54, 0x45, 0x4d, 0x50, 0x4f, 0x52, 0x41, 0x52, 0x59, 0x10, 0x04, 0x42, 0x36, 0x0a,
    0x25, 0x6f, 0x72, 0x67, 0x2e, 0x61, 0x70, 0x61, 0x63, 0x68, 0x65, 0x2e, 0x68, 0x61, 0x64, 0x6f,
    0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x42, 0x0a, 0x48, 0x64, 0x66, 0x73, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x73, 0xa0, 0x01, 0x01, 0x4a, 0xb2, 0xb6, 0x01, 0x0a, 0x07, 0x12, 0x05, 0x1c, 0x00, 0xe1,
    0x04, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x1c, 0x00, 0x3e, 0x0a, 0x0b, 0x0a, 0x04,
    0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x1c, 0x00, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x12, 0x03, 0x1c, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x1c, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x1c, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x07, 0x12,
    0x03, 0x1c, 0x16, 0x3d, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x1d, 0x00, 0x2b, 0x0a, 0x0b,
    0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01, 0x12, 0x03, 0x1d, 0x00, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x08,
    0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x1d, 0x07, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x07, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x07, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01,
    0x07, 0x12, 0x03, 0x1d, 0x1e, 0x2a, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x1e, 0x00, 0x2c,
    0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x02, 0x12, 0x03, 0x1e, 0x00, 0x2c, 0x0a, 0x0c, 0x0a,
    0x05, 0x08, 0xe7, 0x07, 0x02, 0x02, 0x12, 0x03, 0x1e, 0x07, 0x24, 0x0a, 0x0d, 0x0a, 0x06, 0x08,
    0xe7, 0x07, 0x02, 0x02, 0x00, 0x12, 0x03, 0x1e, 0x07, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7,
    0x07, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x07, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7,
    0x07, 0x02, 0x03, 0x12, 0x03, 0x1e, 0x27, 0x2b, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x1f,
    0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x21, 0x07, 0x17, 0x0a, 0x2f, 0x0a,
    0x02, 0x04, 0x00, 0x12, 0x04, 0x26, 0x00, 0x2c, 0x01, 0x1a, 0x23, 0x2a, 0x0a, 0x20, 0x45, 0x78,
    0x74, 0x65, 0x6e, 0x64, 0x65, 0x64, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x69, 0x64, 0x65,
    0x6e, 0x66, 0x69, 0x65, 0x73, 0x20, 0x61, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x26, 0x08, 0x1a, 0x0a, 0x3e, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x27, 0x02, 0x1d, 0x22, 0x31, 0x20, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x20,
    0x70, 0x6f, 0x6f, 0x6c, 0x20, 0x69, 0x64, 0x20, 0x2d, 0x20, 0x67, 0x6c, 0x6f, 0x61, 0x62, 0x6c,
    0x6c, 0x79, 0x20, 0x75, 0x6e, 0x69, 0x71, 0x75, 0x65, 0x20, 0x61, 0x63, 0x72, 0x6f, 0x73, 0x73,
    0x20, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x27, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x27, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x27, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x27,
    0x1b, 0x1c, 0x0a, 0x29, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x28, 0x02, 0x1e, 0x22,
    0x1c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x20, 0x69, 0x64, 0x20, 0x77,
    0x69, 0x74, 0x68, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x70, 0x6f, 0x6f, 0x6c, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x28, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x28, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x28, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x28, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x29,
    0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x29, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x29, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x29, 0x12, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x29, 0x24, 0x25, 0x0a, 0x2b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x03, 0x12, 0x03, 0x2a, 0x02, 0x2d, 0x22, 0x1e, 0x20, 0x6c, 0x65, 0x6e, 0x20, 0x64, 0x6f,
    0x65, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x62, 0x65, 0x6c, 0x6f, 0x6e, 0x67, 0x20, 0x69, 0x6e,
    0x20, 0x65, 0x62, 0x69, 0x64, 0x20, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x2a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x2a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2a, 0x12,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x2a, 0x1d, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x08, 0x12, 0x03, 0x2a, 0x1f, 0x2c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x07, 0x12, 0x03, 0x2a, 0x2a, 0x2b, 0x0a, 0x25, 0x0a, 0x02, 0x04,
    0x01, 0x12, 0x04, 0x31, 0x00, 0x3c, 0x01, 0x1a, 0x19, 0x2a, 0x0a, 0x20, 0x49, 0x64, 0x65, 0x6e,
    0x74, 0x69, 0x66, 0x69, 0x65, 0x73, 0x20, 0x61, 0x20, 0x44, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64,
    0x65, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x31, 0x08, 0x17, 0x0a, 0x19,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x32, 0x02, 0x1d, 0x22, 0x0c, 0x20, 0x49, 0x50,
    0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x32, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x32, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x32, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x32, 0x1b,
    0x1c, 0x0a, 0x17, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x33, 0x02, 0x1f, 0x22, 0x0a,
    0x20, 0x68, 0x6f, 0x73, 0x74, 0x6e, 0x61, 0x6d, 0x65, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x33, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x33, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x33, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x33,
    0x1d, 0x1e, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x34, 0x02, 0x23, 0x22,
    0x24, 0x20, 0x55, 0x55, 0x49, 0x44, 0x20, 0x61, 0x73, 0x73, 0x69, 0x67, 0x6e, 0x65, 0x64, 0x20,
    0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x44, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x2e,
    0x20, 0x46, 0x6f, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x34, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x34, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x34, 0x12, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x34, 0x21, 0x22, 0x0a, 0x75, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x38, 0x02, 0x1f, 0x1a, 0x51, 0x20, 0x75, 0x70, 0x67,
    0x72, 0x61, 0x64, 0x65, 0x64, 0x20, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x73, 0x20, 0x74,
    0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x61, 0x6d, 0x65, 0x0a,
    0x20, 0x61, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c,
    0x20, 0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x49, 0x44, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68,
    0x65, 0x0a, 0x20, 0x44, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x2e, 0x0a, 0x22, 0x15, 0x20,
    0x64, 0x61, 0x74, 0x61, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x20, 0x70,
    0x6f, 0x72, 0x74, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x38,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x38, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x38, 0x12, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x38, 0x1d, 0x1e, 0x0a, 0x21, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x39, 0x02, 0x1f, 0x22, 0x14, 0x20, 0x64, 0x61, 0x74, 0x61,
    0x6e, 0x6f, 0x64, 0x65, 0x20, 0x68, 0x74, 0x74, 0x70, 0x20, 0x70, 0x6f, 0x72, 0x74, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x03, 0x39, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x39, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x39, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x04, 0x03, 0x12, 0x03, 0x39, 0x1d, 0x1e, 0x0a, 0x1e, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x05, 0x12,
    0x03, 0x3a, 0x02, 0x1e, 0x22, 0x11, 0x20, 0x69, 0x70, 0x63, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65,
    0x72, 0x20, 0x70, 0x6f, 0x72, 0x74, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x04,
    0x12, 0x03, 0x3a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x05, 0x12, 0x03,
    0x3a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x3a, 0x12,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x3a, 0x1c, 0x1d, 0x0a,
    0x22, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x06, 0x12, 0x03, 0x3b, 0x02, 0x33, 0x22, 0x15, 0x20, 0x64,
    0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x20, 0x68, 0x74, 0x74, 0x70, 0x73, 0x20, 0x70, 0x6f,
    0x72, 0x74, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x04, 0x12, 0x03, 0x3b, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x05, 0x12, 0x03, 0x3b, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x06, 0x01, 0x12, 0x03, 0x3b, 0x12, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x06, 0x03, 0x12, 0x03, 0x3b, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x06, 0x08, 0x12, 0x03, 0x3b, 0x25, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x06, 0x07, 0x12, 0x03, 0x3b, 0x30, 0x31, 0x0a, 0x2a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x41,
    0x00, 0x45, 0x01, 0x1a, 0x1e, 0x2a, 0x0a, 0x20, 0x44, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65,
    0x20, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x41, 0x08, 0x1e, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x42, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x42, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x42, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x42, 0x12, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x42, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x43, 0x02,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x43, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x43, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x43, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x43, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x02, 0x12, 0x03, 0x44, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x44, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x44,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x44, 0x12, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x44, 0x1b, 0x1c, 0x0a, 0x22,
    0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x4a, 0x00, 0x4c, 0x01, 0x1a, 0x16, 0x2a, 0x0a, 0x20, 0x44,
    0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x20, 0x61, 0x72, 0x72, 0x61,
    0x79, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x4a, 0x08, 0x1a, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x4b, 0x02, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x4b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x4b, 0x0b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x4b, 0x1d, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x4b, 0x29, 0x2a, 0x0a, 0x28, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x51, 0x00, 0x64, 0x01, 0x1a,
    0x1c, 0x2a, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x20, 0x6f,
    0x66, 0x20, 0x61, 0x20, 0x44, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x51, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x00, 0x12, 0x03, 0x52, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x52, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x52,
    0x0b, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x52, 0x1b, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x52, 0x20, 0x21, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x53, 0x02, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x53, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x53, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x53, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x53, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x08, 0x12, 0x03, 0x53, 0x1f,
    0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x07, 0x12, 0x03, 0x53, 0x2a, 0x2b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x54, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x54, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x54, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x54, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x54, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x08, 0x12, 0x03, 0x54,
    0x1e, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x07, 0x12, 0x03, 0x54, 0x29, 0x2a,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12, 0x03, 0x55, 0x02, 0x2e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x03, 0x04, 0x12, 0x03, 0x55, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x03, 0x05, 0x12, 0x03, 0x55, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x55, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x55, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x08, 0x12, 0x03,
    0x55, 0x20, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x07, 0x12, 0x03, 0x55, 0x2b,
    0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x04, 0x12, 0x03, 0x56, 0x02, 0x32, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x04, 0x12, 0x03, 0x56, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x04, 0x05, 0x12, 0x03, 0x56, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x56, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x56, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x08, 0x12,
    0x03, 0x56, 0x24, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x04, 0x07, 0x12, 0x03, 0x56,
    0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x05, 0x12, 0x03, 0x57, 0x02, 0x2f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x04, 0x12, 0x03, 0x57, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x05, 0x05, 0x12, 0x03, 0x57, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x05, 0x01, 0x12, 0x03, 0x57, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x05, 0x03, 0x12, 0x03, 0x57, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x08,
    0x12, 0x03, 0x57, 0x21, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x05, 0x07, 0x12, 0x03,
    0x57, 0x2c, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x06, 0x12, 0x03, 0x58, 0x02, 0x31,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x04, 0x12, 0x03, 0x58, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x05, 0x12, 0x03, 0x58, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x06, 0x01, 0x12, 0x03, 0x58, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x06, 0x03, 0x12, 0x03, 0x58, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06,
    0x08, 0x12, 0x03, 0x58, 0x23, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x06, 0x07, 0x12,
    0x03, 0x58, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x07, 0x12, 0x03, 0x59, 0x02,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x04, 0x12, 0x03, 0x59, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x07, 0x05, 0x12, 0x03, 0x59, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x07, 0x01, 0x12, 0x03, 0x59, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x07, 0x03, 0x12, 0x03, 0x59, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x04, 0x04,
    0x00, 0x12, 0x04, 0x5a, 0x02, 0x5e, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x5a, 0x07, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x5b, 0x04, 0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x5b, 0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x5b, 0x0d, 0x0e, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x5c, 0x04, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x5c, 0x04, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03,
    0x5c, 0x1e, 0x1f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x5d,
    0x04, 0x17, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5d,
    0x04, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x04, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x5d,
    0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x08, 0x12, 0x03, 0x60, 0x02, 0x39, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x04, 0x12, 0x03, 0x60, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x08, 0x06, 0x12, 0x03, 0x60, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x08, 0x01, 0x12, 0x03, 0x60, 0x16, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x08, 0x03, 0x12, 0x03, 0x60, 0x23, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x08,
    0x12, 0x03, 0x60, 0x26, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x08, 0x07, 0x12, 0x03,
    0x60, 0x31, 0x37, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x09, 0x12, 0x03, 0x61, 0x02, 0x33,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x09, 0x04, 0x12, 0x03, 0x61, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x09, 0x05, 0x12, 0x03, 0x61, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x09, 0x01, 0x12, 0x03, 0x61, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x09, 0x03, 0x12, 0x03, 0x61, 0x22, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x09,
    0x08, 0x12, 0x03, 0x61, 0x25, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x09, 0x07, 0x12,
    0x03, 0x61, 0x30, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x0a, 0x12, 0x03, 0x62, 0x02,
    0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0a, 0x04, 0x12, 0x03, 0x62, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x62, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x62, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x62, 0x1e, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x0a, 0x08, 0x12, 0x03, 0x62, 0x21, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0a, 0x07,
    0x12, 0x03, 0x62, 0x2c, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x0b, 0x12, 0x03, 0x63,
    0x02, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0b, 0x04, 0x12, 0x03, 0x63, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0b, 0x05, 0x12, 0x03, 0x63, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x63, 0x12, 0x25, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x63, 0x28, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x0b, 0x08, 0x12, 0x03, 0x63, 0x2b, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x0b,
    0x07, 0x12, 0x03, 0x63, 0x36, 0x37, 0x0a, 0x3e, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x69, 0x00,
    0x72, 0x01, 0x1a, 0x32, 0x2a, 0x0a, 0x20, 0x52, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74,
    0x73, 0x20, 0x61, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x20, 0x61, 0x76, 0x61, 0x69,
    0x6c, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x61, 0x74,
    0x61, 0x6e, 0x6f, 0x64, 0x65, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x69,
    0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x05, 0x04, 0x00, 0x12, 0x04, 0x6a, 0x02, 0x6d, 0x03,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x04, 0x00, 0x01, 0x12, 0x03, 0x6a, 0x07, 0x13, 0x0a, 0x0d,
    0x0a, 0x06, 0x04, 0x05, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x6b, 0x04, 0x0f, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6b, 0x04, 0x0a, 0x0a, 0x0e, 0x0a,
    0x07, 0x04, 0x05, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x6b, 0x0d, 0x0e, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x05, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x6c, 0x04, 0x19, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6c, 0x04, 0x14, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x05, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x6c, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x6f, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x6f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x6f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x6f, 0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6f, 0x20,
    0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x70, 0x02, 0x35, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x70, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x01, 0x06, 0x12, 0x03, 0x70, 0x0b, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x70, 0x18, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x70, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x08, 0x12,
    0x03, 0x70, 0x22, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x07, 0x12, 0x03, 0x70,
    0x2d, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x71, 0x02, 0x3d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x71, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x02, 0x06, 0x12, 0x03, 0x71, 0x0b, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x71, 0x1c, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x71, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x08,
    0x12, 0x03, 0x71, 0x2c, 0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x07, 0x12, 0x03,
    0x71, 0x37, 0x3b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x74, 0x00, 0x7c, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x74, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x00, 0x12, 0x03, 0x75, 0x02, 0x38, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x75, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x75, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x75,
    0x12, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x75, 0x20, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x08, 0x12, 0x03, 0x75, 0x22, 0x37, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x06, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x75, 0x24, 0x35, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x06, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x75, 0x24,
    0x2e, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x06, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x75, 0x24, 0x2e, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x06, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x75, 0x24, 0x2e, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x06, 0x02, 0x00,
    0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x75, 0x31, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06,
    0x02, 0x01, 0x12, 0x03, 0x76, 0x02, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x76, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x76, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x76, 0x10,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x76, 0x19, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x08, 0x12, 0x03, 0x76, 0x1b, 0x2e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x01, 0x07, 0x12, 0x03, 0x76, 0x27, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x02, 0x12, 0x03, 0x77, 0x02, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x77, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x77, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x01, 0x12, 0x03, 0x77,
    0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12, 0x03, 0x77, 0x1d, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x08, 0x12, 0x03, 0x77, 0x1f, 0x2e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x07, 0x12, 0x03, 0x77, 0x2b, 0x2c, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x06, 0x02, 0x03, 0x12, 0x03, 0x78, 0x02, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x03, 0x04, 0x12, 0x03, 0x78, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x05,
    0x12, 0x03, 0x78, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x78, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x03, 0x12, 0x03, 0x78, 0x1c,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x08, 0x12, 0x03, 0x78, 0x1e, 0x2d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x07, 0x12, 0x03, 0x78, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x06, 0x02, 0x04, 0x12, 0x03, 0x79, 0x02, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x04, 0x04, 0x12, 0x03, 0x79, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04,
    0x05, 0x12, 0x03, 0x79, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x79, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x03, 0x12, 0x03, 0x79,
    0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x08, 0x12, 0x03, 0x79, 0x20, 0x2f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x07, 0x12, 0x03, 0x79, 0x2c, 0x2d, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x06, 0x02, 0x05, 0x12, 0x03, 0x7a, 0x02, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x05, 0x04, 0x12, 0x03, 0x7a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x05, 0x05, 0x12, 0x03, 0x7a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x01,
    0x12, 0x03, 0x7a, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x03, 0x12, 0x03,
    0x7a, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x08, 0x12, 0x03, 0x7a, 0x24,
    0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x07, 0x12, 0x03, 0x7a, 0x30, 0x31, 0x0a,
    0x25, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x06, 0x12, 0x03, 0x7b, 0x02, 0x2c, 0x22, 0x18, 0x20, 0x73,
    0x75, 0x70, 0x65, 0x72, 0x73, 0x65, 0x64, 0x65, 0x73, 0x20, 0x53, 0x74, 0x6f, 0x72, 0x61, 0x67,
    0x65, 0x55, 0x75, 0x69, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x04, 0x12,
    0x03, 0x7b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x06, 0x12, 0x03, 0x7b,
    0x0b, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x01, 0x12, 0x03, 0x7b, 0x20, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x03, 0x12, 0x03, 0x7b, 0x2a, 0x2b, 0x0a, 0x30,
    0x0a, 0x02, 0x04, 0x07, 0x12, 0x06, 0x81, 0x01, 0x00, 0x89, 0x01, 0x01, 0x1a, 0x22, 0x2a, 0x0a,
    0x20, 0x53, 0x75, 0x6d, 0x6d, 0x61, 0x72, 0x79, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x66, 0x69,
    0x6c, 0x65, 0x20, 0x6f, 0x72, 0x20, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x0a,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x04, 0x81, 0x01, 0x08, 0x1b, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x04, 0x82, 0x01, 0x02, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x00, 0x04, 0x12, 0x04, 0x82, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x00, 0x05, 0x12, 0x04, 0x82, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x00, 0x01, 0x12, 0x04, 0x82, 0x01, 0x12, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x03, 0x12, 0x04, 0x82, 0x01, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12,
    0x04, 0x83, 0x01, 0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x83, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x04, 0x83,
    0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x04, 0x83, 0x01,
    0x12, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x04, 0x83, 0x01, 0x1e,
    0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x02, 0x12, 0x04, 0x84, 0x01, 0x02, 0x25, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x04, 0x12, 0x04, 0x84, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x05, 0x12, 0x04, 0x84, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x04, 0x84, 0x01, 0x12, 0x20, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x04, 0x84, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x07, 0x02, 0x03, 0x12, 0x04, 0x85, 0x01, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x03, 0x04, 0x12, 0x04, 0x85, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03,
    0x05, 0x12, 0x04, 0x85, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x01,
    0x12, 0x04, 0x85, 0x01, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x03, 0x12,
    0x04, 0x85, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x04, 0x12, 0x04, 0x86,
    0x01, 0x02, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x04, 0x12, 0x04, 0x86, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x05, 0x12, 0x04, 0x86, 0x01, 0x0b,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x01, 0x12, 0x04, 0x86, 0x01, 0x12, 0x1f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x03, 0x12, 0x04, 0x86, 0x01, 0x22, 0x23, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x05, 0x12, 0x04, 0x87, 0x01, 0x02, 0x21, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x05, 0x04, 0x12, 0x04, 0x87, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x05, 0x05, 0x12, 0x04, 0x87, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x05, 0x01, 0x12, 0x04, 0x87, 0x01, 0x12, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x05, 0x03, 0x12, 0x04, 0x87, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07, 0x02,
    0x06, 0x12, 0x04, 0x88, 0x01, 0x02, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x06, 0x04,
    0x12, 0x04, 0x88, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x06, 0x06, 0x12,
    0x04, 0x88, 0x01, 0x0b, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x06, 0x01, 0x12, 0x04,
    0x88, 0x01, 0x26, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x06, 0x03, 0x12, 0x04, 0x88,
    0x01, 0x37, 0x38, 0x0a, 0x51, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x06, 0x8e, 0x01, 0x00, 0x90, 0x01,
    0x01, 0x1a, 0x43, 0x2a, 0x0a, 0x20, 0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x20, 0x74, 0x79,
    0x70, 0x65, 0x20, 0x71, 0x75, 0x6f, 0x74, 0x61, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x75, 0x73, 0x61,
    0x67, 0x65, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f,
    0x66, 0x20, 0x61, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x6f, 0x72, 0x20, 0x64, 0x69, 0x72, 0x65,
    0x63, 0x74, 0x6f, 0x72, 0x79, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x04, 0x8e,
    0x01, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x04, 0x8f, 0x01, 0x02,
    0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8f, 0x01, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12, 0x04, 0x8f, 0x01, 0x0b, 0x24, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8f, 0x01, 0x25, 0x32, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8f, 0x01, 0x35, 0x36, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x09, 0x12, 0x06, 0x92, 0x01, 0x00, 0x96, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x09, 0x01, 0x12, 0x04, 0x92, 0x01, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00,
    0x12, 0x04, 0x93, 0x01, 0x02, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x93, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x06, 0x12, 0x04,
    0x93, 0x01, 0x0b, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x04, 0x93,
    0x01, 0x1c, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x04, 0x93, 0x01,
    0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x04, 0x94, 0x01, 0x02, 0x1c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x04, 0x94, 0x01, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x05, 0x12, 0x04, 0x94, 0x01, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x04, 0x94, 0x01, 0x12, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x04, 0x94, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x09, 0x02, 0x02, 0x12, 0x04, 0x95, 0x01, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x02, 0x04, 0x12, 0x04, 0x95, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x02, 0x05, 0x12, 0x04, 0x95, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02,
    0x01, 0x12, 0x04, 0x95, 0x01, 0x12, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x03,
    0x12, 0x04, 0x95, 0x01, 0x1d, 0x1e, 0x0a, 0x95, 0x01, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x06, 0x9d,
    0x01, 0x00, 0xa0, 0x01, 0x01, 0x1a, 0x86, 0x01, 0x2a, 0x0a, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x61,
    0x69, 0x6e, 0x73, 0x20, 0x61, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x70, 0x61,
    0x74, 0x68, 0x73, 0x20, 0x63, 0x6f, 0x72, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x64, 0x69, 0x6e,
    0x67, 0x20, 0x74, 0x6f, 0x20, 0x63, 0x6f, 0x72, 0x72, 0x75, 0x70, 0x74, 0x20, 0x66, 0x69, 0x6c,
    0x65, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6f, 0x6b, 0x69, 0x65, 0x0a,
    0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x69, 0x74, 0x65, 0x72, 0x61, 0x74,
    0x69, 0x76, 0x65, 0x20, 0x63, 0x61, 0x6c, 0x6c, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x4e, 0x61, 0x6d,
    0x65, 0x4e, 0x6f, 0x64, 0x65, 0x2e, 0x6c, 0x69, 0x73, 0x74, 0x43, 0x6f, 0x72, 0x72, 0x75, 0x70,
    0x74, 0x46, 0x69, 0x6c, 0x65, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x04, 0x9d, 0x01, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x0a, 0x02, 0x00, 0x12, 0x04, 0x9e, 0x01, 0x01, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x9e, 0x01, 0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00,
    0x05, 0x12, 0x04, 0x9e, 0x01, 0x0a, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x9e, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x9e, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x01, 0x12, 0x04, 0x9f,
    0x01, 0x01, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x04, 0x12, 0x04, 0x9f, 0x01,
    0x01, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x05, 0x12, 0x04, 0x9f, 0x01, 0x0a,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9f, 0x01, 0x13, 0x19,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x04, 0x9f, 0x01, 0x1c, 0x1d, 0x0a,
    0x42, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x06, 0xa5, 0x01, 0x00, 0xa7, 0x01, 0x01, 0x1a, 0x34, 0x2a,
    0x0a, 0x20, 0x46, 0x69, 0x6c, 0x65, 0x20, 0x6f, 0x72, 0x20, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74,
    0x6f, 0x72, 0x79, 0x20, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x2d, 0x20,
    0x73, 0x61, 0x6d, 0x65, 0x20, 0x73, 0x70, 0x65, 0x63, 0x20, 0x61, 0x73, 0x20, 0x70, 0x6f, 0x73,
    0x69, 0x78, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x19,
    0x0a, 0x33, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x04, 0xa6, 0x01, 0x02, 0x1b, 0x22, 0x25,
    0x20, 0x41, 0x63, 0x74, 0x75, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x61, 0x20, 0x73, 0x68, 0x6f, 0x72,
    0x74, 0x20, 0x2d, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x31, 0x36, 0x62, 0x69, 0x74, 0x73, 0x20,
    0x75, 0x73, 0x65, 0x64, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xa6, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa6,
    0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa6, 0x01,
    0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa6, 0x01, 0x19,
    0x1a, 0x0a, 0x34, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x06, 0xac, 0x01, 0x00, 0xb1, 0x01, 0x01, 0x1a,
    0x26, 0x2a, 0x0a, 0x20, 0x54, 0x79, 0x70, 0x65, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x72, 0x65, 0x63,
    0x6f, 0x67, 0x6e, 0x69, 0x7a, 0x65, 0x64, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x20,
    0x6d, 0x65, 0x64, 0x69, 0x61, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x04,
    0xac, 0x01, 0x05, 0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x04, 0xad, 0x01,
    0x02, 0x0b, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xad, 0x01, 0x02,
    0x06, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xad, 0x01, 0x09, 0x0a,
    0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x04, 0xae, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xae, 0x01, 0x02, 0x05, 0x0a, 0x0d, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xae, 0x01, 0x08, 0x09, 0x0a, 0x0c, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x02, 0x12, 0x04, 0xaf, 0x01, 0x02, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x04, 0xaf, 0x01, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x02, 0x02, 0x12, 0x04, 0xaf, 0x01, 0x0c, 0x0d, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03,
    0x12, 0x04, 0xb0, 0x01, 0x02, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12,
    0x04, 0xb0, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04,
    0xb0, 0x01, 0x0d, 0x0e, 0x0a, 0x2b, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x06, 0xb6, 0x01, 0x00, 0xb8,
    0x01, 0x01, 0x1a, 0x1d, 0x2a, 0x0a, 0x20, 0x41, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66,
    0x20, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x20,
    0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x04, 0xb6, 0x01, 0x08, 0x19, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x04, 0xb7, 0x01, 0x02, 0x2d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb7, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x00, 0x06, 0x12, 0x04, 0xb7, 0x01, 0x0b, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xb7, 0x01, 0x1c, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xb7, 0x01, 0x2b, 0x2c, 0x0a, 0x2f, 0x0a, 0x02, 0x04, 0x0d, 0x12, 0x06,
    0xbd, 0x01, 0x00, 0xc6, 0x01, 0x01, 0x1a, 0x21, 0x2a, 0x0a, 0x20, 0x42, 0x6c, 0x6f, 0x63, 0x6b,
    0x20, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65,
    0x20, 0x70, 0x6f, 0x6c, 0x69, 0x63, 0x79, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0d, 0x01,
    0x12, 0x04, 0xbd, 0x01, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x04,
    0xbe, 0x01, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x04, 0xbe,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x05, 0x12, 0x04, 0xbe, 0x01,
    0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbe, 0x01, 0x12,
    0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbe, 0x01, 0x1d, 0x1e,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x01, 0x12, 0x04, 0xbf, 0x01, 0x02, 0x1b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x04, 0x12, 0x04, 0xbf, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x01, 0x05, 0x12, 0x04, 0xbf, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x01, 0x01, 0x12, 0x04, 0xbf, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x01, 0x03, 0x12, 0x04, 0xbf, 0x01, 0x19, 0x1a, 0x0a, 0x5e, 0x0a, 0x04, 0x04, 0x0d,
    0x02, 0x02, 0x12, 0x04, 0xc2, 0x01, 0x02, 0x30, 0x1a, 0x50, 0x20, 0x61, 0x20, 0x6c, 0x69, 0x73,
    0x74, 0x20, 0x6f, 0x66, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x20, 0x74, 0x79, 0x70,
    0x65, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61,
    0x73, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x69, 0x6e, 0x67, 0x20,
    0x61, 0x0a, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x02, 0x04, 0x12, 0x04, 0xc2, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x02, 0x06, 0x12, 0x04, 0xc2, 0x01, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02,
    0x01, 0x12, 0x04, 0xc2, 0x01, 0x1d, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x03,
    0x12, 0x04, 0xc2, 0x01, 0x2e, 0x2f, 0x0a, 0x46, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x03, 0x12, 0x04,
    0xc4, 0x01, 0x02, 0x38, 0x1a, 0x38, 0x20, 0x41, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66,
    0x20, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x73, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x61, 0x6c, 0x6c,
    0x62, 0x61, 0x63, 0x6b, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x2e, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x04, 0x12, 0x04, 0xc4, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x03, 0x06, 0x12, 0x04, 0xc4, 0x01, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x03, 0x01, 0x12, 0x04, 0xc4, 0x01, 0x1d, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x03, 0x03, 0x12, 0x04, 0xc4, 0x01, 0x36, 0x37, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d,
    0x02, 0x04, 0x12, 0x04, 0xc5, 0x01, 0x02, 0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04,
    0x04, 0x12, 0x04, 0xc5, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x06,
    0x12, 0x04, 0xc5, 0x01, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x01, 0x12,
    0x04, 0xc5, 0x01, 0x1d, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x03, 0x12, 0x04,
    0xc5, 0x01, 0x39, 0x3a, 0x0a, 0x29, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x06, 0xcb, 0x01, 0x00, 0xcd,
    0x01, 0x01, 0x1a, 0x1b, 0x2a, 0x0a, 0x20, 0x41, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66,
    0x20, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x20, 0x49, 0x44, 0x73, 0x2e, 0x20, 0x0a, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x04, 0xcb, 0x01, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x0e, 0x02, 0x00, 0x12, 0x04, 0xcc, 0x01, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xcc, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xcc, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xcc, 0x01, 0x12, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xcc, 0x01, 0x21, 0x22, 0x0a, 0x52, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x06, 0xd2, 0x01,
    0x00, 0xde, 0x01, 0x01, 0x1a, 0x44, 0x2a, 0x0a, 0x20, 0x41, 0x20, 0x4c, 0x6f, 0x63, 0x61, 0x74,
    0x65, 0x64, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x67, 0x69, 0x76, 0x65, 0x73, 0x20, 0x69, 0x6e,
    0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x61, 0x62, 0x6f, 0x75, 0x74, 0x20,
    0x61, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x69, 0x74, 0x73, 0x20,
    0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0f,
    0x01, 0x12, 0x04, 0xd2, 0x01, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00, 0x12,
    0x04, 0xd3, 0x01, 0x02, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xd3, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x06, 0x12, 0x04, 0xd3,
    0x01, 0x0b, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd3, 0x01,
    0x1e, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd3, 0x01, 0x23,
    0x24, 0x0a, 0x39, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x01, 0x12, 0x04, 0xd4, 0x01, 0x02, 0x1d, 0x22,
    0x2b, 0x20, 0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x66, 0x69, 0x72, 0x73,
    0x74, 0x20, 0x62, 0x79, 0x74, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20,
    0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd4, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x01, 0x05, 0x12, 0x04, 0xd4, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xd4, 0x01, 0x12, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xd4, 0x01, 0x1b, 0x1c, 0x0a, 0x3b, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x02,
    0x12, 0x04, 0xd5, 0x01, 0x02, 0x26, 0x22, 0x2d, 0x20, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x20, 0x6f, 0x72, 0x64, 0x65, 0x72, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x70, 0x72,
    0x6f, 0x78, 0x69, 0x6d, 0x69, 0x74, 0x79, 0x20, 0x74, 0x6f, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x20, 0x69, 0x70, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x04, 0x12, 0x04,
    0xd5, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x06, 0x12, 0x04, 0xd5,
    0x01, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x01, 0x12, 0x04, 0xd5, 0x01,
    0x1d, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x02, 0x03, 0x12, 0x04, 0xd5, 0x01, 0x24,
    0x25, 0x0a, 0x47, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x03, 0x12, 0x04, 0xd6, 0x01, 0x02, 0x1c, 0x22,
    0x39, 0x20, 0x74, 0x72, 0x75, 0x65, 0x20, 0x69, 0x66, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x72, 0x65,
    0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x62, 0x6c, 0x6f, 0x63,
    0x6b, 0x20, 0x61, 0x72, 0x65, 0x20, 0x63, 0x6f, 0x72, 0x72, 0x75, 0x70, 0x74, 0x2c, 0x20, 0x65,
    0x6c, 0x73, 0x65, 0x20, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x03, 0x04, 0x12, 0x04, 0xd6, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x03, 0x05, 0x12, 0x04, 0xd6, 0x01, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03,
    0x01, 0x12, 0x04, 0xd6, 0x01, 0x10, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x03, 0x03,
    0x12, 0x04, 0xd6, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x04, 0x12, 0x04,
    0xda, 0x01, 0x02, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x04, 0x12, 0x04, 0xda,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x06, 0x12, 0x04, 0xda, 0x01,
    0x0b, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x01, 0x12, 0x04, 0xda, 0x01, 0x24,
    0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x04, 0x03, 0x12, 0x04, 0xda, 0x01, 0x31, 0x32,
    0x0a, 0x2f, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x05, 0x12, 0x04, 0xdb, 0x01, 0x02, 0x2b, 0x22, 0x21,
    0x20, 0x69, 0x66, 0x20, 0x61, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69,
    0x6e, 0x20, 0x6c, 0x6f, 0x63, 0x73, 0x20, 0x69, 0x73, 0x20, 0x63, 0x61, 0x63, 0x68, 0x65, 0x64,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x05, 0x04, 0x12, 0x04, 0xdb, 0x01, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x05, 0x05, 0x12, 0x04, 0xdb, 0x01, 0x0b, 0x0f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x05, 0x01, 0x12, 0x04, 0xdb, 0x01, 0x10, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x05, 0x03, 0x12, 0x04, 0xdb, 0x01, 0x1b, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x05, 0x08, 0x12, 0x04, 0xdb, 0x01, 0x1d, 0x2a, 0x0a, 0x10, 0x0a, 0x08,
    0x04, 0x0f, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x04, 0xdb, 0x01, 0x1e, 0x29, 0x0a, 0x11,
    0x0a, 0x09, 0x04, 0x0f, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x04, 0xdb, 0x01, 0x1e,
    0x24, 0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x0f, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12,
    0x04, 0xdb, 0x01, 0x1e, 0x24, 0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x0f, 0x02, 0x05, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xdb, 0x01, 0x1e, 0x24, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x0f,
    0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x04, 0xdb, 0x01, 0x25, 0x29, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x0f, 0x02, 0x06, 0x12, 0x04, 0xdc, 0x01, 0x02, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x06, 0x04, 0x12, 0x04, 0xdc, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x06, 0x06, 0x12, 0x04, 0xdc, 0x01, 0x0b, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x06, 0x01, 0x12, 0x04, 0xdc, 0x01, 0x1c, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x06,
    0x03, 0x12, 0x04, 0xdc, 0x01, 0x2b, 0x2c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x07, 0x12,
    0x04, 0xdd, 0x01, 0x02, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x07, 0x04, 0x12, 0x04,
    0xdd, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x07, 0x05, 0x12, 0x04, 0xdd,
    0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x07, 0x01, 0x12, 0x04, 0xdd, 0x01,
    0x12, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x07, 0x03, 0x12, 0x04, 0xdd, 0x01, 0x1f,
    0x20, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06, 0xe0, 0x01, 0x00, 0xe7, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0xe0, 0x01, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x10, 0x02, 0x00, 0x12, 0x04, 0xe1, 0x01, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xe1, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xe1, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xe1, 0x01, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xe1, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x01, 0x12, 0x04,
    0xe2, 0x01, 0x02, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x04, 0x12, 0x04, 0xe2,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x05, 0x12, 0x04, 0xe2, 0x01,
    0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe2, 0x01, 0x12,
    0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe2, 0x01, 0x20, 0x21,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x02, 0x12, 0x04, 0xe3, 0x01, 0x02, 0x1b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x04, 0x12, 0x04, 0xe3, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x02, 0x05, 0x12, 0x04, 0xe3, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x02, 0x01, 0x12, 0x04, 0xe3, 0x01, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x02, 0x03, 0x12, 0x04, 0xe3, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10,
    0x02, 0x03, 0x12, 0x04, 0xe4, 0x01, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03,
    0x04, 0x12, 0x04, 0xe4, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x05,
    0x12, 0x04, 0xe4, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x01, 0x12,
    0x04, 0xe4, 0x01, 0x11, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x03, 0x12, 0x04,
    0xe4, 0x01, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x04, 0x12, 0x04, 0xe5, 0x01,
    0x02, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x04, 0x04, 0x12, 0x04, 0xe5, 0x01, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x04, 0x05, 0x12, 0x04, 0xe5, 0x01, 0x0b, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x04, 0x01, 0x12, 0x04, 0xe5, 0x01, 0x12, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x04, 0x03, 0x12, 0x04, 0xe5, 0x01, 0x1f, 0x20, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x10, 0x02, 0x05, 0x12, 0x04, 0xe6, 0x01, 0x02, 0x2a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x05, 0x04, 0x12, 0x04, 0xe6, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x05, 0x05, 0x12, 0x04, 0xe6, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x05, 0x01, 0x12, 0x04, 0xe6, 0x01, 0x12, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x05, 0x03, 0x12, 0x04, 0xe6, 0x01, 0x28, 0x29, 0x0a, 0x1f, 0x0a, 0x02, 0x05, 0x01, 0x12, 0x06,
    0xec, 0x01, 0x00, 0xef, 0x01, 0x01, 0x1a, 0x11, 0x2a, 0x0a, 0x20, 0x43, 0x69, 0x70, 0x68, 0x65,
    0x72, 0x20, 0x73, 0x75, 0x69, 0x74, 0x65, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x05, 0x01, 0x01,
    0x12, 0x04, 0xec, 0x01, 0x05, 0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00, 0x12, 0x04,
    0xed, 0x01, 0x04, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x04, 0xed,
    0x01, 0x04, 0x0b, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x04, 0xed, 0x01,
    0x0e, 0x0f, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x01, 0x12, 0x04, 0xee, 0x01, 0x04, 0x1a,
    0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01, 0x12, 0x04, 0xee, 0x01, 0x04, 0x15, 0x0a,
    0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x04, 0xee, 0x01, 0x18, 0x19, 0x0a, 0x49,
    0x0a, 0x02, 0x05, 0x02, 0x12, 0x06, 0xf4, 0x01, 0x00, 0xf7, 0x01, 0x01, 0x1a, 0x3b, 0x2a, 0x0a,
    0x20, 0x43, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c,
    0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f,
    0x20, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x20, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x65,
    0x64, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x05, 0x02, 0x01,
    0x12, 0x04, 0xf4, 0x01, 0x05, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x00, 0x12, 0x04,
    0xf5, 0x01, 0x04, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf5,
    0x01, 0x04, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x00, 0x02, 0x12, 0x04, 0xf5, 0x01,
    0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x04, 0xf6, 0x01, 0x04, 0x19,
    0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf6, 0x01, 0x04, 0x14, 0x0a,
    0x0d, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x01, 0x02, 0x12, 0x04, 0xf6, 0x01, 0x17, 0x18, 0x0a, 0x34,
    0x0a, 0x02, 0x04, 0x11, 0x12, 0x06, 0xfc, 0x01, 0x00, 0x83, 0x02, 0x01, 0x1a, 0x26, 0x2a, 0x0a,
    0x20, 0x45, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x6e, 0x66, 0x6f,
    0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x66, 0x69,
    0x6c, 0x65, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x11, 0x01, 0x12, 0x04, 0xfc, 0x01, 0x08,
    0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12, 0x04, 0xfd, 0x01, 0x02, 0x26, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x04, 0xfd, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x06, 0x12, 0x04, 0xfd, 0x01, 0x0b, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x04, 0xfd, 0x01, 0x1c, 0x21, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x04, 0xfd, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x11, 0x02, 0x01, 0x12, 0x04, 0xfe, 0x01, 0x02, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xfe, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01,
    0x06, 0x12, 0x04, 0xfe, 0x01, 0x0b, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xfe, 0x01, 0x26, 0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xfe, 0x01, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x02, 0x12, 0x04, 0xff,
    0x01, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x04, 0x12, 0x04, 0xff, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x05, 0x12, 0x04, 0xff, 0x01, 0x0b,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x01, 0x12, 0x04, 0xff, 0x01, 0x11, 0x14,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x03, 0x12, 0x04, 0xff, 0x01, 0x17, 0x18, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x03, 0x12, 0x04, 0x80, 0x02, 0x02, 0x18, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x03, 0x04, 0x12, 0x04, 0x80, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x03, 0x05, 0x12, 0x04, 0x80, 0x02, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x03, 0x01, 0x12, 0x04, 0x80, 0x02, 0x11, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x03, 0x03, 0x12, 0x04, 0x80, 0x02, 0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02,
    0x04, 0x12, 0x04, 0x81, 0x02, 0x02, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x81, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x04, 0x05, 0x12,
    0x04, 0x81, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x81, 0x02, 0x12, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x04, 0x03, 0x12, 0x04, 0x81,
    0x02, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x05, 0x12, 0x04, 0x82, 0x02, 0x02,
    0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x05, 0x04, 0x12, 0x04, 0x82, 0x02, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x05, 0x05, 0x12, 0x04, 0x82, 0x02, 0x0b, 0x11, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x05, 0x01, 0x12, 0x04, 0x82, 0x02, 0x12, 0x22, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x05, 0x03, 0x12, 0x04, 0x82, 0x02, 0x25, 0x26, 0x0a, 0x5a, 0x0a,
    0x02, 0x04, 0x12, 0x12, 0x06, 0x89, 0x02, 0x00, 0x8d, 0x02, 0x01, 0x1a, 0x4c, 0x2a, 0x0a, 0x20,
    0x45, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72,
    0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x6e, 0x20, 0x69, 0x6e,
    0x64, 0x69, 0x76, 0x69, 0x64, 0x75, 0x61, 0x6c, 0x0a, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x77,
    0x69, 0x74, 0x68, 0x69, 0x6e, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x7a, 0x6f, 0x6e, 0x65, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x12, 0x01,
    0x12, 0x04, 0x89, 0x02, 0x08, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x00, 0x12, 0x04,
    0x8a, 0x02, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8a,
    0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x05, 0x12, 0x04, 0x8a, 0x02,
    0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8a, 0x02, 0x11,
    0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8a, 0x02, 0x17, 0x18,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x01, 0x12, 0x04, 0x8b, 0x02, 0x02, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x04, 0x12, 0x04, 0x8b, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x12, 0x02, 0x01, 0x05, 0x12, 0x04, 0x8b, 0x02, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8b, 0x02, 0x11, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x12, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8b, 0x02, 0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12,
    0x02, 0x02, 0x12, 0x04, 0x8c, 0x02, 0x02, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02,
    0x04, 0x12, 0x04, 0x8c, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x05,
    0x12, 0x04, 0x8c, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x01, 0x12,
    0x04, 0x8c, 0x02, 0x12, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x02, 0x03, 0x12, 0x04,
    0x8c, 0x02, 0x25, 0x26, 0x0a, 0x40, 0x0a, 0x02, 0x04, 0x13, 0x12, 0x06, 0x93, 0x02, 0x00, 0x97,
    0x02, 0x01, 0x1a, 0x32, 0x2a, 0x0a, 0x20, 0x45, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f,
    0x72, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x0a,
    0x20, 0x7a, 0x6f, 0x6e, 0x65, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x04, 0x93,
    0x02, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x04, 0x94, 0x02, 0x02,
    0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x04, 0x94, 0x02, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x06, 0x12, 0x04, 0x94, 0x02, 0x0b, 0x1b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x04, 0x94, 0x02, 0x1c, 0x21, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x04, 0x94, 0x02, 0x24, 0x25, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x13, 0x02, 0x01, 0x12, 0x04, 0x95, 0x02, 0x02, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x01, 0x04, 0x12, 0x04, 0x95, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x01, 0x06, 0x12, 0x04, 0x95, 0x02, 0x0b, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x01, 0x01, 0x12, 0x04, 0x95, 0x02, 0x26, 0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01,
    0x03, 0x12, 0x04, 0x95, 0x02, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x02, 0x12,
    0x04, 0x96, 0x02, 0x02, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x04, 0x12, 0x04,
    0x96, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x05, 0x12, 0x04, 0x96,
    0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x01, 0x12, 0x04, 0x96, 0x02,
    0x12, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x03, 0x12, 0x04, 0x96, 0x02, 0x1c,
    0x1d, 0x0a, 0x1f, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x06, 0x9c, 0x02, 0x00, 0xa2, 0x02, 0x01, 0x1a,
    0x11, 0x2a, 0x0a, 0x20, 0x43, 0x69, 0x70, 0x68, 0x65, 0x72, 0x20, 0x6f, 0x70, 0x74, 0x69, 0x6f,
    0x6e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x04, 0x9c, 0x02, 0x08, 0x19, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x00, 0x12, 0x04, 0x9d, 0x02, 0x02, 0x26, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9d, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x00, 0x06, 0x12, 0x04, 0x9d, 0x02, 0x0b, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9d, 0x02, 0x1c, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x00, 0x03, 0x12, 0x04, 0x9d, 0x02, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02,
    0x01, 0x12, 0x04, 0x9e, 0x02, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x9e, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x05, 0x12,
    0x04, 0x9e, 0x02, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x01, 0x12, 0x04,
    0x9e, 0x02, 0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x03, 0x12, 0x04, 0x9e,
    0x02, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x02, 0x12, 0x04, 0x9f, 0x02, 0x02,
    0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x04, 0x12, 0x04, 0x9f, 0x02, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x05, 0x12, 0x04, 0x9f, 0x02, 0x0b, 0x10, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x01, 0x12, 0x04, 0x9f, 0x02, 0x11, 0x15, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x03, 0x12, 0x04, 0x9f, 0x02, 0x18, 0x19, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x14, 0x02, 0x03, 0x12, 0x04, 0xa0, 0x02, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x03, 0x04, 0x12, 0x04, 0xa0, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x03, 0x05, 0x12, 0x04, 0xa0, 0x02, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x03, 0x01, 0x12, 0x04, 0xa0, 0x02, 0x11, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03,
    0x03, 0x12, 0x04, 0xa0, 0x02, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x04, 0x12,
    0x04, 0xa1, 0x02, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x04, 0x04, 0x12, 0x04,
    0xa1, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x04, 0x05, 0x12, 0x04, 0xa1,
    0x02, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x04, 0x01, 0x12, 0x04, 0xa1, 0x02,
    0x11, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x04, 0x03, 0x12, 0x04, 0xa1, 0x02, 0x19,
    0x1a, 0x0a, 0x3b, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x06, 0xa7, 0x02, 0x00, 0xae, 0x02, 0x01, 0x1a,
    0x2d, 0x2a, 0x0a, 0x20, 0x41, 0x20, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x66, 0x69, 0x6c,
    0x65, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65,
    0x69, 0x72, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x0a, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x15, 0x01, 0x12, 0x04, 0xa7, 0x02, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x15, 0x02, 0x00, 0x12, 0x04, 0xa8, 0x02, 0x02, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xa8, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xa8, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xa8, 0x02, 0x12, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xa8, 0x02, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x01, 0x12, 0x04, 0xa9,
    0x02, 0x02, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa9, 0x02,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x06, 0x12, 0x04, 0xa9, 0x02, 0x0b,
    0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa9, 0x02, 0x1d, 0x23,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa9, 0x02, 0x26, 0x27, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x02, 0x12, 0x04, 0xaa, 0x02, 0x02, 0x26, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x02, 0x04, 0x12, 0x04, 0xaa, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x02, 0x05, 0x12, 0x04, 0xaa, 0x02, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x02, 0x01, 0x12, 0x04, 0xaa, 0x02, 0x10, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x02, 0x03, 0x12, 0x04, 0xaa, 0x02, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02,
    0x03, 0x12, 0x04, 0xab, 0x02, 0x02, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x04,
    0x12, 0x04, 0xab, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x06, 0x12,
    0x04, 0xab, 0x02, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x01, 0x12, 0x04,
    0xab, 0x02, 0x1d, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x03, 0x12, 0x04, 0xab,
    0x02, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x04, 0x12, 0x04, 0xac, 0x02, 0x02,
    0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x04, 0x04, 0x12, 0x04, 0xac, 0x02, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x04, 0x05, 0x12, 0x04, 0xac, 0x02, 0x0b, 0x0f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x04, 0x01, 0x12, 0x04, 0xac, 0x02, 0x10, 0x23, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x04, 0x03, 0x12, 0x04, 0xac, 0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x15, 0x02, 0x05, 0x12, 0x04, 0xad, 0x02, 0x02, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x05, 0x04, 0x12, 0x04, 0xad, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x05, 0x06, 0x12, 0x04, 0xad, 0x02, 0x0b, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x05, 0x01, 0x12, 0x04, 0xad, 0x02, 0x23, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x05,
    0x03, 0x12, 0x04, 0xad, 0x02, 0x38, 0x39, 0x0a, 0x8f, 0x01, 0x0a, 0x02, 0x04, 0x16, 0x12, 0x06,
    0xb4, 0x02, 0x00, 0xd2, 0x02, 0x01, 0x1a, 0x80, 0x01, 0x2a, 0x0a, 0x20, 0x53, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x2c, 0x20, 0x64, 0x69,
    0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x20, 0x6f, 0x72, 0x20, 0x73, 0x79, 0x6d, 0x6c, 0x69,
    0x6e, 0x6b, 0x0a, 0x20, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x69,
    0x6e, 0x63, 0x6c, 0x75, 0x64, 0x65, 0x73, 0x20, 0x61, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x27, 0x73,
    0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x20, 0x69, 0x66, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x65, 0x64, 0x20, 0x62, 0x79,
    0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72,
    0x70, 0x63, 0x20, 0x63, 0x61, 0x6c, 0x6c, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x16, 0x01,
    0x12, 0x04, 0xb4, 0x02, 0x08, 0x1b, 0x0a, 0x0e, 0x0a, 0x04, 0x04, 0x16, 0x04, 0x00, 0x12, 0x06,
    0xb5, 0x02, 0x02, 0xb9, 0x02, 0x03, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x04, 0x00, 0x01, 0x12,
    0x04, 0xb5, 0x02, 0x07, 0x0f, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x16, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x04, 0xb6, 0x02, 0x04, 0x0f, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x16, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xb6, 0x02, 0x04, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x16, 0x04, 0x00, 0x02, 0x00,
    0x02, 0x12, 0x04, 0xb6, 0x02, 0x0d, 0x0e, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x16, 0x04, 0x00, 0x02,
    0x01, 0x12, 0x04, 0xb7, 0x02, 0x04, 0x10, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x16, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xb7, 0x02, 0x04, 0x0b, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x16, 0x04, 0x00,
    0x02, 0x01, 0x02, 0x12, 0x04, 0xb7, 0x02, 0x0e, 0x0f, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x16, 0x04,
    0x00, 0x02, 0x02, 0x12, 0x04, 0xb8, 0x02, 0x04, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x16, 0x04,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb8, 0x02, 0x04, 0x0e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x16,
    0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xb8, 0x02, 0x11, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x16, 0x02, 0x00, 0x12, 0x04, 0xba, 0x02, 0x02, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xba, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00,
    0x06, 0x12, 0x04, 0xba, 0x02, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xba, 0x02, 0x14, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xba, 0x02, 0x1f, 0x20, 0x0a, 0x35, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x01, 0x12, 0x04, 0xbb,
    0x02, 0x02, 0x1a, 0x22, 0x27, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x20, 0x6e, 0x61, 0x6d, 0x65,
    0x20, 0x6f, 0x66, 0x20, 0x69, 0x6e, 0x6f, 0x64, 0x65, 0x20, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x65,
    0x64, 0x20, 0x6a, 0x61, 0x76, 0x61, 0x20, 0x55, 0x54, 0x46, 0x38, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x16, 0x02, 0x01, 0x04, 0x12, 0x04, 0xbb, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x16, 0x02, 0x01, 0x05, 0x12, 0x04, 0xbb, 0x02, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xbb, 0x02, 0x11, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xbb, 0x02, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x02,
    0x12, 0x04, 0xbc, 0x02, 0x02, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x02, 0x04, 0x12,
    0x04, 0xbc, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x02, 0x05, 0x12, 0x04,
    0xbc, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x02, 0x01, 0x12, 0x04, 0xbc,
    0x02, 0x12, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x02, 0x03, 0x12, 0x04, 0xbc, 0x02,
    0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x03, 0x12, 0x04, 0xbd, 0x02, 0x02, 0x2c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x03, 0x04, 0x12, 0x04, 0xbd, 0x02, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x03, 0x06, 0x12, 0x04, 0xbd, 0x02, 0x0b, 0x1c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x16, 0x02, 0x03, 0x01, 0x12, 0x04, 0xbd, 0x02, 0x1d, 0x27, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x16, 0x02, 0x03, 0x03, 0x12, 0x04, 0xbd, 0x02, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x16, 0x02, 0x04, 0x12, 0x04, 0xbe, 0x02, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x04, 0x04, 0x12, 0x04, 0xbe, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x04, 0x05, 0x12, 0x04, 0xbe, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x04,
    0x01, 0x12, 0x04, 0xbe, 0x02, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x04, 0x03,
    0x12, 0x04, 0xbe, 0x02, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x05, 0x12, 0x04,
    0xbf, 0x02, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x05, 0x04, 0x12, 0x04, 0xbf,
    0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x05, 0x05, 0x12, 0x04, 0xbf, 0x02,
    0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x05, 0x01, 0x12, 0x04, 0xbf, 0x02, 0x12,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x05, 0x03, 0x12, 0x04, 0xbf, 0x02, 0x1a, 0x1b,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x06, 0x12, 0x04, 0xc0, 0x02, 0x02, 0x28, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x16, 0x02, 0x06, 0x04, 0x12, 0x04, 0xc0, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x16, 0x02, 0x06, 0x05, 0x12, 0x04, 0xc0, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x16, 0x02, 0x06, 0x01, 0x12, 0x04, 0xc0, 0x02, 0x12, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x16, 0x02, 0x06, 0x03, 0x12, 0x04, 0xc0, 0x02, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16,
    0x02, 0x07, 0x12, 0x04, 0xc1, 0x02, 0x02, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x07,
    0x04, 0x12, 0x04, 0xc1, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x07, 0x05,
    0x12, 0x04, 0xc1, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x07, 0x01, 0x12,
    0x04, 0xc1, 0x02, 0x12, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x07, 0x03, 0x12, 0x04,
    0xc1, 0x02, 0x20, 0x21, 0x0a, 0x54, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x08, 0x12, 0x04, 0xc4, 0x02,
    0x02, 0x1d, 0x1a, 0x1d, 0x20, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x66, 0x69,
    0x65, 0x6c, 0x64, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x73, 0x79, 0x6d, 0x6c, 0x69, 0x6e, 0x6b,
    0x0a, 0x22, 0x27, 0x20, 0x69, 0x66, 0x20, 0x73, 0x79, 0x6d, 0x6c, 0x69, 0x6e, 0x6b, 0x2c, 0x20,
    0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x20, 0x65, 0x6e, 0x63, 0x6f, 0x64, 0x65, 0x64, 0x20, 0x6a,
    0x61, 0x76, 0x61, 0x20, 0x55, 0x54, 0x46, 0x38, 0x20, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x08, 0x04, 0x12, 0x04, 0xc4, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x08, 0x05, 0x12, 0x04, 0xc4, 0x02, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x08,
    0x01, 0x12, 0x04, 0xc4, 0x02, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x08, 0x03,
    0x12, 0x04, 0xc4, 0x02, 0x1b, 0x1c, 0x0a, 0x3c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x09, 0x12, 0x04,
    0xc7, 0x02, 0x02, 0x37, 0x1a, 0x1a, 0x20, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20,
    0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x0a,
    0x22, 0x12, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x31, 0x36, 0x62, 0x69, 0x74, 0x73, 0x20, 0x75,
    0x73, 0x65, 0x64, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x09, 0x04, 0x12, 0x04, 0xc7,
    0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x09, 0x05, 0x12, 0x04, 0xc7, 0x02,
    0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x09, 0x01, 0x12, 0x04, 0xc7, 0x02, 0x12,
    0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x09, 0x03, 0x12, 0x04, 0xc7, 0x02, 0x26, 0x28,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x09, 0x08, 0x12, 0x04, 0xc7, 0x02, 0x29, 0x36, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x09, 0x07, 0x12, 0x04, 0xc7, 0x02, 0x34, 0x35, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x16, 0x02, 0x0a, 0x12, 0x04, 0xc8, 0x02, 0x02, 0x2f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x16, 0x02, 0x0a, 0x04, 0x12, 0x04, 0xc8, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x16, 0x02, 0x0a, 0x05, 0x12, 0x04, 0xc8, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x0a, 0x01, 0x12, 0x04, 0xc8, 0x02, 0x12, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x0a, 0x03, 0x12, 0x04, 0xc8, 0x02, 0x1e, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0a,
    0x08, 0x12, 0x04, 0xc8, 0x02, 0x21, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0a, 0x07,
    0x12, 0x04, 0xc8, 0x02, 0x2c, 0x2d, 0x0a, 0x2f, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x0b, 0x12, 0x04,
    0xc9, 0x02, 0x02, 0x2d, 0x22, 0x21, 0x20, 0x73, 0x75, 0x70, 0x70, 0x6c, 0x65, 0x64, 0x20, 0x6f,
    0x6e, 0x6c, 0x79, 0x20, 0x69, 0x66, 0x20, 0x61, 0x73, 0x6b, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20,
    0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0b, 0x04,
    0x12, 0x04, 0xc9, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0b, 0x06, 0x12,
    0x04, 0xc9, 0x02, 0x0b, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0b, 0x01, 0x12, 0x04,
    0xc9, 0x02, 0x1e, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0b, 0x03, 0x12, 0x04, 0xc9,
    0x02, 0x2a, 0x2c, 0x0a, 0x45, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x0c, 0x12, 0x04, 0xcc, 0x02, 0x02,
    0x2c, 0x1a, 0x1b, 0x20, 0x4f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x49, 0x64, 0x0a, 0x22, 0x1a,
    0x20, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x61, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x69,
    0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x20, 0x69, 0x64, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x0c, 0x04, 0x12, 0x04, 0xcc, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x0c, 0x05, 0x12, 0x04, 0xcc, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0c,
    0x01, 0x12, 0x04, 0xcc, 0x02, 0x12, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0c, 0x03,
    0x12, 0x04, 0xcc, 0x02, 0x1b, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0c, 0x08, 0x12,
    0x04, 0xcc, 0x02, 0x1e, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0c, 0x07, 0x12, 0x04,
    0xcc, 0x02, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x0d, 0x12, 0x04, 0xcd, 0x02,
    0x02, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0d, 0x04, 0x12, 0x04, 0xcd, 0x02, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0d, 0x05, 0x12, 0x04, 0xcd, 0x02, 0x0b, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0d, 0x01, 0x12, 0x04, 0xcd, 0x02, 0x11, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0d, 0x03, 0x12, 0x04, 0xcd, 0x02, 0x1f, 0x21, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x16, 0x02, 0x0d, 0x08, 0x12, 0x04, 0xcd, 0x02, 0x22, 0x30, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x16, 0x02, 0x0d, 0x07, 0x12, 0x04, 0xcd, 0x02, 0x2d, 0x2f, 0x0a, 0x32, 0x0a, 0x04,
    0x04, 0x16, 0x02, 0x0e, 0x12, 0x04, 0xcf, 0x02, 0x02, 0x3b, 0x1a, 0x24, 0x20, 0x4f, 0x70, 0x74,
    0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20,
    0x66, 0x69, 0x6c, 0x65, 0x20, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0e, 0x04, 0x12, 0x04, 0xcf, 0x02, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0e, 0x06, 0x12, 0x04, 0xcf, 0x02, 0x0b, 0x22, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x16, 0x02, 0x0e, 0x01, 0x12, 0x04, 0xcf, 0x02, 0x23, 0x35, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x16, 0x02, 0x0e, 0x03, 0x12, 0x04, 0xcf, 0x02, 0x38, 0x3a, 0x0a, 0x27, 0x0a, 0x04,
    0x04, 0x16, 0x02, 0x0f, 0x12, 0x04, 0xd1, 0x02, 0x02, 0x33, 0x22, 0x19, 0x20, 0x62, 0x6c, 0x6f,
    0x63, 0x6b, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x20, 0x70, 0x6f, 0x6c, 0x69, 0x63,
    0x79, 0x20, 0x69, 0x64, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0f, 0x04, 0x12, 0x04,
    0xd1, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0f, 0x05, 0x12, 0x04, 0xd1,
    0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0f, 0x01, 0x12, 0x04, 0xd1, 0x02,
    0x12, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0f, 0x03, 0x12, 0x04, 0xd1, 0x02, 0x22,
    0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0f, 0x08, 0x12, 0x04, 0xd1, 0x02, 0x25, 0x32,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x0f, 0x07, 0x12, 0x04, 0xd1, 0x02, 0x30, 0x31, 0x0a,
    0xb5, 0x01, 0x0a, 0x02, 0x05, 0x03, 0x12, 0x06, 0xd9, 0x02, 0x00, 0xdd, 0x02, 0x01, 0x1a, 0xa6,
    0x01, 0x2a, 0x0a, 0x20, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x20, 0x61, 0x6c, 0x67,
    0x6f, 0x72, 0x69, 0x74, 0x68, 0x6d, 0x73, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x20, 0x75, 0x73,
    0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x48, 0x44, 0x46, 0x53, 0x0a, 0x20, 0x4d, 0x61, 0x6b, 0x65,
    0x20, 0x73, 0x75, 0x72, 0x65, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x65, 0x6e, 0x75, 0x6d, 0x27,
    0x73, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x67, 0x65, 0x72, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73,
    0x20, 0x6d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x65, 0x6e, 0x75, 0x6d, 0x20, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x73, 0x27, 0x20, 0x69, 0x64, 0x20, 0x70, 0x72, 0x6f, 0x70, 0x65, 0x72, 0x74, 0x69, 0x65,
    0x73, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x64, 0x0a, 0x20, 0x69, 0x6e, 0x20, 0x6f, 0x72,
    0x67, 0x2e, 0x61, 0x70, 0x61, 0x63, 0x68, 0x65, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e,
    0x75, 0x74, 0x69, 0x6c, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75,
    0x6d, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x05, 0x03, 0x01, 0x12, 0x04,
    0xd9, 0x02, 0x05, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x00, 0x12, 0x04, 0xda, 0x02,
    0x02, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x00, 0x01, 0x12, 0x04, 0xda, 0x02, 0x02,
    0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x03, 0x02, 0x00, 0x02, 0x12, 0x04, 0xda, 0x02, 0x12, 0x13,
    0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x03, 0x02, 0x01, 0x12, 0x04, 0xdb, 0x02, 0x02, 0x15, 0x0a, 0x0d,
    0x0a, 0x05, 0x05, 0x03, 0x02, 0x01, 0x01, 0x12, 0x04, 0xdb, 0x02, 0x02, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x05, 0x03, 0x02, 0x01, 0x02, 0x12, 0x04, 0xdb, 0x02, 0x13, 0x14, 0x0a, 0x0c, 0x0a, 0x04,
    0x05, 0x03, 0x02, 0x02, 0x12, 0x04, 0xdc, 0x02, 0x02, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x03,
    0x02, 0x02, 0x01, 0x12, 0x04, 0xdc, 0x02, 0x02, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x03, 0x02,
    0x02, 0x02, 0x12, 0x04, 0xdc, 0x02, 0x14, 0x15, 0x0a, 0x26, 0x0a, 0x02, 0x04, 0x17, 0x12, 0x06,
    0xe2, 0x02, 0x00, 0xeb, 0x02, 0x01, 0x1a, 0x18, 0x2a, 0x0a, 0x20, 0x48, 0x44, 0x46, 0x53, 0x20,
    0x53, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x73, 0x0a,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x17, 0x01, 0x12, 0x04, 0xe2, 0x02, 0x08, 0x1d, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x17, 0x02, 0x00, 0x12, 0x04, 0xe3, 0x02, 0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x17, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe3, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xe3, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xe3, 0x02, 0x12, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xe3, 0x02, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x01, 0x12,
    0x04, 0xe4, 0x02, 0x02, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x04, 0x12, 0x04,
    0xe4, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x05, 0x12, 0x04, 0xe4,
    0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe4, 0x02,
    0x12, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe4, 0x02, 0x25,
    0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x02, 0x12, 0x04, 0xe5, 0x02, 0x02, 0x26, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x04, 0x12, 0x04, 0xe5, 0x02, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x05, 0x12, 0x04, 0xe5, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x02, 0x01, 0x12, 0x04, 0xe5, 0x02, 0x12, 0x21, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x17, 0x02, 0x02, 0x03, 0x12, 0x04, 0xe5, 0x02, 0x24, 0x25, 0x0a, 0x34, 0x0a, 0x04, 0x04,
    0x17, 0x02, 0x03, 0x12, 0x04, 0xe6, 0x02, 0x02, 0x22, 0x22, 0x26, 0x20, 0x41, 0x63, 0x74, 0x75,
    0x61, 0x6c, 0x6c, 0x79, 0x20, 0x61, 0x20, 0x73, 0x68, 0x6f, 0x72, 0x74, 0x20, 0x2d, 0x20, 0x6f,
    0x6e, 0x6c, 0x79, 0x20, 0x31, 0x36, 0x20, 0x62, 0x69, 0x74, 0x73, 0x20, 0x75, 0x73, 0x65, 0x64,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x03, 0x04, 0x12, 0x04, 0xe6, 0x02, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x03, 0x05, 0x12, 0x04, 0xe6, 0x02, 0x0b, 0x11, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x03, 0x01, 0x12, 0x04, 0xe6, 0x02, 0x12, 0x1d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x03, 0x03, 0x12, 0x04, 0xe6, 0x02, 0x20, 0x21, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x17, 0x02, 0x04, 0x12, 0x04, 0xe7, 0x02, 0x02, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x17, 0x02, 0x04, 0x04, 0x12, 0x04, 0xe7, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17,
    0x02, 0x04, 0x05, 0x12, 0x04, 0xe7, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02,
    0x04, 0x01, 0x12, 0x04, 0xe7, 0x02, 0x12, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x04,
    0x03, 0x12, 0x04, 0xe7, 0x02, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x05, 0x12,
    0x04, 0xe8, 0x02, 0x02, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x05, 0x04, 0x12, 0x04,
    0xe8, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x05, 0x05, 0x12, 0x04, 0xe8,
    0x02, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x05, 0x01, 0x12, 0x04, 0xe8, 0x02,
    0x10, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x05, 0x03, 0x12, 0x04, 0xe8, 0x02, 0x26,
    0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x05, 0x08, 0x12, 0x04, 0xe8, 0x02, 0x28, 0x39,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x05, 0x07, 0x12, 0x04, 0xe8, 0x02, 0x33, 0x38, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x06, 0x12, 0x04, 0xe9, 0x02, 0x02, 0x32, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x17, 0x02, 0x06, 0x04, 0x12, 0x04, 0xe9, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x17, 0x02, 0x06, 0x05, 0x12, 0x04, 0xe9, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x17, 0x02, 0x06, 0x01, 0x12, 0x04, 0xe9, 0x02, 0x12, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17,
    0x02, 0x06, 0x03, 0x12, 0x04, 0xe9, 0x02, 0x22, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02,
    0x06, 0x08, 0x12, 0x04, 0xe9, 0x02, 0x24, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x06,
    0x07, 0x12, 0x04, 0xe9, 0x02, 0x2f, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x07, 0x12,
    0x04, 0xea, 0x02, 0x02, 0x49, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x07, 0x04, 0x12, 0x04,
    0xea, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x07, 0x06, 0x12, 0x04, 0xea,
    0x02, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x07, 0x01, 0x12, 0x04, 0xea, 0x02,
    0x1d, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x07, 0x03, 0x12, 0x04, 0xea, 0x02, 0x2c,
    0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x07, 0x08, 0x12, 0x04, 0xea, 0x02, 0x2e, 0x48,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x07, 0x07, 0x12, 0x04, 0xea, 0x02, 0x39, 0x47, 0x0a,
    0x23, 0x0a, 0x02, 0x04, 0x18, 0x12, 0x06, 0xf1, 0x02, 0x00, 0xf4, 0x02, 0x01, 0x1a, 0x15, 0x2a,
    0x0a, 0x20, 0x44, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x20, 0x6c, 0x69, 0x73, 0x74,
    0x69, 0x6e, 0x67, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x18, 0x01, 0x12, 0x04, 0xf1, 0x02, 0x08,
    0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x00, 0x12, 0x04, 0xf2, 0x02, 0x02, 0x32, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x04, 0x12, 0x04, 0xf2, 0x02, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x06, 0x12, 0x04, 0xf2, 0x02, 0x0b, 0x1e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x18, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf2, 0x02, 0x1f, 0x2d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x18, 0x02, 0x00, 0x03, 0x12, 0x04, 0xf2, 0x02, 0x30, 0x31, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x18, 0x02, 0x01, 0x12, 0x04, 0xf3, 0x02, 0x02, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xf3, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01,
    0x05, 0x12, 0x04, 0xf3, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xf3, 0x02, 0x12, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xf3, 0x02, 0x26, 0x27, 0x0a, 0xcf, 0x01, 0x0a, 0x02, 0x04, 0x19, 0x12, 0x06, 0xfb, 0x02,
    0x00, 0x82, 0x03, 0x01, 0x1a, 0xc0, 0x01, 0x2a, 0x0a, 0x20, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x74, 0x61,
    0x62, 0x6c, 0x65, 0x20, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x3a, 0x20, 0x62,
    0x65, 0x73, 0x69, 0x64, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x6f, 0x72, 0x6d, 0x61,
    0x6c, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f,
    0x72, 0x20, 0x0a, 0x20, 0x61, 0x20, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x20,
    0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2c, 0x20, 0x61, 0x6c, 0x73, 0x6f, 0x20, 0x69, 0x6e, 0x63,
    0x6c, 0x75, 0x64, 0x65, 0x20, 0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x20, 0x71, 0x75,
    0x6f, 0x74, 0x61, 0x2c, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x73,
    0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x73, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x0a, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x66, 0x75, 0x6c, 0x6c, 0x20, 0x70, 0x61, 0x74, 0x68, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x64, 0x69, 0x72, 0x65, 0x63,
    0x74, 0x6f, 0x72, 0x79, 0x2e, 0x20, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x19, 0x01, 0x12, 0x04,
    0xfb, 0x02, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x00, 0x12, 0x04, 0xfc, 0x02,
    0x02, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x04, 0x12, 0x04, 0xfc, 0x02, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x06, 0x12, 0x04, 0xfc, 0x02, 0x0b, 0x1e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x01, 0x12, 0x04, 0xfc, 0x02, 0x1f, 0x28, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x03, 0x12, 0x04, 0xfc, 0x02, 0x2b, 0x2c, 0x0a, 0x3b,
    0x0a, 0x04, 0x04, 0x19, 0x02, 0x01, 0x12, 0x04, 0xff, 0x02, 0x02, 0x25, 0x1a, 0x2d, 0x20, 0x46,
    0x69, 0x65, 0x6c, 0x64, 0x73, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x74, 0x61, 0x62, 0x6c, 0x65,
    0x20, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x19, 0x02, 0x01, 0x04, 0x12, 0x04, 0xff, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19,
    0x02, 0x01, 0x05, 0x12, 0x04, 0xff, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xff, 0x02, 0x12, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xff, 0x02, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x02, 0x12,
    0x04, 0x80, 0x03, 0x02, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x04, 0x12, 0x04,
    0x80, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x05, 0x12, 0x04, 0x80,
    0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x01, 0x12, 0x04, 0x80, 0x03,
    0x12, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x03, 0x12, 0x04, 0x80, 0x03, 0x24,
    0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x03, 0x12, 0x04, 0x81, 0x03, 0x02, 0x25, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x03, 0x04, 0x12, 0x04, 0x81, 0x03, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x19, 0x02, 0x03, 0x05, 0x12, 0x04, 0x81, 0x03, 0x0b, 0x10, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x19, 0x02, 0x03, 0x01, 0x12, 0x04, 0x81, 0x03, 0x11, 0x20, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x19, 0x02, 0x03, 0x03, 0x12, 0x04, 0x81, 0x03, 0x23, 0x24, 0x0a, 0x31, 0x0a, 0x02, 0x04,
    0x1a, 0x12, 0x06, 0x87, 0x03, 0x00, 0x89, 0x03, 0x01, 0x1a, 0x23, 0x2a, 0x0a, 0x20, 0x53, 0x6e,
    0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x74, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x64, 0x69, 0x72, 0x65,
    0x63, 0x74, 0x6f, 0x72, 0x79, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x0a, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x1a, 0x01, 0x12, 0x04, 0x87, 0x03, 0x08, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x1a, 0x02, 0x00, 0x12, 0x04, 0x88, 0x03, 0x02, 0x49, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x88, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00,
    0x06, 0x12, 0x04, 0x88, 0x03, 0x0b, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x88, 0x03, 0x2d, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x88, 0x03, 0x47, 0x48, 0x0a, 0x2c, 0x0a, 0x02, 0x04, 0x1b, 0x12, 0x06, 0x8e, 0x03, 0x00,
    0x92, 0x03, 0x01, 0x1a, 0x1e, 0x2a, 0x0a, 0x20, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74,
    0x20, 0x64, 0x69, 0x66, 0x66, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x65, 0x6e, 0x74,
    0x72, 0x79, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1b, 0x01, 0x12, 0x04, 0x8e, 0x03, 0x08, 0x24,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x00, 0x12, 0x04, 0x8f, 0x03, 0x02, 0x1e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8f, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1b, 0x02, 0x00, 0x05, 0x12, 0x04, 0x8f, 0x03, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1b, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8f, 0x03, 0x11, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1b, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8f, 0x03, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b,
    0x02, 0x01, 0x12, 0x04, 0x90, 0x03, 0x02, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x90, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x05,
    0x12, 0x04, 0x90, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x01, 0x12,
    0x04, 0x90, 0x03, 0x12, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x03, 0x12, 0x04,
    0x90, 0x03, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x02, 0x12, 0x04, 0x91, 0x03,
    0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x02, 0x04, 0x12, 0x04, 0x91, 0x03, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x02, 0x05, 0x12, 0x04, 0x91, 0x03, 0x0b, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x02, 0x01, 0x12, 0x04, 0x91, 0x03, 0x11, 0x1b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x02, 0x03, 0x12, 0x04, 0x91, 0x03, 0x1e, 0x1f, 0x0a, 0x26,
    0x0a, 0x02, 0x04, 0x1c, 0x12, 0x06, 0x97, 0x03, 0x00, 0x9d, 0x03, 0x01, 0x1a, 0x18, 0x2a, 0x0a,
    0x20, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x20, 0x64, 0x69, 0x66, 0x66, 0x20, 0x72,
    0x65, 0x70, 0x6f, 0x72, 0x74, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1c, 0x01, 0x12, 0x04, 0x97,
    0x03, 0x08, 0x1f, 0x0a, 0x45, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x00, 0x12, 0x04, 0x99, 0x03, 0x02,
    0x23, 0x1a, 0x37, 0x20, 0x66, 0x75, 0x6c, 0x6c, 0x20, 0x70, 0x61, 0x74, 0x68, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x20, 0x77,
    0x68, 0x65, 0x72, 0x65, 0x20, 0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x73, 0x20, 0x77,
    0x65, 0x72, 0x65, 0x20, 0x74, 0x61, 0x6b, 0x65, 0x6e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x99, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02,
    0x00, 0x05, 0x12, 0x04, 0x99, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x99, 0x03, 0x12, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x00, 0x03,
    0x12, 0x04, 0x99, 0x03, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x01, 0x12, 0x04,
    0x9a, 0x03, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x04, 0x12, 0x04, 0x9a,
    0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x05, 0x12, 0x04, 0x9a, 0x03,
    0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9a, 0x03, 0x12,
    0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x01, 0x03, 0x12, 0x04, 0x9a, 0x03, 0x21, 0x22,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1c, 0x02, 0x02, 0x12, 0x04, 0x9b, 0x03, 0x02, 0x21, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1c, 0x02, 0x02, 0x04, 0x12, 0x04, 0x9b, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1c, 0x02, 0x02, 0x05, 0x12, 0x04, 0x9b, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1c, 0x02, 0x02, 0x01, 0x12, 0x04, 0x9b, 0x03, 0x12, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1c, 0x02, 0x02, 0x03, 0x12, 0x04, 0x9b, 0x03, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1c,
    0x02, 0x03, 0x12, 0x04, 0x9c, 0x03, 0x02, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x03,
    0x04, 0x12, 0x04, 0x9c, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x03, 0x06,
    0x12, 0x04, 0x9c, 0x03, 0x0b, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x03, 0x01, 0x12,
    0x04, 0x9c, 0x03, 0x28, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1c, 0x02, 0x03, 0x03, 0x12, 0x04,
    0x9c, 0x03, 0x3c, 0x3d, 0x0a, 0x50, 0x0a, 0x02, 0x04, 0x1d, 0x12, 0x06, 0xa2, 0x03, 0x00, 0xa7,
    0x03, 0x01, 0x1a, 0x42, 0x2a, 0x0a, 0x20, 0x43, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x20, 0x6e, 0x6f,
    0x64, 0x65, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x73,
    0x68, 0x61, 0x72, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x6e, 0x6f, 0x64, 0x65, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c,
    0x75, 0x73, 0x74, 0x65, 0x72, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1d, 0x01, 0x12, 0x04, 0xa2,
    0x03, 0x08, 0x18, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x00, 0x12, 0x04, 0xa3, 0x03, 0x02,
    0x24, 0x22, 0x23, 0x20, 0x4c, 0x61, 0x79, 0x6f, 0x75, 0x74, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x73,
    0x79, 0x73, 0x74, 0x65, 0x6d, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xa3, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x05, 0x12, 0x04,
    0xa3, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa3,
    0x03, 0x12, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa3, 0x03,
    0x22, 0x23, 0x0a, 0x28, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x01, 0x12, 0x04, 0xa4, 0x03, 0x02, 0x21,
    0x22, 0x1a, 0x20, 0x46, 0x69, 0x6c, 0x65, 0x20, 0x73, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x20, 0x6e,
    0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x20, 0x49, 0x44, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1d, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa4, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1d, 0x02, 0x01, 0x05, 0x12, 0x04, 0xa4, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d,
    0x02, 0x01, 0x01, 0x12, 0x04, 0xa4, 0x03, 0x12, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02,
    0x01, 0x03, 0x12, 0x04, 0xa4, 0x03, 0x1f, 0x20, 0x0a, 0x21, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x02,
    0x12, 0x04, 0xa5, 0x03, 0x02, 0x20, 0x22, 0x13, 0x20, 0x49, 0x44, 0x20, 0x6f, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1d, 0x02, 0x02, 0x04, 0x12, 0x04, 0xa5, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d,
    0x02, 0x02, 0x05, 0x12, 0x04, 0xa5, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02,
    0x02, 0x01, 0x12, 0x04, 0xa5, 0x03, 0x12, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x02,
    0x03, 0x12, 0x04, 0xa5, 0x03, 0x1e, 0x1f, 0x0a, 0x29, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x03, 0x12,
    0x04, 0xa6, 0x03, 0x02, 0x1c, 0x22, 0x1b, 0x20, 0x46, 0x69, 0x6c, 0x65, 0x20, 0x73, 0x79, 0x73,
    0x74, 0x65, 0x6d, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x69, 0x6d,
    0x65, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x03, 0x04, 0x12, 0x04, 0xa6, 0x03, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x03, 0x05, 0x12, 0x04, 0xa6, 0x03, 0x0b, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x03, 0x01, 0x12, 0x04, 0xa6, 0x03, 0x12, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x03, 0x03, 0x12, 0x04, 0xa6, 0x03, 0x1a, 0x1b, 0x0a, 0x5c,
    0x0a, 0x02, 0x04, 0x1e, 0x12, 0x06, 0xac, 0x03, 0x00, 0xb6, 0x03, 0x01, 0x1a, 0x4e, 0x2a, 0x0a,
    0x20, 0x49, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x73, 0x65, 0x6e,
    0x74, 0x20, 0x62, 0x79, 0x20, 0x61, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x6e, 0x6f, 0x64, 0x65, 0x20,
    0x74, 0x6f, 0x20, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x79, 0x20, 0x69, 0x74, 0x73, 0x65,
    0x6c, 0x66, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72,
    0x79, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x6e, 0x6f, 0x64, 0x65, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x1e, 0x01, 0x12, 0x04, 0xac, 0x03, 0x08, 0x21, 0x0a, 0x35, 0x0a, 0x04, 0x04, 0x1e, 0x02,
    0x00, 0x12, 0x04, 0xad, 0x03, 0x02, 0x21, 0x22, 0x27, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x3a, 0x70,
    0x6f, 0x72, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x6e,
    0x6f, 0x64, 0x65, 0x20, 0x52, 0x50, 0x43, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x04, 0x12, 0x04, 0xad, 0x03, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x05, 0x12, 0x04, 0xad, 0x03, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x01, 0x12, 0x04, 0xad, 0x03, 0x12, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1e, 0x02, 0x00, 0x03, 0x12, 0x04, 0xad, 0x03, 0x1f, 0x20, 0x0a, 0x35, 0x0a, 0x04,
    0x04, 0x1e, 0x02, 0x01, 0x12, 0x04, 0xae, 0x03, 0x02, 0x22, 0x22, 0x27, 0x20, 0x68, 0x6f, 0x73,
    0x74, 0x3a, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x61,
    0x6d, 0x65, 0x6e, 0x6f, 0x64, 0x65, 0x20, 0x68, 0x74, 0x74, 0x70, 0x20, 0x73, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x04, 0x12, 0x04, 0xae, 0x03,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x05, 0x12, 0x04, 0xae, 0x03, 0x0b,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x01, 0x12, 0x04, 0xae, 0x03, 0x12, 0x1d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x03, 0x12, 0x04, 0xae, 0x03, 0x20, 0x21, 0x0a,
    0x0e, 0x0a, 0x04, 0x04, 0x1e, 0x04, 0x00, 0x12, 0x06, 0xaf, 0x03, 0x02, 0xb3, 0x03, 0x03, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x04, 0x00, 0x01, 0x12, 0x04, 0xaf, 0x03, 0x07, 0x18, 0x0a, 0x0e,
    0x0a, 0x06, 0x04, 0x1e, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xb0, 0x03, 0x04, 0x11, 0x0a, 0x0f,
    0x0a, 0x07, 0x04, 0x1e, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb0, 0x03, 0x04, 0x0c, 0x0a,
    0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xb0, 0x03, 0x0f, 0x10,
    0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1e, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xb1, 0x03, 0x04, 0x0f,
    0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb1, 0x03, 0x04,
    0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04, 0xb1, 0x03,
    0x0d, 0x0e, 0x0a, 0x0e, 0x0a, 0x06, 0x04, 0x1e, 0x04, 0x00, 0x02, 0x02, 0x12, 0x04, 0xb2, 0x03,
    0x04, 0x13, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb2,
    0x03, 0x04, 0x0e, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x1e, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04,
    0xb2, 0x03, 0x11, 0x12, 0x0a, 0x20, 0x0a, 0x04, 0x04, 0x1e, 0x02, 0x02, 0x12, 0x04, 0xb4, 0x03,
    0x02, 0x2c, 0x22, 0x12, 0x20, 0x4e, 0x6f, 0x64, 0x65, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02, 0x04, 0x12,
    0x04, 0xb4, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02, 0x06, 0x12, 0x04,
    0xb4, 0x03, 0x0b, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb4,
    0x03, 0x1c, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x02, 0x03, 0x12, 0x04, 0xb4, 0x03,
    0x2a, 0x2b, 0x0a, 0x1d, 0x0a, 0x04, 0x04, 0x1e, 0x02, 0x03, 0x12, 0x04, 0xb5, 0x03, 0x02, 0x3b,
    0x22, 0x0f, 0x20, 0x4e, 0x61, 0x6d, 0x65, 0x6e, 0x6f, 0x64, 0x65, 0x20, 0x72, 0x6f, 0x6c, 0x65,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x03, 0x04, 0x12, 0x04, 0xb5, 0x03, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x03, 0x06, 0x12, 0x04, 0xb5, 0x03, 0x0b, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x03, 0x01, 0x12, 0x04, 0xb5, 0x03, 0x1d, 0x21, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1e, 0x02, 0x03, 0x03, 0x12, 0x04, 0xb5, 0x03, 0x24, 0x25, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1e, 0x02, 0x03, 0x08, 0x12, 0x04, 0xb5, 0x03, 0x26, 0x3a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1e, 0x02, 0x03, 0x07, 0x12, 0x04, 0xb5, 0x03, 0x31, 0x39, 0x0a, 0x47, 0x0a, 0x02, 0x04,
    0x1f, 0x12, 0x06, 0xbb, 0x03, 0x00, 0xc0, 0x03, 0x01, 0x1a, 0x39, 0x2a, 0x0a, 0x20, 0x55, 0x6e,
    0x69, 0x71, 0x75, 0x65, 0x20, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x20, 0x74,
    0x6f, 0x20, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x79, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b,
    0x70, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1f, 0x01, 0x12, 0x04, 0xbb, 0x03, 0x08,
    0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x00, 0x12, 0x04, 0xbc, 0x03, 0x02, 0x22, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x04, 0x12, 0x04, 0xbc, 0x03, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x05, 0x12, 0x04, 0xbc, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1f, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbc, 0x03, 0x12, 0x1d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1f, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbc, 0x03, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x1f, 0x02, 0x01, 0x12, 0x04, 0xbd, 0x03, 0x02, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xbd, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01,
    0x05, 0x12, 0x04, 0xbd, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xbd, 0x03, 0x12, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xbd, 0x03, 0x2d, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x02, 0x12, 0x04, 0xbe,
    0x03, 0x02, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x02, 0x04, 0x12, 0x04, 0xbe, 0x03,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x02, 0x05, 0x12, 0x04, 0xbe, 0x03, 0x0b,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x02, 0x01, 0x12, 0x04, 0xbe, 0x03, 0x12, 0x20,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x02, 0x03, 0x12, 0x04, 0xbe, 0x03, 0x23, 0x24, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x03, 0x12, 0x04, 0xbf, 0x03, 0x02, 0x2c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1f, 0x02, 0x03, 0x04, 0x12, 0x04, 0xbf, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1f, 0x02, 0x03, 0x06, 0x12, 0x04, 0xbf, 0x03, 0x0b, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1f, 0x02, 0x03, 0x01, 0x12, 0x04, 0xbf, 0x03, 0x1c, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f,
    0x02, 0x03, 0x03, 0x12, 0x04, 0xbf, 0x03, 0x2a, 0x2b, 0x0a, 0x45, 0x0a, 0x02, 0x04, 0x20, 0x12,
    0x06, 0xc5, 0x03, 0x00, 0xcd, 0x03, 0x01, 0x1a, 0x37, 0x2a, 0x0a, 0x20, 0x43, 0x6f, 0x6d, 0x6d,
    0x61, 0x6e, 0x64, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x6f, 0x6e,
    0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x6e, 0x6f, 0x64, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x6e,
    0x6f, 0x74, 0x68, 0x65, 0x72, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x6e, 0x6f, 0x64, 0x65, 0x2e, 0x0a,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x20, 0x01, 0x12, 0x04, 0xc5, 0x03, 0x08, 0x1c, 0x0a, 0x0e, 0x0a,
    0x04, 0x04, 0x20, 0x04, 0x00, 0x12, 0x06, 0xc6, 0x03, 0x02, 0xc9, 0x03, 0x03, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x20, 0x04, 0x00, 0x01, 0x12, 0x04, 0xc6, 0x03, 0x07, 0x0b, 0x0a, 0x1e, 0x0a, 0x06,
    0x04, 0x20, 0x04, 0x00, 0x02, 0x00, 0x12, 0x04, 0xc7, 0x03, 0x04, 0x18, 0x22, 0x0e, 0x20, 0x42,
    0x61, 0x73, 0x65, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x0a, 0x0a, 0x0f, 0x0a, 0x07,
    0x04, 0x20, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc7, 0x03, 0x04, 0x13, 0x0a, 0x0f, 0x0a,
    0x07, 0x04, 0x20, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x04, 0xc7, 0x03, 0x16, 0x17, 0x0a, 0x25,
    0x0a, 0x06, 0x04, 0x20, 0x04, 0x00, 0x02, 0x01, 0x12, 0x04, 0xc8, 0x03, 0x04, 0x1a, 0x22, 0x15,
    0x20, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x20, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x63, 0x6f, 0x6d,
    0x6d, 0x61, 0x6e, 0x64, 0x0a, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x20, 0x04, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xc8, 0x03, 0x04, 0x15, 0x0a, 0x0f, 0x0a, 0x07, 0x04, 0x20, 0x04, 0x00, 0x02, 0x01,
    0x02, 0x12, 0x04, 0xc8, 0x03, 0x18, 0x19, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x00, 0x12,
    0x04, 0xca, 0x03, 0x02, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xca, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x05, 0x12, 0x04, 0xca,
    0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x01, 0x12, 0x04, 0xca, 0x03,
    0x12, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x03, 0x12, 0x04, 0xca, 0x03, 0x1b,
    0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x01, 0x12, 0x04, 0xcb, 0x03, 0x02, 0x19, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x04, 0x12, 0x04, 0xcb, 0x03, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x06, 0x12, 0x04, 0xcb, 0x03, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x20, 0x02, 0x01, 0x01, 0x12, 0x04, 0xcb, 0x03, 0x10, 0x14, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x20, 0x02, 0x01, 0x03, 0x12, 0x04, 0xcb, 0x03, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x20, 0x02, 0x02, 0x12, 0x04, 0xcc, 0x03, 0x02, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02,
    0x02, 0x04, 0x12, 0x04, 0xcc, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x02,
    0x06, 0x12, 0x04, 0xcc, 0x03, 0x0b, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x02, 0x01,
    0x12, 0x04, 0xcc, 0x03, 0x22, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x02, 0x03, 0x12,
    0x04, 0xcc, 0x03, 0x32, 0x33, 0x0a, 0xdd, 0x01, 0x0a, 0x02, 0x04, 0x21, 0x12, 0x06, 0xd5, 0x03,
    0x00, 0xdb, 0x03, 0x01, 0x1a, 0xce, 0x01, 0x2a, 0x0a, 0x20, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e,
    0x64, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x65, 0x64, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20,
    0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x20, 0x74, 0x6f, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b,
    0x70, 0x6f, 0x69, 0x6e, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x6e, 0x6f, 0x64,
    0x65, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64,
    0x20, 0x68, 0x61, 0x73, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x20,
    0x73, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x69,
    0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x73, 0x0a, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b,
    0x70, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x65, 0x65, 0x64, 0x65, 0x64, 0x20,
    0x66, 0x6f, 0x72, 0x20, 0x66, 0x75, 0x72, 0x74, 0x68, 0x65, 0x72, 0x0a, 0x20, 0x63, 0x6f, 0x6d,
    0x6d, 0x75, 0x6e, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x72, 0x65, 0x6c, 0x61, 0x74,
    0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x70, 0x6f, 0x69, 0x6e, 0x74,
    0x69, 0x6e, 0x67, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x21, 0x01, 0x12, 0x04, 0xd5, 0x03,
    0x08, 0x1e, 0x0a, 0x42, 0x0a, 0x04, 0x04, 0x21, 0x02, 0x00, 0x12, 0x04, 0xd7, 0x03, 0x02, 0x32,
    0x1a, 0x34, 0x20, 0x55, 0x6e, 0x69, 0x71, 0x75, 0x65, 0x20, 0x73, 0x69, 0x67, 0x6e, 0x61, 0x74,
    0x75, 0x72, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x79, 0x20,
    0x63, 0x68, 0x65, 0x63, 0x6b, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xd7, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x06, 0x12, 0x04,
    0xd7, 0x03, 0x0b, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd7,
    0x03, 0x24, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd7, 0x03,
    0x30, 0x31, 0x0a, 0x5b, 0x0a, 0x04, 0x04, 0x21, 0x02, 0x01, 0x12, 0x04, 0xda, 0x03, 0x02, 0x26,
    0x1a, 0x4d, 0x20, 0x49, 0x66, 0x20, 0x74, 0x72, 0x75, 0x65, 0x2c, 0x20, 0x72, 0x65, 0x74, 0x75,
    0x72, 0x6e, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x20, 0x69, 0x6d, 0x61, 0x67,
    0x65, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x20, 0x75, 0x70, 0x6f,
    0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x65, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x6f, 0x66, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x04, 0x12, 0x04, 0xda, 0x03, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x05, 0x12, 0x04, 0xda, 0x03, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x21, 0x02, 0x01, 0x01, 0x12, 0x04, 0xda, 0x03, 0x10, 0x21, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x21, 0x02, 0x01, 0x03, 0x12, 0x04, 0xda, 0x03, 0x24, 0x25, 0x0a, 0xa1, 0x02, 0x0a, 0x02,
    0x04, 0x22, 0x12, 0x06, 0xe6, 0x03, 0x00, 0xea, 0x03, 0x01, 0x1a, 0x92, 0x02, 0x2a, 0x0a, 0x20,
    0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x0a, 0x0a, 0x20, 0x50, 0x6c, 0x65, 0x61, 0x73, 0x65, 0x20, 0x62, 0x65, 0x20, 0x77, 0x61,
    0x72, 0x79, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x64, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x64, 0x64,
    0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x20, 0x68,
    0x65, 0x72, 0x65, 0x2c, 0x20, 0x73, 0x69, 0x6e, 0x63, 0x65, 0x20, 0x49, 0x4e, 0x6f, 0x64, 0x65,
    0x46, 0x69, 0x6c, 0x65, 0x73, 0x0a, 0x20, 0x6e, 0x65, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x66,
    0x69, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x50, 0x42, 0x27, 0x73, 0x20, 0x64, 0x65, 0x66, 0x61, 0x75,
    0x6c, 0x74, 0x20, 0x6d, 0x61, 0x78, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x73,
    0x69, 0x7a, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x36, 0x34, 0x4d, 0x42, 0x2e, 0x0a, 0x20, 0x57, 0x65,
    0x20, 0x72, 0x65, 0x73, 0x74, 0x72, 0x69, 0x63, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61,
    0x78, 0x20, 0x23, 0x20, 0x6f, 0x66, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x73, 0x20, 0x70, 0x65,
    0x72, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x0a, 0x20, 0x28, 0x64, 0x66, 0x73, 0x2e, 0x6e, 0x61, 0x6d,
    0x65, 0x6e, 0x6f, 0x64, 0x65, 0x2e, 0x66, 0x73, 0x2d, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x73, 0x2e,
    0x6d, 0x61, 0x78, 0x2d, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x73, 0x2d, 0x70, 0x65, 0x72, 0x2d, 0x66,
    0x69, 0x6c, 0x65, 0x29, 0x2c, 0x20, 0x62, 0x75, 0x74, 0x20, 0x69, 0x74, 0x27, 0x73, 0x20, 0x62,
    0x65, 0x74, 0x74, 0x65, 0x72, 0x0a, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x76, 0x6f, 0x69, 0x64, 0x20,
    0x63, 0x68, 0x61, 0x6e, 0x67, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x69, 0x73, 0x2e, 0x0a, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x22, 0x01, 0x12, 0x04, 0xe6, 0x03, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x22, 0x02, 0x00, 0x12, 0x04, 0xe7, 0x03, 0x02, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xe7, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xe7, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xe7, 0x03, 0x12, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xe7, 0x03, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x22, 0x02, 0x01, 0x12, 0x04,
    0xe8, 0x03, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x04, 0x12, 0x04, 0xe8,
    0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x05, 0x12, 0x04, 0xe8, 0x03,
    0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe8, 0x03, 0x12,
    0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe8, 0x03, 0x1d, 0x1e,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x22, 0x02, 0x02, 0x12, 0x04, 0xe9, 0x03, 0x02, 0x2d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x22, 0x02, 0x02, 0x04, 0x12, 0x04, 0xe9, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x22, 0x02, 0x02, 0x05, 0x12, 0x04, 0xe9, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x22, 0x02, 0x02, 0x01, 0x12, 0x04, 0xe9, 0x03, 0x12, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x22, 0x02, 0x02, 0x03, 0x12, 0x04, 0xe9, 0x03, 0x1d, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22,
    0x02, 0x02, 0x08, 0x12, 0x04, 0xe9, 0x03, 0x1f, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02,
    0x02, 0x07, 0x12, 0x04, 0xe9, 0x03, 0x2a, 0x2b, 0x0a, 0x39, 0x0a, 0x02, 0x04, 0x23, 0x12, 0x06,
    0xef, 0x03, 0x00, 0xf4, 0x03, 0x01, 0x1a, 0x2b, 0x2a, 0x0a, 0x20, 0x42, 0x6c, 0x6f, 0x63, 0x6b,
    0x20, 0x61, 0x6e, 0x64, 0x20, 0x64, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x73, 0x20, 0x77,
    0x68, 0x65, 0x72, 0x65, 0x20, 0x69, 0x73, 0x20, 0x69, 0x74, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x74,
    0x65, 0x64, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x23, 0x01, 0x12, 0x04, 0xef, 0x03, 0x08, 0x1f,
    0x0a, 0x15, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x00, 0x12, 0x04, 0xf0, 0x03, 0x02, 0x20, 0x22, 0x07,
    0x20, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xf0, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x06, 0x12,
    0x04, 0xf0, 0x03, 0x0b, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xf0, 0x03, 0x16, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x03, 0x12, 0x04, 0xf0,
    0x03, 0x1e, 0x1f, 0x0a, 0x34, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x01, 0x12, 0x04, 0xf1, 0x03, 0x02,
    0x24, 0x22, 0x26, 0x20, 0x44, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x73, 0x20, 0x77, 0x69,
    0x74, 0x68, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xf1, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x01,
    0x05, 0x12, 0x04, 0xf1, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xf1, 0x03, 0x12, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xf1, 0x03, 0x22, 0x23, 0x0a, 0x33, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x02, 0x12, 0x04, 0xf2,
    0x03, 0x02, 0x23, 0x22, 0x25, 0x20, 0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x73, 0x20, 0x77,
    0x69, 0x74, 0x68, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x73, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23,
    0x02, 0x02, 0x04, 0x12, 0x04, 0xf2, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02,
    0x02, 0x05, 0x12, 0x04, 0xf2, 0x03, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x02,
    0x01, 0x12, 0x04, 0xf2, 0x03, 0x12, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x02, 0x03,
    0x12, 0x04, 0xf2, 0x03, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x03, 0x12, 0x04,
    0xf3, 0x03, 0x02, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x03, 0x04, 0x12, 0x04, 0xf3,
    0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x03, 0x06, 0x12, 0x04, 0xf3, 0x03,
    0x0b, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x03, 0x01, 0x12, 0x04, 0xf3, 0x03, 0x1c,
    0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x03, 0x03, 0x12, 0x04, 0xf3, 0x03, 0x2b, 0x2c,
    0x0a, 0x2e, 0x0a, 0x02, 0x04, 0x24, 0x12, 0x06, 0xf9, 0x03, 0x00, 0xfb, 0x03, 0x01, 0x1a, 0x20,
    0x2a, 0x0a, 0x20, 0x4c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b,
    0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x0a,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x24, 0x01, 0x12, 0x04, 0xf9, 0x03, 0x08, 0x20, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x24, 0x02, 0x00, 0x12, 0x04, 0xfa, 0x03, 0x02, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x24, 0x02, 0x00, 0x04, 0x12, 0x04, 0xfa, 0x03, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24,
    0x02, 0x00, 0x06, 0x12, 0x04, 0xfa, 0x03, 0x0b, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xfa, 0x03, 0x23, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xfa, 0x03, 0x2c, 0x2d, 0x0a, 0x41, 0x0a, 0x02, 0x04, 0x25, 0x12, 0x06, 0x80,
    0x04, 0x00, 0x84, 0x04, 0x01, 0x1a, 0x33, 0x2a, 0x0a, 0x20, 0x45, 0x64, 0x69, 0x74, 0x6c, 0x6f,
    0x67, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x77, 0x69,
    0x74, 0x68, 0x20, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x74, 0x72, 0x61,
    0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x25,
    0x01, 0x12, 0x04, 0x80, 0x04, 0x08, 0x1a, 0x0a, 0x37, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x00, 0x12,
    0x04, 0x81, 0x04, 0x02, 0x20, 0x22, 0x29, 0x20, 0x53, 0x74, 0x61, 0x72, 0x74, 0x69, 0x6e, 0x67,
    0x20, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x65, 0x64, 0x69, 0x74, 0x20,
    0x6c, 0x6f, 0x67, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x04, 0x12, 0x04, 0x81, 0x04, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x05, 0x12, 0x04, 0x81, 0x04, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x25, 0x02, 0x00, 0x01, 0x12, 0x04, 0x81, 0x04, 0x12, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x25, 0x02, 0x00, 0x03, 0x12, 0x04, 0x81, 0x04, 0x1e, 0x1f, 0x0a, 0x35, 0x0a, 0x04,
    0x04, 0x25, 0x02, 0x01, 0x12, 0x04, 0x82, 0x04, 0x02, 0x1e, 0x22, 0x27, 0x20, 0x45, 0x6e, 0x64,
    0x69, 0x6e, 0x67, 0x20, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x65, 0x64,
    0x69, 0x74, 0x20, 0x6c, 0x6f, 0x67, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x04, 0x12, 0x04, 0x82, 0x04,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x05, 0x12, 0x04, 0x82, 0x04, 0x0b,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x01, 0x12, 0x04, 0x82, 0x04, 0x12, 0x19,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x03, 0x12, 0x04, 0x82, 0x04, 0x1c, 0x1d, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x02, 0x12, 0x04, 0x83, 0x04, 0x02, 0x33, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x25, 0x02, 0x02, 0x04, 0x12, 0x04, 0x83, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x25, 0x02, 0x02, 0x05, 0x12, 0x04, 0x83, 0x04, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x25, 0x02, 0x02, 0x01, 0x12, 0x04, 0x83, 0x04, 0x10, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25,
    0x02, 0x02, 0x03, 0x12, 0x04, 0x83, 0x04, 0x1f, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02,
    0x02, 0x08, 0x12, 0x04, 0x83, 0x04, 0x21, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x02,
    0x07, 0x12, 0x04, 0x83, 0x04, 0x2c, 0x31, 0x0a, 0x48, 0x0a, 0x02, 0x04, 0x26, 0x12, 0x06, 0x89,
    0x04, 0x00, 0x8b, 0x04, 0x01, 0x1a, 0x3a, 0x2a, 0x0a, 0x20, 0x45, 0x6e, 0x75, 0x6d, 0x65, 0x72,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x65, 0x64, 0x69, 0x74, 0x6c, 0x6f, 0x67,
    0x73, 0x20, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x6f, 0x6e, 0x20, 0x61,
    0x20, 0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x6e, 0x6f, 0x64, 0x65,
    0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x26, 0x01, 0x12, 0x04, 0x89, 0x04, 0x08, 0x22, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x26, 0x02, 0x00, 0x12, 0x04, 0x8a, 0x04, 0x02, 0x27, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x26, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8a, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x26, 0x02, 0x00, 0x06, 0x12, 0x04, 0x8a, 0x04, 0x0b, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26,
    0x02, 0x00, 0x01, 0x12, 0x04, 0x8a, 0x04, 0x1e, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02,
    0x00, 0x03, 0x12, 0x04, 0x8a, 0x04, 0x25, 0x26, 0x0a, 0x4e, 0x0a, 0x02, 0x04, 0x27, 0x12, 0x06,
    0x90, 0x04, 0x00, 0x97, 0x04, 0x01, 0x1a, 0x40, 0x2a, 0x0a, 0x20, 0x4e, 0x61, 0x6d, 0x65, 0x73,
    0x70, 0x61, 0x63, 0x65, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x73, 0x20,
    0x6e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x20, 0x6f, 0x6e, 0x20, 0x61, 0x20, 0x6e,
    0x61, 0x6d, 0x65, 0x6e, 0x6f, 0x64, 0x65, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x27, 0x01, 0x12,
    0x04, 0x90, 0x04, 0x08, 0x1a, 0x0a, 0x47, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x00, 0x12, 0x04, 0x91,
    0x04, 0x02, 0x23, 0x22, 0x39, 0x20, 0x53, 0x6f, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x20, 0x72,
    0x65, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20,
    0x28, 0x65, 0x2e, 0x67, 0x2e, 0x20, 0x61, 0x6e, 0x20, 0x73, 0x76, 0x6e, 0x20, 0x6f, 0x72, 0x20,
    0x67, 0x69, 0x74, 0x20, 0x72, 0x65, 0x76, 0x69, 0x73, 0x69, 0x6f, 0x6e, 0x29, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x27, 0x02, 0x00, 0x04, 0x12, 0x04, 0x91, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x27, 0x02, 0x00, 0x05, 0x12, 0x04, 0x91, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x27, 0x02, 0x00, 0x01, 0x12, 0x04, 0x91, 0x04, 0x12, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x27, 0x02, 0x00, 0x03, 0x12, 0x04, 0x91, 0x04, 0x21, 0x22, 0x0a, 0x33, 0x0a, 0x04, 0x04, 0x27,
    0x02, 0x01, 0x12, 0x04, 0x92, 0x04, 0x02, 0x1d, 0x22, 0x25, 0x20, 0x52, 0x65, 0x74, 0x61, 0x69,
    0x6e, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x77, 0x61, 0x72, 0x64,
    0x20, 0x63, 0x6f, 0x6d, 0x70, 0x61, 0x74, 0x69, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x01, 0x04, 0x12, 0x04, 0x92, 0x04, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x27, 0x02, 0x01, 0x05, 0x12, 0x04, 0x92, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x27, 0x02, 0x01, 0x01, 0x12, 0x04, 0x92, 0x04, 0x12, 0x18, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x27, 0x02, 0x01, 0x03, 0x12, 0x04, 0x92, 0x04, 0x1b, 0x1c, 0x0a, 0x30, 0x0a, 0x04, 0x04,
    0x27, 0x02, 0x02, 0x12, 0x04, 0x93, 0x04, 0x02, 0x22, 0x22, 0x22, 0x20, 0x62, 0x6c, 0x6f, 0x63,
    0x6b, 0x20, 0x70, 0x6f, 0x6f, 0x6c, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x70, 0x61, 0x63, 0x65, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x27, 0x02, 0x02, 0x04, 0x12, 0x04, 0x93, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x27, 0x02, 0x02, 0x05, 0x12, 0x04, 0x93, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x27, 0x02, 0x02, 0x01, 0x12, 0x04, 0x93, 0x04, 0x12, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27,
    0x02, 0x02, 0x03, 0x12, 0x04, 0x93, 0x04, 0x20, 0x21, 0x0a, 0x20, 0x0a, 0x04, 0x04, 0x27, 0x02,
    0x03, 0x12, 0x04, 0x94, 0x04, 0x02, 0x2c, 0x22, 0x12, 0x20, 0x4e, 0x6f, 0x64, 0x65, 0x20, 0x69,
    0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x27, 0x02, 0x03, 0x04, 0x12, 0x04, 0x94, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27,
    0x02, 0x03, 0x06, 0x12, 0x04, 0x94, 0x04, 0x0b, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02,
    0x03, 0x01, 0x12, 0x04, 0x94, 0x04, 0x1c, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x03,
    0x03, 0x12, 0x04, 0x94, 0x04, 0x2a, 0x2b, 0x0a, 0x34, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x04, 0x12,
    0x04, 0x95, 0x04, 0x02, 0x26, 0x22, 0x26, 0x20, 0x53, 0x6f, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65,
    0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20,
    0x28, 0x65, 0x2e, 0x67, 0x2e, 0x20, 0x32, 0x2e, 0x30, 0x2e, 0x30, 0x29, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x27, 0x02, 0x04, 0x04, 0x12, 0x04, 0x95, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x27, 0x02, 0x04, 0x05, 0x12, 0x04, 0x95, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x27, 0x02, 0x04, 0x01, 0x12, 0x04, 0x95, 0x04, 0x12, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27,
    0x02, 0x04, 0x03, 0x12, 0x04, 0x95, 0x04, 0x24, 0x25, 0x0a, 0x1d, 0x0a, 0x04, 0x04, 0x27, 0x02,
    0x05, 0x12, 0x04, 0x96, 0x04, 0x02, 0x31, 0x22, 0x0f, 0x20, 0x66, 0x65, 0x61, 0x74, 0x75, 0x72,
    0x65, 0x20, 0x66, 0x6c, 0x61, 0x67, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x05,
    0x04, 0x12, 0x04, 0x96, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x05, 0x05,
    0x12, 0x04, 0x96, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x05, 0x01, 0x12,
    0x04, 0x96, 0x04, 0x12, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x05, 0x03, 0x12, 0x04,
    0x96, 0x04, 0x21, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x05, 0x08, 0x12, 0x04, 0x96,
    0x04, 0x23, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x05, 0x07, 0x12, 0x04, 0x96, 0x04,
    0x2e, 0x2f, 0x0a, 0x30, 0x0a, 0x02, 0x04, 0x28, 0x12, 0x06, 0x9c, 0x04, 0x00, 0xa0, 0x04, 0x01,
    0x1a, 0x22, 0x2a, 0x0a, 0x20, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x61, 0x63, 0x63, 0x65, 0x73,
    0x73, 0x20, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x28, 0x01, 0x12, 0x04, 0x9c, 0x04, 0x08,
    0x15, 0x0a, 0x1e, 0x0a, 0x04, 0x04, 0x28, 0x02, 0x00, 0x12, 0x04, 0x9d, 0x04, 0x02, 0x1c, 0x22,
    0x10, 0x20, 0x4b, 0x65, 0x79, 0x20, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x72,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9d, 0x04, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x00, 0x05, 0x12, 0x04, 0x9d, 0x04, 0x0b, 0x11, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9d, 0x04, 0x12, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x28, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9d, 0x04, 0x1a, 0x1b, 0x0a, 0x2b, 0x0a,
    0x04, 0x04, 0x28, 0x02, 0x01, 0x12, 0x04, 0x9e, 0x04, 0x02, 0x21, 0x22, 0x1d, 0x20, 0x45, 0x78,
    0x70, 0x69, 0x72, 0x79, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x6d, 0x69, 0x6c,
    0x6c, 0x69, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28,
    0x02, 0x01, 0x04, 0x12, 0x04, 0x9e, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02,
    0x01, 0x05, 0x12, 0x04, 0x9e, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x01,
    0x01, 0x12, 0x04, 0x9e, 0x04, 0x12, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x01, 0x03,
    0x12, 0x04, 0x9e, 0x04, 0x1f, 0x20, 0x0a, 0x1a, 0x0a, 0x04, 0x04, 0x28, 0x02, 0x02, 0x12, 0x04,
    0x9f, 0x04, 0x02, 0x1e, 0x22, 0x0c, 0x20, 0x4b, 0x65, 0x79, 0x20, 0x73, 0x65, 0x63, 0x72, 0x65,
    0x74, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x02, 0x04, 0x12, 0x04, 0x9f, 0x04, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x02, 0x05, 0x12, 0x04, 0x9f, 0x04, 0x0b, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x02, 0x01, 0x12, 0x04, 0x9f, 0x04, 0x11, 0x19, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x28, 0x02, 0x02, 0x03, 0x12, 0x04, 0x9f, 0x04, 0x1c, 0x1d, 0x0a, 0x44,
    0x0a, 0x02, 0x04, 0x29, 0x12, 0x06, 0xa5, 0x04, 0x00, 0xab, 0x04, 0x01, 0x1a, 0x36, 0x2a, 0x0a,
    0x20, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x61, 0x6e, 0x64,
    0x20, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x6b, 0x65,
    0x79, 0x73, 0x20, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x6e, 0x6f,
    0x64, 0x65, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x29, 0x01, 0x12, 0x04, 0xa5, 0x04, 0x08,
    0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x00, 0x12, 0x04, 0xa6, 0x04, 0x02, 0x28, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa6, 0x04, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x29, 0x02, 0x00, 0x05, 0x12, 0x04, 0xa6, 0x04, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x29, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa6, 0x04, 0x10, 0x23, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x29, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa6, 0x04, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x29, 0x02, 0x01, 0x12, 0x04, 0xa7, 0x04, 0x02, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xa7, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x01,
    0x05, 0x12, 0x04, 0xa7, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xa7, 0x04, 0x12, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xa7, 0x04, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x02, 0x12, 0x04, 0xa8,
    0x04, 0x02, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x02, 0x04, 0x12, 0x04, 0xa8, 0x04,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x02, 0x05, 0x12, 0x04, 0xa8, 0x04, 0x0b,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x02, 0x01, 0x12, 0x04, 0xa8, 0x04, 0x12, 0x1f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x02, 0x03, 0x12, 0x04, 0xa8, 0x04, 0x22, 0x23, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02, 0x03, 0x12, 0x04, 0xa9, 0x04, 0x02, 0x28, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x29, 0x02, 0x03, 0x04, 0x12, 0x04, 0xa9, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x29, 0x02, 0x03, 0x06, 0x12, 0x04, 0xa9, 0x04, 0x0b, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x29, 0x02, 0x03, 0x01, 0x12, 0x04, 0xa9, 0x04, 0x19, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29,
    0x02, 0x03, 0x03, 0x12, 0x04, 0xa9, 0x04, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x29, 0x02,
    0x04, 0x12, 0x04, 0xaa, 0x04, 0x02, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x04, 0x04,
    0x12, 0x04, 0xaa, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x04, 0x06, 0x12,
    0x04, 0xaa, 0x04, 0x0b, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x04, 0x01, 0x12, 0x04,
    0xaa, 0x04, 0x19, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x29, 0x02, 0x04, 0x03, 0x12, 0x04, 0xaa,
    0x04, 0x23, 0x24, 0x0a, 0x38, 0x0a, 0x02, 0x05, 0x04, 0x12, 0x06, 0xb0, 0x04, 0x00, 0xb6, 0x04,
    0x01, 0x1a, 0x2a, 0x2a, 0x0a, 0x20, 0x53, 0x74, 0x61, 0x74, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x61,
    0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x20, 0x61,
    0x74, 0x20, 0x61, 0x20, 0x64, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x0a, 0x0a, 0x0b, 0x0a,
    0x03, 0x05, 0x04, 0x01, 0x12, 0x04, 0xb0, 0x04, 0x05, 0x16, 0x0a, 0x3a, 0x0a, 0x04, 0x05, 0x04,
    0x02, 0x00, 0x12, 0x04, 0xb1, 0x04, 0x02, 0x10, 0x22, 0x2c, 0x20, 0x53, 0x74, 0x61, 0x74, 0x65,
    0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x20, 0x77, 0x68,
    0x65, 0x6e, 0x20, 0x69, 0x74, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x6d, 0x6f, 0x64,
    0x69, 0x66, 0x69, 0x65, 0x64, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xb1, 0x04, 0x02, 0x0b, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x00, 0x02, 0x12, 0x04,
    0xb1, 0x04, 0x0e, 0x0f, 0x0a, 0x39, 0x0a, 0x04, 0x05, 0x04, 0x02, 0x01, 0x12, 0x04, 0xb2, 0x04,
    0x02, 0x0a, 0x22, 0x2b, 0x20, 0x53, 0x74, 0x61, 0x74, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x72, 0x65,
    0x70, 0x6c, 0x69, 0x63, 0x61, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x69, 0x73, 0x20, 0x62, 0x65,
    0x69, 0x6e, 0x67, 0x20, 0x77, 0x72, 0x69, 0x74, 0x74, 0x65, 0x6e, 0x20, 0x74, 0x6f, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb2, 0x04, 0x02, 0x05, 0x0a, 0x0d,
    0x0a, 0x05, 0x05, 0x04, 0x02, 0x01, 0x02, 0x12, 0x04, 0xb2, 0x04, 0x08, 0x09, 0x0a, 0x40, 0x0a,
    0x04, 0x05, 0x04, 0x02, 0x02, 0x12, 0x04, 0xb3, 0x04, 0x02, 0x0a, 0x22, 0x32, 0x20, 0x53, 0x74,
    0x61, 0x74, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x20, 0x74,
    0x68, 0x61, 0x74, 0x20, 0x69, 0x73, 0x20, 0x77, 0x61, 0x69, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x74,
    0x6f, 0x20, 0x62, 0x65, 0x20, 0x72, 0x65, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x65, 0x64, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb3, 0x04, 0x02, 0x05, 0x0a, 0x0d,
    0x0a, 0x05, 0x05, 0x04, 0x02, 0x02, 0x02, 0x12, 0x04, 0xb3, 0x04, 0x08, 0x09, 0x0a, 0x37, 0x0a,
    0x04, 0x05, 0x04, 0x02, 0x03, 0x12, 0x04, 0xb4, 0x04, 0x02, 0x0a, 0x22, 0x29, 0x20, 0x53, 0x74,
    0x61, 0x74, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x20, 0x74,
    0x68, 0x61, 0x74, 0x20, 0x69, 0x73, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x20, 0x72, 0x65, 0x63,
    0x6f, 0x76, 0x65, 0x72, 0x79, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x03, 0x01, 0x12,
    0x04, 0xb4, 0x04, 0x02, 0x05, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x03, 0x02, 0x12, 0x04,
    0xb4, 0x04, 0x08, 0x09, 0x0a, 0x40, 0x0a, 0x04, 0x05, 0x04, 0x02, 0x04, 0x12, 0x04, 0xb5, 0x04,
    0x02, 0x10, 0x22, 0x32, 0x20, 0x53, 0x74, 0x61, 0x74, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x72, 0x65,
    0x70, 0x6c, 0x69, 0x63, 0x61, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x69, 0x73, 0x20, 0x63, 0x72,
    0x65, 0x61, 0x74, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x04, 0x01, 0x12,
    0x04, 0xb5, 0x04, 0x02, 0x0b, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x04, 0x02, 0x04, 0x02, 0x12, 0x04,
    0xb5, 0x04, 0x0e, 0x0f, 0x0a, 0x4b, 0x0a, 0x02, 0x04, 0x2a, 0x12, 0x06, 0xbb, 0x04, 0x00, 0xbf,
    0x04, 0x01, 0x1a, 0x3d, 0x2a, 0x0a, 0x20, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x74, 0x68, 0x61,
    0x74, 0x20, 0x6e, 0x65, 0x65, 0x64, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x72, 0x65,
    0x63, 0x6f, 0x76, 0x65, 0x72, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x61, 0x74, 0x20,
    0x61, 0x20, 0x67, 0x69, 0x76, 0x65, 0x6e, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2a, 0x01, 0x12, 0x04, 0xbb, 0x04, 0x08, 0x1c, 0x0a, 0x2a,
    0x0a, 0x04, 0x04, 0x2a, 0x02, 0x00, 0x12, 0x04, 0xbc, 0x04, 0x02, 0x22, 0x22, 0x1c, 0x20, 0x4e,
    0x65, 0x77, 0x20, 0x67, 0x65, 0x6e, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x20, 0x70, 0x6f, 0x73, 0x74,
    0x20, 0x72, 0x65, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xbc, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xbc, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xbc, 0x04, 0x12, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xbc, 0x04, 0x20, 0x21, 0x0a, 0x25, 0x0a, 0x04, 0x04, 0x2a, 0x02, 0x01, 0x12, 0x04,
    0xbd, 0x04, 0x02, 0x27, 0x22, 0x17, 0x20, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x74, 0x6f, 0x20,
    0x62, 0x65, 0x20, 0x72, 0x65, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x65, 0x64, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2a, 0x02, 0x01, 0x04, 0x12, 0x04, 0xbd, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2a, 0x02, 0x01, 0x06, 0x12, 0x04, 0xbd, 0x04, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2a, 0x02, 0x01, 0x01, 0x12, 0x04, 0xbd, 0x04, 0x1d, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a,
    0x02, 0x01, 0x03, 0x12, 0x04, 0xbd, 0x04, 0x25, 0x26, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x2a, 0x02,
    0x02, 0x12, 0x04, 0xbe, 0x04, 0x02, 0x28, 0x22, 0x23, 0x20, 0x4e, 0x65, 0x77, 0x20, 0x62, 0x6c,
    0x6f, 0x63, 0x6b, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x72, 0x65, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79,
    0x20, 0x28, 0x74, 0x72, 0x75, 0x6e, 0x63, 0x61, 0x74, 0x65, 0x29, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2a, 0x02, 0x02, 0x04, 0x12, 0x04, 0xbe, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2a, 0x02, 0x02, 0x06, 0x12, 0x04, 0xbe, 0x04, 0x0b, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a,
    0x02, 0x02, 0x01, 0x12, 0x04, 0xbe, 0x04, 0x16, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02,
    0x02, 0x03, 0x12, 0x04, 0xbe, 0x04, 0x26, 0x27, 0x0a, 0x1e, 0x0a, 0x02, 0x04, 0x2b, 0x12, 0x06,
    0xc4, 0x04, 0x00, 0xc5, 0x04, 0x01, 0x1a, 0x10, 0x2a, 0x0a, 0x20, 0x76, 0x6f, 0x69, 0x64, 0x20,
    0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2b, 0x01, 0x12,
    0x04, 0xc4, 0x04, 0x08, 0x1b, 0x0a, 0x31, 0x0a, 0x02, 0x04, 0x2c, 0x12, 0x06, 0xca, 0x04, 0x00,
    0xcc, 0x04, 0x01, 0x1a, 0x23, 0x2a, 0x0a, 0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20,
    0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x6e, 0x61,
    0x6d, 0x65, 0x6e, 0x6f, 0x64, 0x65, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2c, 0x01, 0x12,
    0x04, 0xca, 0x04, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2c, 0x02, 0x00, 0x12, 0x04, 0xcb,
    0x04, 0x02, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x00, 0x04, 0x12, 0x04, 0xcb, 0x04,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x00, 0x06, 0x12, 0x04, 0xcb, 0x04, 0x0b,
    0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x00, 0x01, 0x12, 0x04, 0xcb, 0x04, 0x1e, 0x22,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2c, 0x02, 0x00, 0x03, 0x12, 0x04, 0xcb, 0x04, 0x25, 0x26, 0x0a,
    0x4f, 0x0a, 0x02, 0x04, 0x2d, 0x12, 0x06, 0xd2, 0x04, 0x00, 0xda, 0x04, 0x01, 0x1a, 0x41, 0x2a,
    0x0a, 0x20, 0x49, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x72, 0x65,
    0x6c, 0x61, 0x74, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x20, 0x73, 0x6e, 0x61, 0x70, 0x73,
    0x68, 0x6f, 0x74, 0x0a, 0x20, 0x54, 0x4f, 0x44, 0x4f, 0x3a, 0x20, 0x61, 0x64, 0x64, 0x20, 0x6d,
    0x6f, 0x72, 0x65, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x0a,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2d, 0x01, 0x12, 0x04, 0xd2, 0x04, 0x08, 0x19, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x2d, 0x02, 0x00, 0x12, 0x04, 0xd3, 0x04, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2d, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd3, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d,
    0x02, 0x00, 0x05, 0x12, 0x04, 0xd3, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xd3, 0x04, 0x12, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xd3, 0x04, 0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2d, 0x02, 0x01, 0x12,
    0x04, 0xd4, 0x04, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x01, 0x04, 0x12, 0x04,
    0xd4, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x01, 0x05, 0x12, 0x04, 0xd4,
    0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd4, 0x04,
    0x12, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd4, 0x04, 0x21,
    0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2d, 0x02, 0x02, 0x12, 0x04, 0xd5, 0x04, 0x02, 0x2c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x02, 0x04, 0x12, 0x04, 0xd5, 0x04, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2d, 0x02, 0x02, 0x06, 0x12, 0x04, 0xd5, 0x04, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2d, 0x02, 0x02, 0x01, 0x12, 0x04, 0xd5, 0x04, 0x1d, 0x27, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2d, 0x02, 0x02, 0x03, 0x12, 0x04, 0xd5, 0x04, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x2d, 0x02, 0x03, 0x12, 0x04, 0xd6, 0x04, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02,
    0x03, 0x04, 0x12, 0x04, 0xd6, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x03,
    0x05, 0x12, 0x04, 0xd6, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x03, 0x01,
    0x12, 0x04, 0xd6, 0x04, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x03, 0x03, 0x12,
    0x04, 0xd6, 0x04, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2d, 0x02, 0x04, 0x12, 0x04, 0xd7,
    0x04, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x04, 0x04, 0x12, 0x04, 0xd7, 0x04,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x04, 0x05, 0x12, 0x04, 0xd7, 0x04, 0x0b,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x04, 0x01, 0x12, 0x04, 0xd7, 0x04, 0x12, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2d, 0x02, 0x04, 0x03, 0x12, 0x04, 0xd7, 0x04, 0x1a, 0x1b, 0x0a,
    0x2d, 0x0a, 0x04, 0x04, 0x2d, 0x02, 0x05, 0x12, 0x04, 0xd8, 0x04, 0x02, 0x21, 0x22, 0x1f, 0x20,
    0x54, 0x4f, 0x44, 0x4f, 0x3a, 0x20, 0x64, 0x6f, 0x20, 0x77, 0x65, 0x20, 0x6e, 0x65, 0x65, 0x64,
    0x20, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x3f, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2d, 0x02, 0x05, 0x04, 0x12, 0x04, 0xd8, 0x04, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2d, 0x02, 0x05, 0x05, 0x12, 0x04, 0xd8, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2d, 0x02, 0x05, 0x01, 0x12, 0x04, 0xd8, 0x04, 0x12, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x2d, 0x02, 0x05, 0x03, 0x12, 0x04, 0xd8, 0x04, 0x1f, 0x20, 0x0a, 0x28, 0x0a, 0x02, 0x04, 0x2e,
    0x12, 0x06, 0xdf, 0x04, 0x00, 0xe1, 0x04, 0x01, 0x1a, 0x1a, 0x2a, 0x0a, 0x20, 0x52, 0x6f, 0x6c,
    0x6c, 0x69, 0x6e, 0x67, 0x20, 0x75, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x20, 0x73, 0x74, 0x61,
    0x74, 0x75, 0x73, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x2e, 0x01, 0x12, 0x04, 0xdf, 0x04, 0x08,
    0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2e, 0x02, 0x00, 0x12, 0x04, 0xe0, 0x04, 0x02, 0x22, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2e, 0x02, 0x00, 0x04, 0x12, 0x04, 0xe0, 0x04, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2e, 0x02, 0x00, 0x05, 0x12, 0x04, 0xe0, 0x04, 0x0b, 0x11, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2e, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe0, 0x04, 0x12, 0x1d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x2e, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe0, 0x04, 0x20, 0x21,
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
