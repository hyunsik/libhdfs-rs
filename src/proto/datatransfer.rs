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
use super::hdfs::ExtendedBlockProto;
use super::hdfs::DatanodeIDProto;
use super::hdfs::DatanodeLocalInfoProto;
use super::hdfs::DatanodeInfosProto;
use super::hdfs::DatanodeInfoProto;
use super::hdfs::DatanodeStorageProto;
use super::hdfs::StorageReportProto;
use super::hdfs::ContentSummaryProto;
use super::hdfs::StorageTypeQuotaInfosProto;
use super::hdfs::StorageTypeQuotaInfoProto;
use super::hdfs::CorruptFileBlocksProto;
use super::hdfs::FsPermissionProto;
use super::hdfs::StorageTypesProto;
use super::hdfs::BlockStoragePolicyProto;
use super::hdfs::StorageUuidsProto;
use super::hdfs::LocatedBlockProto;
use super::hdfs::DataEncryptionKeyProto;
use super::hdfs::FileEncryptionInfoProto;
use super::hdfs::PerFileEncryptionInfoProto;
use super::hdfs::ZoneEncryptionInfoProto;
use super::hdfs::CipherOptionProto;
use super::hdfs::LocatedBlocksProto;
use super::hdfs::HdfsFileStatusProto;
use super::hdfs::FsServerDefaultsProto;
use super::hdfs::DirectoryListingProto;
use super::hdfs::SnapshottableDirectoryStatusProto;
use super::hdfs::SnapshottableDirectoryListingProto;
use super::hdfs::SnapshotDiffReportEntryProto;
use super::hdfs::SnapshotDiffReportProto;
use super::hdfs::StorageInfoProto;
use super::hdfs::NamenodeRegistrationProto;
use super::hdfs::CheckpointSignatureProto;
use super::hdfs::NamenodeCommandProto;
use super::hdfs::CheckpointCommandProto;
use super::hdfs::BlockProto;
use super::hdfs::BlockWithLocationsProto;
use super::hdfs::BlocksWithLocationsProto;
use super::hdfs::RemoteEditLogProto;
use super::hdfs::RemoteEditLogManifestProto;
use super::hdfs::NamespaceInfoProto;
use super::hdfs::BlockKeyProto;
use super::hdfs::ExportedBlockKeysProto;
use super::hdfs::RecoveringBlockProto;
use super::hdfs::VersionRequestProto;
use super::hdfs::VersionResponseProto;
use super::hdfs::SnapshotInfoProto;
use super::hdfs::RollingUpgradeStatusProto;
use super::hdfs::StorageTypeProto;
use super::hdfs::CipherSuiteProto;
use super::hdfs::CryptoProtocolVersionProto;
use super::hdfs::ChecksumTypeProto;
use super::hdfs::ReplicaStateProto;

#[derive(Clone,Default)]
pub struct DataTransferEncryptorMessageProto {
    // message fields
    status: ::std::option::Option<DataTransferEncryptorMessageProto_DataTransferEncryptorStatus>,
    payload: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    message: ::protobuf::SingularField<::std::string::String>,
    cipherOption: ::protobuf::RepeatedField<CipherOptionProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl DataTransferEncryptorMessageProto {
    pub fn new() -> DataTransferEncryptorMessageProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataTransferEncryptorMessageProto {
        static mut instance: ::protobuf::lazy::Lazy<DataTransferEncryptorMessageProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataTransferEncryptorMessageProto,
        };
        unsafe {
            instance.get(|| {
                DataTransferEncryptorMessageProto {
                    status: ::std::option::Option::None,
                    payload: ::protobuf::SingularField::none(),
                    message: ::protobuf::SingularField::none(),
                    cipherOption: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.DataTransferEncryptorMessageProto.DataTransferEncryptorStatus status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: DataTransferEncryptorMessageProto_DataTransferEncryptorStatus) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status<'a>(&self) -> DataTransferEncryptorMessageProto_DataTransferEncryptorStatus {
        self.status.unwrap_or(DataTransferEncryptorMessageProto_DataTransferEncryptorStatus::SUCCESS)
    }

    // optional bytes payload = 2;

    pub fn clear_payload(&mut self) {
        self.payload.clear();
    }

    pub fn has_payload(&self) -> bool {
        self.payload.is_some()
    }

    // Param is passed by value, moved
    pub fn set_payload(&mut self, v: ::std::vec::Vec<u8>) {
        self.payload = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_payload<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.payload.is_none() {
            self.payload.set_default();
        };
        self.payload.as_mut().unwrap()
    }

    // Take field
    pub fn take_payload(&mut self) -> ::std::vec::Vec<u8> {
        self.payload.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_payload<'a>(&'a self) -> &'a [u8] {
        match self.payload.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional string message = 3;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.message.is_none() {
            self.message.set_default();
        };
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        self.message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message<'a>(&'a self) -> &'a str {
        match self.message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated .hadoop.hdfs.CipherOptionProto cipherOption = 4;

    pub fn clear_cipherOption(&mut self) {
        self.cipherOption.clear();
    }

    // Param is passed by value, moved
    pub fn set_cipherOption(&mut self, v: ::protobuf::RepeatedField<CipherOptionProto>) {
        self.cipherOption = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cipherOption<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<CipherOptionProto> {
        &mut self.cipherOption
    }

    // Take field
    pub fn take_cipherOption(&mut self) -> ::protobuf::RepeatedField<CipherOptionProto> {
        ::std::mem::replace(&mut self.cipherOption, ::protobuf::RepeatedField::new())
    }

    pub fn get_cipherOption<'a>(&'a self) -> &'a [CipherOptionProto] {
        &self.cipherOption
    }
}

impl ::protobuf::Message for DataTransferEncryptorMessageProto {
    fn is_initialized(&self) -> bool {
        if self.status.is_none() {
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
                    self.status = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.payload.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.message.set_default();
                    try!(is.read_string_into(tmp))
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.cipherOption));
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
        for value in self.status.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.payload.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in self.message.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in self.cipherOption.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.payload.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.message.as_ref() {
            try!(os.write_string(3, &v));
        };
        for v in self.cipherOption.iter() {
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
        ::std::any::TypeId::of::<DataTransferEncryptorMessageProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DataTransferEncryptorMessageProto {
    fn new() -> DataTransferEncryptorMessageProto {
        DataTransferEncryptorMessageProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataTransferEncryptorMessageProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "status",
                    DataTransferEncryptorMessageProto::has_status,
                    DataTransferEncryptorMessageProto::get_status,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "payload",
                    DataTransferEncryptorMessageProto::has_payload,
                    DataTransferEncryptorMessageProto::get_payload,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "message",
                    DataTransferEncryptorMessageProto::has_message,
                    DataTransferEncryptorMessageProto::get_message,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "cipherOption",
                    DataTransferEncryptorMessageProto::get_cipherOption,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DataTransferEncryptorMessageProto>(
                    "DataTransferEncryptorMessageProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataTransferEncryptorMessageProto {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_payload();
        self.clear_message();
        self.clear_cipherOption();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DataTransferEncryptorMessageProto {
    fn eq(&self, other: &DataTransferEncryptorMessageProto) -> bool {
        self.status == other.status &&
        self.payload == other.payload &&
        self.message == other.message &&
        self.cipherOption == other.cipherOption &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DataTransferEncryptorMessageProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DataTransferEncryptorMessageProto_DataTransferEncryptorStatus {
    SUCCESS = 0,
    ERROR_UNKNOWN_KEY = 1,
    ERROR = 2,
}

impl ::protobuf::ProtobufEnum for DataTransferEncryptorMessageProto_DataTransferEncryptorStatus {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DataTransferEncryptorMessageProto_DataTransferEncryptorStatus> {
        match value {
            0 => ::std::option::Option::Some(DataTransferEncryptorMessageProto_DataTransferEncryptorStatus::SUCCESS),
            1 => ::std::option::Option::Some(DataTransferEncryptorMessageProto_DataTransferEncryptorStatus::ERROR_UNKNOWN_KEY),
            2 => ::std::option::Option::Some(DataTransferEncryptorMessageProto_DataTransferEncryptorStatus::ERROR),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<DataTransferEncryptorMessageProto_DataTransferEncryptorStatus>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DataTransferEncryptorMessageProto_DataTransferEncryptorStatus", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DataTransferEncryptorMessageProto_DataTransferEncryptorStatus {
}

#[derive(Clone,Default)]
pub struct BaseHeaderProto {
    // message fields
    block: ::protobuf::SingularPtrField<ExtendedBlockProto>,
    token: ::protobuf::SingularPtrField<TokenProto>,
    traceInfo: ::protobuf::SingularPtrField<DataTransferTraceInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl BaseHeaderProto {
    pub fn new() -> BaseHeaderProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BaseHeaderProto {
        static mut instance: ::protobuf::lazy::Lazy<BaseHeaderProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BaseHeaderProto,
        };
        unsafe {
            instance.get(|| {
                BaseHeaderProto {
                    block: ::protobuf::SingularPtrField::none(),
                    token: ::protobuf::SingularPtrField::none(),
                    traceInfo: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.ExtendedBlockProto block = 1;

    pub fn clear_block(&mut self) {
        self.block.clear();
    }

    pub fn has_block(&self) -> bool {
        self.block.is_some()
    }

    // Param is passed by value, moved
    pub fn set_block(&mut self, v: ExtendedBlockProto) {
        self.block = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block<'a>(&'a mut self) -> &'a mut ExtendedBlockProto {
        if self.block.is_none() {
            self.block.set_default();
        };
        self.block.as_mut().unwrap()
    }

    // Take field
    pub fn take_block(&mut self) -> ExtendedBlockProto {
        self.block.take().unwrap_or_else(|| ExtendedBlockProto::new())
    }

    pub fn get_block<'a>(&'a self) -> &'a ExtendedBlockProto {
        self.block.as_ref().unwrap_or_else(|| ExtendedBlockProto::default_instance())
    }

    // optional .hadoop.common.TokenProto token = 2;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: TokenProto) {
        self.token = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token<'a>(&'a mut self) -> &'a mut TokenProto {
        if self.token.is_none() {
            self.token.set_default();
        };
        self.token.as_mut().unwrap()
    }

    // Take field
    pub fn take_token(&mut self) -> TokenProto {
        self.token.take().unwrap_or_else(|| TokenProto::new())
    }

    pub fn get_token<'a>(&'a self) -> &'a TokenProto {
        self.token.as_ref().unwrap_or_else(|| TokenProto::default_instance())
    }

    // optional .hadoop.hdfs.DataTransferTraceInfoProto traceInfo = 3;

    pub fn clear_traceInfo(&mut self) {
        self.traceInfo.clear();
    }

    pub fn has_traceInfo(&self) -> bool {
        self.traceInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_traceInfo(&mut self, v: DataTransferTraceInfoProto) {
        self.traceInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_traceInfo<'a>(&'a mut self) -> &'a mut DataTransferTraceInfoProto {
        if self.traceInfo.is_none() {
            self.traceInfo.set_default();
        };
        self.traceInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_traceInfo(&mut self) -> DataTransferTraceInfoProto {
        self.traceInfo.take().unwrap_or_else(|| DataTransferTraceInfoProto::new())
    }

    pub fn get_traceInfo<'a>(&'a self) -> &'a DataTransferTraceInfoProto {
        self.traceInfo.as_ref().unwrap_or_else(|| DataTransferTraceInfoProto::default_instance())
    }
}

impl ::protobuf::Message for BaseHeaderProto {
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
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.token.set_default();
                    try!(is.merge_message(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.traceInfo.set_default();
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
        for value in self.block.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.token.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.traceInfo.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
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
        if let Some(v) = self.token.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.traceInfo.as_ref() {
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
        ::std::any::TypeId::of::<BaseHeaderProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BaseHeaderProto {
    fn new() -> BaseHeaderProto {
        BaseHeaderProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<BaseHeaderProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "block",
                    BaseHeaderProto::has_block,
                    BaseHeaderProto::get_block,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "token",
                    BaseHeaderProto::has_token,
                    BaseHeaderProto::get_token,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "traceInfo",
                    BaseHeaderProto::has_traceInfo,
                    BaseHeaderProto::get_traceInfo,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BaseHeaderProto>(
                    "BaseHeaderProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BaseHeaderProto {
    fn clear(&mut self) {
        self.clear_block();
        self.clear_token();
        self.clear_traceInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BaseHeaderProto {
    fn eq(&self, other: &BaseHeaderProto) -> bool {
        self.block == other.block &&
        self.token == other.token &&
        self.traceInfo == other.traceInfo &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BaseHeaderProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DataTransferTraceInfoProto {
    // message fields
    traceId: ::std::option::Option<u64>,
    parentId: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl DataTransferTraceInfoProto {
    pub fn new() -> DataTransferTraceInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DataTransferTraceInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<DataTransferTraceInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DataTransferTraceInfoProto,
        };
        unsafe {
            instance.get(|| {
                DataTransferTraceInfoProto {
                    traceId: ::std::option::Option::None,
                    parentId: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint64 traceId = 1;

    pub fn clear_traceId(&mut self) {
        self.traceId = ::std::option::Option::None;
    }

    pub fn has_traceId(&self) -> bool {
        self.traceId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_traceId(&mut self, v: u64) {
        self.traceId = ::std::option::Option::Some(v);
    }

    pub fn get_traceId<'a>(&self) -> u64 {
        self.traceId.unwrap_or(0)
    }

    // required uint64 parentId = 2;

    pub fn clear_parentId(&mut self) {
        self.parentId = ::std::option::Option::None;
    }

    pub fn has_parentId(&self) -> bool {
        self.parentId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parentId(&mut self, v: u64) {
        self.parentId = ::std::option::Option::Some(v);
    }

    pub fn get_parentId<'a>(&self) -> u64 {
        self.parentId.unwrap_or(0)
    }
}

impl ::protobuf::Message for DataTransferTraceInfoProto {
    fn is_initialized(&self) -> bool {
        if self.traceId.is_none() {
            return false;
        };
        if self.parentId.is_none() {
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
                    self.traceId = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.parentId = ::std::option::Option::Some(tmp);
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
        for value in self.traceId.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.parentId.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.traceId {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.parentId {
            try!(os.write_uint64(2, v));
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
        ::std::any::TypeId::of::<DataTransferTraceInfoProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DataTransferTraceInfoProto {
    fn new() -> DataTransferTraceInfoProto {
        DataTransferTraceInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DataTransferTraceInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "traceId",
                    DataTransferTraceInfoProto::has_traceId,
                    DataTransferTraceInfoProto::get_traceId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "parentId",
                    DataTransferTraceInfoProto::has_parentId,
                    DataTransferTraceInfoProto::get_parentId,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DataTransferTraceInfoProto>(
                    "DataTransferTraceInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DataTransferTraceInfoProto {
    fn clear(&mut self) {
        self.clear_traceId();
        self.clear_parentId();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DataTransferTraceInfoProto {
    fn eq(&self, other: &DataTransferTraceInfoProto) -> bool {
        self.traceId == other.traceId &&
        self.parentId == other.parentId &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DataTransferTraceInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ClientOperationHeaderProto {
    // message fields
    baseHeader: ::protobuf::SingularPtrField<BaseHeaderProto>,
    clientName: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ClientOperationHeaderProto {
    pub fn new() -> ClientOperationHeaderProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientOperationHeaderProto {
        static mut instance: ::protobuf::lazy::Lazy<ClientOperationHeaderProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientOperationHeaderProto,
        };
        unsafe {
            instance.get(|| {
                ClientOperationHeaderProto {
                    baseHeader: ::protobuf::SingularPtrField::none(),
                    clientName: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.BaseHeaderProto baseHeader = 1;

    pub fn clear_baseHeader(&mut self) {
        self.baseHeader.clear();
    }

    pub fn has_baseHeader(&self) -> bool {
        self.baseHeader.is_some()
    }

    // Param is passed by value, moved
    pub fn set_baseHeader(&mut self, v: BaseHeaderProto) {
        self.baseHeader = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_baseHeader<'a>(&'a mut self) -> &'a mut BaseHeaderProto {
        if self.baseHeader.is_none() {
            self.baseHeader.set_default();
        };
        self.baseHeader.as_mut().unwrap()
    }

    // Take field
    pub fn take_baseHeader(&mut self) -> BaseHeaderProto {
        self.baseHeader.take().unwrap_or_else(|| BaseHeaderProto::new())
    }

    pub fn get_baseHeader<'a>(&'a self) -> &'a BaseHeaderProto {
        self.baseHeader.as_ref().unwrap_or_else(|| BaseHeaderProto::default_instance())
    }

    // required string clientName = 2;

    pub fn clear_clientName(&mut self) {
        self.clientName.clear();
    }

    pub fn has_clientName(&self) -> bool {
        self.clientName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clientName(&mut self, v: ::std::string::String) {
        self.clientName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clientName<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.clientName.is_none() {
            self.clientName.set_default();
        };
        self.clientName.as_mut().unwrap()
    }

    // Take field
    pub fn take_clientName(&mut self) -> ::std::string::String {
        self.clientName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_clientName<'a>(&'a self) -> &'a str {
        match self.clientName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for ClientOperationHeaderProto {
    fn is_initialized(&self) -> bool {
        if self.baseHeader.is_none() {
            return false;
        };
        if self.clientName.is_none() {
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
                    let tmp = self.baseHeader.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.clientName.set_default();
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
        for value in self.baseHeader.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.clientName.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.baseHeader.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.clientName.as_ref() {
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
        ::std::any::TypeId::of::<ClientOperationHeaderProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ClientOperationHeaderProto {
    fn new() -> ClientOperationHeaderProto {
        ClientOperationHeaderProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientOperationHeaderProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "baseHeader",
                    ClientOperationHeaderProto::has_baseHeader,
                    ClientOperationHeaderProto::get_baseHeader,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "clientName",
                    ClientOperationHeaderProto::has_clientName,
                    ClientOperationHeaderProto::get_clientName,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClientOperationHeaderProto>(
                    "ClientOperationHeaderProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientOperationHeaderProto {
    fn clear(&mut self) {
        self.clear_baseHeader();
        self.clear_clientName();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ClientOperationHeaderProto {
    fn eq(&self, other: &ClientOperationHeaderProto) -> bool {
        self.baseHeader == other.baseHeader &&
        self.clientName == other.clientName &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ClientOperationHeaderProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CachingStrategyProto {
    // message fields
    dropBehind: ::std::option::Option<bool>,
    readahead: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CachingStrategyProto {
    pub fn new() -> CachingStrategyProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CachingStrategyProto {
        static mut instance: ::protobuf::lazy::Lazy<CachingStrategyProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CachingStrategyProto,
        };
        unsafe {
            instance.get(|| {
                CachingStrategyProto {
                    dropBehind: ::std::option::Option::None,
                    readahead: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool dropBehind = 1;

    pub fn clear_dropBehind(&mut self) {
        self.dropBehind = ::std::option::Option::None;
    }

    pub fn has_dropBehind(&self) -> bool {
        self.dropBehind.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dropBehind(&mut self, v: bool) {
        self.dropBehind = ::std::option::Option::Some(v);
    }

    pub fn get_dropBehind<'a>(&self) -> bool {
        self.dropBehind.unwrap_or(false)
    }

    // optional int64 readahead = 2;

    pub fn clear_readahead(&mut self) {
        self.readahead = ::std::option::Option::None;
    }

    pub fn has_readahead(&self) -> bool {
        self.readahead.is_some()
    }

    // Param is passed by value, moved
    pub fn set_readahead(&mut self, v: i64) {
        self.readahead = ::std::option::Option::Some(v);
    }

    pub fn get_readahead<'a>(&self) -> i64 {
        self.readahead.unwrap_or(0)
    }
}

impl ::protobuf::Message for CachingStrategyProto {
    fn is_initialized(&self) -> bool {
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
                    self.dropBehind = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.readahead = ::std::option::Option::Some(tmp);
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
        if self.dropBehind.is_some() {
            my_size += 2;
        };
        for value in self.readahead.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.dropBehind {
            try!(os.write_bool(1, v));
        };
        if let Some(v) = self.readahead {
            try!(os.write_int64(2, v));
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
        ::std::any::TypeId::of::<CachingStrategyProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CachingStrategyProto {
    fn new() -> CachingStrategyProto {
        CachingStrategyProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CachingStrategyProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "dropBehind",
                    CachingStrategyProto::has_dropBehind,
                    CachingStrategyProto::get_dropBehind,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "readahead",
                    CachingStrategyProto::has_readahead,
                    CachingStrategyProto::get_readahead,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CachingStrategyProto>(
                    "CachingStrategyProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CachingStrategyProto {
    fn clear(&mut self) {
        self.clear_dropBehind();
        self.clear_readahead();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CachingStrategyProto {
    fn eq(&self, other: &CachingStrategyProto) -> bool {
        self.dropBehind == other.dropBehind &&
        self.readahead == other.readahead &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CachingStrategyProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct OpReadBlockProto {
    // message fields
    header: ::protobuf::SingularPtrField<ClientOperationHeaderProto>,
    offset: ::std::option::Option<u64>,
    len: ::std::option::Option<u64>,
    sendChecksums: ::std::option::Option<bool>,
    cachingStrategy: ::protobuf::SingularPtrField<CachingStrategyProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl OpReadBlockProto {
    pub fn new() -> OpReadBlockProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpReadBlockProto {
        static mut instance: ::protobuf::lazy::Lazy<OpReadBlockProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpReadBlockProto,
        };
        unsafe {
            instance.get(|| {
                OpReadBlockProto {
                    header: ::protobuf::SingularPtrField::none(),
                    offset: ::std::option::Option::None,
                    len: ::std::option::Option::None,
                    sendChecksums: ::std::option::Option::None,
                    cachingStrategy: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.ClientOperationHeaderProto header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: ClientOperationHeaderProto) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header<'a>(&'a mut self) -> &'a mut ClientOperationHeaderProto {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ClientOperationHeaderProto {
        self.header.take().unwrap_or_else(|| ClientOperationHeaderProto::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a ClientOperationHeaderProto {
        self.header.as_ref().unwrap_or_else(|| ClientOperationHeaderProto::default_instance())
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

    // required uint64 len = 3;

    pub fn clear_len(&mut self) {
        self.len = ::std::option::Option::None;
    }

    pub fn has_len(&self) -> bool {
        self.len.is_some()
    }

    // Param is passed by value, moved
    pub fn set_len(&mut self, v: u64) {
        self.len = ::std::option::Option::Some(v);
    }

    pub fn get_len<'a>(&self) -> u64 {
        self.len.unwrap_or(0)
    }

    // optional bool sendChecksums = 4;

    pub fn clear_sendChecksums(&mut self) {
        self.sendChecksums = ::std::option::Option::None;
    }

    pub fn has_sendChecksums(&self) -> bool {
        self.sendChecksums.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sendChecksums(&mut self, v: bool) {
        self.sendChecksums = ::std::option::Option::Some(v);
    }

    pub fn get_sendChecksums<'a>(&self) -> bool {
        self.sendChecksums.unwrap_or(true)
    }

    // optional .hadoop.hdfs.CachingStrategyProto cachingStrategy = 5;

    pub fn clear_cachingStrategy(&mut self) {
        self.cachingStrategy.clear();
    }

    pub fn has_cachingStrategy(&self) -> bool {
        self.cachingStrategy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cachingStrategy(&mut self, v: CachingStrategyProto) {
        self.cachingStrategy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cachingStrategy<'a>(&'a mut self) -> &'a mut CachingStrategyProto {
        if self.cachingStrategy.is_none() {
            self.cachingStrategy.set_default();
        };
        self.cachingStrategy.as_mut().unwrap()
    }

    // Take field
    pub fn take_cachingStrategy(&mut self) -> CachingStrategyProto {
        self.cachingStrategy.take().unwrap_or_else(|| CachingStrategyProto::new())
    }

    pub fn get_cachingStrategy<'a>(&'a self) -> &'a CachingStrategyProto {
        self.cachingStrategy.as_ref().unwrap_or_else(|| CachingStrategyProto::default_instance())
    }
}

impl ::protobuf::Message for OpReadBlockProto {
    fn is_initialized(&self) -> bool {
        if self.header.is_none() {
            return false;
        };
        if self.offset.is_none() {
            return false;
        };
        if self.len.is_none() {
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
                    let tmp = self.header.set_default();
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.len = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.sendChecksums = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.cachingStrategy.set_default();
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.offset.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.len.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.sendChecksums.is_some() {
            my_size += 2;
        };
        for value in self.cachingStrategy.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.offset {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.len {
            try!(os.write_uint64(3, v));
        };
        if let Some(v) = self.sendChecksums {
            try!(os.write_bool(4, v));
        };
        if let Some(v) = self.cachingStrategy.as_ref() {
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
        ::std::any::TypeId::of::<OpReadBlockProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OpReadBlockProto {
    fn new() -> OpReadBlockProto {
        OpReadBlockProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpReadBlockProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    OpReadBlockProto::has_header,
                    OpReadBlockProto::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "offset",
                    OpReadBlockProto::has_offset,
                    OpReadBlockProto::get_offset,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "len",
                    OpReadBlockProto::has_len,
                    OpReadBlockProto::get_len,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "sendChecksums",
                    OpReadBlockProto::has_sendChecksums,
                    OpReadBlockProto::get_sendChecksums,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cachingStrategy",
                    OpReadBlockProto::has_cachingStrategy,
                    OpReadBlockProto::get_cachingStrategy,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpReadBlockProto>(
                    "OpReadBlockProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpReadBlockProto {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_offset();
        self.clear_len();
        self.clear_sendChecksums();
        self.clear_cachingStrategy();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for OpReadBlockProto {
    fn eq(&self, other: &OpReadBlockProto) -> bool {
        self.header == other.header &&
        self.offset == other.offset &&
        self.len == other.len &&
        self.sendChecksums == other.sendChecksums &&
        self.cachingStrategy == other.cachingStrategy &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for OpReadBlockProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ChecksumProto {
    // message fields
    field_type: ::std::option::Option<ChecksumTypeProto>,
    bytesPerChecksum: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ChecksumProto {
    pub fn new() -> ChecksumProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ChecksumProto {
        static mut instance: ::protobuf::lazy::Lazy<ChecksumProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChecksumProto,
        };
        unsafe {
            instance.get(|| {
                ChecksumProto {
                    field_type: ::std::option::Option::None,
                    bytesPerChecksum: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.ChecksumTypeProto type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ChecksumTypeProto) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type<'a>(&self) -> ChecksumTypeProto {
        self.field_type.unwrap_or(ChecksumTypeProto::CHECKSUM_NULL)
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
}

impl ::protobuf::Message for ChecksumProto {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        };
        if self.bytesPerChecksum.is_none() {
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
                    let tmp = try!(is.read_uint32());
                    self.bytesPerChecksum = ::std::option::Option::Some(tmp);
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
        for value in self.bytesPerChecksum.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.bytesPerChecksum {
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
        ::std::any::TypeId::of::<ChecksumProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ChecksumProto {
    fn new() -> ChecksumProto {
        ChecksumProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ChecksumProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "field_type",
                    ChecksumProto::has_field_type,
                    ChecksumProto::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "bytesPerChecksum",
                    ChecksumProto::has_bytesPerChecksum,
                    ChecksumProto::get_bytesPerChecksum,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ChecksumProto>(
                    "ChecksumProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ChecksumProto {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_bytesPerChecksum();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ChecksumProto {
    fn eq(&self, other: &ChecksumProto) -> bool {
        self.field_type == other.field_type &&
        self.bytesPerChecksum == other.bytesPerChecksum &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ChecksumProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct OpWriteBlockProto {
    // message fields
    header: ::protobuf::SingularPtrField<ClientOperationHeaderProto>,
    targets: ::protobuf::RepeatedField<DatanodeInfoProto>,
    source: ::protobuf::SingularPtrField<DatanodeInfoProto>,
    stage: ::std::option::Option<OpWriteBlockProto_BlockConstructionStage>,
    pipelineSize: ::std::option::Option<u32>,
    minBytesRcvd: ::std::option::Option<u64>,
    maxBytesRcvd: ::std::option::Option<u64>,
    latestGenerationStamp: ::std::option::Option<u64>,
    requestedChecksum: ::protobuf::SingularPtrField<ChecksumProto>,
    cachingStrategy: ::protobuf::SingularPtrField<CachingStrategyProto>,
    storageType: ::std::option::Option<StorageTypeProto>,
    targetStorageTypes: ::std::vec::Vec<StorageTypeProto>,
    allowLazyPersist: ::std::option::Option<bool>,
    pinning: ::std::option::Option<bool>,
    targetPinnings: ::std::vec::Vec<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl OpWriteBlockProto {
    pub fn new() -> OpWriteBlockProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpWriteBlockProto {
        static mut instance: ::protobuf::lazy::Lazy<OpWriteBlockProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpWriteBlockProto,
        };
        unsafe {
            instance.get(|| {
                OpWriteBlockProto {
                    header: ::protobuf::SingularPtrField::none(),
                    targets: ::protobuf::RepeatedField::new(),
                    source: ::protobuf::SingularPtrField::none(),
                    stage: ::std::option::Option::None,
                    pipelineSize: ::std::option::Option::None,
                    minBytesRcvd: ::std::option::Option::None,
                    maxBytesRcvd: ::std::option::Option::None,
                    latestGenerationStamp: ::std::option::Option::None,
                    requestedChecksum: ::protobuf::SingularPtrField::none(),
                    cachingStrategy: ::protobuf::SingularPtrField::none(),
                    storageType: ::std::option::Option::None,
                    targetStorageTypes: ::std::vec::Vec::new(),
                    allowLazyPersist: ::std::option::Option::None,
                    pinning: ::std::option::Option::None,
                    targetPinnings: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.ClientOperationHeaderProto header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: ClientOperationHeaderProto) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header<'a>(&'a mut self) -> &'a mut ClientOperationHeaderProto {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ClientOperationHeaderProto {
        self.header.take().unwrap_or_else(|| ClientOperationHeaderProto::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a ClientOperationHeaderProto {
        self.header.as_ref().unwrap_or_else(|| ClientOperationHeaderProto::default_instance())
    }

    // repeated .hadoop.hdfs.DatanodeInfoProto targets = 2;

    pub fn clear_targets(&mut self) {
        self.targets.clear();
    }

    // Param is passed by value, moved
    pub fn set_targets(&mut self, v: ::protobuf::RepeatedField<DatanodeInfoProto>) {
        self.targets = v;
    }

    // Mutable pointer to the field.
    pub fn mut_targets<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<DatanodeInfoProto> {
        &mut self.targets
    }

    // Take field
    pub fn take_targets(&mut self) -> ::protobuf::RepeatedField<DatanodeInfoProto> {
        ::std::mem::replace(&mut self.targets, ::protobuf::RepeatedField::new())
    }

    pub fn get_targets<'a>(&'a self) -> &'a [DatanodeInfoProto] {
        &self.targets
    }

    // optional .hadoop.hdfs.DatanodeInfoProto source = 3;

    pub fn clear_source(&mut self) {
        self.source.clear();
    }

    pub fn has_source(&self) -> bool {
        self.source.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source(&mut self, v: DatanodeInfoProto) {
        self.source = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source<'a>(&'a mut self) -> &'a mut DatanodeInfoProto {
        if self.source.is_none() {
            self.source.set_default();
        };
        self.source.as_mut().unwrap()
    }

    // Take field
    pub fn take_source(&mut self) -> DatanodeInfoProto {
        self.source.take().unwrap_or_else(|| DatanodeInfoProto::new())
    }

    pub fn get_source<'a>(&'a self) -> &'a DatanodeInfoProto {
        self.source.as_ref().unwrap_or_else(|| DatanodeInfoProto::default_instance())
    }

    // required .hadoop.hdfs.OpWriteBlockProto.BlockConstructionStage stage = 4;

    pub fn clear_stage(&mut self) {
        self.stage = ::std::option::Option::None;
    }

    pub fn has_stage(&self) -> bool {
        self.stage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stage(&mut self, v: OpWriteBlockProto_BlockConstructionStage) {
        self.stage = ::std::option::Option::Some(v);
    }

    pub fn get_stage<'a>(&self) -> OpWriteBlockProto_BlockConstructionStage {
        self.stage.unwrap_or(OpWriteBlockProto_BlockConstructionStage::PIPELINE_SETUP_APPEND)
    }

    // required uint32 pipelineSize = 5;

    pub fn clear_pipelineSize(&mut self) {
        self.pipelineSize = ::std::option::Option::None;
    }

    pub fn has_pipelineSize(&self) -> bool {
        self.pipelineSize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pipelineSize(&mut self, v: u32) {
        self.pipelineSize = ::std::option::Option::Some(v);
    }

    pub fn get_pipelineSize<'a>(&self) -> u32 {
        self.pipelineSize.unwrap_or(0)
    }

    // required uint64 minBytesRcvd = 6;

    pub fn clear_minBytesRcvd(&mut self) {
        self.minBytesRcvd = ::std::option::Option::None;
    }

    pub fn has_minBytesRcvd(&self) -> bool {
        self.minBytesRcvd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_minBytesRcvd(&mut self, v: u64) {
        self.minBytesRcvd = ::std::option::Option::Some(v);
    }

    pub fn get_minBytesRcvd<'a>(&self) -> u64 {
        self.minBytesRcvd.unwrap_or(0)
    }

    // required uint64 maxBytesRcvd = 7;

    pub fn clear_maxBytesRcvd(&mut self) {
        self.maxBytesRcvd = ::std::option::Option::None;
    }

    pub fn has_maxBytesRcvd(&self) -> bool {
        self.maxBytesRcvd.is_some()
    }

    // Param is passed by value, moved
    pub fn set_maxBytesRcvd(&mut self, v: u64) {
        self.maxBytesRcvd = ::std::option::Option::Some(v);
    }

    pub fn get_maxBytesRcvd<'a>(&self) -> u64 {
        self.maxBytesRcvd.unwrap_or(0)
    }

    // required uint64 latestGenerationStamp = 8;

    pub fn clear_latestGenerationStamp(&mut self) {
        self.latestGenerationStamp = ::std::option::Option::None;
    }

    pub fn has_latestGenerationStamp(&self) -> bool {
        self.latestGenerationStamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_latestGenerationStamp(&mut self, v: u64) {
        self.latestGenerationStamp = ::std::option::Option::Some(v);
    }

    pub fn get_latestGenerationStamp<'a>(&self) -> u64 {
        self.latestGenerationStamp.unwrap_or(0)
    }

    // required .hadoop.hdfs.ChecksumProto requestedChecksum = 9;

    pub fn clear_requestedChecksum(&mut self) {
        self.requestedChecksum.clear();
    }

    pub fn has_requestedChecksum(&self) -> bool {
        self.requestedChecksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_requestedChecksum(&mut self, v: ChecksumProto) {
        self.requestedChecksum = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_requestedChecksum<'a>(&'a mut self) -> &'a mut ChecksumProto {
        if self.requestedChecksum.is_none() {
            self.requestedChecksum.set_default();
        };
        self.requestedChecksum.as_mut().unwrap()
    }

    // Take field
    pub fn take_requestedChecksum(&mut self) -> ChecksumProto {
        self.requestedChecksum.take().unwrap_or_else(|| ChecksumProto::new())
    }

    pub fn get_requestedChecksum<'a>(&'a self) -> &'a ChecksumProto {
        self.requestedChecksum.as_ref().unwrap_or_else(|| ChecksumProto::default_instance())
    }

    // optional .hadoop.hdfs.CachingStrategyProto cachingStrategy = 10;

    pub fn clear_cachingStrategy(&mut self) {
        self.cachingStrategy.clear();
    }

    pub fn has_cachingStrategy(&self) -> bool {
        self.cachingStrategy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cachingStrategy(&mut self, v: CachingStrategyProto) {
        self.cachingStrategy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cachingStrategy<'a>(&'a mut self) -> &'a mut CachingStrategyProto {
        if self.cachingStrategy.is_none() {
            self.cachingStrategy.set_default();
        };
        self.cachingStrategy.as_mut().unwrap()
    }

    // Take field
    pub fn take_cachingStrategy(&mut self) -> CachingStrategyProto {
        self.cachingStrategy.take().unwrap_or_else(|| CachingStrategyProto::new())
    }

    pub fn get_cachingStrategy<'a>(&'a self) -> &'a CachingStrategyProto {
        self.cachingStrategy.as_ref().unwrap_or_else(|| CachingStrategyProto::default_instance())
    }

    // optional .hadoop.hdfs.StorageTypeProto storageType = 11;

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

    // repeated .hadoop.hdfs.StorageTypeProto targetStorageTypes = 12;

    pub fn clear_targetStorageTypes(&mut self) {
        self.targetStorageTypes.clear();
    }

    // Param is passed by value, moved
    pub fn set_targetStorageTypes(&mut self, v: ::std::vec::Vec<StorageTypeProto>) {
        self.targetStorageTypes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_targetStorageTypes<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<StorageTypeProto> {
        &mut self.targetStorageTypes
    }

    // Take field
    pub fn take_targetStorageTypes(&mut self) -> ::std::vec::Vec<StorageTypeProto> {
        ::std::mem::replace(&mut self.targetStorageTypes, ::std::vec::Vec::new())
    }

    pub fn get_targetStorageTypes<'a>(&'a self) -> &'a [StorageTypeProto] {
        &self.targetStorageTypes
    }

    // optional bool allowLazyPersist = 13;

    pub fn clear_allowLazyPersist(&mut self) {
        self.allowLazyPersist = ::std::option::Option::None;
    }

    pub fn has_allowLazyPersist(&self) -> bool {
        self.allowLazyPersist.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allowLazyPersist(&mut self, v: bool) {
        self.allowLazyPersist = ::std::option::Option::Some(v);
    }

    pub fn get_allowLazyPersist<'a>(&self) -> bool {
        self.allowLazyPersist.unwrap_or(false)
    }

    // optional bool pinning = 14;

    pub fn clear_pinning(&mut self) {
        self.pinning = ::std::option::Option::None;
    }

    pub fn has_pinning(&self) -> bool {
        self.pinning.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pinning(&mut self, v: bool) {
        self.pinning = ::std::option::Option::Some(v);
    }

    pub fn get_pinning<'a>(&self) -> bool {
        self.pinning.unwrap_or(false)
    }

    // repeated bool targetPinnings = 15;

    pub fn clear_targetPinnings(&mut self) {
        self.targetPinnings.clear();
    }

    // Param is passed by value, moved
    pub fn set_targetPinnings(&mut self, v: ::std::vec::Vec<bool>) {
        self.targetPinnings = v;
    }

    // Mutable pointer to the field.
    pub fn mut_targetPinnings<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<bool> {
        &mut self.targetPinnings
    }

    // Take field
    pub fn take_targetPinnings(&mut self) -> ::std::vec::Vec<bool> {
        ::std::mem::replace(&mut self.targetPinnings, ::std::vec::Vec::new())
    }

    pub fn get_targetPinnings<'a>(&'a self) -> &'a [bool] {
        &self.targetPinnings
    }
}

impl ::protobuf::Message for OpWriteBlockProto {
    fn is_initialized(&self) -> bool {
        if self.header.is_none() {
            return false;
        };
        if self.stage.is_none() {
            return false;
        };
        if self.pipelineSize.is_none() {
            return false;
        };
        if self.minBytesRcvd.is_none() {
            return false;
        };
        if self.maxBytesRcvd.is_none() {
            return false;
        };
        if self.latestGenerationStamp.is_none() {
            return false;
        };
        if self.requestedChecksum.is_none() {
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
                    let tmp = self.header.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.targets));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.source.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.stage = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.pipelineSize = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.minBytesRcvd = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.maxBytesRcvd = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.latestGenerationStamp = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.requestedChecksum.set_default();
                    try!(is.merge_message(tmp))
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.cachingStrategy.set_default();
                    try!(is.merge_message(tmp))
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.storageType = ::std::option::Option::Some(tmp);
                },
                12 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.targetStorageTypes));
                },
                13 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.allowLazyPersist = ::std::option::Option::Some(tmp);
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.pinning = ::std::option::Option::Some(tmp);
                },
                15 => {
                    try!(::protobuf::rt::read_repeated_bool_into(wire_type, is, &mut self.targetPinnings));
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.targets.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.source.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.stage.iter() {
            my_size += ::protobuf::rt::enum_size(4, *value);
        };
        for value in self.pipelineSize.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.minBytesRcvd.iter() {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.maxBytesRcvd.iter() {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.latestGenerationStamp.iter() {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.requestedChecksum.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.cachingStrategy.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.storageType.iter() {
            my_size += ::protobuf::rt::enum_size(11, *value);
        };
        for value in self.targetStorageTypes.iter() {
            my_size += ::protobuf::rt::enum_size(12, *value);
        };
        if self.allowLazyPersist.is_some() {
            my_size += 2;
        };
        if self.pinning.is_some() {
            my_size += 2;
        };
        my_size += 2 * self.targetPinnings.len() as u32;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.targets.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.source.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.stage {
            try!(os.write_enum(4, v as i32));
        };
        if let Some(v) = self.pipelineSize {
            try!(os.write_uint32(5, v));
        };
        if let Some(v) = self.minBytesRcvd {
            try!(os.write_uint64(6, v));
        };
        if let Some(v) = self.maxBytesRcvd {
            try!(os.write_uint64(7, v));
        };
        if let Some(v) = self.latestGenerationStamp {
            try!(os.write_uint64(8, v));
        };
        if let Some(v) = self.requestedChecksum.as_ref() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cachingStrategy.as_ref() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.storageType {
            try!(os.write_enum(11, v as i32));
        };
        for v in self.targetStorageTypes.iter() {
            try!(os.write_enum(12, *v as i32));
        };
        if let Some(v) = self.allowLazyPersist {
            try!(os.write_bool(13, v));
        };
        if let Some(v) = self.pinning {
            try!(os.write_bool(14, v));
        };
        for v in self.targetPinnings.iter() {
            try!(os.write_bool(15, *v));
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
        ::std::any::TypeId::of::<OpWriteBlockProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OpWriteBlockProto {
    fn new() -> OpWriteBlockProto {
        OpWriteBlockProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpWriteBlockProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    OpWriteBlockProto::has_header,
                    OpWriteBlockProto::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "targets",
                    OpWriteBlockProto::get_targets,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "source",
                    OpWriteBlockProto::has_source,
                    OpWriteBlockProto::get_source,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "stage",
                    OpWriteBlockProto::has_stage,
                    OpWriteBlockProto::get_stage,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "pipelineSize",
                    OpWriteBlockProto::has_pipelineSize,
                    OpWriteBlockProto::get_pipelineSize,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "minBytesRcvd",
                    OpWriteBlockProto::has_minBytesRcvd,
                    OpWriteBlockProto::get_minBytesRcvd,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "maxBytesRcvd",
                    OpWriteBlockProto::has_maxBytesRcvd,
                    OpWriteBlockProto::get_maxBytesRcvd,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "latestGenerationStamp",
                    OpWriteBlockProto::has_latestGenerationStamp,
                    OpWriteBlockProto::get_latestGenerationStamp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "requestedChecksum",
                    OpWriteBlockProto::has_requestedChecksum,
                    OpWriteBlockProto::get_requestedChecksum,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cachingStrategy",
                    OpWriteBlockProto::has_cachingStrategy,
                    OpWriteBlockProto::get_cachingStrategy,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "storageType",
                    OpWriteBlockProto::has_storageType,
                    OpWriteBlockProto::get_storageType,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_enum_accessor(
                    "targetStorageTypes",
                    OpWriteBlockProto::get_targetStorageTypes,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "allowLazyPersist",
                    OpWriteBlockProto::has_allowLazyPersist,
                    OpWriteBlockProto::get_allowLazyPersist,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "pinning",
                    OpWriteBlockProto::has_pinning,
                    OpWriteBlockProto::get_pinning,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bool_accessor(
                    "targetPinnings",
                    OpWriteBlockProto::get_targetPinnings,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpWriteBlockProto>(
                    "OpWriteBlockProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpWriteBlockProto {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_targets();
        self.clear_source();
        self.clear_stage();
        self.clear_pipelineSize();
        self.clear_minBytesRcvd();
        self.clear_maxBytesRcvd();
        self.clear_latestGenerationStamp();
        self.clear_requestedChecksum();
        self.clear_cachingStrategy();
        self.clear_storageType();
        self.clear_targetStorageTypes();
        self.clear_allowLazyPersist();
        self.clear_pinning();
        self.clear_targetPinnings();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for OpWriteBlockProto {
    fn eq(&self, other: &OpWriteBlockProto) -> bool {
        self.header == other.header &&
        self.targets == other.targets &&
        self.source == other.source &&
        self.stage == other.stage &&
        self.pipelineSize == other.pipelineSize &&
        self.minBytesRcvd == other.minBytesRcvd &&
        self.maxBytesRcvd == other.maxBytesRcvd &&
        self.latestGenerationStamp == other.latestGenerationStamp &&
        self.requestedChecksum == other.requestedChecksum &&
        self.cachingStrategy == other.cachingStrategy &&
        self.storageType == other.storageType &&
        self.targetStorageTypes == other.targetStorageTypes &&
        self.allowLazyPersist == other.allowLazyPersist &&
        self.pinning == other.pinning &&
        self.targetPinnings == other.targetPinnings &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for OpWriteBlockProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum OpWriteBlockProto_BlockConstructionStage {
    PIPELINE_SETUP_APPEND = 0,
    PIPELINE_SETUP_APPEND_RECOVERY = 1,
    DATA_STREAMING = 2,
    PIPELINE_SETUP_STREAMING_RECOVERY = 3,
    PIPELINE_CLOSE = 4,
    PIPELINE_CLOSE_RECOVERY = 5,
    PIPELINE_SETUP_CREATE = 6,
    TRANSFER_RBW = 7,
    TRANSFER_FINALIZED = 8,
}

impl ::protobuf::ProtobufEnum for OpWriteBlockProto_BlockConstructionStage {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<OpWriteBlockProto_BlockConstructionStage> {
        match value {
            0 => ::std::option::Option::Some(OpWriteBlockProto_BlockConstructionStage::PIPELINE_SETUP_APPEND),
            1 => ::std::option::Option::Some(OpWriteBlockProto_BlockConstructionStage::PIPELINE_SETUP_APPEND_RECOVERY),
            2 => ::std::option::Option::Some(OpWriteBlockProto_BlockConstructionStage::DATA_STREAMING),
            3 => ::std::option::Option::Some(OpWriteBlockProto_BlockConstructionStage::PIPELINE_SETUP_STREAMING_RECOVERY),
            4 => ::std::option::Option::Some(OpWriteBlockProto_BlockConstructionStage::PIPELINE_CLOSE),
            5 => ::std::option::Option::Some(OpWriteBlockProto_BlockConstructionStage::PIPELINE_CLOSE_RECOVERY),
            6 => ::std::option::Option::Some(OpWriteBlockProto_BlockConstructionStage::PIPELINE_SETUP_CREATE),
            7 => ::std::option::Option::Some(OpWriteBlockProto_BlockConstructionStage::TRANSFER_RBW),
            8 => ::std::option::Option::Some(OpWriteBlockProto_BlockConstructionStage::TRANSFER_FINALIZED),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<OpWriteBlockProto_BlockConstructionStage>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("OpWriteBlockProto_BlockConstructionStage", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for OpWriteBlockProto_BlockConstructionStage {
}

#[derive(Clone,Default)]
pub struct OpTransferBlockProto {
    // message fields
    header: ::protobuf::SingularPtrField<ClientOperationHeaderProto>,
    targets: ::protobuf::RepeatedField<DatanodeInfoProto>,
    targetStorageTypes: ::std::vec::Vec<StorageTypeProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl OpTransferBlockProto {
    pub fn new() -> OpTransferBlockProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpTransferBlockProto {
        static mut instance: ::protobuf::lazy::Lazy<OpTransferBlockProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpTransferBlockProto,
        };
        unsafe {
            instance.get(|| {
                OpTransferBlockProto {
                    header: ::protobuf::SingularPtrField::none(),
                    targets: ::protobuf::RepeatedField::new(),
                    targetStorageTypes: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.ClientOperationHeaderProto header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: ClientOperationHeaderProto) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header<'a>(&'a mut self) -> &'a mut ClientOperationHeaderProto {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ClientOperationHeaderProto {
        self.header.take().unwrap_or_else(|| ClientOperationHeaderProto::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a ClientOperationHeaderProto {
        self.header.as_ref().unwrap_or_else(|| ClientOperationHeaderProto::default_instance())
    }

    // repeated .hadoop.hdfs.DatanodeInfoProto targets = 2;

    pub fn clear_targets(&mut self) {
        self.targets.clear();
    }

    // Param is passed by value, moved
    pub fn set_targets(&mut self, v: ::protobuf::RepeatedField<DatanodeInfoProto>) {
        self.targets = v;
    }

    // Mutable pointer to the field.
    pub fn mut_targets<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<DatanodeInfoProto> {
        &mut self.targets
    }

    // Take field
    pub fn take_targets(&mut self) -> ::protobuf::RepeatedField<DatanodeInfoProto> {
        ::std::mem::replace(&mut self.targets, ::protobuf::RepeatedField::new())
    }

    pub fn get_targets<'a>(&'a self) -> &'a [DatanodeInfoProto] {
        &self.targets
    }

    // repeated .hadoop.hdfs.StorageTypeProto targetStorageTypes = 3;

    pub fn clear_targetStorageTypes(&mut self) {
        self.targetStorageTypes.clear();
    }

    // Param is passed by value, moved
    pub fn set_targetStorageTypes(&mut self, v: ::std::vec::Vec<StorageTypeProto>) {
        self.targetStorageTypes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_targetStorageTypes<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<StorageTypeProto> {
        &mut self.targetStorageTypes
    }

    // Take field
    pub fn take_targetStorageTypes(&mut self) -> ::std::vec::Vec<StorageTypeProto> {
        ::std::mem::replace(&mut self.targetStorageTypes, ::std::vec::Vec::new())
    }

    pub fn get_targetStorageTypes<'a>(&'a self) -> &'a [StorageTypeProto] {
        &self.targetStorageTypes
    }
}

impl ::protobuf::Message for OpTransferBlockProto {
    fn is_initialized(&self) -> bool {
        if self.header.is_none() {
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
                    let tmp = self.header.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.targets));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.targetStorageTypes));
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.targets.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.targetStorageTypes.iter() {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.targets.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.targetStorageTypes.iter() {
            try!(os.write_enum(3, *v as i32));
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
        ::std::any::TypeId::of::<OpTransferBlockProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OpTransferBlockProto {
    fn new() -> OpTransferBlockProto {
        OpTransferBlockProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpTransferBlockProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    OpTransferBlockProto::has_header,
                    OpTransferBlockProto::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "targets",
                    OpTransferBlockProto::get_targets,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_enum_accessor(
                    "targetStorageTypes",
                    OpTransferBlockProto::get_targetStorageTypes,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpTransferBlockProto>(
                    "OpTransferBlockProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpTransferBlockProto {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_targets();
        self.clear_targetStorageTypes();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for OpTransferBlockProto {
    fn eq(&self, other: &OpTransferBlockProto) -> bool {
        self.header == other.header &&
        self.targets == other.targets &&
        self.targetStorageTypes == other.targetStorageTypes &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for OpTransferBlockProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct OpReplaceBlockProto {
    // message fields
    header: ::protobuf::SingularPtrField<BaseHeaderProto>,
    delHint: ::protobuf::SingularField<::std::string::String>,
    source: ::protobuf::SingularPtrField<DatanodeInfoProto>,
    storageType: ::std::option::Option<StorageTypeProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl OpReplaceBlockProto {
    pub fn new() -> OpReplaceBlockProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpReplaceBlockProto {
        static mut instance: ::protobuf::lazy::Lazy<OpReplaceBlockProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpReplaceBlockProto,
        };
        unsafe {
            instance.get(|| {
                OpReplaceBlockProto {
                    header: ::protobuf::SingularPtrField::none(),
                    delHint: ::protobuf::SingularField::none(),
                    source: ::protobuf::SingularPtrField::none(),
                    storageType: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.BaseHeaderProto header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: BaseHeaderProto) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header<'a>(&'a mut self) -> &'a mut BaseHeaderProto {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> BaseHeaderProto {
        self.header.take().unwrap_or_else(|| BaseHeaderProto::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a BaseHeaderProto {
        self.header.as_ref().unwrap_or_else(|| BaseHeaderProto::default_instance())
    }

    // required string delHint = 2;

    pub fn clear_delHint(&mut self) {
        self.delHint.clear();
    }

    pub fn has_delHint(&self) -> bool {
        self.delHint.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delHint(&mut self, v: ::std::string::String) {
        self.delHint = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delHint<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.delHint.is_none() {
            self.delHint.set_default();
        };
        self.delHint.as_mut().unwrap()
    }

    // Take field
    pub fn take_delHint(&mut self) -> ::std::string::String {
        self.delHint.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_delHint<'a>(&'a self) -> &'a str {
        match self.delHint.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required .hadoop.hdfs.DatanodeInfoProto source = 3;

    pub fn clear_source(&mut self) {
        self.source.clear();
    }

    pub fn has_source(&self) -> bool {
        self.source.is_some()
    }

    // Param is passed by value, moved
    pub fn set_source(&mut self, v: DatanodeInfoProto) {
        self.source = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_source<'a>(&'a mut self) -> &'a mut DatanodeInfoProto {
        if self.source.is_none() {
            self.source.set_default();
        };
        self.source.as_mut().unwrap()
    }

    // Take field
    pub fn take_source(&mut self) -> DatanodeInfoProto {
        self.source.take().unwrap_or_else(|| DatanodeInfoProto::new())
    }

    pub fn get_source<'a>(&'a self) -> &'a DatanodeInfoProto {
        self.source.as_ref().unwrap_or_else(|| DatanodeInfoProto::default_instance())
    }

    // optional .hadoop.hdfs.StorageTypeProto storageType = 4;

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

impl ::protobuf::Message for OpReplaceBlockProto {
    fn is_initialized(&self) -> bool {
        if self.header.is_none() {
            return false;
        };
        if self.delHint.is_none() {
            return false;
        };
        if self.source.is_none() {
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
                    let tmp = self.header.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.delHint.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.source.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.delHint.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.source.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.storageType.iter() {
            my_size += ::protobuf::rt::enum_size(4, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.delHint.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.source.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.storageType {
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
        ::std::any::TypeId::of::<OpReplaceBlockProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OpReplaceBlockProto {
    fn new() -> OpReplaceBlockProto {
        OpReplaceBlockProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpReplaceBlockProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    OpReplaceBlockProto::has_header,
                    OpReplaceBlockProto::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "delHint",
                    OpReplaceBlockProto::has_delHint,
                    OpReplaceBlockProto::get_delHint,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "source",
                    OpReplaceBlockProto::has_source,
                    OpReplaceBlockProto::get_source,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "storageType",
                    OpReplaceBlockProto::has_storageType,
                    OpReplaceBlockProto::get_storageType,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpReplaceBlockProto>(
                    "OpReplaceBlockProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpReplaceBlockProto {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_delHint();
        self.clear_source();
        self.clear_storageType();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for OpReplaceBlockProto {
    fn eq(&self, other: &OpReplaceBlockProto) -> bool {
        self.header == other.header &&
        self.delHint == other.delHint &&
        self.source == other.source &&
        self.storageType == other.storageType &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for OpReplaceBlockProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct OpCopyBlockProto {
    // message fields
    header: ::protobuf::SingularPtrField<BaseHeaderProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl OpCopyBlockProto {
    pub fn new() -> OpCopyBlockProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpCopyBlockProto {
        static mut instance: ::protobuf::lazy::Lazy<OpCopyBlockProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpCopyBlockProto,
        };
        unsafe {
            instance.get(|| {
                OpCopyBlockProto {
                    header: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.BaseHeaderProto header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: BaseHeaderProto) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header<'a>(&'a mut self) -> &'a mut BaseHeaderProto {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> BaseHeaderProto {
        self.header.take().unwrap_or_else(|| BaseHeaderProto::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a BaseHeaderProto {
        self.header.as_ref().unwrap_or_else(|| BaseHeaderProto::default_instance())
    }
}

impl ::protobuf::Message for OpCopyBlockProto {
    fn is_initialized(&self) -> bool {
        if self.header.is_none() {
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
                    let tmp = self.header.set_default();
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
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
        ::std::any::TypeId::of::<OpCopyBlockProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OpCopyBlockProto {
    fn new() -> OpCopyBlockProto {
        OpCopyBlockProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpCopyBlockProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    OpCopyBlockProto::has_header,
                    OpCopyBlockProto::get_header,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpCopyBlockProto>(
                    "OpCopyBlockProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpCopyBlockProto {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for OpCopyBlockProto {
    fn eq(&self, other: &OpCopyBlockProto) -> bool {
        self.header == other.header &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for OpCopyBlockProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct OpBlockChecksumProto {
    // message fields
    header: ::protobuf::SingularPtrField<BaseHeaderProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl OpBlockChecksumProto {
    pub fn new() -> OpBlockChecksumProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpBlockChecksumProto {
        static mut instance: ::protobuf::lazy::Lazy<OpBlockChecksumProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpBlockChecksumProto,
        };
        unsafe {
            instance.get(|| {
                OpBlockChecksumProto {
                    header: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.BaseHeaderProto header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: BaseHeaderProto) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header<'a>(&'a mut self) -> &'a mut BaseHeaderProto {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> BaseHeaderProto {
        self.header.take().unwrap_or_else(|| BaseHeaderProto::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a BaseHeaderProto {
        self.header.as_ref().unwrap_or_else(|| BaseHeaderProto::default_instance())
    }
}

impl ::protobuf::Message for OpBlockChecksumProto {
    fn is_initialized(&self) -> bool {
        if self.header.is_none() {
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
                    let tmp = self.header.set_default();
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
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
        ::std::any::TypeId::of::<OpBlockChecksumProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OpBlockChecksumProto {
    fn new() -> OpBlockChecksumProto {
        OpBlockChecksumProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpBlockChecksumProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    OpBlockChecksumProto::has_header,
                    OpBlockChecksumProto::get_header,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpBlockChecksumProto>(
                    "OpBlockChecksumProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpBlockChecksumProto {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for OpBlockChecksumProto {
    fn eq(&self, other: &OpBlockChecksumProto) -> bool {
        self.header == other.header &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for OpBlockChecksumProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ShortCircuitShmIdProto {
    // message fields
    hi: ::std::option::Option<i64>,
    lo: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ShortCircuitShmIdProto {
    pub fn new() -> ShortCircuitShmIdProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ShortCircuitShmIdProto {
        static mut instance: ::protobuf::lazy::Lazy<ShortCircuitShmIdProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ShortCircuitShmIdProto,
        };
        unsafe {
            instance.get(|| {
                ShortCircuitShmIdProto {
                    hi: ::std::option::Option::None,
                    lo: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int64 hi = 1;

    pub fn clear_hi(&mut self) {
        self.hi = ::std::option::Option::None;
    }

    pub fn has_hi(&self) -> bool {
        self.hi.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hi(&mut self, v: i64) {
        self.hi = ::std::option::Option::Some(v);
    }

    pub fn get_hi<'a>(&self) -> i64 {
        self.hi.unwrap_or(0)
    }

    // required int64 lo = 2;

    pub fn clear_lo(&mut self) {
        self.lo = ::std::option::Option::None;
    }

    pub fn has_lo(&self) -> bool {
        self.lo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lo(&mut self, v: i64) {
        self.lo = ::std::option::Option::Some(v);
    }

    pub fn get_lo<'a>(&self) -> i64 {
        self.lo.unwrap_or(0)
    }
}

impl ::protobuf::Message for ShortCircuitShmIdProto {
    fn is_initialized(&self) -> bool {
        if self.hi.is_none() {
            return false;
        };
        if self.lo.is_none() {
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
                    let tmp = try!(is.read_int64());
                    self.hi = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.lo = ::std::option::Option::Some(tmp);
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
        for value in self.hi.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.lo.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hi {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.lo {
            try!(os.write_int64(2, v));
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
        ::std::any::TypeId::of::<ShortCircuitShmIdProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ShortCircuitShmIdProto {
    fn new() -> ShortCircuitShmIdProto {
        ShortCircuitShmIdProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ShortCircuitShmIdProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "hi",
                    ShortCircuitShmIdProto::has_hi,
                    ShortCircuitShmIdProto::get_hi,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "lo",
                    ShortCircuitShmIdProto::has_lo,
                    ShortCircuitShmIdProto::get_lo,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ShortCircuitShmIdProto>(
                    "ShortCircuitShmIdProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ShortCircuitShmIdProto {
    fn clear(&mut self) {
        self.clear_hi();
        self.clear_lo();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ShortCircuitShmIdProto {
    fn eq(&self, other: &ShortCircuitShmIdProto) -> bool {
        self.hi == other.hi &&
        self.lo == other.lo &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ShortCircuitShmIdProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ShortCircuitShmSlotProto {
    // message fields
    shmId: ::protobuf::SingularPtrField<ShortCircuitShmIdProto>,
    slotIdx: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ShortCircuitShmSlotProto {
    pub fn new() -> ShortCircuitShmSlotProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ShortCircuitShmSlotProto {
        static mut instance: ::protobuf::lazy::Lazy<ShortCircuitShmSlotProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ShortCircuitShmSlotProto,
        };
        unsafe {
            instance.get(|| {
                ShortCircuitShmSlotProto {
                    shmId: ::protobuf::SingularPtrField::none(),
                    slotIdx: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.ShortCircuitShmIdProto shmId = 1;

    pub fn clear_shmId(&mut self) {
        self.shmId.clear();
    }

    pub fn has_shmId(&self) -> bool {
        self.shmId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shmId(&mut self, v: ShortCircuitShmIdProto) {
        self.shmId = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_shmId<'a>(&'a mut self) -> &'a mut ShortCircuitShmIdProto {
        if self.shmId.is_none() {
            self.shmId.set_default();
        };
        self.shmId.as_mut().unwrap()
    }

    // Take field
    pub fn take_shmId(&mut self) -> ShortCircuitShmIdProto {
        self.shmId.take().unwrap_or_else(|| ShortCircuitShmIdProto::new())
    }

    pub fn get_shmId<'a>(&'a self) -> &'a ShortCircuitShmIdProto {
        self.shmId.as_ref().unwrap_or_else(|| ShortCircuitShmIdProto::default_instance())
    }

    // required int32 slotIdx = 2;

    pub fn clear_slotIdx(&mut self) {
        self.slotIdx = ::std::option::Option::None;
    }

    pub fn has_slotIdx(&self) -> bool {
        self.slotIdx.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slotIdx(&mut self, v: i32) {
        self.slotIdx = ::std::option::Option::Some(v);
    }

    pub fn get_slotIdx<'a>(&self) -> i32 {
        self.slotIdx.unwrap_or(0)
    }
}

impl ::protobuf::Message for ShortCircuitShmSlotProto {
    fn is_initialized(&self) -> bool {
        if self.shmId.is_none() {
            return false;
        };
        if self.slotIdx.is_none() {
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
                    let tmp = self.shmId.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.slotIdx = ::std::option::Option::Some(tmp);
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
        for value in self.shmId.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.slotIdx.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.shmId.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.slotIdx {
            try!(os.write_int32(2, v));
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
        ::std::any::TypeId::of::<ShortCircuitShmSlotProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ShortCircuitShmSlotProto {
    fn new() -> ShortCircuitShmSlotProto {
        ShortCircuitShmSlotProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ShortCircuitShmSlotProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "shmId",
                    ShortCircuitShmSlotProto::has_shmId,
                    ShortCircuitShmSlotProto::get_shmId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "slotIdx",
                    ShortCircuitShmSlotProto::has_slotIdx,
                    ShortCircuitShmSlotProto::get_slotIdx,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ShortCircuitShmSlotProto>(
                    "ShortCircuitShmSlotProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ShortCircuitShmSlotProto {
    fn clear(&mut self) {
        self.clear_shmId();
        self.clear_slotIdx();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ShortCircuitShmSlotProto {
    fn eq(&self, other: &ShortCircuitShmSlotProto) -> bool {
        self.shmId == other.shmId &&
        self.slotIdx == other.slotIdx &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ShortCircuitShmSlotProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct OpRequestShortCircuitAccessProto {
    // message fields
    header: ::protobuf::SingularPtrField<BaseHeaderProto>,
    maxVersion: ::std::option::Option<u32>,
    slotId: ::protobuf::SingularPtrField<ShortCircuitShmSlotProto>,
    supportsReceiptVerification: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl OpRequestShortCircuitAccessProto {
    pub fn new() -> OpRequestShortCircuitAccessProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpRequestShortCircuitAccessProto {
        static mut instance: ::protobuf::lazy::Lazy<OpRequestShortCircuitAccessProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpRequestShortCircuitAccessProto,
        };
        unsafe {
            instance.get(|| {
                OpRequestShortCircuitAccessProto {
                    header: ::protobuf::SingularPtrField::none(),
                    maxVersion: ::std::option::Option::None,
                    slotId: ::protobuf::SingularPtrField::none(),
                    supportsReceiptVerification: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.BaseHeaderProto header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: BaseHeaderProto) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header<'a>(&'a mut self) -> &'a mut BaseHeaderProto {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> BaseHeaderProto {
        self.header.take().unwrap_or_else(|| BaseHeaderProto::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a BaseHeaderProto {
        self.header.as_ref().unwrap_or_else(|| BaseHeaderProto::default_instance())
    }

    // required uint32 maxVersion = 2;

    pub fn clear_maxVersion(&mut self) {
        self.maxVersion = ::std::option::Option::None;
    }

    pub fn has_maxVersion(&self) -> bool {
        self.maxVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_maxVersion(&mut self, v: u32) {
        self.maxVersion = ::std::option::Option::Some(v);
    }

    pub fn get_maxVersion<'a>(&self) -> u32 {
        self.maxVersion.unwrap_or(0)
    }

    // optional .hadoop.hdfs.ShortCircuitShmSlotProto slotId = 3;

    pub fn clear_slotId(&mut self) {
        self.slotId.clear();
    }

    pub fn has_slotId(&self) -> bool {
        self.slotId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slotId(&mut self, v: ShortCircuitShmSlotProto) {
        self.slotId = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_slotId<'a>(&'a mut self) -> &'a mut ShortCircuitShmSlotProto {
        if self.slotId.is_none() {
            self.slotId.set_default();
        };
        self.slotId.as_mut().unwrap()
    }

    // Take field
    pub fn take_slotId(&mut self) -> ShortCircuitShmSlotProto {
        self.slotId.take().unwrap_or_else(|| ShortCircuitShmSlotProto::new())
    }

    pub fn get_slotId<'a>(&'a self) -> &'a ShortCircuitShmSlotProto {
        self.slotId.as_ref().unwrap_or_else(|| ShortCircuitShmSlotProto::default_instance())
    }

    // optional bool supportsReceiptVerification = 4;

    pub fn clear_supportsReceiptVerification(&mut self) {
        self.supportsReceiptVerification = ::std::option::Option::None;
    }

    pub fn has_supportsReceiptVerification(&self) -> bool {
        self.supportsReceiptVerification.is_some()
    }

    // Param is passed by value, moved
    pub fn set_supportsReceiptVerification(&mut self, v: bool) {
        self.supportsReceiptVerification = ::std::option::Option::Some(v);
    }

    pub fn get_supportsReceiptVerification<'a>(&self) -> bool {
        self.supportsReceiptVerification.unwrap_or(false)
    }
}

impl ::protobuf::Message for OpRequestShortCircuitAccessProto {
    fn is_initialized(&self) -> bool {
        if self.header.is_none() {
            return false;
        };
        if self.maxVersion.is_none() {
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
                    let tmp = self.header.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.maxVersion = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.slotId.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.supportsReceiptVerification = ::std::option::Option::Some(tmp);
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.maxVersion.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.slotId.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.supportsReceiptVerification.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.maxVersion {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.slotId.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.supportsReceiptVerification {
            try!(os.write_bool(4, v));
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
        ::std::any::TypeId::of::<OpRequestShortCircuitAccessProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OpRequestShortCircuitAccessProto {
    fn new() -> OpRequestShortCircuitAccessProto {
        OpRequestShortCircuitAccessProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpRequestShortCircuitAccessProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    OpRequestShortCircuitAccessProto::has_header,
                    OpRequestShortCircuitAccessProto::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "maxVersion",
                    OpRequestShortCircuitAccessProto::has_maxVersion,
                    OpRequestShortCircuitAccessProto::get_maxVersion,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "slotId",
                    OpRequestShortCircuitAccessProto::has_slotId,
                    OpRequestShortCircuitAccessProto::get_slotId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "supportsReceiptVerification",
                    OpRequestShortCircuitAccessProto::has_supportsReceiptVerification,
                    OpRequestShortCircuitAccessProto::get_supportsReceiptVerification,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpRequestShortCircuitAccessProto>(
                    "OpRequestShortCircuitAccessProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpRequestShortCircuitAccessProto {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_maxVersion();
        self.clear_slotId();
        self.clear_supportsReceiptVerification();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for OpRequestShortCircuitAccessProto {
    fn eq(&self, other: &OpRequestShortCircuitAccessProto) -> bool {
        self.header == other.header &&
        self.maxVersion == other.maxVersion &&
        self.slotId == other.slotId &&
        self.supportsReceiptVerification == other.supportsReceiptVerification &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for OpRequestShortCircuitAccessProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ReleaseShortCircuitAccessRequestProto {
    // message fields
    slotId: ::protobuf::SingularPtrField<ShortCircuitShmSlotProto>,
    traceInfo: ::protobuf::SingularPtrField<DataTransferTraceInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ReleaseShortCircuitAccessRequestProto {
    pub fn new() -> ReleaseShortCircuitAccessRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReleaseShortCircuitAccessRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<ReleaseShortCircuitAccessRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReleaseShortCircuitAccessRequestProto,
        };
        unsafe {
            instance.get(|| {
                ReleaseShortCircuitAccessRequestProto {
                    slotId: ::protobuf::SingularPtrField::none(),
                    traceInfo: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.ShortCircuitShmSlotProto slotId = 1;

    pub fn clear_slotId(&mut self) {
        self.slotId.clear();
    }

    pub fn has_slotId(&self) -> bool {
        self.slotId.is_some()
    }

    // Param is passed by value, moved
    pub fn set_slotId(&mut self, v: ShortCircuitShmSlotProto) {
        self.slotId = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_slotId<'a>(&'a mut self) -> &'a mut ShortCircuitShmSlotProto {
        if self.slotId.is_none() {
            self.slotId.set_default();
        };
        self.slotId.as_mut().unwrap()
    }

    // Take field
    pub fn take_slotId(&mut self) -> ShortCircuitShmSlotProto {
        self.slotId.take().unwrap_or_else(|| ShortCircuitShmSlotProto::new())
    }

    pub fn get_slotId<'a>(&'a self) -> &'a ShortCircuitShmSlotProto {
        self.slotId.as_ref().unwrap_or_else(|| ShortCircuitShmSlotProto::default_instance())
    }

    // optional .hadoop.hdfs.DataTransferTraceInfoProto traceInfo = 2;

    pub fn clear_traceInfo(&mut self) {
        self.traceInfo.clear();
    }

    pub fn has_traceInfo(&self) -> bool {
        self.traceInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_traceInfo(&mut self, v: DataTransferTraceInfoProto) {
        self.traceInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_traceInfo<'a>(&'a mut self) -> &'a mut DataTransferTraceInfoProto {
        if self.traceInfo.is_none() {
            self.traceInfo.set_default();
        };
        self.traceInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_traceInfo(&mut self) -> DataTransferTraceInfoProto {
        self.traceInfo.take().unwrap_or_else(|| DataTransferTraceInfoProto::new())
    }

    pub fn get_traceInfo<'a>(&'a self) -> &'a DataTransferTraceInfoProto {
        self.traceInfo.as_ref().unwrap_or_else(|| DataTransferTraceInfoProto::default_instance())
    }
}

impl ::protobuf::Message for ReleaseShortCircuitAccessRequestProto {
    fn is_initialized(&self) -> bool {
        if self.slotId.is_none() {
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
                    let tmp = self.slotId.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.traceInfo.set_default();
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
        for value in self.slotId.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.traceInfo.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.slotId.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.traceInfo.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<ReleaseShortCircuitAccessRequestProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReleaseShortCircuitAccessRequestProto {
    fn new() -> ReleaseShortCircuitAccessRequestProto {
        ReleaseShortCircuitAccessRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReleaseShortCircuitAccessRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "slotId",
                    ReleaseShortCircuitAccessRequestProto::has_slotId,
                    ReleaseShortCircuitAccessRequestProto::get_slotId,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "traceInfo",
                    ReleaseShortCircuitAccessRequestProto::has_traceInfo,
                    ReleaseShortCircuitAccessRequestProto::get_traceInfo,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReleaseShortCircuitAccessRequestProto>(
                    "ReleaseShortCircuitAccessRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReleaseShortCircuitAccessRequestProto {
    fn clear(&mut self) {
        self.clear_slotId();
        self.clear_traceInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ReleaseShortCircuitAccessRequestProto {
    fn eq(&self, other: &ReleaseShortCircuitAccessRequestProto) -> bool {
        self.slotId == other.slotId &&
        self.traceInfo == other.traceInfo &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ReleaseShortCircuitAccessRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ReleaseShortCircuitAccessResponseProto {
    // message fields
    status: ::std::option::Option<Status>,
    error: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ReleaseShortCircuitAccessResponseProto {
    pub fn new() -> ReleaseShortCircuitAccessResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReleaseShortCircuitAccessResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<ReleaseShortCircuitAccessResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReleaseShortCircuitAccessResponseProto,
        };
        unsafe {
            instance.get(|| {
                ReleaseShortCircuitAccessResponseProto {
                    status: ::std::option::Option::None,
                    error: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status<'a>(&self) -> Status {
        self.status.unwrap_or(Status::SUCCESS)
    }

    // optional string error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        self.error.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error<'a>(&'a self) -> &'a str {
        match self.error.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for ReleaseShortCircuitAccessResponseProto {
    fn is_initialized(&self) -> bool {
        if self.status.is_none() {
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
                    self.status = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.error.set_default();
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
        for value in self.status.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.error.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.error.as_ref() {
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
        ::std::any::TypeId::of::<ReleaseShortCircuitAccessResponseProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReleaseShortCircuitAccessResponseProto {
    fn new() -> ReleaseShortCircuitAccessResponseProto {
        ReleaseShortCircuitAccessResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReleaseShortCircuitAccessResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "status",
                    ReleaseShortCircuitAccessResponseProto::has_status,
                    ReleaseShortCircuitAccessResponseProto::get_status,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "error",
                    ReleaseShortCircuitAccessResponseProto::has_error,
                    ReleaseShortCircuitAccessResponseProto::get_error,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReleaseShortCircuitAccessResponseProto>(
                    "ReleaseShortCircuitAccessResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReleaseShortCircuitAccessResponseProto {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ReleaseShortCircuitAccessResponseProto {
    fn eq(&self, other: &ReleaseShortCircuitAccessResponseProto) -> bool {
        self.status == other.status &&
        self.error == other.error &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ReleaseShortCircuitAccessResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ShortCircuitShmRequestProto {
    // message fields
    clientName: ::protobuf::SingularField<::std::string::String>,
    traceInfo: ::protobuf::SingularPtrField<DataTransferTraceInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ShortCircuitShmRequestProto {
    pub fn new() -> ShortCircuitShmRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ShortCircuitShmRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<ShortCircuitShmRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ShortCircuitShmRequestProto,
        };
        unsafe {
            instance.get(|| {
                ShortCircuitShmRequestProto {
                    clientName: ::protobuf::SingularField::none(),
                    traceInfo: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string clientName = 1;

    pub fn clear_clientName(&mut self) {
        self.clientName.clear();
    }

    pub fn has_clientName(&self) -> bool {
        self.clientName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clientName(&mut self, v: ::std::string::String) {
        self.clientName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clientName<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.clientName.is_none() {
            self.clientName.set_default();
        };
        self.clientName.as_mut().unwrap()
    }

    // Take field
    pub fn take_clientName(&mut self) -> ::std::string::String {
        self.clientName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_clientName<'a>(&'a self) -> &'a str {
        match self.clientName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .hadoop.hdfs.DataTransferTraceInfoProto traceInfo = 2;

    pub fn clear_traceInfo(&mut self) {
        self.traceInfo.clear();
    }

    pub fn has_traceInfo(&self) -> bool {
        self.traceInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_traceInfo(&mut self, v: DataTransferTraceInfoProto) {
        self.traceInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_traceInfo<'a>(&'a mut self) -> &'a mut DataTransferTraceInfoProto {
        if self.traceInfo.is_none() {
            self.traceInfo.set_default();
        };
        self.traceInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_traceInfo(&mut self) -> DataTransferTraceInfoProto {
        self.traceInfo.take().unwrap_or_else(|| DataTransferTraceInfoProto::new())
    }

    pub fn get_traceInfo<'a>(&'a self) -> &'a DataTransferTraceInfoProto {
        self.traceInfo.as_ref().unwrap_or_else(|| DataTransferTraceInfoProto::default_instance())
    }
}

impl ::protobuf::Message for ShortCircuitShmRequestProto {
    fn is_initialized(&self) -> bool {
        if self.clientName.is_none() {
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
                    let tmp = self.clientName.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.traceInfo.set_default();
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
        for value in self.clientName.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.traceInfo.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.clientName.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.traceInfo.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<ShortCircuitShmRequestProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ShortCircuitShmRequestProto {
    fn new() -> ShortCircuitShmRequestProto {
        ShortCircuitShmRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ShortCircuitShmRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "clientName",
                    ShortCircuitShmRequestProto::has_clientName,
                    ShortCircuitShmRequestProto::get_clientName,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "traceInfo",
                    ShortCircuitShmRequestProto::has_traceInfo,
                    ShortCircuitShmRequestProto::get_traceInfo,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ShortCircuitShmRequestProto>(
                    "ShortCircuitShmRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ShortCircuitShmRequestProto {
    fn clear(&mut self) {
        self.clear_clientName();
        self.clear_traceInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ShortCircuitShmRequestProto {
    fn eq(&self, other: &ShortCircuitShmRequestProto) -> bool {
        self.clientName == other.clientName &&
        self.traceInfo == other.traceInfo &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ShortCircuitShmRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ShortCircuitShmResponseProto {
    // message fields
    status: ::std::option::Option<Status>,
    error: ::protobuf::SingularField<::std::string::String>,
    id: ::protobuf::SingularPtrField<ShortCircuitShmIdProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ShortCircuitShmResponseProto {
    pub fn new() -> ShortCircuitShmResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ShortCircuitShmResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<ShortCircuitShmResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ShortCircuitShmResponseProto,
        };
        unsafe {
            instance.get(|| {
                ShortCircuitShmResponseProto {
                    status: ::std::option::Option::None,
                    error: ::protobuf::SingularField::none(),
                    id: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status<'a>(&self) -> Status {
        self.status.unwrap_or(Status::SUCCESS)
    }

    // optional string error = 2;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        self.error.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error<'a>(&'a self) -> &'a str {
        match self.error.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .hadoop.hdfs.ShortCircuitShmIdProto id = 3;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ShortCircuitShmIdProto) {
        self.id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id<'a>(&'a mut self) -> &'a mut ShortCircuitShmIdProto {
        if self.id.is_none() {
            self.id.set_default();
        };
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> ShortCircuitShmIdProto {
        self.id.take().unwrap_or_else(|| ShortCircuitShmIdProto::new())
    }

    pub fn get_id<'a>(&'a self) -> &'a ShortCircuitShmIdProto {
        self.id.as_ref().unwrap_or_else(|| ShortCircuitShmIdProto::default_instance())
    }
}

impl ::protobuf::Message for ShortCircuitShmResponseProto {
    fn is_initialized(&self) -> bool {
        if self.status.is_none() {
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
                    self.status = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.error.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.id.set_default();
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
        for value in self.status.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.error.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.error.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.id.as_ref() {
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
        ::std::any::TypeId::of::<ShortCircuitShmResponseProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ShortCircuitShmResponseProto {
    fn new() -> ShortCircuitShmResponseProto {
        ShortCircuitShmResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ShortCircuitShmResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "status",
                    ShortCircuitShmResponseProto::has_status,
                    ShortCircuitShmResponseProto::get_status,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "error",
                    ShortCircuitShmResponseProto::has_error,
                    ShortCircuitShmResponseProto::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "id",
                    ShortCircuitShmResponseProto::has_id,
                    ShortCircuitShmResponseProto::get_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ShortCircuitShmResponseProto>(
                    "ShortCircuitShmResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ShortCircuitShmResponseProto {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_error();
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ShortCircuitShmResponseProto {
    fn eq(&self, other: &ShortCircuitShmResponseProto) -> bool {
        self.status == other.status &&
        self.error == other.error &&
        self.id == other.id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ShortCircuitShmResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PacketHeaderProto {
    // message fields
    offsetInBlock: ::std::option::Option<i64>,
    seqno: ::std::option::Option<i64>,
    lastPacketInBlock: ::std::option::Option<bool>,
    dataLen: ::std::option::Option<i32>,
    syncBlock: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl PacketHeaderProto {
    pub fn new() -> PacketHeaderProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PacketHeaderProto {
        static mut instance: ::protobuf::lazy::Lazy<PacketHeaderProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PacketHeaderProto,
        };
        unsafe {
            instance.get(|| {
                PacketHeaderProto {
                    offsetInBlock: ::std::option::Option::None,
                    seqno: ::std::option::Option::None,
                    lastPacketInBlock: ::std::option::Option::None,
                    dataLen: ::std::option::Option::None,
                    syncBlock: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required sfixed64 offsetInBlock = 1;

    pub fn clear_offsetInBlock(&mut self) {
        self.offsetInBlock = ::std::option::Option::None;
    }

    pub fn has_offsetInBlock(&self) -> bool {
        self.offsetInBlock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_offsetInBlock(&mut self, v: i64) {
        self.offsetInBlock = ::std::option::Option::Some(v);
    }

    pub fn get_offsetInBlock<'a>(&self) -> i64 {
        self.offsetInBlock.unwrap_or(0)
    }

    // required sfixed64 seqno = 2;

    pub fn clear_seqno(&mut self) {
        self.seqno = ::std::option::Option::None;
    }

    pub fn has_seqno(&self) -> bool {
        self.seqno.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seqno(&mut self, v: i64) {
        self.seqno = ::std::option::Option::Some(v);
    }

    pub fn get_seqno<'a>(&self) -> i64 {
        self.seqno.unwrap_or(0)
    }

    // required bool lastPacketInBlock = 3;

    pub fn clear_lastPacketInBlock(&mut self) {
        self.lastPacketInBlock = ::std::option::Option::None;
    }

    pub fn has_lastPacketInBlock(&self) -> bool {
        self.lastPacketInBlock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastPacketInBlock(&mut self, v: bool) {
        self.lastPacketInBlock = ::std::option::Option::Some(v);
    }

    pub fn get_lastPacketInBlock<'a>(&self) -> bool {
        self.lastPacketInBlock.unwrap_or(false)
    }

    // required sfixed32 dataLen = 4;

    pub fn clear_dataLen(&mut self) {
        self.dataLen = ::std::option::Option::None;
    }

    pub fn has_dataLen(&self) -> bool {
        self.dataLen.is_some()
    }

    // Param is passed by value, moved
    pub fn set_dataLen(&mut self, v: i32) {
        self.dataLen = ::std::option::Option::Some(v);
    }

    pub fn get_dataLen<'a>(&self) -> i32 {
        self.dataLen.unwrap_or(0)
    }

    // optional bool syncBlock = 5;

    pub fn clear_syncBlock(&mut self) {
        self.syncBlock = ::std::option::Option::None;
    }

    pub fn has_syncBlock(&self) -> bool {
        self.syncBlock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_syncBlock(&mut self, v: bool) {
        self.syncBlock = ::std::option::Option::Some(v);
    }

    pub fn get_syncBlock<'a>(&self) -> bool {
        self.syncBlock.unwrap_or(false)
    }
}

impl ::protobuf::Message for PacketHeaderProto {
    fn is_initialized(&self) -> bool {
        if self.offsetInBlock.is_none() {
            return false;
        };
        if self.seqno.is_none() {
            return false;
        };
        if self.lastPacketInBlock.is_none() {
            return false;
        };
        if self.dataLen.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_sfixed64());
                    self.offsetInBlock = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed64 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_sfixed64());
                    self.seqno = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.lastPacketInBlock = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeFixed32 {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_sfixed32());
                    self.dataLen = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.syncBlock = ::std::option::Option::Some(tmp);
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
        if self.offsetInBlock.is_some() {
            my_size += 9;
        };
        if self.seqno.is_some() {
            my_size += 9;
        };
        if self.lastPacketInBlock.is_some() {
            my_size += 2;
        };
        if self.dataLen.is_some() {
            my_size += 5;
        };
        if self.syncBlock.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.offsetInBlock {
            try!(os.write_sfixed64(1, v));
        };
        if let Some(v) = self.seqno {
            try!(os.write_sfixed64(2, v));
        };
        if let Some(v) = self.lastPacketInBlock {
            try!(os.write_bool(3, v));
        };
        if let Some(v) = self.dataLen {
            try!(os.write_sfixed32(4, v));
        };
        if let Some(v) = self.syncBlock {
            try!(os.write_bool(5, v));
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
        ::std::any::TypeId::of::<PacketHeaderProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PacketHeaderProto {
    fn new() -> PacketHeaderProto {
        PacketHeaderProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<PacketHeaderProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "offsetInBlock",
                    PacketHeaderProto::has_offsetInBlock,
                    PacketHeaderProto::get_offsetInBlock,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "seqno",
                    PacketHeaderProto::has_seqno,
                    PacketHeaderProto::get_seqno,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "lastPacketInBlock",
                    PacketHeaderProto::has_lastPacketInBlock,
                    PacketHeaderProto::get_lastPacketInBlock,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "dataLen",
                    PacketHeaderProto::has_dataLen,
                    PacketHeaderProto::get_dataLen,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "syncBlock",
                    PacketHeaderProto::has_syncBlock,
                    PacketHeaderProto::get_syncBlock,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PacketHeaderProto>(
                    "PacketHeaderProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PacketHeaderProto {
    fn clear(&mut self) {
        self.clear_offsetInBlock();
        self.clear_seqno();
        self.clear_lastPacketInBlock();
        self.clear_dataLen();
        self.clear_syncBlock();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PacketHeaderProto {
    fn eq(&self, other: &PacketHeaderProto) -> bool {
        self.offsetInBlock == other.offsetInBlock &&
        self.seqno == other.seqno &&
        self.lastPacketInBlock == other.lastPacketInBlock &&
        self.dataLen == other.dataLen &&
        self.syncBlock == other.syncBlock &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PacketHeaderProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PipelineAckProto {
    // message fields
    seqno: ::std::option::Option<i64>,
    reply: ::std::vec::Vec<Status>,
    downstreamAckTimeNanos: ::std::option::Option<u64>,
    flag: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl PipelineAckProto {
    pub fn new() -> PipelineAckProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PipelineAckProto {
        static mut instance: ::protobuf::lazy::Lazy<PipelineAckProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PipelineAckProto,
        };
        unsafe {
            instance.get(|| {
                PipelineAckProto {
                    seqno: ::std::option::Option::None,
                    reply: ::std::vec::Vec::new(),
                    downstreamAckTimeNanos: ::std::option::Option::None,
                    flag: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required sint64 seqno = 1;

    pub fn clear_seqno(&mut self) {
        self.seqno = ::std::option::Option::None;
    }

    pub fn has_seqno(&self) -> bool {
        self.seqno.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seqno(&mut self, v: i64) {
        self.seqno = ::std::option::Option::Some(v);
    }

    pub fn get_seqno<'a>(&self) -> i64 {
        self.seqno.unwrap_or(0)
    }

    // repeated .hadoop.hdfs.Status reply = 2;

    pub fn clear_reply(&mut self) {
        self.reply.clear();
    }

    // Param is passed by value, moved
    pub fn set_reply(&mut self, v: ::std::vec::Vec<Status>) {
        self.reply = v;
    }

    // Mutable pointer to the field.
    pub fn mut_reply<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<Status> {
        &mut self.reply
    }

    // Take field
    pub fn take_reply(&mut self) -> ::std::vec::Vec<Status> {
        ::std::mem::replace(&mut self.reply, ::std::vec::Vec::new())
    }

    pub fn get_reply<'a>(&'a self) -> &'a [Status] {
        &self.reply
    }

    // optional uint64 downstreamAckTimeNanos = 3;

    pub fn clear_downstreamAckTimeNanos(&mut self) {
        self.downstreamAckTimeNanos = ::std::option::Option::None;
    }

    pub fn has_downstreamAckTimeNanos(&self) -> bool {
        self.downstreamAckTimeNanos.is_some()
    }

    // Param is passed by value, moved
    pub fn set_downstreamAckTimeNanos(&mut self, v: u64) {
        self.downstreamAckTimeNanos = ::std::option::Option::Some(v);
    }

    pub fn get_downstreamAckTimeNanos<'a>(&self) -> u64 {
        self.downstreamAckTimeNanos.unwrap_or(0u64)
    }

    // repeated uint32 flag = 4;

    pub fn clear_flag(&mut self) {
        self.flag.clear();
    }

    // Param is passed by value, moved
    pub fn set_flag(&mut self, v: ::std::vec::Vec<u32>) {
        self.flag = v;
    }

    // Mutable pointer to the field.
    pub fn mut_flag<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u32> {
        &mut self.flag
    }

    // Take field
    pub fn take_flag(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.flag, ::std::vec::Vec::new())
    }

    pub fn get_flag<'a>(&'a self) -> &'a [u32] {
        &self.flag
    }
}

impl ::protobuf::Message for PipelineAckProto {
    fn is_initialized(&self) -> bool {
        if self.seqno.is_none() {
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
                    let tmp = try!(is.read_sint64());
                    self.seqno = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.reply));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.downstreamAckTimeNanos = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.flag));
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
        for value in self.seqno.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.reply.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in self.downstreamAckTimeNanos.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if !self.flag.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(4, &self.flag);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.seqno {
            try!(os.write_sint64(1, v));
        };
        for v in self.reply.iter() {
            try!(os.write_enum(2, *v as i32));
        };
        if let Some(v) = self.downstreamAckTimeNanos {
            try!(os.write_uint64(3, v));
        };
        if !self.flag.is_empty() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.flag)));
            for v in self.flag.iter() {
                try!(os.write_uint32_no_tag(*v));
            };
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
        ::std::any::TypeId::of::<PipelineAckProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PipelineAckProto {
    fn new() -> PipelineAckProto {
        PipelineAckProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<PipelineAckProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "seqno",
                    PipelineAckProto::has_seqno,
                    PipelineAckProto::get_seqno,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_enum_accessor(
                    "reply",
                    PipelineAckProto::get_reply,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "downstreamAckTimeNanos",
                    PipelineAckProto::has_downstreamAckTimeNanos,
                    PipelineAckProto::get_downstreamAckTimeNanos,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_u32_accessor(
                    "flag",
                    PipelineAckProto::get_flag,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PipelineAckProto>(
                    "PipelineAckProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PipelineAckProto {
    fn clear(&mut self) {
        self.clear_seqno();
        self.clear_reply();
        self.clear_downstreamAckTimeNanos();
        self.clear_flag();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PipelineAckProto {
    fn eq(&self, other: &PipelineAckProto) -> bool {
        self.seqno == other.seqno &&
        self.reply == other.reply &&
        self.downstreamAckTimeNanos == other.downstreamAckTimeNanos &&
        self.flag == other.flag &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PipelineAckProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ReadOpChecksumInfoProto {
    // message fields
    checksum: ::protobuf::SingularPtrField<ChecksumProto>,
    chunkOffset: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ReadOpChecksumInfoProto {
    pub fn new() -> ReadOpChecksumInfoProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReadOpChecksumInfoProto {
        static mut instance: ::protobuf::lazy::Lazy<ReadOpChecksumInfoProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReadOpChecksumInfoProto,
        };
        unsafe {
            instance.get(|| {
                ReadOpChecksumInfoProto {
                    checksum: ::protobuf::SingularPtrField::none(),
                    chunkOffset: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.ChecksumProto checksum = 1;

    pub fn clear_checksum(&mut self) {
        self.checksum.clear();
    }

    pub fn has_checksum(&self) -> bool {
        self.checksum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_checksum(&mut self, v: ChecksumProto) {
        self.checksum = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_checksum<'a>(&'a mut self) -> &'a mut ChecksumProto {
        if self.checksum.is_none() {
            self.checksum.set_default();
        };
        self.checksum.as_mut().unwrap()
    }

    // Take field
    pub fn take_checksum(&mut self) -> ChecksumProto {
        self.checksum.take().unwrap_or_else(|| ChecksumProto::new())
    }

    pub fn get_checksum<'a>(&'a self) -> &'a ChecksumProto {
        self.checksum.as_ref().unwrap_or_else(|| ChecksumProto::default_instance())
    }

    // required uint64 chunkOffset = 2;

    pub fn clear_chunkOffset(&mut self) {
        self.chunkOffset = ::std::option::Option::None;
    }

    pub fn has_chunkOffset(&self) -> bool {
        self.chunkOffset.is_some()
    }

    // Param is passed by value, moved
    pub fn set_chunkOffset(&mut self, v: u64) {
        self.chunkOffset = ::std::option::Option::Some(v);
    }

    pub fn get_chunkOffset<'a>(&self) -> u64 {
        self.chunkOffset.unwrap_or(0)
    }
}

impl ::protobuf::Message for ReadOpChecksumInfoProto {
    fn is_initialized(&self) -> bool {
        if self.checksum.is_none() {
            return false;
        };
        if self.chunkOffset.is_none() {
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
                    let tmp = self.checksum.set_default();
                    try!(is.merge_message(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.chunkOffset = ::std::option::Option::Some(tmp);
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
        for value in self.checksum.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.chunkOffset.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.checksum.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.chunkOffset {
            try!(os.write_uint64(2, v));
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
        ::std::any::TypeId::of::<ReadOpChecksumInfoProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReadOpChecksumInfoProto {
    fn new() -> ReadOpChecksumInfoProto {
        ReadOpChecksumInfoProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReadOpChecksumInfoProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "checksum",
                    ReadOpChecksumInfoProto::has_checksum,
                    ReadOpChecksumInfoProto::get_checksum,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "chunkOffset",
                    ReadOpChecksumInfoProto::has_chunkOffset,
                    ReadOpChecksumInfoProto::get_chunkOffset,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReadOpChecksumInfoProto>(
                    "ReadOpChecksumInfoProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReadOpChecksumInfoProto {
    fn clear(&mut self) {
        self.clear_checksum();
        self.clear_chunkOffset();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ReadOpChecksumInfoProto {
    fn eq(&self, other: &ReadOpChecksumInfoProto) -> bool {
        self.checksum == other.checksum &&
        self.chunkOffset == other.chunkOffset &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ReadOpChecksumInfoProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BlockOpResponseProto {
    // message fields
    status: ::std::option::Option<Status>,
    firstBadLink: ::protobuf::SingularField<::std::string::String>,
    checksumResponse: ::protobuf::SingularPtrField<OpBlockChecksumResponseProto>,
    readOpChecksumInfo: ::protobuf::SingularPtrField<ReadOpChecksumInfoProto>,
    message: ::protobuf::SingularField<::std::string::String>,
    shortCircuitAccessVersion: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl BlockOpResponseProto {
    pub fn new() -> BlockOpResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockOpResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<BlockOpResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockOpResponseProto,
        };
        unsafe {
            instance.get(|| {
                BlockOpResponseProto {
                    status: ::std::option::Option::None,
                    firstBadLink: ::protobuf::SingularField::none(),
                    checksumResponse: ::protobuf::SingularPtrField::none(),
                    readOpChecksumInfo: ::protobuf::SingularPtrField::none(),
                    message: ::protobuf::SingularField::none(),
                    shortCircuitAccessVersion: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status<'a>(&self) -> Status {
        self.status.unwrap_or(Status::SUCCESS)
    }

    // optional string firstBadLink = 2;

    pub fn clear_firstBadLink(&mut self) {
        self.firstBadLink.clear();
    }

    pub fn has_firstBadLink(&self) -> bool {
        self.firstBadLink.is_some()
    }

    // Param is passed by value, moved
    pub fn set_firstBadLink(&mut self, v: ::std::string::String) {
        self.firstBadLink = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_firstBadLink<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.firstBadLink.is_none() {
            self.firstBadLink.set_default();
        };
        self.firstBadLink.as_mut().unwrap()
    }

    // Take field
    pub fn take_firstBadLink(&mut self) -> ::std::string::String {
        self.firstBadLink.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_firstBadLink<'a>(&'a self) -> &'a str {
        match self.firstBadLink.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .hadoop.hdfs.OpBlockChecksumResponseProto checksumResponse = 3;

    pub fn clear_checksumResponse(&mut self) {
        self.checksumResponse.clear();
    }

    pub fn has_checksumResponse(&self) -> bool {
        self.checksumResponse.is_some()
    }

    // Param is passed by value, moved
    pub fn set_checksumResponse(&mut self, v: OpBlockChecksumResponseProto) {
        self.checksumResponse = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_checksumResponse<'a>(&'a mut self) -> &'a mut OpBlockChecksumResponseProto {
        if self.checksumResponse.is_none() {
            self.checksumResponse.set_default();
        };
        self.checksumResponse.as_mut().unwrap()
    }

    // Take field
    pub fn take_checksumResponse(&mut self) -> OpBlockChecksumResponseProto {
        self.checksumResponse.take().unwrap_or_else(|| OpBlockChecksumResponseProto::new())
    }

    pub fn get_checksumResponse<'a>(&'a self) -> &'a OpBlockChecksumResponseProto {
        self.checksumResponse.as_ref().unwrap_or_else(|| OpBlockChecksumResponseProto::default_instance())
    }

    // optional .hadoop.hdfs.ReadOpChecksumInfoProto readOpChecksumInfo = 4;

    pub fn clear_readOpChecksumInfo(&mut self) {
        self.readOpChecksumInfo.clear();
    }

    pub fn has_readOpChecksumInfo(&self) -> bool {
        self.readOpChecksumInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_readOpChecksumInfo(&mut self, v: ReadOpChecksumInfoProto) {
        self.readOpChecksumInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_readOpChecksumInfo<'a>(&'a mut self) -> &'a mut ReadOpChecksumInfoProto {
        if self.readOpChecksumInfo.is_none() {
            self.readOpChecksumInfo.set_default();
        };
        self.readOpChecksumInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_readOpChecksumInfo(&mut self) -> ReadOpChecksumInfoProto {
        self.readOpChecksumInfo.take().unwrap_or_else(|| ReadOpChecksumInfoProto::new())
    }

    pub fn get_readOpChecksumInfo<'a>(&'a self) -> &'a ReadOpChecksumInfoProto {
        self.readOpChecksumInfo.as_ref().unwrap_or_else(|| ReadOpChecksumInfoProto::default_instance())
    }

    // optional string message = 5;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.message.is_none() {
            self.message.set_default();
        };
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        self.message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message<'a>(&'a self) -> &'a str {
        match self.message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional uint32 shortCircuitAccessVersion = 6;

    pub fn clear_shortCircuitAccessVersion(&mut self) {
        self.shortCircuitAccessVersion = ::std::option::Option::None;
    }

    pub fn has_shortCircuitAccessVersion(&self) -> bool {
        self.shortCircuitAccessVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_shortCircuitAccessVersion(&mut self, v: u32) {
        self.shortCircuitAccessVersion = ::std::option::Option::Some(v);
    }

    pub fn get_shortCircuitAccessVersion<'a>(&self) -> u32 {
        self.shortCircuitAccessVersion.unwrap_or(0)
    }
}

impl ::protobuf::Message for BlockOpResponseProto {
    fn is_initialized(&self) -> bool {
        if self.status.is_none() {
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
                    self.status = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.firstBadLink.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.checksumResponse.set_default();
                    try!(is.merge_message(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.readOpChecksumInfo.set_default();
                    try!(is.merge_message(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.message.set_default();
                    try!(is.read_string_into(tmp))
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint32());
                    self.shortCircuitAccessVersion = ::std::option::Option::Some(tmp);
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
        for value in self.status.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.firstBadLink.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.checksumResponse.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.readOpChecksumInfo.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.message.iter() {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        for value in self.shortCircuitAccessVersion.iter() {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.firstBadLink.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.checksumResponse.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.readOpChecksumInfo.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.message.as_ref() {
            try!(os.write_string(5, &v));
        };
        if let Some(v) = self.shortCircuitAccessVersion {
            try!(os.write_uint32(6, v));
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
        ::std::any::TypeId::of::<BlockOpResponseProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BlockOpResponseProto {
    fn new() -> BlockOpResponseProto {
        BlockOpResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockOpResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "status",
                    BlockOpResponseProto::has_status,
                    BlockOpResponseProto::get_status,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "firstBadLink",
                    BlockOpResponseProto::has_firstBadLink,
                    BlockOpResponseProto::get_firstBadLink,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "checksumResponse",
                    BlockOpResponseProto::has_checksumResponse,
                    BlockOpResponseProto::get_checksumResponse,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "readOpChecksumInfo",
                    BlockOpResponseProto::has_readOpChecksumInfo,
                    BlockOpResponseProto::get_readOpChecksumInfo,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "message",
                    BlockOpResponseProto::has_message,
                    BlockOpResponseProto::get_message,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "shortCircuitAccessVersion",
                    BlockOpResponseProto::has_shortCircuitAccessVersion,
                    BlockOpResponseProto::get_shortCircuitAccessVersion,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockOpResponseProto>(
                    "BlockOpResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockOpResponseProto {
    fn clear(&mut self) {
        self.clear_status();
        self.clear_firstBadLink();
        self.clear_checksumResponse();
        self.clear_readOpChecksumInfo();
        self.clear_message();
        self.clear_shortCircuitAccessVersion();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BlockOpResponseProto {
    fn eq(&self, other: &BlockOpResponseProto) -> bool {
        self.status == other.status &&
        self.firstBadLink == other.firstBadLink &&
        self.checksumResponse == other.checksumResponse &&
        self.readOpChecksumInfo == other.readOpChecksumInfo &&
        self.message == other.message &&
        self.shortCircuitAccessVersion == other.shortCircuitAccessVersion &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BlockOpResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ClientReadStatusProto {
    // message fields
    status: ::std::option::Option<Status>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ClientReadStatusProto {
    pub fn new() -> ClientReadStatusProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClientReadStatusProto {
        static mut instance: ::protobuf::lazy::Lazy<ClientReadStatusProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClientReadStatusProto,
        };
        unsafe {
            instance.get(|| {
                ClientReadStatusProto {
                    status: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status<'a>(&self) -> Status {
        self.status.unwrap_or(Status::SUCCESS)
    }
}

impl ::protobuf::Message for ClientReadStatusProto {
    fn is_initialized(&self) -> bool {
        if self.status.is_none() {
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
                    self.status = ::std::option::Option::Some(tmp);
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
        for value in self.status.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            try!(os.write_enum(1, v as i32));
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
        ::std::any::TypeId::of::<ClientReadStatusProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ClientReadStatusProto {
    fn new() -> ClientReadStatusProto {
        ClientReadStatusProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClientReadStatusProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "status",
                    ClientReadStatusProto::has_status,
                    ClientReadStatusProto::get_status,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClientReadStatusProto>(
                    "ClientReadStatusProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClientReadStatusProto {
    fn clear(&mut self) {
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ClientReadStatusProto {
    fn eq(&self, other: &ClientReadStatusProto) -> bool {
        self.status == other.status &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ClientReadStatusProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DNTransferAckProto {
    // message fields
    status: ::std::option::Option<Status>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl DNTransferAckProto {
    pub fn new() -> DNTransferAckProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DNTransferAckProto {
        static mut instance: ::protobuf::lazy::Lazy<DNTransferAckProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DNTransferAckProto,
        };
        unsafe {
            instance.get(|| {
                DNTransferAckProto {
                    status: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.Status status = 1;

    pub fn clear_status(&mut self) {
        self.status = ::std::option::Option::None;
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Status) {
        self.status = ::std::option::Option::Some(v);
    }

    pub fn get_status<'a>(&self) -> Status {
        self.status.unwrap_or(Status::SUCCESS)
    }
}

impl ::protobuf::Message for DNTransferAckProto {
    fn is_initialized(&self) -> bool {
        if self.status.is_none() {
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
                    self.status = ::std::option::Option::Some(tmp);
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
        for value in self.status.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.status {
            try!(os.write_enum(1, v as i32));
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
        ::std::any::TypeId::of::<DNTransferAckProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DNTransferAckProto {
    fn new() -> DNTransferAckProto {
        DNTransferAckProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DNTransferAckProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "status",
                    DNTransferAckProto::has_status,
                    DNTransferAckProto::get_status,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DNTransferAckProto>(
                    "DNTransferAckProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DNTransferAckProto {
    fn clear(&mut self) {
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DNTransferAckProto {
    fn eq(&self, other: &DNTransferAckProto) -> bool {
        self.status == other.status &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DNTransferAckProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct OpBlockChecksumResponseProto {
    // message fields
    bytesPerCrc: ::std::option::Option<u32>,
    crcPerBlock: ::std::option::Option<u64>,
    md5: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    crcType: ::std::option::Option<ChecksumTypeProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl OpBlockChecksumResponseProto {
    pub fn new() -> OpBlockChecksumResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OpBlockChecksumResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<OpBlockChecksumResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OpBlockChecksumResponseProto,
        };
        unsafe {
            instance.get(|| {
                OpBlockChecksumResponseProto {
                    bytesPerCrc: ::std::option::Option::None,
                    crcPerBlock: ::std::option::Option::None,
                    md5: ::protobuf::SingularField::none(),
                    crcType: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required uint32 bytesPerCrc = 1;

    pub fn clear_bytesPerCrc(&mut self) {
        self.bytesPerCrc = ::std::option::Option::None;
    }

    pub fn has_bytesPerCrc(&self) -> bool {
        self.bytesPerCrc.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bytesPerCrc(&mut self, v: u32) {
        self.bytesPerCrc = ::std::option::Option::Some(v);
    }

    pub fn get_bytesPerCrc<'a>(&self) -> u32 {
        self.bytesPerCrc.unwrap_or(0)
    }

    // required uint64 crcPerBlock = 2;

    pub fn clear_crcPerBlock(&mut self) {
        self.crcPerBlock = ::std::option::Option::None;
    }

    pub fn has_crcPerBlock(&self) -> bool {
        self.crcPerBlock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_crcPerBlock(&mut self, v: u64) {
        self.crcPerBlock = ::std::option::Option::Some(v);
    }

    pub fn get_crcPerBlock<'a>(&self) -> u64 {
        self.crcPerBlock.unwrap_or(0)
    }

    // required bytes md5 = 3;

    pub fn clear_md5(&mut self) {
        self.md5.clear();
    }

    pub fn has_md5(&self) -> bool {
        self.md5.is_some()
    }

    // Param is passed by value, moved
    pub fn set_md5(&mut self, v: ::std::vec::Vec<u8>) {
        self.md5 = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_md5<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.md5.is_none() {
            self.md5.set_default();
        };
        self.md5.as_mut().unwrap()
    }

    // Take field
    pub fn take_md5(&mut self) -> ::std::vec::Vec<u8> {
        self.md5.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_md5<'a>(&'a self) -> &'a [u8] {
        match self.md5.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional .hadoop.hdfs.ChecksumTypeProto crcType = 4;

    pub fn clear_crcType(&mut self) {
        self.crcType = ::std::option::Option::None;
    }

    pub fn has_crcType(&self) -> bool {
        self.crcType.is_some()
    }

    // Param is passed by value, moved
    pub fn set_crcType(&mut self, v: ChecksumTypeProto) {
        self.crcType = ::std::option::Option::Some(v);
    }

    pub fn get_crcType<'a>(&self) -> ChecksumTypeProto {
        self.crcType.unwrap_or(ChecksumTypeProto::CHECKSUM_NULL)
    }
}

impl ::protobuf::Message for OpBlockChecksumResponseProto {
    fn is_initialized(&self) -> bool {
        if self.bytesPerCrc.is_none() {
            return false;
        };
        if self.crcPerBlock.is_none() {
            return false;
        };
        if self.md5.is_none() {
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
                    self.bytesPerCrc = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_uint64());
                    self.crcPerBlock = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.md5.set_default();
                    try!(is.read_bytes_into(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.crcType = ::std::option::Option::Some(tmp);
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
        for value in self.bytesPerCrc.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.crcPerBlock.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.md5.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in self.crcType.iter() {
            my_size += ::protobuf::rt::enum_size(4, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bytesPerCrc {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.crcPerBlock {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.md5.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        if let Some(v) = self.crcType {
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
        ::std::any::TypeId::of::<OpBlockChecksumResponseProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for OpBlockChecksumResponseProto {
    fn new() -> OpBlockChecksumResponseProto {
        OpBlockChecksumResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<OpBlockChecksumResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "bytesPerCrc",
                    OpBlockChecksumResponseProto::has_bytesPerCrc,
                    OpBlockChecksumResponseProto::get_bytesPerCrc,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "crcPerBlock",
                    OpBlockChecksumResponseProto::has_crcPerBlock,
                    OpBlockChecksumResponseProto::get_crcPerBlock,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "md5",
                    OpBlockChecksumResponseProto::has_md5,
                    OpBlockChecksumResponseProto::get_md5,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "crcType",
                    OpBlockChecksumResponseProto::has_crcType,
                    OpBlockChecksumResponseProto::get_crcType,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OpBlockChecksumResponseProto>(
                    "OpBlockChecksumResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OpBlockChecksumResponseProto {
    fn clear(&mut self) {
        self.clear_bytesPerCrc();
        self.clear_crcPerBlock();
        self.clear_md5();
        self.clear_crcType();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for OpBlockChecksumResponseProto {
    fn eq(&self, other: &OpBlockChecksumResponseProto) -> bool {
        self.bytesPerCrc == other.bytesPerCrc &&
        self.crcPerBlock == other.crcPerBlock &&
        self.md5 == other.md5 &&
        self.crcType == other.crcType &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for OpBlockChecksumResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Status {
    SUCCESS = 0,
    ERROR = 1,
    ERROR_CHECKSUM = 2,
    ERROR_INVALID = 3,
    ERROR_EXISTS = 4,
    ERROR_ACCESS_TOKEN = 5,
    CHECKSUM_OK = 6,
    ERROR_UNSUPPORTED = 7,
    OOB_RESTART = 8,
    OOB_RESERVED1 = 9,
    OOB_RESERVED2 = 10,
    OOB_RESERVED3 = 11,
    IN_PROGRESS = 12,
}

impl ::protobuf::ProtobufEnum for Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Status> {
        match value {
            0 => ::std::option::Option::Some(Status::SUCCESS),
            1 => ::std::option::Option::Some(Status::ERROR),
            2 => ::std::option::Option::Some(Status::ERROR_CHECKSUM),
            3 => ::std::option::Option::Some(Status::ERROR_INVALID),
            4 => ::std::option::Option::Some(Status::ERROR_EXISTS),
            5 => ::std::option::Option::Some(Status::ERROR_ACCESS_TOKEN),
            6 => ::std::option::Option::Some(Status::CHECKSUM_OK),
            7 => ::std::option::Option::Some(Status::ERROR_UNSUPPORTED),
            8 => ::std::option::Option::Some(Status::OOB_RESTART),
            9 => ::std::option::Option::Some(Status::OOB_RESERVED1),
            10 => ::std::option::Option::Some(Status::OOB_RESERVED2),
            11 => ::std::option::Option::Some(Status::OOB_RESERVED3),
            12 => ::std::option::Option::Some(Status::IN_PROGRESS),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<Status>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Status", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Status {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ShortCircuitFdResponse {
    DO_NOT_USE_RECEIPT_VERIFICATION = 0,
    USE_RECEIPT_VERIFICATION = 1,
}

impl ::protobuf::ProtobufEnum for ShortCircuitFdResponse {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ShortCircuitFdResponse> {
        match value {
            0 => ::std::option::Option::Some(ShortCircuitFdResponse::DO_NOT_USE_RECEIPT_VERIFICATION),
            1 => ::std::option::Option::Some(ShortCircuitFdResponse::USE_RECEIPT_VERIFICATION),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<ShortCircuitFdResponse>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ShortCircuitFdResponse", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ShortCircuitFdResponse {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x12, 0x64, 0x61, 0x74, 0x61, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66,
    0x73, 0x1a, 0x0e, 0x53, 0x65, 0x63, 0x75, 0x72, 0x69, 0x74, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x0a, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xa5, 0x02,
    0x0a, 0x21, 0x44, 0x61, 0x74, 0x61, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x45, 0x6e,
    0x63, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x5a, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x0e, 0x32, 0x4a, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66,
    0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x45, 0x6e,
    0x63, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72,
    0x45, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x72, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12,
    0x0f, 0x0a, 0x07, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c,
    0x12, 0x0f, 0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x09, 0x12, 0x34, 0x0a, 0x0c, 0x63, 0x69, 0x70, 0x68, 0x65, 0x72, 0x4f, 0x70, 0x74, 0x69, 0x6f,
    0x6e, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70,
    0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x43, 0x69, 0x70, 0x68, 0x65, 0x72, 0x4f, 0x70, 0x74, 0x69,
    0x6f, 0x6e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x4c, 0x0a, 0x1b, 0x44, 0x61, 0x74, 0x61, 0x54,
    0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x45, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x72,
    0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53,
    0x53, 0x10, 0x00, 0x12, 0x15, 0x0a, 0x11, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x55, 0x4e, 0x4b,
    0x4e, 0x4f, 0x57, 0x4e, 0x5f, 0x4b, 0x45, 0x59, 0x10, 0x01, 0x12, 0x09, 0x0a, 0x05, 0x45, 0x52,
    0x52, 0x4f, 0x52, 0x10, 0x02, 0x22, 0xa7, 0x01, 0x0a, 0x0f, 0x42, 0x61, 0x73, 0x65, 0x48, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2e, 0x0a, 0x05, 0x62, 0x6c, 0x6f,
    0x63, 0x6b, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f,
    0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x45, 0x78, 0x74, 0x65, 0x6e, 0x64, 0x65, 0x64, 0x42,
    0x6c, 0x6f, 0x63, 0x6b, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x28, 0x0a, 0x05, 0x74, 0x6f, 0x6b,
    0x65, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f,
    0x70, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x54, 0x6f, 0x6b, 0x65, 0x6e, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x3a, 0x0a, 0x09, 0x74, 0x72, 0x61, 0x63, 0x65, 0x49, 0x6e, 0x66, 0x6f,
    0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x27, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e,
    0x68, 0x64, 0x66, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65,
    0x72, 0x54, 0x72, 0x61, 0x63, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22,
    0x3f, 0x0a, 0x1a, 0x44, 0x61, 0x74, 0x61, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x54,
    0x72, 0x61, 0x63, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0f, 0x0a,
    0x07, 0x74, 0x72, 0x61, 0x63, 0x65, 0x49, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x12, 0x10,
    0x0a, 0x08, 0x70, 0x61, 0x72, 0x65, 0x6e, 0x74, 0x49, 0x64, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04,
    0x22, 0x62, 0x0a, 0x1a, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4f, 0x70, 0x65, 0x72, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x30,
    0x0a, 0x0a, 0x62, 0x61, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73,
    0x2e, 0x42, 0x61, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x12, 0x0a, 0x0a, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4e, 0x61, 0x6d, 0x65, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x09, 0x22, 0x3d, 0x0a, 0x14, 0x43, 0x61, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x53,
    0x74, 0x72, 0x61, 0x74, 0x65, 0x67, 0x79, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x12, 0x0a, 0x0a,
    0x64, 0x72, 0x6f, 0x70, 0x42, 0x65, 0x68, 0x69, 0x6e, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08,
    0x12, 0x11, 0x0a, 0x09, 0x72, 0x65, 0x61, 0x64, 0x61, 0x68, 0x65, 0x61, 0x64, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x03, 0x22, 0xc1, 0x01, 0x0a, 0x10, 0x4f, 0x70, 0x52, 0x65, 0x61, 0x64, 0x42, 0x6c,
    0x6f, 0x63, 0x6b, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x37, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64,
    0x65, 0x72, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x27, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f,
    0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4f, 0x70, 0x65,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x0e, 0x0a, 0x06, 0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x18, 0x02, 0x20, 0x02, 0x28,
    0x04, 0x12, 0x0b, 0x0a, 0x03, 0x6c, 0x65, 0x6e, 0x18, 0x03, 0x20, 0x02, 0x28, 0x04, 0x12, 0x1b,
    0x0a, 0x0d, 0x73, 0x65, 0x6e, 0x64, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x73, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x04, 0x74, 0x72, 0x75, 0x65, 0x12, 0x3a, 0x0a, 0x0f, 0x63,
    0x61, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x53, 0x74, 0x72, 0x61, 0x74, 0x65, 0x67, 0x79, 0x18, 0x05,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64,
    0x66, 0x73, 0x2e, 0x43, 0x61, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x53, 0x74, 0x72, 0x61, 0x74, 0x65,
    0x67, 0x79, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x57, 0x0a, 0x0d, 0x43, 0x68, 0x65, 0x63, 0x6b,
    0x73, 0x75, 0x6d, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2c, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x1e, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e,
    0x68, 0x64, 0x66, 0x73, 0x2e, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x54, 0x79, 0x70,
    0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x18, 0x0a, 0x10, 0x62, 0x79, 0x74, 0x65, 0x73, 0x50,
    0x65, 0x72, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0d,
    0x22, 0x98, 0x07, 0x0a, 0x11, 0x4f, 0x70, 0x57, 0x72, 0x69, 0x74, 0x65, 0x42, 0x6c, 0x6f, 0x63,
    0x6b, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x37, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x27, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e,
    0x68, 0x64, 0x66, 0x73, 0x2e, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4f, 0x70, 0x65, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x2f, 0x0a, 0x07, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x1e, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x44,
    0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x2e, 0x0a, 0x06, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x1e, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x44,
    0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x44, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x67, 0x65, 0x18, 0x04, 0x20, 0x02, 0x28, 0x0e, 0x32,
    0x35, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x4f, 0x70,
    0x57, 0x72, 0x69, 0x74, 0x65, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x2e,
    0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x43, 0x6f, 0x6e, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x53, 0x74, 0x61, 0x67, 0x65, 0x12, 0x14, 0x0a, 0x0c, 0x70, 0x69, 0x70, 0x65, 0x6c, 0x69,
    0x6e, 0x65, 0x53, 0x69, 0x7a, 0x65, 0x18, 0x05, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x14, 0x0a, 0x0c,
    0x6d, 0x69, 0x6e, 0x42, 0x79, 0x74, 0x65, 0x73, 0x52, 0x63, 0x76, 0x64, 0x18, 0x06, 0x20, 0x02,
    0x28, 0x04, 0x12, 0x14, 0x0a, 0x0c, 0x6d, 0x61, 0x78, 0x42, 0x79, 0x74, 0x65, 0x73, 0x52, 0x63,
    0x76, 0x64, 0x18, 0x07, 0x20, 0x02, 0x28, 0x04, 0x12, 0x1d, 0x0a, 0x15, 0x6c, 0x61, 0x74, 0x65,
    0x73, 0x74, 0x47, 0x65, 0x6e, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x6d,
    0x70, 0x18, 0x08, 0x20, 0x02, 0x28, 0x04, 0x12, 0x35, 0x0a, 0x11, 0x72, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x65, 0x64, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x18, 0x09, 0x20, 0x02,
    0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73,
    0x2e, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x3a,
    0x0a, 0x0f, 0x63, 0x61, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x53, 0x74, 0x72, 0x61, 0x74, 0x65, 0x67,
    0x79, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70,
    0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x43, 0x61, 0x63, 0x68, 0x69, 0x6e, 0x67, 0x53, 0x74, 0x72,
    0x61, 0x74, 0x65, 0x67, 0x79, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x38, 0x0a, 0x0b, 0x73, 0x74,
    0x6f, 0x72, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0e, 0x32,
    0x1d, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x53, 0x74,
    0x6f, 0x72, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x3a, 0x04,
    0x44, 0x49, 0x53, 0x4b, 0x12, 0x39, 0x0a, 0x12, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x53, 0x74,
    0x6f, 0x72, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x73, 0x18, 0x0c, 0x20, 0x03, 0x28, 0x0e,
    0x32, 0x1d, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x53,
    0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x1f, 0x0a, 0x10, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x4c, 0x61, 0x7a, 0x79, 0x50, 0x65, 0x72, 0x73,
    0x69, 0x73, 0x74, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x05, 0x66, 0x61, 0x6c, 0x73, 0x65,
    0x12, 0x16, 0x0a, 0x07, 0x70, 0x69, 0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x18, 0x0e, 0x20, 0x01, 0x28,
    0x08, 0x3a, 0x05, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x12, 0x16, 0x0a, 0x0e, 0x74, 0x61, 0x72, 0x67,
    0x65, 0x74, 0x50, 0x69, 0x6e, 0x6e, 0x69, 0x6e, 0x67, 0x73, 0x18, 0x0f, 0x20, 0x03, 0x28, 0x08,
    0x22, 0x88, 0x02, 0x0a, 0x16, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x43, 0x6f, 0x6e, 0x73, 0x74, 0x72,
    0x75, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x67, 0x65, 0x12, 0x19, 0x0a, 0x15, 0x50,
    0x49, 0x50, 0x45, 0x4c, 0x49, 0x4e, 0x45, 0x5f, 0x53, 0x45, 0x54, 0x55, 0x50, 0x5f, 0x41, 0x50,
    0x50, 0x45, 0x4e, 0x44, 0x10, 0x00, 0x12, 0x22, 0x0a, 0x1e, 0x50, 0x49, 0x50, 0x45, 0x4c, 0x49,
    0x4e, 0x45, 0x5f, 0x53, 0x45, 0x54, 0x55, 0x50, 0x5f, 0x41, 0x50, 0x50, 0x45, 0x4e, 0x44, 0x5f,
    0x52, 0x45, 0x43, 0x4f, 0x56, 0x45, 0x52, 0x59, 0x10, 0x01, 0x12, 0x12, 0x0a, 0x0e, 0x44, 0x41,
    0x54, 0x41, 0x5f, 0x53, 0x54, 0x52, 0x45, 0x41, 0x4d, 0x49, 0x4e, 0x47, 0x10, 0x02, 0x12, 0x25,
    0x0a, 0x21, 0x50, 0x49, 0x50, 0x45, 0x4c, 0x49, 0x4e, 0x45, 0x5f, 0x53, 0x45, 0x54, 0x55, 0x50,
    0x5f, 0x53, 0x54, 0x52, 0x45, 0x41, 0x4d, 0x49, 0x4e, 0x47, 0x5f, 0x52, 0x45, 0x43, 0x4f, 0x56,
    0x45, 0x52, 0x59, 0x10, 0x03, 0x12, 0x12, 0x0a, 0x0e, 0x50, 0x49, 0x50, 0x45, 0x4c, 0x49, 0x4e,
    0x45, 0x5f, 0x43, 0x4c, 0x4f, 0x53, 0x45, 0x10, 0x04, 0x12, 0x1b, 0x0a, 0x17, 0x50, 0x49, 0x50,
    0x45, 0x4c, 0x49, 0x4e, 0x45, 0x5f, 0x43, 0x4c, 0x4f, 0x53, 0x45, 0x5f, 0x52, 0x45, 0x43, 0x4f,
    0x56, 0x45, 0x52, 0x59, 0x10, 0x05, 0x12, 0x19, 0x0a, 0x15, 0x50, 0x49, 0x50, 0x45, 0x4c, 0x49,
    0x4e, 0x45, 0x5f, 0x53, 0x45, 0x54, 0x55, 0x50, 0x5f, 0x43, 0x52, 0x45, 0x41, 0x54, 0x45, 0x10,
    0x06, 0x12, 0x10, 0x0a, 0x0c, 0x54, 0x52, 0x41, 0x4e, 0x53, 0x46, 0x45, 0x52, 0x5f, 0x52, 0x42,
    0x57, 0x10, 0x07, 0x12, 0x16, 0x0a, 0x12, 0x54, 0x52, 0x41, 0x4e, 0x53, 0x46, 0x45, 0x52, 0x5f,
    0x46, 0x49, 0x4e, 0x41, 0x4c, 0x49, 0x5a, 0x45, 0x44, 0x10, 0x08, 0x22, 0xbb, 0x01, 0x0a, 0x14,
    0x4f, 0x70, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x37, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x0b, 0x32, 0x27, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64,
    0x66, 0x73, 0x2e, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x4f, 0x70, 0x65, 0x72, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2f, 0x0a,
    0x07, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1e,
    0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x44, 0x61, 0x74,
    0x61, 0x6e, 0x6f, 0x64, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x39,
    0x0a, 0x12, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x54,
    0x79, 0x70, 0x65, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0e, 0x32, 0x1d, 0x2e, 0x68, 0x61, 0x64,
    0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65,
    0x54, 0x79, 0x70, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xbe, 0x01, 0x0a, 0x13, 0x4f, 0x70,
    0x52, 0x65, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x2c, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x0b, 0x32, 0x1c, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e,
    0x42, 0x61, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x0f, 0x0a, 0x07, 0x64, 0x65, 0x6c, 0x48, 0x69, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09,
    0x12, 0x2e, 0x0a, 0x06, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0b,
    0x32, 0x1e, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x44,
    0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x38, 0x0a, 0x0b, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1d, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68,
    0x64, 0x66, 0x73, 0x2e, 0x53, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x3a, 0x04, 0x44, 0x49, 0x53, 0x4b, 0x22, 0x40, 0x0a, 0x10, 0x4f, 0x70,
    0x43, 0x6f, 0x70, 0x79, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2c,
    0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x1c,
    0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x42, 0x61, 0x73,
    0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x44, 0x0a, 0x14,
    0x4f, 0x70, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2c, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64,
    0x66, 0x73, 0x2e, 0x42, 0x61, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x22, 0x30, 0x0a, 0x16, 0x53, 0x68, 0x6f, 0x72, 0x74, 0x43, 0x69, 0x72, 0x63, 0x75,
    0x69, 0x74, 0x53, 0x68, 0x6d, 0x49, 0x64, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0a, 0x0a, 0x02,
    0x68, 0x69, 0x18, 0x01, 0x20, 0x02, 0x28, 0x03, 0x12, 0x0a, 0x0a, 0x02, 0x6c, 0x6f, 0x18, 0x02,
    0x20, 0x02, 0x28, 0x03, 0x22, 0x5f, 0x0a, 0x18, 0x53, 0x68, 0x6f, 0x72, 0x74, 0x43, 0x69, 0x72,
    0x63, 0x75, 0x69, 0x74, 0x53, 0x68, 0x6d, 0x53, 0x6c, 0x6f, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x32, 0x0a, 0x05, 0x73, 0x68, 0x6d, 0x49, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32,
    0x23, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x53, 0x68,
    0x6f, 0x72, 0x74, 0x43, 0x69, 0x72, 0x63, 0x75, 0x69, 0x74, 0x53, 0x68, 0x6d, 0x49, 0x64, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0f, 0x0a, 0x07, 0x73, 0x6c, 0x6f, 0x74, 0x49, 0x64, 0x78, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x05, 0x22, 0xc7, 0x01, 0x0a, 0x20, 0x4f, 0x70, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x53, 0x68, 0x6f, 0x72, 0x74, 0x43, 0x69, 0x72, 0x63, 0x75, 0x69, 0x74, 0x41,
    0x63, 0x63, 0x65, 0x73, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2c, 0x0a, 0x06, 0x68, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x68, 0x61, 0x64,
    0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x42, 0x61, 0x73, 0x65, 0x48, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x12, 0x0a, 0x0a, 0x6d, 0x61, 0x78, 0x56,
    0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x35, 0x0a, 0x06,
    0x73, 0x6c, 0x6f, 0x74, 0x49, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x68,
    0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x53, 0x68, 0x6f, 0x72, 0x74,
    0x43, 0x69, 0x72, 0x63, 0x75, 0x69, 0x74, 0x53, 0x68, 0x6d, 0x53, 0x6c, 0x6f, 0x74, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x2a, 0x0a, 0x1b, 0x73, 0x75, 0x70, 0x70, 0x6f, 0x72, 0x74, 0x73, 0x52,
    0x65, 0x63, 0x65, 0x69, 0x70, 0x74, 0x56, 0x65, 0x72, 0x69, 0x66, 0x69, 0x63, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x05, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x22,
    0x9a, 0x01, 0x0a, 0x25, 0x52, 0x65, 0x6c, 0x65, 0x61, 0x73, 0x65, 0x53, 0x68, 0x6f, 0x72, 0x74,
    0x43, 0x69, 0x72, 0x63, 0x75, 0x69, 0x74, 0x41, 0x63, 0x63, 0x65, 0x73, 0x73, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x35, 0x0a, 0x06, 0x73, 0x6c, 0x6f,
    0x74, 0x49, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x68, 0x61, 0x64, 0x6f,
    0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x53, 0x68, 0x6f, 0x72, 0x74, 0x43, 0x69, 0x72,
    0x63, 0x75, 0x69, 0x74, 0x53, 0x68, 0x6d, 0x53, 0x6c, 0x6f, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x3a, 0x0a, 0x09, 0x74, 0x72, 0x61, 0x63, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x27, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66,
    0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x54, 0x72,
    0x61, 0x63, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x5c, 0x0a, 0x26,
    0x52, 0x65, 0x6c, 0x65, 0x61, 0x73, 0x65, 0x53, 0x68, 0x6f, 0x72, 0x74, 0x43, 0x69, 0x72, 0x63,
    0x75, 0x69, 0x74, 0x41, 0x63, 0x63, 0x65, 0x73, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x23, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x13, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e,
    0x68, 0x64, 0x66, 0x73, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x0d, 0x0a, 0x05, 0x65,
    0x72, 0x72, 0x6f, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x22, 0x6d, 0x0a, 0x1b, 0x53, 0x68,
    0x6f, 0x72, 0x74, 0x43, 0x69, 0x72, 0x63, 0x75, 0x69, 0x74, 0x53, 0x68, 0x6d, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x12, 0x0a, 0x0a, 0x63, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x4e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x3a, 0x0a,
    0x09, 0x74, 0x72, 0x61, 0x63, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x27, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x44,
    0x61, 0x74, 0x61, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x54, 0x72, 0x61, 0x63, 0x65,
    0x49, 0x6e, 0x66, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x83, 0x01, 0x0a, 0x1c, 0x53, 0x68,
    0x6f, 0x72, 0x74, 0x43, 0x69, 0x72, 0x63, 0x75, 0x69, 0x74, 0x53, 0x68, 0x6d, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x23, 0x0a, 0x06, 0x73, 0x74,
    0x61, 0x74, 0x75, 0x73, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x13, 0x2e, 0x68, 0x61, 0x64,
    0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12,
    0x0d, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x2f,
    0x0a, 0x02, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x68, 0x61, 0x64,
    0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x53, 0x68, 0x6f, 0x72, 0x74, 0x43, 0x69,
    0x72, 0x63, 0x75, 0x69, 0x74, 0x53, 0x68, 0x6d, 0x49, 0x64, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22,
    0x7f, 0x0a, 0x11, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x15, 0x0a, 0x0d, 0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x49, 0x6e,
    0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x01, 0x20, 0x02, 0x28, 0x10, 0x12, 0x0d, 0x0a, 0x05, 0x73,
    0x65, 0x71, 0x6e, 0x6f, 0x18, 0x02, 0x20, 0x02, 0x28, 0x10, 0x12, 0x19, 0x0a, 0x11, 0x6c, 0x61,
    0x73, 0x74, 0x50, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x49, 0x6e, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x18,
    0x03, 0x20, 0x02, 0x28, 0x08, 0x12, 0x0f, 0x0a, 0x07, 0x64, 0x61, 0x74, 0x61, 0x4c, 0x65, 0x6e,
    0x18, 0x04, 0x20, 0x02, 0x28, 0x0f, 0x12, 0x18, 0x0a, 0x09, 0x73, 0x79, 0x6e, 0x63, 0x42, 0x6c,
    0x6f, 0x63, 0x6b, 0x18, 0x05, 0x20, 0x01, 0x28, 0x08, 0x3a, 0x05, 0x66, 0x61, 0x6c, 0x73, 0x65,
    0x22, 0x7a, 0x0a, 0x10, 0x50, 0x69, 0x70, 0x65, 0x6c, 0x69, 0x6e, 0x65, 0x41, 0x63, 0x6b, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0d, 0x0a, 0x05, 0x73, 0x65, 0x71, 0x6e, 0x6f, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x12, 0x12, 0x22, 0x0a, 0x05, 0x72, 0x65, 0x70, 0x6c, 0x79, 0x18, 0x02, 0x20, 0x03,
    0x28, 0x0e, 0x32, 0x13, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73,
    0x2e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x21, 0x0a, 0x16, 0x64, 0x6f, 0x77, 0x6e, 0x73,
    0x74, 0x72, 0x65, 0x61, 0x6d, 0x41, 0x63, 0x6b, 0x54, 0x69, 0x6d, 0x65, 0x4e, 0x61, 0x6e, 0x6f,
    0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x3a, 0x01, 0x30, 0x12, 0x10, 0x0a, 0x04, 0x66, 0x6c,
    0x61, 0x67, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0d, 0x42, 0x02, 0x10, 0x01, 0x22, 0x5c, 0x0a, 0x17,
    0x52, 0x65, 0x61, 0x64, 0x4f, 0x70, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x49, 0x6e,
    0x66, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2c, 0x0a, 0x08, 0x63, 0x68, 0x65, 0x63, 0x6b,
    0x73, 0x75, 0x6d, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x68, 0x61, 0x64, 0x6f,
    0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x13, 0x0a, 0x0b, 0x63, 0x68, 0x75, 0x6e, 0x6b, 0x4f, 0x66,
    0x66, 0x73, 0x65, 0x74, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x22, 0x8c, 0x02, 0x0a, 0x14, 0x42,
    0x6c, 0x6f, 0x63, 0x6b, 0x4f, 0x70, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x23, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x0e, 0x32, 0x13, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66,
    0x73, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x14, 0x0a, 0x0c, 0x66, 0x69, 0x72, 0x73,
    0x74, 0x42, 0x61, 0x64, 0x4c, 0x69, 0x6e, 0x6b, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x43,
    0x0a, 0x10, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x29, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f,
    0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x4f, 0x70, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x43, 0x68,
    0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x40, 0x0a, 0x12, 0x72, 0x65, 0x61, 0x64, 0x4f, 0x70, 0x43, 0x68, 0x65,
    0x63, 0x6b, 0x73, 0x75, 0x6d, 0x49, 0x6e, 0x66, 0x6f, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x24, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x52, 0x65,
    0x61, 0x64, 0x4f, 0x70, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x49, 0x6e, 0x66, 0x6f,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0f, 0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x18, 0x05, 0x20, 0x01, 0x28, 0x09, 0x12, 0x21, 0x0a, 0x19, 0x73, 0x68, 0x6f, 0x72, 0x74, 0x43,
    0x69, 0x72, 0x63, 0x75, 0x69, 0x74, 0x41, 0x63, 0x63, 0x65, 0x73, 0x73, 0x56, 0x65, 0x72, 0x73,
    0x69, 0x6f, 0x6e, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x22, 0x3c, 0x0a, 0x15, 0x43, 0x6c, 0x69,
    0x65, 0x6e, 0x74, 0x52, 0x65, 0x61, 0x64, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x23, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x0e, 0x32, 0x13, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73,
    0x2e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x22, 0x39, 0x0a, 0x12, 0x44, 0x4e, 0x54, 0x72, 0x61,
    0x6e, 0x73, 0x66, 0x65, 0x72, 0x41, 0x63, 0x6b, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x23, 0x0a,
    0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x13, 0x2e,
    0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x53, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x22, 0x86, 0x01, 0x0a, 0x1c, 0x4f, 0x70, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x43, 0x68,
    0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x13, 0x0a, 0x0b, 0x62, 0x79, 0x74, 0x65, 0x73, 0x50, 0x65, 0x72, 0x43,
    0x72, 0x63, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0d, 0x12, 0x13, 0x0a, 0x0b, 0x63, 0x72, 0x63, 0x50,
    0x65, 0x72, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x02, 0x20, 0x02, 0x28, 0x04, 0x12, 0x0b, 0x0a,
    0x03, 0x6d, 0x64, 0x35, 0x18, 0x03, 0x20, 0x02, 0x28, 0x0c, 0x12, 0x2f, 0x0a, 0x07, 0x63, 0x72,
    0x63, 0x54, 0x79, 0x70, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1e, 0x2e, 0x68, 0x61,
    0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x73,
    0x75, 0x6d, 0x54, 0x79, 0x70, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x2a, 0xf4, 0x01, 0x0a, 0x06,
    0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x0b, 0x0a, 0x07, 0x53, 0x55, 0x43, 0x43, 0x45, 0x53,
    0x53, 0x10, 0x00, 0x12, 0x09, 0x0a, 0x05, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x10, 0x01, 0x12, 0x12,
    0x0a, 0x0e, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x43, 0x48, 0x45, 0x43, 0x4b, 0x53, 0x55, 0x4d,
    0x10, 0x02, 0x12, 0x11, 0x0a, 0x0d, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x49, 0x4e, 0x56, 0x41,
    0x4c, 0x49, 0x44, 0x10, 0x03, 0x12, 0x10, 0x0a, 0x0c, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x45,
    0x58, 0x49, 0x53, 0x54, 0x53, 0x10, 0x04, 0x12, 0x16, 0x0a, 0x12, 0x45, 0x52, 0x52, 0x4f, 0x52,
    0x5f, 0x41, 0x43, 0x43, 0x45, 0x53, 0x53, 0x5f, 0x54, 0x4f, 0x4b, 0x45, 0x4e, 0x10, 0x05, 0x12,
    0x0f, 0x0a, 0x0b, 0x43, 0x48, 0x45, 0x43, 0x4b, 0x53, 0x55, 0x4d, 0x5f, 0x4f, 0x4b, 0x10, 0x06,
    0x12, 0x15, 0x0a, 0x11, 0x45, 0x52, 0x52, 0x4f, 0x52, 0x5f, 0x55, 0x4e, 0x53, 0x55, 0x50, 0x50,
    0x4f, 0x52, 0x54, 0x45, 0x44, 0x10, 0x07, 0x12, 0x0f, 0x0a, 0x0b, 0x4f, 0x4f, 0x42, 0x5f, 0x52,
    0x45, 0x53, 0x54, 0x41, 0x52, 0x54, 0x10, 0x08, 0x12, 0x11, 0x0a, 0x0d, 0x4f, 0x4f, 0x42, 0x5f,
    0x52, 0x45, 0x53, 0x45, 0x52, 0x56, 0x45, 0x44, 0x31, 0x10, 0x09, 0x12, 0x11, 0x0a, 0x0d, 0x4f,
    0x4f, 0x42, 0x5f, 0x52, 0x45, 0x53, 0x45, 0x52, 0x56, 0x45, 0x44, 0x32, 0x10, 0x0a, 0x12, 0x11,
    0x0a, 0x0d, 0x4f, 0x4f, 0x42, 0x5f, 0x52, 0x45, 0x53, 0x45, 0x52, 0x56, 0x45, 0x44, 0x33, 0x10,
    0x0b, 0x12, 0x0f, 0x0a, 0x0b, 0x49, 0x4e, 0x5f, 0x50, 0x52, 0x4f, 0x47, 0x52, 0x45, 0x53, 0x53,
    0x10, 0x0c, 0x2a, 0x5b, 0x0a, 0x16, 0x53, 0x68, 0x6f, 0x72, 0x74, 0x43, 0x69, 0x72, 0x63, 0x75,
    0x69, 0x74, 0x46, 0x64, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x23, 0x0a, 0x1f,
    0x44, 0x4f, 0x5f, 0x4e, 0x4f, 0x54, 0x5f, 0x55, 0x53, 0x45, 0x5f, 0x52, 0x45, 0x43, 0x45, 0x49,
    0x50, 0x54, 0x5f, 0x56, 0x45, 0x52, 0x49, 0x46, 0x49, 0x43, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x10,
    0x00, 0x12, 0x1c, 0x0a, 0x18, 0x55, 0x53, 0x45, 0x5f, 0x52, 0x45, 0x43, 0x45, 0x49, 0x50, 0x54,
    0x5f, 0x56, 0x45, 0x52, 0x49, 0x46, 0x49, 0x43, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x10, 0x01, 0x42,
    0x3e, 0x0a, 0x25, 0x6f, 0x72, 0x67, 0x2e, 0x61, 0x70, 0x61, 0x63, 0x68, 0x65, 0x2e, 0x68, 0x61,
    0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63,
    0x6f, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x42, 0x12, 0x44, 0x61, 0x74, 0x61, 0x54, 0x72,
    0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0xa0, 0x01, 0x01, 0x4a,
    0x88, 0x54, 0x0a, 0x07, 0x12, 0x05, 0x1b, 0x00, 0xaf, 0x02, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x08,
    0x12, 0x03, 0x1b, 0x00, 0x3e, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x1b,
    0x00, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x1b, 0x07, 0x13,
    0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1b, 0x07, 0x13, 0x0a,
    0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x07, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x03, 0x1b, 0x16, 0x3d, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x1c, 0x00, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01, 0x12,
    0x03, 0x1c, 0x00, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x1c,
    0x07, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1c, 0x07,
    0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x07,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x07, 0x12, 0x03, 0x1c, 0x1e, 0x32, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x1d, 0x00, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07,
    0x02, 0x12, 0x03, 0x1d, 0x00, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x02, 0x02, 0x12,
    0x03, 0x1d, 0x07, 0x24, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x02, 0x02, 0x00, 0x12, 0x03,
    0x1d, 0x07, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x1d, 0x07, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x02, 0x03, 0x12, 0x03, 0x1d, 0x27,
    0x2b, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x1e, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x00, 0x12, 0x03, 0x20, 0x07, 0x17, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x21, 0x07,
    0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x23, 0x00, 0x2d, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x23, 0x08, 0x29, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x04,
    0x00, 0x12, 0x04, 0x24, 0x02, 0x28, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x24, 0x07, 0x22, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x25, 0x04, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x25, 0x04, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12,
    0x03, 0x25, 0x0e, 0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03,
    0x26, 0x04, 0x1a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x26, 0x04, 0x15, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03,
    0x26, 0x18, 0x19, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x27,
    0x04, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x27,
    0x04, 0x09, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x27,
    0x0c, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x29, 0x02, 0x32, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x29, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x29, 0x0b, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x29, 0x27, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x29, 0x30, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x2a, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2a,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2a, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2a, 0x11, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2a, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x2b, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x2b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x2b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x2b, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2b, 0x1c,
    0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x2c, 0x02, 0x2e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x2c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x2c, 0x0b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x2c, 0x1d, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x2c, 0x2c, 0x2d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x2f, 0x00,
    0x33, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x2f, 0x08, 0x17, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x30, 0x02, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x30, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x30, 0x0b, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x30, 0x1e, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x30, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x31, 0x02, 0x2e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x31, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x31, 0x0b, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x31, 0x24, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x31, 0x2c, 0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02,
    0x12, 0x03, 0x32, 0x02, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x32, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x06, 0x12, 0x03, 0x32, 0x0b,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x32, 0x26, 0x2f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x32, 0x32, 0x33, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x02, 0x12, 0x04, 0x35, 0x00, 0x38, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01,
    0x12, 0x03, 0x35, 0x08, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x36,
    0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x36, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x36, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x36, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x36, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x37, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x37, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x37, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x37, 0x12,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x37, 0x1d, 0x1e, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x3a, 0x00, 0x3d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x03, 0x01, 0x12, 0x03, 0x3a, 0x08, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12,
    0x03, 0x3b, 0x02, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3b,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x3b, 0x0b, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3b, 0x1b, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3b, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x3c, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x3c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x3c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x3c, 0x12, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3c, 0x1f,
    0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x3f, 0x00, 0x42, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x3f, 0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02,
    0x00, 0x12, 0x03, 0x40, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x40, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x40,
    0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x40, 0x10, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x40, 0x1d, 0x1e, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x41, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x41, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x41, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x41, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x41, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x44, 0x00, 0x4a, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x44, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x00, 0x12, 0x03, 0x45, 0x02, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x45, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x45, 0x0b, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x45,
    0x26, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x45, 0x2f, 0x30,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x46, 0x02, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x46, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x46, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x46, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x46, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x47,
    0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x47, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x47, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x47, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x47, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x03, 0x12, 0x03, 0x48, 0x02, 0x33, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x48, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x05, 0x12, 0x03,
    0x48, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12, 0x03, 0x48, 0x10,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x03, 0x12, 0x03, 0x48, 0x20, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x08, 0x12, 0x03, 0x48, 0x22, 0x32, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x03, 0x07, 0x12, 0x03, 0x48, 0x2d, 0x31, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x04, 0x12, 0x03, 0x49, 0x02, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04,
    0x04, 0x12, 0x03, 0x49, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x06, 0x12,
    0x03, 0x49, 0x0b, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x01, 0x12, 0x03, 0x49,
    0x20, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x03, 0x12, 0x03, 0x49, 0x32, 0x33,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x4d, 0x00, 0x50, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x06, 0x01, 0x12, 0x03, 0x4d, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00,
    0x12, 0x03, 0x4e, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x4e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x06, 0x12, 0x03, 0x4e, 0x0b,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4e, 0x1d, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4e, 0x24, 0x25, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x4f, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x4f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x4f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x4f, 0x12, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x4f,
    0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x05, 0x52, 0x00, 0x80, 0x01, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x52, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x07, 0x02, 0x00, 0x12, 0x03, 0x53, 0x02, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x53, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x53, 0x0b, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x53,
    0x26, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x53, 0x2f, 0x30,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x54, 0x02, 0x29, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x03, 0x54, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x01, 0x06, 0x12, 0x03, 0x54, 0x0b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x54, 0x1d, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x54, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x02, 0x12, 0x03, 0x55,
    0x02, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x04, 0x12, 0x03, 0x55, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x06, 0x12, 0x03, 0x55, 0x0b, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x03, 0x55, 0x1d, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x03, 0x55, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x07,
    0x04, 0x00, 0x12, 0x04, 0x56, 0x02, 0x68, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x56, 0x07, 0x1d, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x07, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x57, 0x04, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x57, 0x04, 0x19, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x00, 0x02,
    0x12, 0x03, 0x57, 0x1c, 0x1d, 0x0a, 0x4a, 0x0a, 0x06, 0x04, 0x07, 0x04, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x59, 0x04, 0x27, 0x1a, 0x3b, 0x20, 0x70, 0x69, 0x70, 0x65, 0x6c, 0x69, 0x6e, 0x65, 0x20,
    0x73, 0x65, 0x74, 0x20, 0x75, 0x70, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x65,
    0x64, 0x20, 0x50, 0x49, 0x50, 0x45, 0x4c, 0x49, 0x4e, 0x45, 0x5f, 0x53, 0x45, 0x54, 0x55, 0x50,
    0x5f, 0x41, 0x50, 0x50, 0x45, 0x4e, 0x44, 0x20, 0x72, 0x65, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x79,
    0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x59, 0x04,
    0x22, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x59, 0x25,
    0x26, 0x0a, 0x1f, 0x0a, 0x06, 0x04, 0x07, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x5b, 0x04, 0x17,
    0x1a, 0x10, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e,
    0x67, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x5b,
    0x04, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x5b,
    0x15, 0x16, 0x0a, 0x42, 0x0a, 0x06, 0x04, 0x07, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x5d, 0x04,
    0x2a, 0x1a, 0x33, 0x20, 0x70, 0x69, 0x70, 0x65, 0x6c, 0x69, 0x6e, 0x65, 0x20, 0x73, 0x65, 0x74,
    0x75, 0x70, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x20, 0x64, 0x61,
    0x74, 0x61, 0x20, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x69, 0x6e, 0x67, 0x20, 0x72, 0x65, 0x63,
    0x6f, 0x76, 0x65, 0x72, 0x79, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x5d, 0x04, 0x25, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x03,
    0x02, 0x12, 0x03, 0x5d, 0x28, 0x29, 0x0a, 0x2d, 0x0a, 0x06, 0x04, 0x07, 0x04, 0x00, 0x02, 0x04,
    0x12, 0x03, 0x5f, 0x04, 0x17, 0x1a, 0x1e, 0x20, 0x63, 0x6c, 0x6f, 0x73, 0x65, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x70, 0x69, 0x70, 0x65,
    0x6c, 0x69, 0x6e, 0x65, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x04, 0x01,
    0x12, 0x03, 0x5f, 0x04, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x04, 0x02,
    0x12, 0x03, 0x5f, 0x15, 0x16, 0x0a, 0x30, 0x0a, 0x06, 0x04, 0x07, 0x04, 0x00, 0x02, 0x05, 0x12,
    0x03, 0x61, 0x04, 0x20, 0x1a, 0x21, 0x20, 0x52, 0x65, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x20, 0x61,
    0x20, 0x66, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x20, 0x50, 0x49, 0x50, 0x45, 0x4c, 0x49, 0x4e, 0x45,
    0x5f, 0x43, 0x4c, 0x4f, 0x53, 0x45, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02,
    0x05, 0x01, 0x12, 0x03, 0x61, 0x04, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02,
    0x05, 0x02, 0x12, 0x03, 0x61, 0x1e, 0x1f, 0x0a, 0x33, 0x0a, 0x06, 0x04, 0x07, 0x04, 0x00, 0x02,
    0x06, 0x12, 0x03, 0x63, 0x04, 0x1e, 0x1a, 0x24, 0x20, 0x70, 0x69, 0x70, 0x65, 0x6c, 0x69, 0x6e,
    0x65, 0x20, 0x73, 0x65, 0x74, 0x20, 0x75, 0x70, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x62, 0x6c, 0x6f,
    0x63, 0x6b, 0x20, 0x63, 0x72, 0x65, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x07, 0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x63, 0x04, 0x19, 0x0a, 0x0e, 0x0a, 0x07,
    0x04, 0x07, 0x04, 0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x63, 0x1c, 0x1d, 0x0a, 0x32, 0x0a, 0x06,
    0x04, 0x07, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x65, 0x04, 0x15, 0x1a, 0x23, 0x20, 0x74, 0x72,
    0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x20, 0x52, 0x42, 0x57, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61,
    0x64, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x64, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x73, 0x0a,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x65, 0x04, 0x10,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07, 0x04, 0x00, 0x02, 0x07, 0x02, 0x12, 0x03, 0x65, 0x13, 0x14,
    0x0a, 0x38, 0x0a, 0x06, 0x04, 0x07, 0x04, 0x00, 0x02, 0x08, 0x12, 0x03, 0x67, 0x04, 0x1b, 0x1a,
    0x29, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x20, 0x46, 0x69, 0x6e, 0x61, 0x6c,
    0x69, 0x7a, 0x65, 0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x64, 0x64, 0x69, 0x6e, 0x67, 0x20,
    0x64, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x73, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07,
    0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x67, 0x04, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x07,
    0x04, 0x00, 0x02, 0x08, 0x02, 0x12, 0x03, 0x67, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07,
    0x02, 0x03, 0x12, 0x03, 0x69, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x04,
    0x12, 0x03, 0x69, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x06, 0x12, 0x03,
    0x69, 0x0b, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x01, 0x12, 0x03, 0x69, 0x22,
    0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03, 0x03, 0x12, 0x03, 0x69, 0x2a, 0x2b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x04, 0x12, 0x03, 0x6a, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x04, 0x04, 0x12, 0x03, 0x6a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x04, 0x05, 0x12, 0x03, 0x6a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x6a, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x03, 0x12,
    0x03, 0x6a, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x05, 0x12, 0x03, 0x6b, 0x02,
    0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x04, 0x12, 0x03, 0x6b, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x05, 0x05, 0x12, 0x03, 0x6b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x05, 0x01, 0x12, 0x03, 0x6b, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x05, 0x03, 0x12, 0x03, 0x6b, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02,
    0x06, 0x12, 0x03, 0x6c, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x06, 0x04, 0x12,
    0x03, 0x6c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x06, 0x05, 0x12, 0x03, 0x6c,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x06, 0x01, 0x12, 0x03, 0x6c, 0x12, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x06, 0x03, 0x12, 0x03, 0x6c, 0x21, 0x22, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x07, 0x02, 0x07, 0x12, 0x03, 0x6d, 0x02, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x07, 0x04, 0x12, 0x03, 0x6d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x07, 0x05, 0x12, 0x03, 0x6d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x07, 0x01,
    0x12, 0x03, 0x6d, 0x12, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x07, 0x03, 0x12, 0x03,
    0x6d, 0x2a, 0x2b, 0x0a, 0x47, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x08, 0x12, 0x03, 0x72, 0x02, 0x2f,
    0x1a, 0x3a, 0x2a, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x65, 0x64, 0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x20, 0x6d, 0x65, 0x63, 0x68,
    0x61, 0x6e, 0x69, 0x73, 0x6d, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x62,
    0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x77, 0x72, 0x69, 0x74, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x08, 0x04, 0x12, 0x03, 0x72, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x08, 0x06, 0x12, 0x03, 0x72, 0x0b, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x08,
    0x01, 0x12, 0x03, 0x72, 0x19, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x08, 0x03, 0x12,
    0x03, 0x72, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x09, 0x12, 0x03, 0x73, 0x02,
    0x35, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x09, 0x04, 0x12, 0x03, 0x73, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x09, 0x06, 0x12, 0x03, 0x73, 0x0b, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x09, 0x01, 0x12, 0x03, 0x73, 0x20, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x09, 0x03, 0x12, 0x03, 0x73, 0x32, 0x34, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02,
    0x0a, 0x12, 0x03, 0x74, 0x02, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0a, 0x04, 0x12,
    0x03, 0x74, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0a, 0x06, 0x12, 0x03, 0x74,
    0x0b, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x74, 0x1c, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x74, 0x2a, 0x2c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x0a, 0x08, 0x12, 0x03, 0x74, 0x2d, 0x3d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x0a, 0x07, 0x12, 0x03, 0x74, 0x38, 0x3c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07,
    0x02, 0x0b, 0x12, 0x03, 0x75, 0x02, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0b, 0x04,
    0x12, 0x03, 0x75, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0b, 0x06, 0x12, 0x03,
    0x75, 0x0b, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x75, 0x1c,
    0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0b, 0x03, 0x12, 0x03, 0x75, 0x31, 0x33, 0x0a,
    0xaf, 0x01, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x0c, 0x12, 0x03, 0x7c, 0x02, 0x38, 0x1a, 0xa1, 0x01,
    0x2a, 0x0a, 0x20, 0x48, 0x69, 0x6e, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x44,
    0x61, 0x74, 0x61, 0x4e, 0x6f, 0x64, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x61, 0x6c,
    0x6c, 0x6f, 0x63, 0x61, 0x74, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73,
    0x69, 0x65, 0x6e, 0x74, 0x0a, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x20, 0x69, 0x2e,
    0x65, 0x2e, 0x20, 0x6d, 0x65, 0x6d, 0x6f, 0x72, 0x79, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x77, 0x72,
    0x69, 0x74, 0x74, 0x65, 0x6e, 0x20, 0x74, 0x6f, 0x20, 0x64, 0x69, 0x73, 0x6b, 0x20, 0x6c, 0x61,
    0x7a, 0x69, 0x6c, 0x79, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x44, 0x61, 0x74, 0x61, 0x4e, 0x6f,
    0x64, 0x65, 0x20, 0x69, 0x73, 0x20, 0x66, 0x72, 0x65, 0x65, 0x0a, 0x20, 0x74, 0x6f, 0x20, 0x69,
    0x67, 0x6e, 0x6f, 0x72, 0x65, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x68, 0x69, 0x6e, 0x74, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0c, 0x04, 0x12, 0x03, 0x7c, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0c, 0x05, 0x12, 0x03, 0x7c, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x7c, 0x10, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x0c, 0x03, 0x12, 0x03, 0x7c, 0x23, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x0c, 0x08, 0x12, 0x03, 0x7c, 0x26, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0c, 0x07,
    0x12, 0x03, 0x7c, 0x31, 0x36, 0x0a, 0x42, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x0d, 0x12, 0x03, 0x7e,
    0x02, 0x2f, 0x1a, 0x35, 0x77, 0x68, 0x65, 0x74, 0x68, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x70,
    0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x2c, 0x20, 0x73, 0x6f,
    0x20, 0x42, 0x61, 0x6c, 0x61, 0x6e, 0x63, 0x65, 0x72, 0x20, 0x77, 0x6f, 0x6e, 0x27, 0x74, 0x20,
    0x6d, 0x6f, 0x76, 0x65, 0x20, 0x69, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x0d, 0x04, 0x12, 0x03, 0x7e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0d, 0x05,
    0x12, 0x03, 0x7e, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0d, 0x01, 0x12, 0x03,
    0x7e, 0x10, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0d, 0x03, 0x12, 0x03, 0x7e, 0x1a,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0d, 0x08, 0x12, 0x03, 0x7e, 0x1d, 0x2e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0d, 0x07, 0x12, 0x03, 0x7e, 0x28, 0x2d, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x07, 0x02, 0x0e, 0x12, 0x03, 0x7f, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x0e, 0x04, 0x12, 0x03, 0x7f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0e,
    0x05, 0x12, 0x03, 0x7f, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0e, 0x01, 0x12,
    0x03, 0x7f, 0x10, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x0e, 0x03, 0x12, 0x03, 0x7f,
    0x21, 0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x06, 0x82, 0x01, 0x00, 0x86, 0x01, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x04, 0x82, 0x01, 0x08, 0x1c, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x04, 0x83, 0x01, 0x02, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x04, 0x12, 0x04, 0x83, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x00, 0x06, 0x12, 0x04, 0x83, 0x01, 0x0b, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x01, 0x12, 0x04, 0x83, 0x01, 0x26, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00,
    0x03, 0x12, 0x04, 0x83, 0x01, 0x2f, 0x30, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12,
    0x04, 0x84, 0x01, 0x02, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x84, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x06, 0x12, 0x04, 0x84,
    0x01, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x04, 0x84, 0x01,
    0x1d, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x04, 0x84, 0x01, 0x27,
    0x28, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x04, 0x85, 0x01, 0x02, 0x33, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x04, 0x12, 0x04, 0x85, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x06, 0x12, 0x04, 0x85, 0x01, 0x0b, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x04, 0x85, 0x01, 0x1c, 0x2e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x02, 0x03, 0x12, 0x04, 0x85, 0x01, 0x31, 0x32, 0x0a, 0x0c, 0x0a, 0x02, 0x04,
    0x09, 0x12, 0x06, 0x88, 0x01, 0x00, 0x8d, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x09, 0x01,
    0x12, 0x04, 0x88, 0x01, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x04,
    0x89, 0x01, 0x02, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x04, 0x89,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x06, 0x12, 0x04, 0x89, 0x01,
    0x0b, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x04, 0x89, 0x01, 0x1b,
    0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x04, 0x89, 0x01, 0x24, 0x25,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x04, 0x8a, 0x01, 0x02, 0x1e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x04, 0x8a, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x01, 0x05, 0x12, 0x04, 0x8a, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8a, 0x01, 0x12, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8a, 0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09,
    0x02, 0x02, 0x12, 0x04, 0x8b, 0x01, 0x02, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02,
    0x04, 0x12, 0x04, 0x8b, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x06,
    0x12, 0x04, 0x8b, 0x01, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x01, 0x12,
    0x04, 0x8b, 0x01, 0x1d, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x03, 0x12, 0x04,
    0x8b, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x03, 0x12, 0x04, 0x8c, 0x01,
    0x02, 0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x04, 0x12, 0x04, 0x8c, 0x01, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x06, 0x12, 0x04, 0x8c, 0x01, 0x0b, 0x1b,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x01, 0x12, 0x04, 0x8c, 0x01, 0x1c, 0x27, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x03, 0x12, 0x04, 0x8c, 0x01, 0x2a, 0x2b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x03, 0x08, 0x12, 0x04, 0x8c, 0x01, 0x2c, 0x3c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x09, 0x02, 0x03, 0x07, 0x12, 0x04, 0x8c, 0x01, 0x37, 0x3b, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x0a, 0x12, 0x06, 0x8f, 0x01, 0x00, 0x91, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0a,
    0x01, 0x12, 0x04, 0x8f, 0x01, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12,
    0x04, 0x90, 0x01, 0x02, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x90, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x06, 0x12, 0x04, 0x90,
    0x01, 0x0b, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x04, 0x90, 0x01,
    0x1b, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x04, 0x90, 0x01, 0x24,
    0x25, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x06, 0x93, 0x01, 0x00, 0x95, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x04, 0x93, 0x01, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x0b, 0x02, 0x00, 0x12, 0x04, 0x94, 0x01, 0x02, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x94, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x00, 0x06, 0x12, 0x04, 0x94, 0x01, 0x0b, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x94, 0x01, 0x1b, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03,
    0x12, 0x04, 0x94, 0x01, 0x24, 0x25, 0x0a, 0x45, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x06, 0x9a, 0x01,
    0x00, 0x9d, 0x01, 0x01, 0x1a, 0x37, 0x2a, 0x0a, 0x20, 0x41, 0x6e, 0x20, 0x49, 0x44, 0x20, 0x75,
    0x6e, 0x69, 0x71, 0x75, 0x65, 0x6c, 0x79, 0x20, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x79,
    0x69, 0x6e, 0x67, 0x20, 0x61, 0x20, 0x73, 0x68, 0x61, 0x72, 0x65, 0x64, 0x20, 0x6d, 0x65, 0x6d,
    0x6f, 0x72, 0x79, 0x20, 0x73, 0x65, 0x67, 0x6d, 0x65, 0x6e, 0x74, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x0c, 0x01, 0x12, 0x04, 0x9a, 0x01, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c,
    0x02, 0x00, 0x12, 0x04, 0x9b, 0x01, 0x02, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x9b, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x05,
    0x12, 0x04, 0x9b, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x9b, 0x01, 0x11, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x04,
    0x9b, 0x01, 0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x04, 0x9c, 0x01,
    0x02, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x04, 0x12, 0x04, 0x9c, 0x01, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x05, 0x12, 0x04, 0x9c, 0x01, 0x0b, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9c, 0x01, 0x11, 0x13, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03, 0x12, 0x04, 0x9c, 0x01, 0x16, 0x17, 0x0a, 0x53,
    0x0a, 0x02, 0x04, 0x0d, 0x12, 0x06, 0xa2, 0x01, 0x00, 0xa5, 0x01, 0x01, 0x1a, 0x45, 0x2a, 0x0a,
    0x20, 0x41, 0x6e, 0x20, 0x49, 0x44, 0x20, 0x75, 0x6e, 0x69, 0x71, 0x75, 0x65, 0x6c, 0x79, 0x20,
    0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x79, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x20, 0x73, 0x6c,
    0x6f, 0x74, 0x20, 0x77, 0x69, 0x74, 0x68, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x73, 0x68, 0x61, 0x72,
    0x65, 0x64, 0x20, 0x6d, 0x65, 0x6d, 0x6f, 0x72, 0x79, 0x20, 0x73, 0x65, 0x67, 0x6d, 0x65, 0x6e,
    0x74, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12, 0x04, 0xa2, 0x01, 0x08, 0x20,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x04, 0xa3, 0x01, 0x02, 0x2c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa3, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x00, 0x06, 0x12, 0x04, 0xa3, 0x01, 0x0b, 0x21, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa3, 0x01, 0x22, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa3, 0x01, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d,
    0x02, 0x01, 0x12, 0x04, 0xa4, 0x01, 0x02, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01,
    0x04, 0x12, 0x04, 0xa4, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x05,
    0x12, 0x04, 0xa4, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xa4, 0x01, 0x11, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xa4, 0x01, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x06, 0xa7, 0x01, 0x00, 0xbb,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x04, 0xa7, 0x01, 0x08, 0x28, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x04, 0xa8, 0x01, 0x02, 0x26, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa8, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x00, 0x06, 0x12, 0x04, 0xa8, 0x01, 0x0b, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa8, 0x01, 0x1b, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xa8, 0x01, 0x24, 0x25, 0x0a, 0x8b, 0x02, 0x0a, 0x04, 0x04, 0x0e,
    0x02, 0x01, 0x12, 0x04, 0xaf, 0x01, 0x02, 0x21, 0x1a, 0xfc, 0x01, 0x2a, 0x20, 0x49, 0x6e, 0x20,
    0x6f, 0x72, 0x64, 0x65, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x67, 0x65, 0x74, 0x20, 0x73, 0x68, 0x6f,
    0x72, 0x74, 0x2d, 0x63, 0x69, 0x72, 0x63, 0x75, 0x69, 0x74, 0x20, 0x61, 0x63, 0x63, 0x65, 0x73,
    0x73, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x64, 0x61, 0x74, 0x61, 0x2c,
    0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x73, 0x65,
    0x74, 0x20, 0x74, 0x68, 0x69, 0x73, 0x0a, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x68,
    0x69, 0x67, 0x68, 0x65, 0x73, 0x74, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x6f,
    0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x64, 0x61, 0x74, 0x61,
    0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x79, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x75,
    0x6e, 0x64, 0x65, 0x72, 0x73, 0x74, 0x61, 0x6e, 0x64, 0x2e, 0x0a, 0x20, 0x43, 0x75, 0x72, 0x72,
    0x65, 0x6e, 0x74, 0x6c, 0x79, 0x20, 0x31, 0x20, 0x69, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f,
    0x6e, 0x6c, 0x79, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x2c, 0x20, 0x62, 0x75, 0x74,
    0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x6d,
    0x61, 0x79, 0x20, 0x65, 0x78, 0x69, 0x73, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x66, 0x75, 0x74, 0x75, 0x72, 0x65, 0x0a, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f,
    0x6e, 0x2d, 0x64, 0x69, 0x73, 0x6b, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x20, 0x63, 0x68,
    0x61, 0x6e, 0x67, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xaf, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x05, 0x12,
    0x04, 0xaf, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xaf, 0x01, 0x12, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x03, 0x12, 0x04, 0xaf,
    0x01, 0x1f, 0x20, 0x0a, 0x45, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x02, 0x12, 0x04, 0xb4, 0x01, 0x02,
    0x2f, 0x1a, 0x37, 0x2a, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x73, 0x68, 0x61, 0x72, 0x65, 0x64,
    0x20, 0x6d, 0x65, 0x6d, 0x6f, 0x72, 0x79, 0x20, 0x73, 0x6c, 0x6f, 0x74, 0x20, 0x74, 0x6f, 0x20,
    0x75, 0x73, 0x65, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x77, 0x65, 0x20, 0x61, 0x72, 0x65, 0x20, 0x75,
    0x73, 0x69, 0x6e, 0x67, 0x20, 0x6f, 0x6e, 0x65, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x02, 0x04, 0x12, 0x04, 0xb4, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x02, 0x06, 0x12, 0x04, 0xb4, 0x01, 0x0b, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02,
    0x01, 0x12, 0x04, 0xb4, 0x01, 0x24, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x02, 0x03,
    0x12, 0x04, 0xb4, 0x01, 0x2d, 0x2e, 0x0a, 0x6d, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x03, 0x12, 0x04,
    0xba, 0x01, 0x02, 0x42, 0x1a, 0x5f, 0x2a, 0x0a, 0x20, 0x54, 0x72, 0x75, 0x65, 0x20, 0x69, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x75, 0x70, 0x70,
    0x6f, 0x72, 0x74, 0x73, 0x20, 0x76, 0x65, 0x72, 0x69, 0x66, 0x79, 0x69, 0x6e, 0x67, 0x20, 0x74,
    0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x64, 0x65, 0x73,
    0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x20, 0x68, 0x61, 0x73, 0x20, 0x62, 0x65, 0x65, 0x6e,
    0x0a, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x75, 0x63, 0x63, 0x65, 0x73, 0x73, 0x66, 0x75,
    0x6c, 0x6c, 0x79, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x04, 0x12, 0x04,
    0xba, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x05, 0x12, 0x04, 0xba,
    0x01, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x01, 0x12, 0x04, 0xba, 0x01,
    0x10, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x03, 0x12, 0x04, 0xba, 0x01, 0x2e,
    0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x08, 0x12, 0x04, 0xba, 0x01, 0x30, 0x41,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x03, 0x07, 0x12, 0x04, 0xba, 0x01, 0x3b, 0x40, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x06, 0xbd, 0x01, 0x00, 0xc0, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x0f, 0x01, 0x12, 0x04, 0xbd, 0x01, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f,
    0x02, 0x00, 0x12, 0x04, 0xbe, 0x01, 0x02, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xbe, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x06,
    0x12, 0x04, 0xbe, 0x01, 0x0b, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xbe, 0x01, 0x24, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xbe, 0x01, 0x2d, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x01, 0x12, 0x04, 0xbf, 0x01,
    0x02, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x04, 0x12, 0x04, 0xbf, 0x01, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x06, 0x12, 0x04, 0xbf, 0x01, 0x0b, 0x25,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x01, 0x12, 0x04, 0xbf, 0x01, 0x26, 0x2f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x03, 0x12, 0x04, 0xbf, 0x01, 0x32, 0x33, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x10, 0x12, 0x06, 0xc2, 0x01, 0x00, 0xc5, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x10, 0x01, 0x12, 0x04, 0xc2, 0x01, 0x08, 0x2e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02,
    0x00, 0x12, 0x04, 0xc3, 0x01, 0x02, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xc3, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x06, 0x12,
    0x04, 0xc3, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xc3, 0x01, 0x12, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x04, 0xc3,
    0x01, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x01, 0x12, 0x04, 0xc4, 0x01, 0x02,
    0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc4, 0x01, 0x02, 0x0a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x05, 0x12, 0x04, 0xc4, 0x01, 0x0b, 0x11, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc4, 0x01, 0x12, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc4, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x11, 0x12, 0x06, 0xc7, 0x01, 0x00, 0xcc, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x11, 0x01, 0x12, 0x04, 0xc7, 0x01, 0x08, 0x23, 0x0a, 0x7f, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00,
    0x12, 0x04, 0xca, 0x01, 0x02, 0x21, 0x1a, 0x71, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6e, 0x61, 0x6d,
    0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20,
    0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73,
    0x68, 0x61, 0x72, 0x65, 0x64, 0x20, 0x6d, 0x65, 0x6d, 0x6f, 0x72, 0x79, 0x20, 0x73, 0x65, 0x67,
    0x6d, 0x65, 0x6e, 0x74, 0x2e, 0x20, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x0a, 0x20,
    0x70, 0x75, 0x72, 0x65, 0x6c, 0x79, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x6c, 0x6f, 0x67, 0x67, 0x69,
    0x6e, 0x67, 0x20, 0x2f, 0x20, 0x64, 0x65, 0x62, 0x75, 0x67, 0x67, 0x69, 0x6e, 0x67, 0x20, 0x70,
    0x75, 0x72, 0x70, 0x6f, 0x73, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xca, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xca, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xca, 0x01, 0x12, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xca, 0x01, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x01, 0x12, 0x04, 0xcb,
    0x01, 0x02, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x04, 0x12, 0x04, 0xcb, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x06, 0x12, 0x04, 0xcb, 0x01, 0x0b,
    0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x01, 0x12, 0x04, 0xcb, 0x01, 0x26, 0x2f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x03, 0x12, 0x04, 0xcb, 0x01, 0x32, 0x33, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x12, 0x12, 0x06, 0xce, 0x01, 0x00, 0xd2, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x12, 0x01, 0x12, 0x04, 0xce, 0x01, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12,
    0x02, 0x00, 0x12, 0x04, 0xcf, 0x01, 0x02, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xcf, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x06,
    0x12, 0x04, 0xcf, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xcf, 0x01, 0x12, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xcf, 0x01, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x01, 0x12, 0x04, 0xd0, 0x01,
    0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd0, 0x01, 0x02,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x05, 0x12, 0x04, 0xd0, 0x01, 0x0b, 0x11,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd0, 0x01, 0x12, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd0, 0x01, 0x1a, 0x1b, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x12, 0x02, 0x02, 0x12, 0x04, 0xd1, 0x01, 0x02, 0x29, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x02, 0x04, 0x12, 0x04, 0xd1, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x12, 0x02, 0x02, 0x06, 0x12, 0x04, 0xd1, 0x01, 0x0b, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12,
    0x02, 0x02, 0x01, 0x12, 0x04, 0xd1, 0x01, 0x22, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x12, 0x02,
    0x02, 0x03, 0x12, 0x04, 0xd1, 0x01, 0x27, 0x28, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x13, 0x12, 0x06,
    0xd4, 0x01, 0x00, 0xdb, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x04, 0xd4,
    0x01, 0x08, 0x19, 0x0a, 0x30, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x04, 0xd6, 0x01, 0x02,
    0x26, 0x1a, 0x22, 0x20, 0x41, 0x6c, 0x6c, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x73, 0x20, 0x6d,
    0x75, 0x73, 0x74, 0x20, 0x62, 0x65, 0x20, 0x66, 0x69, 0x78, 0x65, 0x64, 0x2d, 0x6c, 0x65, 0x6e,
    0x67, 0x74, 0x68, 0x21, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x04,
    0xd6, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x05, 0x12, 0x04, 0xd6,
    0x01, 0x0b, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd6, 0x01,
    0x14, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd6, 0x01, 0x24,
    0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x01, 0x12, 0x04, 0xd7, 0x01, 0x02, 0x1e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd7, 0x01, 0x02, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x05, 0x12, 0x04, 0xd7, 0x01, 0x0b, 0x13, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd7, 0x01, 0x14, 0x19, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd7, 0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x13, 0x02, 0x02, 0x12, 0x04, 0xd8, 0x01, 0x02, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x02, 0x04, 0x12, 0x04, 0xd8, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02,
    0x05, 0x12, 0x04, 0xd8, 0x01, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x01,
    0x12, 0x04, 0xd8, 0x01, 0x10, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x03, 0x12,
    0x04, 0xd8, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x03, 0x12, 0x04, 0xd9,
    0x01, 0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x04, 0x12, 0x04, 0xd9, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x05, 0x12, 0x04, 0xd9, 0x01, 0x0b,
    0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x01, 0x12, 0x04, 0xd9, 0x01, 0x14, 0x1b,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x03, 0x12, 0x04, 0xd9, 0x01, 0x1e, 0x1f, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x04, 0x12, 0x04, 0xda, 0x01, 0x02, 0x30, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x04, 0x04, 0x12, 0x04, 0xda, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x04, 0x05, 0x12, 0x04, 0xda, 0x01, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x04, 0x01, 0x12, 0x04, 0xda, 0x01, 0x10, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x04, 0x03, 0x12, 0x04, 0xda, 0x01, 0x1c, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x04, 0x08, 0x12, 0x04, 0xda, 0x01, 0x1e, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x04,
    0x07, 0x12, 0x04, 0xda, 0x01, 0x29, 0x2e, 0x0a, 0x26, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x06, 0xde,
    0x01, 0x00, 0xec, 0x01, 0x01, 0x1a, 0x18, 0x20, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x20, 0x69,
    0x73, 0x20, 0x61, 0x20, 0x34, 0x2d, 0x62, 0x69, 0x74, 0x20, 0x65, 0x6e, 0x75, 0x6d, 0x0a, 0x0a,
    0x0b, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x04, 0xde, 0x01, 0x05, 0x0b, 0x0a, 0x0c, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x00, 0x12, 0x04, 0xdf, 0x01, 0x02, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xdf, 0x01, 0x02, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x00, 0x02, 0x12, 0x04, 0xdf, 0x01, 0x0c, 0x0d, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01,
    0x12, 0x04, 0xe0, 0x01, 0x02, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xe0, 0x01, 0x02, 0x07, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x04,
    0xe0, 0x01, 0x0a, 0x0b, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x04, 0xe1, 0x01,
    0x02, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xe1, 0x01, 0x02,
    0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xe1, 0x01, 0x13, 0x14,
    0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x04, 0xe2, 0x01, 0x02, 0x14, 0x0a, 0x0d,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xe2, 0x01, 0x02, 0x0f, 0x0a, 0x0d, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0xe2, 0x01, 0x12, 0x13, 0x0a, 0x0c, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x04, 0x12, 0x04, 0xe3, 0x01, 0x02, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x04, 0x01, 0x12, 0x04, 0xe3, 0x01, 0x02, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x04, 0x02, 0x12, 0x04, 0xe3, 0x01, 0x11, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x05,
    0x12, 0x04, 0xe4, 0x01, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x01, 0x12,
    0x04, 0xe4, 0x01, 0x02, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02, 0x12, 0x04,
    0xe4, 0x01, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x06, 0x12, 0x04, 0xe5, 0x01,
    0x02, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x01, 0x12, 0x04, 0xe5, 0x01, 0x02,
    0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x02, 0x12, 0x04, 0xe5, 0x01, 0x10, 0x11,
    0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x07, 0x12, 0x04, 0xe6, 0x01, 0x02, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x01, 0x12, 0x04, 0xe6, 0x01, 0x02, 0x13, 0x0a, 0x0d, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x07, 0x02, 0x12, 0x04, 0xe6, 0x01, 0x16, 0x17, 0x0a, 0x1d, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x08, 0x12, 0x04, 0xe7, 0x01, 0x02, 0x12, 0x22, 0x0f, 0x20, 0x51, 0x75, 0x69,
    0x63, 0x6b, 0x20, 0x72, 0x65, 0x73, 0x74, 0x61, 0x72, 0x74, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x08, 0x01, 0x12, 0x04, 0xe7, 0x01, 0x02, 0x0d, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x08, 0x02, 0x12, 0x04, 0xe7, 0x01, 0x10, 0x11, 0x0a, 0x18, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x09, 0x12, 0x04, 0xe8, 0x01, 0x02, 0x14, 0x22, 0x0a, 0x20, 0x52, 0x65, 0x73, 0x65, 0x72, 0x76,
    0x65, 0x64, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09, 0x01, 0x12, 0x04, 0xe8, 0x01,
    0x02, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09, 0x02, 0x12, 0x04, 0xe8, 0x01, 0x12,
    0x13, 0x0a, 0x18, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0a, 0x12, 0x04, 0xe9, 0x01, 0x02, 0x15, 0x22,
    0x0a, 0x20, 0x52, 0x65, 0x73, 0x65, 0x72, 0x76, 0x65, 0x64, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x0a, 0x01, 0x12, 0x04, 0xe9, 0x01, 0x02, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x0a, 0x02, 0x12, 0x04, 0xe9, 0x01, 0x12, 0x14, 0x0a, 0x18, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x0b, 0x12, 0x04, 0xea, 0x01, 0x02, 0x15, 0x22, 0x0a, 0x20, 0x52, 0x65, 0x73, 0x65, 0x72, 0x76,
    0x65, 0x64, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x04, 0xea, 0x01,
    0x02, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0b, 0x02, 0x12, 0x04, 0xea, 0x01, 0x12,
    0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0c, 0x12, 0x04, 0xeb, 0x01, 0x02, 0x13, 0x0a,
    0x0d, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x04, 0xeb, 0x01, 0x02, 0x0d, 0x0a, 0x0d,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c, 0x02, 0x12, 0x04, 0xeb, 0x01, 0x10, 0x12, 0x0a, 0x0c, 0x0a,
    0x02, 0x05, 0x01, 0x12, 0x06, 0xee, 0x01, 0x00, 0xf1, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x05,
    0x01, 0x01, 0x12, 0x04, 0xee, 0x01, 0x05, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00,
    0x12, 0x04, 0xef, 0x01, 0x02, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xef, 0x01, 0x02, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x04,
    0xef, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x01, 0x12, 0x04, 0xf0, 0x01,
    0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf0, 0x01, 0x02,
    0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x04, 0xf0, 0x01, 0x1d, 0x1e,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x06, 0xf3, 0x01, 0x00, 0xf8, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x04, 0xf3, 0x01, 0x08, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x14, 0x02, 0x00, 0x12, 0x04, 0xf4, 0x01, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x00, 0x04, 0x12, 0x04, 0xf4, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00,
    0x05, 0x12, 0x04, 0xf4, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x01,
    0x12, 0x04, 0xf4, 0x01, 0x12, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x03, 0x12,
    0x04, 0xf4, 0x01, 0x1a, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x01, 0x12, 0x04, 0xf5,
    0x01, 0x02, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x04, 0x12, 0x04, 0xf5, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x06, 0x12, 0x04, 0xf5, 0x01, 0x0b,
    0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf5, 0x01, 0x12, 0x17,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x03, 0x12, 0x04, 0xf5, 0x01, 0x1a, 0x1b, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x02, 0x12, 0x04, 0xf6, 0x01, 0x02, 0x3b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x02, 0x04, 0x12, 0x04, 0xf6, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x02, 0x05, 0x12, 0x04, 0xf6, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x02, 0x01, 0x12, 0x04, 0xf6, 0x01, 0x12, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x02, 0x03, 0x12, 0x04, 0xf6, 0x01, 0x2b, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x02, 0x08, 0x12, 0x04, 0xf6, 0x01, 0x2d, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02,
    0x07, 0x12, 0x04, 0xf6, 0x01, 0x38, 0x39, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x03, 0x12,
    0x04, 0xf7, 0x01, 0x02, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x04, 0x12, 0x04,
    0xf7, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x05, 0x12, 0x04, 0xf7,
    0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x01, 0x12, 0x04, 0xf7, 0x01,
    0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x03, 0x12, 0x04, 0xf7, 0x01, 0x19,
    0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x08, 0x12, 0x04, 0xf7, 0x01, 0x1b, 0x28,
    0x0a, 0x10, 0x0a, 0x08, 0x04, 0x14, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x04, 0xf7, 0x01,
    0x1c, 0x27, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x14, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12,
    0x04, 0xf7, 0x01, 0x1c, 0x22, 0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x14, 0x02, 0x03, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x12, 0x04, 0xf7, 0x01, 0x1c, 0x22, 0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x14, 0x02,
    0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf7, 0x01, 0x1c, 0x22, 0x0a, 0x11,
    0x0a, 0x09, 0x04, 0x14, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x04, 0xf7, 0x01, 0x23,
    0x27, 0x0a, 0x65, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x06, 0xfe, 0x01, 0x00, 0x87, 0x02, 0x01, 0x1a,
    0x57, 0x2a, 0x0a, 0x20, 0x53, 0x65, 0x6e, 0x74, 0x20, 0x61, 0x73, 0x20, 0x70, 0x61, 0x72, 0x74,
    0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x4f, 0x70, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x0a, 0x20, 0x66, 0x6f,
    0x72, 0x20, 0x52, 0x45, 0x41, 0x44, 0x5f, 0x42, 0x4c, 0x4f, 0x43, 0x4b, 0x20, 0x61, 0x6e, 0x64,
    0x20, 0x43, 0x4f, 0x50, 0x59, 0x5f, 0x42, 0x4c, 0x4f, 0x43, 0x4b, 0x20, 0x6f, 0x70, 0x65, 0x72,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15, 0x01, 0x12,
    0x04, 0xfe, 0x01, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x00, 0x12, 0x04, 0xff,
    0x01, 0x02, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x04, 0x12, 0x04, 0xff, 0x01,
    0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x06, 0x12, 0x04, 0xff, 0x01, 0x0b,
    0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12, 0x04, 0xff, 0x01, 0x19, 0x21,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x04, 0xff, 0x01, 0x24, 0x25, 0x0a,
    0xa5, 0x01, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x01, 0x12, 0x04, 0x86, 0x02, 0x02, 0x22, 0x1a, 0x96,
    0x01, 0x2a, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6f, 0x66, 0x66, 0x73, 0x65, 0x74, 0x20, 0x69,
    0x6e, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x61, 0x74,
    0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74,
    0x20, 0x70, 0x61, 0x63, 0x6b, 0x65, 0x74, 0x0a, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x73, 0x74,
    0x61, 0x72, 0x74, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x65, 0x63,
    0x65, 0x73, 0x73, 0x61, 0x72, 0x79, 0x20, 0x73, 0x69, 0x6e, 0x63, 0x65, 0x20, 0x72, 0x65, 0x61,
    0x64, 0x73, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x61, 0x6c, 0x69, 0x67, 0x6e, 0x0a, 0x20, 0x62,
    0x61, 0x63, 0x6b, 0x77, 0x61, 0x72, 0x64, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x20, 0x63, 0x68,
    0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x20, 0x63, 0x68, 0x75, 0x6e, 0x6b, 0x20, 0x62, 0x6f, 0x75,
    0x6e, 0x64, 0x61, 0x72, 0x79, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x86, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x05, 0x12,
    0x04, 0x86, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x01, 0x12, 0x04,
    0x86, 0x02, 0x12, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x03, 0x12, 0x04, 0x86,
    0x02, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x16, 0x12, 0x06, 0x89, 0x02, 0x00, 0x9c, 0x02,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x16, 0x01, 0x12, 0x04, 0x89, 0x02, 0x08, 0x1c, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x16, 0x02, 0x00, 0x12, 0x04, 0x8a, 0x02, 0x02, 0x1d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x16, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8a, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x16, 0x02, 0x00, 0x06, 0x12, 0x04, 0x8a, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x00, 0x01, 0x12, 0x04, 0x8a, 0x02, 0x12, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x00, 0x03, 0x12, 0x04, 0x8a, 0x02, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x01,
    0x12, 0x04, 0x8c, 0x02, 0x02, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x04, 0x12,
    0x04, 0x8c, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x05, 0x12, 0x04,
    0x8c, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8c,
    0x02, 0x12, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8c, 0x02,
    0x21, 0x22, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x02, 0x12, 0x04, 0x8d, 0x02, 0x02, 0x3d,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x02, 0x04, 0x12, 0x04, 0x8d, 0x02, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x02, 0x06, 0x12, 0x04, 0x8d, 0x02, 0x0b, 0x27, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x16, 0x02, 0x02, 0x01, 0x12, 0x04, 0x8d, 0x02, 0x28, 0x38, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x16, 0x02, 0x02, 0x03, 0x12, 0x04, 0x8d, 0x02, 0x3b, 0x3c, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x16, 0x02, 0x03, 0x12, 0x04, 0x8e, 0x02, 0x02, 0x3a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x03, 0x04, 0x12, 0x04, 0x8e, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x03, 0x06, 0x12, 0x04, 0x8e, 0x02, 0x0b, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x03,
    0x01, 0x12, 0x04, 0x8e, 0x02, 0x23, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x03, 0x03,
    0x12, 0x04, 0x8e, 0x02, 0x38, 0x39, 0x0a, 0x4f, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x04, 0x12, 0x04,
    0x91, 0x02, 0x02, 0x1e, 0x1a, 0x41, 0x2a, 0x20, 0x65, 0x78, 0x70, 0x6c, 0x61, 0x6e, 0x61, 0x74,
    0x6f, 0x72, 0x79, 0x20, 0x74, 0x65, 0x78, 0x74, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x6d,
    0x61, 0x79, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x66, 0x75, 0x6c, 0x20, 0x74, 0x6f, 0x20,
    0x6c, 0x6f, 0x67, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x20, 0x73, 0x69, 0x64, 0x65, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x91, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x04, 0x05, 0x12,
    0x04, 0x91, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x91, 0x02, 0x12, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x04, 0x03, 0x12, 0x04, 0x91,
    0x02, 0x1c, 0x1d, 0x0a, 0xc7, 0x02, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x05, 0x12, 0x04, 0x9b, 0x02,
    0x02, 0x30, 0x1a, 0xb8, 0x02, 0x2a, 0x20, 0x49, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65,
    0x72, 0x76, 0x65, 0x72, 0x20, 0x63, 0x68, 0x6f, 0x6f, 0x73, 0x65, 0x73, 0x20, 0x74, 0x6f, 0x20,
    0x61, 0x67, 0x72, 0x65, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x20, 0x66, 0x6f, 0x72, 0x0a, 0x20, 0x73, 0x68, 0x6f, 0x72, 0x74, 0x2d, 0x63, 0x69, 0x72, 0x63,
    0x75, 0x69, 0x74, 0x20, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x2c, 0x20, 0x69, 0x74, 0x20, 0x77,
    0x69, 0x6c, 0x6c, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x20, 0x61, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x77, 0x69, 0x74, 0x68,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x6c, 0x65, 0x76, 0x61, 0x6e, 0x74, 0x0a, 0x20, 0x66,
    0x69, 0x6c, 0x65, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x73, 0x20,
    0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x20, 0x49, 0x6e, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x62, 0x6f, 0x64, 0x79, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2c, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x76, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x77, 0x69, 0x6c, 0x6c,
    0x20, 0x62, 0x65, 0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20,
    0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x62,
    0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x69, 0x73, 0x20, 0x61, 0x62, 0x6f,
    0x75, 0x74, 0x20, 0x74, 0x6f, 0x0a, 0x20, 0x72, 0x65, 0x61, 0x64, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x16, 0x02, 0x05, 0x04, 0x12, 0x04, 0x9b, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x16, 0x02, 0x05, 0x05, 0x12, 0x04, 0x9b, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x16, 0x02, 0x05, 0x01, 0x12, 0x04, 0x9b, 0x02, 0x12, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x05, 0x03, 0x12, 0x04, 0x9b, 0x02, 0x2e, 0x2f, 0x0a, 0x60, 0x0a, 0x02, 0x04, 0x17, 0x12,
    0x06, 0xa2, 0x02, 0x00, 0xa4, 0x02, 0x01, 0x1a, 0x52, 0x2a, 0x0a, 0x20, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x44, 0x4e, 0x20, 0x61, 0x66, 0x74, 0x65, 0x72, 0x20, 0x72, 0x65, 0x61, 0x64, 0x69, 0x6e, 0x67,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x65, 0x6e, 0x74, 0x69, 0x72, 0x65, 0x0a, 0x20, 0x72, 0x65, 0x61,
    0x64, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x17, 0x01, 0x12, 0x04, 0xa2, 0x02, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x00,
    0x12, 0x04, 0xa3, 0x02, 0x02, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xa3, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x06, 0x12, 0x04,
    0xa3, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa3,
    0x02, 0x12, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa3, 0x02,
    0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x18, 0x12, 0x06, 0xa6, 0x02, 0x00, 0xa8, 0x02, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x18, 0x01, 0x12, 0x04, 0xa6, 0x02, 0x08, 0x1a, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x18, 0x02, 0x00, 0x12, 0x04, 0xa7, 0x02, 0x02, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x00, 0x04, 0x12, 0x04, 0xa7, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x00, 0x06, 0x12, 0x04, 0xa7, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xa7, 0x02, 0x12, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00,
    0x03, 0x12, 0x04, 0xa7, 0x02, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x19, 0x12, 0x06, 0xaa,
    0x02, 0x00, 0xaf, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x19, 0x01, 0x12, 0x04, 0xaa, 0x02,
    0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x00, 0x12, 0x04, 0xab, 0x02, 0x02, 0x22,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x04, 0x12, 0x04, 0xab, 0x02, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x05, 0x12, 0x04, 0xab, 0x02, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x01, 0x12, 0x04, 0xab, 0x02, 0x12, 0x1d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x19, 0x02, 0x00, 0x03, 0x12, 0x04, 0xab, 0x02, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x19, 0x02, 0x01, 0x12, 0x04, 0xac, 0x02, 0x02, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19,
    0x02, 0x01, 0x04, 0x12, 0x04, 0xac, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02,
    0x01, 0x05, 0x12, 0x04, 0xac, 0x02, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xac, 0x02, 0x12, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x03,
    0x12, 0x04, 0xac, 0x02, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x02, 0x12, 0x04,
    0xad, 0x02, 0x02, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x04, 0x12, 0x04, 0xad,
    0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x05, 0x12, 0x04, 0xad, 0x02,
    0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x01, 0x12, 0x04, 0xad, 0x02, 0x11,
    0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x03, 0x12, 0x04, 0xad, 0x02, 0x17, 0x18,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x03, 0x12, 0x04, 0xae, 0x02, 0x02, 0x29, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x19, 0x02, 0x03, 0x04, 0x12, 0x04, 0xae, 0x02, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x19, 0x02, 0x03, 0x06, 0x12, 0x04, 0xae, 0x02, 0x0b, 0x1c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x19, 0x02, 0x03, 0x01, 0x12, 0x04, 0xae, 0x02, 0x1d, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x19, 0x02, 0x03, 0x03, 0x12, 0x04, 0xae, 0x02, 0x27, 0x28,
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
