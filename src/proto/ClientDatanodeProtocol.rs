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
pub struct GetReplicaVisibleLengthRequestProto {
    // message fields
    block: ::protobuf::SingularPtrField<ExtendedBlockProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl GetReplicaVisibleLengthRequestProto {
    pub fn new() -> GetReplicaVisibleLengthRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetReplicaVisibleLengthRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetReplicaVisibleLengthRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetReplicaVisibleLengthRequestProto,
        };
        unsafe {
            instance.get(|| {
                GetReplicaVisibleLengthRequestProto {
                    block: ::protobuf::SingularPtrField::none(),
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
}

impl ::protobuf::Message for GetReplicaVisibleLengthRequestProto {
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
        ::std::any::TypeId::of::<GetReplicaVisibleLengthRequestProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetReplicaVisibleLengthRequestProto {
    fn new() -> GetReplicaVisibleLengthRequestProto {
        GetReplicaVisibleLengthRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetReplicaVisibleLengthRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "block",
                    GetReplicaVisibleLengthRequestProto::has_block,
                    GetReplicaVisibleLengthRequestProto::get_block,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetReplicaVisibleLengthRequestProto>(
                    "GetReplicaVisibleLengthRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetReplicaVisibleLengthRequestProto {
    fn clear(&mut self) {
        self.clear_block();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetReplicaVisibleLengthRequestProto {
    fn eq(&self, other: &GetReplicaVisibleLengthRequestProto) -> bool {
        self.block == other.block &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetReplicaVisibleLengthRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetReplicaVisibleLengthResponseProto {
    // message fields
    length: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl GetReplicaVisibleLengthResponseProto {
    pub fn new() -> GetReplicaVisibleLengthResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetReplicaVisibleLengthResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetReplicaVisibleLengthResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetReplicaVisibleLengthResponseProto,
        };
        unsafe {
            instance.get(|| {
                GetReplicaVisibleLengthResponseProto {
                    length: ::std::option::Option::None,
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
}

impl ::protobuf::Message for GetReplicaVisibleLengthResponseProto {
    fn is_initialized(&self) -> bool {
        if self.length.is_none() {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.length {
            try!(os.write_uint64(1, v));
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
        ::std::any::TypeId::of::<GetReplicaVisibleLengthResponseProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetReplicaVisibleLengthResponseProto {
    fn new() -> GetReplicaVisibleLengthResponseProto {
        GetReplicaVisibleLengthResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetReplicaVisibleLengthResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "length",
                    GetReplicaVisibleLengthResponseProto::has_length,
                    GetReplicaVisibleLengthResponseProto::get_length,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetReplicaVisibleLengthResponseProto>(
                    "GetReplicaVisibleLengthResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetReplicaVisibleLengthResponseProto {
    fn clear(&mut self) {
        self.clear_length();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetReplicaVisibleLengthResponseProto {
    fn eq(&self, other: &GetReplicaVisibleLengthResponseProto) -> bool {
        self.length == other.length &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetReplicaVisibleLengthResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RefreshNamenodesRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RefreshNamenodesRequestProto {
    pub fn new() -> RefreshNamenodesRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RefreshNamenodesRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<RefreshNamenodesRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RefreshNamenodesRequestProto,
        };
        unsafe {
            instance.get(|| {
                RefreshNamenodesRequestProto {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for RefreshNamenodesRequestProto {
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
        ::std::any::TypeId::of::<RefreshNamenodesRequestProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RefreshNamenodesRequestProto {
    fn new() -> RefreshNamenodesRequestProto {
        RefreshNamenodesRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RefreshNamenodesRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RefreshNamenodesRequestProto>(
                    "RefreshNamenodesRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RefreshNamenodesRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RefreshNamenodesRequestProto {
    fn eq(&self, other: &RefreshNamenodesRequestProto) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RefreshNamenodesRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RefreshNamenodesResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RefreshNamenodesResponseProto {
    pub fn new() -> RefreshNamenodesResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RefreshNamenodesResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<RefreshNamenodesResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RefreshNamenodesResponseProto,
        };
        unsafe {
            instance.get(|| {
                RefreshNamenodesResponseProto {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for RefreshNamenodesResponseProto {
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
        ::std::any::TypeId::of::<RefreshNamenodesResponseProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RefreshNamenodesResponseProto {
    fn new() -> RefreshNamenodesResponseProto {
        RefreshNamenodesResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RefreshNamenodesResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RefreshNamenodesResponseProto>(
                    "RefreshNamenodesResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RefreshNamenodesResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RefreshNamenodesResponseProto {
    fn eq(&self, other: &RefreshNamenodesResponseProto) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RefreshNamenodesResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DeleteBlockPoolRequestProto {
    // message fields
    blockPool: ::protobuf::SingularField<::std::string::String>,
    force: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl DeleteBlockPoolRequestProto {
    pub fn new() -> DeleteBlockPoolRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteBlockPoolRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<DeleteBlockPoolRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteBlockPoolRequestProto,
        };
        unsafe {
            instance.get(|| {
                DeleteBlockPoolRequestProto {
                    blockPool: ::protobuf::SingularField::none(),
                    force: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string blockPool = 1;

    pub fn clear_blockPool(&mut self) {
        self.blockPool.clear();
    }

    pub fn has_blockPool(&self) -> bool {
        self.blockPool.is_some()
    }

    // Param is passed by value, moved
    pub fn set_blockPool(&mut self, v: ::std::string::String) {
        self.blockPool = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_blockPool<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.blockPool.is_none() {
            self.blockPool.set_default();
        };
        self.blockPool.as_mut().unwrap()
    }

    // Take field
    pub fn take_blockPool(&mut self) -> ::std::string::String {
        self.blockPool.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_blockPool<'a>(&'a self) -> &'a str {
        match self.blockPool.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required bool force = 2;

    pub fn clear_force(&mut self) {
        self.force = ::std::option::Option::None;
    }

    pub fn has_force(&self) -> bool {
        self.force.is_some()
    }

    // Param is passed by value, moved
    pub fn set_force(&mut self, v: bool) {
        self.force = ::std::option::Option::Some(v);
    }

    pub fn get_force<'a>(&self) -> bool {
        self.force.unwrap_or(false)
    }
}

impl ::protobuf::Message for DeleteBlockPoolRequestProto {
    fn is_initialized(&self) -> bool {
        if self.blockPool.is_none() {
            return false;
        };
        if self.force.is_none() {
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
                    let tmp = self.blockPool.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.force = ::std::option::Option::Some(tmp);
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
        for value in self.blockPool.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if self.force.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.blockPool.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.force {
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
        ::std::any::TypeId::of::<DeleteBlockPoolRequestProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DeleteBlockPoolRequestProto {
    fn new() -> DeleteBlockPoolRequestProto {
        DeleteBlockPoolRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteBlockPoolRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "blockPool",
                    DeleteBlockPoolRequestProto::has_blockPool,
                    DeleteBlockPoolRequestProto::get_blockPool,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "force",
                    DeleteBlockPoolRequestProto::has_force,
                    DeleteBlockPoolRequestProto::get_force,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteBlockPoolRequestProto>(
                    "DeleteBlockPoolRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteBlockPoolRequestProto {
    fn clear(&mut self) {
        self.clear_blockPool();
        self.clear_force();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DeleteBlockPoolRequestProto {
    fn eq(&self, other: &DeleteBlockPoolRequestProto) -> bool {
        self.blockPool == other.blockPool &&
        self.force == other.force &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DeleteBlockPoolRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct DeleteBlockPoolResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl DeleteBlockPoolResponseProto {
    pub fn new() -> DeleteBlockPoolResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteBlockPoolResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<DeleteBlockPoolResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteBlockPoolResponseProto,
        };
        unsafe {
            instance.get(|| {
                DeleteBlockPoolResponseProto {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for DeleteBlockPoolResponseProto {
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
        ::std::any::TypeId::of::<DeleteBlockPoolResponseProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for DeleteBlockPoolResponseProto {
    fn new() -> DeleteBlockPoolResponseProto {
        DeleteBlockPoolResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteBlockPoolResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<DeleteBlockPoolResponseProto>(
                    "DeleteBlockPoolResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteBlockPoolResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for DeleteBlockPoolResponseProto {
    fn eq(&self, other: &DeleteBlockPoolResponseProto) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for DeleteBlockPoolResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetBlockLocalPathInfoRequestProto {
    // message fields
    block: ::protobuf::SingularPtrField<ExtendedBlockProto>,
    token: ::protobuf::SingularPtrField<TokenProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl GetBlockLocalPathInfoRequestProto {
    pub fn new() -> GetBlockLocalPathInfoRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetBlockLocalPathInfoRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetBlockLocalPathInfoRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetBlockLocalPathInfoRequestProto,
        };
        unsafe {
            instance.get(|| {
                GetBlockLocalPathInfoRequestProto {
                    block: ::protobuf::SingularPtrField::none(),
                    token: ::protobuf::SingularPtrField::none(),
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

    // required .hadoop.common.TokenProto token = 2;

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
}

impl ::protobuf::Message for GetBlockLocalPathInfoRequestProto {
    fn is_initialized(&self) -> bool {
        if self.block.is_none() {
            return false;
        };
        if self.token.is_none() {
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
        ::std::any::TypeId::of::<GetBlockLocalPathInfoRequestProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetBlockLocalPathInfoRequestProto {
    fn new() -> GetBlockLocalPathInfoRequestProto {
        GetBlockLocalPathInfoRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetBlockLocalPathInfoRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "block",
                    GetBlockLocalPathInfoRequestProto::has_block,
                    GetBlockLocalPathInfoRequestProto::get_block,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "token",
                    GetBlockLocalPathInfoRequestProto::has_token,
                    GetBlockLocalPathInfoRequestProto::get_token,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetBlockLocalPathInfoRequestProto>(
                    "GetBlockLocalPathInfoRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetBlockLocalPathInfoRequestProto {
    fn clear(&mut self) {
        self.clear_block();
        self.clear_token();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetBlockLocalPathInfoRequestProto {
    fn eq(&self, other: &GetBlockLocalPathInfoRequestProto) -> bool {
        self.block == other.block &&
        self.token == other.token &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetBlockLocalPathInfoRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetBlockLocalPathInfoResponseProto {
    // message fields
    block: ::protobuf::SingularPtrField<ExtendedBlockProto>,
    localPath: ::protobuf::SingularField<::std::string::String>,
    localMetaPath: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl GetBlockLocalPathInfoResponseProto {
    pub fn new() -> GetBlockLocalPathInfoResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetBlockLocalPathInfoResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetBlockLocalPathInfoResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetBlockLocalPathInfoResponseProto,
        };
        unsafe {
            instance.get(|| {
                GetBlockLocalPathInfoResponseProto {
                    block: ::protobuf::SingularPtrField::none(),
                    localPath: ::protobuf::SingularField::none(),
                    localMetaPath: ::protobuf::SingularField::none(),
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

    // required string localPath = 2;

    pub fn clear_localPath(&mut self) {
        self.localPath.clear();
    }

    pub fn has_localPath(&self) -> bool {
        self.localPath.is_some()
    }

    // Param is passed by value, moved
    pub fn set_localPath(&mut self, v: ::std::string::String) {
        self.localPath = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_localPath<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.localPath.is_none() {
            self.localPath.set_default();
        };
        self.localPath.as_mut().unwrap()
    }

    // Take field
    pub fn take_localPath(&mut self) -> ::std::string::String {
        self.localPath.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_localPath<'a>(&'a self) -> &'a str {
        match self.localPath.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string localMetaPath = 3;

    pub fn clear_localMetaPath(&mut self) {
        self.localMetaPath.clear();
    }

    pub fn has_localMetaPath(&self) -> bool {
        self.localMetaPath.is_some()
    }

    // Param is passed by value, moved
    pub fn set_localMetaPath(&mut self, v: ::std::string::String) {
        self.localMetaPath = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_localMetaPath<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.localMetaPath.is_none() {
            self.localMetaPath.set_default();
        };
        self.localMetaPath.as_mut().unwrap()
    }

    // Take field
    pub fn take_localMetaPath(&mut self) -> ::std::string::String {
        self.localMetaPath.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_localMetaPath<'a>(&'a self) -> &'a str {
        match self.localMetaPath.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for GetBlockLocalPathInfoResponseProto {
    fn is_initialized(&self) -> bool {
        if self.block.is_none() {
            return false;
        };
        if self.localPath.is_none() {
            return false;
        };
        if self.localMetaPath.is_none() {
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
                    let tmp = self.localPath.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.localMetaPath.set_default();
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
        for value in self.block.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.localPath.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.localMetaPath.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
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
        if let Some(v) = self.localPath.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.localMetaPath.as_ref() {
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
        ::std::any::TypeId::of::<GetBlockLocalPathInfoResponseProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetBlockLocalPathInfoResponseProto {
    fn new() -> GetBlockLocalPathInfoResponseProto {
        GetBlockLocalPathInfoResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetBlockLocalPathInfoResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "block",
                    GetBlockLocalPathInfoResponseProto::has_block,
                    GetBlockLocalPathInfoResponseProto::get_block,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "localPath",
                    GetBlockLocalPathInfoResponseProto::has_localPath,
                    GetBlockLocalPathInfoResponseProto::get_localPath,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "localMetaPath",
                    GetBlockLocalPathInfoResponseProto::has_localMetaPath,
                    GetBlockLocalPathInfoResponseProto::get_localMetaPath,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetBlockLocalPathInfoResponseProto>(
                    "GetBlockLocalPathInfoResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetBlockLocalPathInfoResponseProto {
    fn clear(&mut self) {
        self.clear_block();
        self.clear_localPath();
        self.clear_localMetaPath();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetBlockLocalPathInfoResponseProto {
    fn eq(&self, other: &GetBlockLocalPathInfoResponseProto) -> bool {
        self.block == other.block &&
        self.localPath == other.localPath &&
        self.localMetaPath == other.localMetaPath &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetBlockLocalPathInfoResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetHdfsBlockLocationsRequestProto {
    // message fields
    tokens: ::protobuf::RepeatedField<TokenProto>,
    blockPoolId: ::protobuf::SingularField<::std::string::String>,
    blockIds: ::std::vec::Vec<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl GetHdfsBlockLocationsRequestProto {
    pub fn new() -> GetHdfsBlockLocationsRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetHdfsBlockLocationsRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetHdfsBlockLocationsRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetHdfsBlockLocationsRequestProto,
        };
        unsafe {
            instance.get(|| {
                GetHdfsBlockLocationsRequestProto {
                    tokens: ::protobuf::RepeatedField::new(),
                    blockPoolId: ::protobuf::SingularField::none(),
                    blockIds: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .hadoop.common.TokenProto tokens = 2;

    pub fn clear_tokens(&mut self) {
        self.tokens.clear();
    }

    // Param is passed by value, moved
    pub fn set_tokens(&mut self, v: ::protobuf::RepeatedField<TokenProto>) {
        self.tokens = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tokens<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<TokenProto> {
        &mut self.tokens
    }

    // Take field
    pub fn take_tokens(&mut self) -> ::protobuf::RepeatedField<TokenProto> {
        ::std::mem::replace(&mut self.tokens, ::protobuf::RepeatedField::new())
    }

    pub fn get_tokens<'a>(&'a self) -> &'a [TokenProto] {
        &self.tokens
    }

    // required string blockPoolId = 3;

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

    // repeated sfixed64 blockIds = 4;

    pub fn clear_blockIds(&mut self) {
        self.blockIds.clear();
    }

    // Param is passed by value, moved
    pub fn set_blockIds(&mut self, v: ::std::vec::Vec<i64>) {
        self.blockIds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_blockIds<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<i64> {
        &mut self.blockIds
    }

    // Take field
    pub fn take_blockIds(&mut self) -> ::std::vec::Vec<i64> {
        ::std::mem::replace(&mut self.blockIds, ::std::vec::Vec::new())
    }

    pub fn get_blockIds<'a>(&'a self) -> &'a [i64] {
        &self.blockIds
    }
}

impl ::protobuf::Message for GetHdfsBlockLocationsRequestProto {
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
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tokens));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.blockPoolId.set_default();
                    try!(is.read_string_into(tmp))
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_sfixed64_into(wire_type, is, &mut self.blockIds));
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
        for value in self.tokens.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.blockPoolId.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        if !self.blockIds.is_empty() {
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(self.blockIds.len() as u32) + (self.blockIds.len() * 8) as u32;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.tokens.iter() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.blockPoolId.as_ref() {
            try!(os.write_string(3, &v));
        };
        if !self.blockIds.is_empty() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32((self.blockIds.len() * 8) as u32));
            for v in self.blockIds.iter() {
                try!(os.write_sfixed64_no_tag(*v));
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
        ::std::any::TypeId::of::<GetHdfsBlockLocationsRequestProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetHdfsBlockLocationsRequestProto {
    fn new() -> GetHdfsBlockLocationsRequestProto {
        GetHdfsBlockLocationsRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetHdfsBlockLocationsRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "tokens",
                    GetHdfsBlockLocationsRequestProto::get_tokens,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "blockPoolId",
                    GetHdfsBlockLocationsRequestProto::has_blockPoolId,
                    GetHdfsBlockLocationsRequestProto::get_blockPoolId,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_i64_accessor(
                    "blockIds",
                    GetHdfsBlockLocationsRequestProto::get_blockIds,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetHdfsBlockLocationsRequestProto>(
                    "GetHdfsBlockLocationsRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetHdfsBlockLocationsRequestProto {
    fn clear(&mut self) {
        self.clear_tokens();
        self.clear_blockPoolId();
        self.clear_blockIds();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetHdfsBlockLocationsRequestProto {
    fn eq(&self, other: &GetHdfsBlockLocationsRequestProto) -> bool {
        self.tokens == other.tokens &&
        self.blockPoolId == other.blockPoolId &&
        self.blockIds == other.blockIds &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetHdfsBlockLocationsRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetHdfsBlockLocationsResponseProto {
    // message fields
    volumeIds: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    volumeIndexes: ::std::vec::Vec<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl GetHdfsBlockLocationsResponseProto {
    pub fn new() -> GetHdfsBlockLocationsResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetHdfsBlockLocationsResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetHdfsBlockLocationsResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetHdfsBlockLocationsResponseProto,
        };
        unsafe {
            instance.get(|| {
                GetHdfsBlockLocationsResponseProto {
                    volumeIds: ::protobuf::RepeatedField::new(),
                    volumeIndexes: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated bytes volumeIds = 1;

    pub fn clear_volumeIds(&mut self) {
        self.volumeIds.clear();
    }

    // Param is passed by value, moved
    pub fn set_volumeIds(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.volumeIds = v;
    }

    // Mutable pointer to the field.
    pub fn mut_volumeIds<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.volumeIds
    }

    // Take field
    pub fn take_volumeIds(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.volumeIds, ::protobuf::RepeatedField::new())
    }

    pub fn get_volumeIds<'a>(&'a self) -> &'a [::std::vec::Vec<u8>] {
        &self.volumeIds
    }

    // repeated uint32 volumeIndexes = 2;

    pub fn clear_volumeIndexes(&mut self) {
        self.volumeIndexes.clear();
    }

    // Param is passed by value, moved
    pub fn set_volumeIndexes(&mut self, v: ::std::vec::Vec<u32>) {
        self.volumeIndexes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_volumeIndexes<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u32> {
        &mut self.volumeIndexes
    }

    // Take field
    pub fn take_volumeIndexes(&mut self) -> ::std::vec::Vec<u32> {
        ::std::mem::replace(&mut self.volumeIndexes, ::std::vec::Vec::new())
    }

    pub fn get_volumeIndexes<'a>(&'a self) -> &'a [u32] {
        &self.volumeIndexes
    }
}

impl ::protobuf::Message for GetHdfsBlockLocationsResponseProto {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.volumeIds));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_uint32_into(wire_type, is, &mut self.volumeIndexes));
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
        for value in self.volumeIds.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        if !self.volumeIndexes.is_empty() {
            my_size += ::protobuf::rt::vec_packed_varint_size(2, &self.volumeIndexes);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.volumeIds.iter() {
            try!(os.write_bytes(1, &v));
        };
        if !self.volumeIndexes.is_empty() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            // TODO: Data size is computed again, it should be cached
            try!(os.write_raw_varint32(::protobuf::rt::vec_packed_varint_data_size(&self.volumeIndexes)));
            for v in self.volumeIndexes.iter() {
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
        ::std::any::TypeId::of::<GetHdfsBlockLocationsResponseProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetHdfsBlockLocationsResponseProto {
    fn new() -> GetHdfsBlockLocationsResponseProto {
        GetHdfsBlockLocationsResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetHdfsBlockLocationsResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "volumeIds",
                    GetHdfsBlockLocationsResponseProto::get_volumeIds,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_u32_accessor(
                    "volumeIndexes",
                    GetHdfsBlockLocationsResponseProto::get_volumeIndexes,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetHdfsBlockLocationsResponseProto>(
                    "GetHdfsBlockLocationsResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetHdfsBlockLocationsResponseProto {
    fn clear(&mut self) {
        self.clear_volumeIds();
        self.clear_volumeIndexes();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetHdfsBlockLocationsResponseProto {
    fn eq(&self, other: &GetHdfsBlockLocationsResponseProto) -> bool {
        self.volumeIds == other.volumeIds &&
        self.volumeIndexes == other.volumeIndexes &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetHdfsBlockLocationsResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ShutdownDatanodeRequestProto {
    // message fields
    forUpgrade: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ShutdownDatanodeRequestProto {
    pub fn new() -> ShutdownDatanodeRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ShutdownDatanodeRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<ShutdownDatanodeRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ShutdownDatanodeRequestProto,
        };
        unsafe {
            instance.get(|| {
                ShutdownDatanodeRequestProto {
                    forUpgrade: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bool forUpgrade = 1;

    pub fn clear_forUpgrade(&mut self) {
        self.forUpgrade = ::std::option::Option::None;
    }

    pub fn has_forUpgrade(&self) -> bool {
        self.forUpgrade.is_some()
    }

    // Param is passed by value, moved
    pub fn set_forUpgrade(&mut self, v: bool) {
        self.forUpgrade = ::std::option::Option::Some(v);
    }

    pub fn get_forUpgrade<'a>(&self) -> bool {
        self.forUpgrade.unwrap_or(false)
    }
}

impl ::protobuf::Message for ShutdownDatanodeRequestProto {
    fn is_initialized(&self) -> bool {
        if self.forUpgrade.is_none() {
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
                    self.forUpgrade = ::std::option::Option::Some(tmp);
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
        if self.forUpgrade.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.forUpgrade {
            try!(os.write_bool(1, v));
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
        ::std::any::TypeId::of::<ShutdownDatanodeRequestProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ShutdownDatanodeRequestProto {
    fn new() -> ShutdownDatanodeRequestProto {
        ShutdownDatanodeRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ShutdownDatanodeRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "forUpgrade",
                    ShutdownDatanodeRequestProto::has_forUpgrade,
                    ShutdownDatanodeRequestProto::get_forUpgrade,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ShutdownDatanodeRequestProto>(
                    "ShutdownDatanodeRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ShutdownDatanodeRequestProto {
    fn clear(&mut self) {
        self.clear_forUpgrade();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ShutdownDatanodeRequestProto {
    fn eq(&self, other: &ShutdownDatanodeRequestProto) -> bool {
        self.forUpgrade == other.forUpgrade &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ShutdownDatanodeRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ShutdownDatanodeResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ShutdownDatanodeResponseProto {
    pub fn new() -> ShutdownDatanodeResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ShutdownDatanodeResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<ShutdownDatanodeResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ShutdownDatanodeResponseProto,
        };
        unsafe {
            instance.get(|| {
                ShutdownDatanodeResponseProto {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for ShutdownDatanodeResponseProto {
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
        ::std::any::TypeId::of::<ShutdownDatanodeResponseProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ShutdownDatanodeResponseProto {
    fn new() -> ShutdownDatanodeResponseProto {
        ShutdownDatanodeResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ShutdownDatanodeResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ShutdownDatanodeResponseProto>(
                    "ShutdownDatanodeResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ShutdownDatanodeResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ShutdownDatanodeResponseProto {
    fn eq(&self, other: &ShutdownDatanodeResponseProto) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ShutdownDatanodeResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetDatanodeInfoRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl GetDatanodeInfoRequestProto {
    pub fn new() -> GetDatanodeInfoRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetDatanodeInfoRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetDatanodeInfoRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetDatanodeInfoRequestProto,
        };
        unsafe {
            instance.get(|| {
                GetDatanodeInfoRequestProto {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for GetDatanodeInfoRequestProto {
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
        ::std::any::TypeId::of::<GetDatanodeInfoRequestProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetDatanodeInfoRequestProto {
    fn new() -> GetDatanodeInfoRequestProto {
        GetDatanodeInfoRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetDatanodeInfoRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetDatanodeInfoRequestProto>(
                    "GetDatanodeInfoRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetDatanodeInfoRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetDatanodeInfoRequestProto {
    fn eq(&self, other: &GetDatanodeInfoRequestProto) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetDatanodeInfoRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetDatanodeInfoResponseProto {
    // message fields
    localInfo: ::protobuf::SingularPtrField<DatanodeLocalInfoProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl GetDatanodeInfoResponseProto {
    pub fn new() -> GetDatanodeInfoResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetDatanodeInfoResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetDatanodeInfoResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetDatanodeInfoResponseProto,
        };
        unsafe {
            instance.get(|| {
                GetDatanodeInfoResponseProto {
                    localInfo: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.DatanodeLocalInfoProto localInfo = 1;

    pub fn clear_localInfo(&mut self) {
        self.localInfo.clear();
    }

    pub fn has_localInfo(&self) -> bool {
        self.localInfo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_localInfo(&mut self, v: DatanodeLocalInfoProto) {
        self.localInfo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_localInfo<'a>(&'a mut self) -> &'a mut DatanodeLocalInfoProto {
        if self.localInfo.is_none() {
            self.localInfo.set_default();
        };
        self.localInfo.as_mut().unwrap()
    }

    // Take field
    pub fn take_localInfo(&mut self) -> DatanodeLocalInfoProto {
        self.localInfo.take().unwrap_or_else(|| DatanodeLocalInfoProto::new())
    }

    pub fn get_localInfo<'a>(&'a self) -> &'a DatanodeLocalInfoProto {
        self.localInfo.as_ref().unwrap_or_else(|| DatanodeLocalInfoProto::default_instance())
    }
}

impl ::protobuf::Message for GetDatanodeInfoResponseProto {
    fn is_initialized(&self) -> bool {
        if self.localInfo.is_none() {
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
                    let tmp = self.localInfo.set_default();
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
        for value in self.localInfo.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.localInfo.as_ref() {
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
        ::std::any::TypeId::of::<GetDatanodeInfoResponseProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetDatanodeInfoResponseProto {
    fn new() -> GetDatanodeInfoResponseProto {
        GetDatanodeInfoResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetDatanodeInfoResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "localInfo",
                    GetDatanodeInfoResponseProto::has_localInfo,
                    GetDatanodeInfoResponseProto::get_localInfo,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetDatanodeInfoResponseProto>(
                    "GetDatanodeInfoResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetDatanodeInfoResponseProto {
    fn clear(&mut self) {
        self.clear_localInfo();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetDatanodeInfoResponseProto {
    fn eq(&self, other: &GetDatanodeInfoResponseProto) -> bool {
        self.localInfo == other.localInfo &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetDatanodeInfoResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StartReconfigurationRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl StartReconfigurationRequestProto {
    pub fn new() -> StartReconfigurationRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StartReconfigurationRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<StartReconfigurationRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StartReconfigurationRequestProto,
        };
        unsafe {
            instance.get(|| {
                StartReconfigurationRequestProto {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for StartReconfigurationRequestProto {
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
        ::std::any::TypeId::of::<StartReconfigurationRequestProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StartReconfigurationRequestProto {
    fn new() -> StartReconfigurationRequestProto {
        StartReconfigurationRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<StartReconfigurationRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<StartReconfigurationRequestProto>(
                    "StartReconfigurationRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StartReconfigurationRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StartReconfigurationRequestProto {
    fn eq(&self, other: &StartReconfigurationRequestProto) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StartReconfigurationRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StartReconfigurationResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl StartReconfigurationResponseProto {
    pub fn new() -> StartReconfigurationResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StartReconfigurationResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<StartReconfigurationResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StartReconfigurationResponseProto,
        };
        unsafe {
            instance.get(|| {
                StartReconfigurationResponseProto {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for StartReconfigurationResponseProto {
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
        ::std::any::TypeId::of::<StartReconfigurationResponseProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StartReconfigurationResponseProto {
    fn new() -> StartReconfigurationResponseProto {
        StartReconfigurationResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<StartReconfigurationResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<StartReconfigurationResponseProto>(
                    "StartReconfigurationResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StartReconfigurationResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StartReconfigurationResponseProto {
    fn eq(&self, other: &StartReconfigurationResponseProto) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StartReconfigurationResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TriggerBlockReportRequestProto {
    // message fields
    incremental: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl TriggerBlockReportRequestProto {
    pub fn new() -> TriggerBlockReportRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TriggerBlockReportRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<TriggerBlockReportRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TriggerBlockReportRequestProto,
        };
        unsafe {
            instance.get(|| {
                TriggerBlockReportRequestProto {
                    incremental: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required bool incremental = 1;

    pub fn clear_incremental(&mut self) {
        self.incremental = ::std::option::Option::None;
    }

    pub fn has_incremental(&self) -> bool {
        self.incremental.is_some()
    }

    // Param is passed by value, moved
    pub fn set_incremental(&mut self, v: bool) {
        self.incremental = ::std::option::Option::Some(v);
    }

    pub fn get_incremental<'a>(&self) -> bool {
        self.incremental.unwrap_or(false)
    }
}

impl ::protobuf::Message for TriggerBlockReportRequestProto {
    fn is_initialized(&self) -> bool {
        if self.incremental.is_none() {
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
                    self.incremental = ::std::option::Option::Some(tmp);
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
        if self.incremental.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.incremental {
            try!(os.write_bool(1, v));
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
        ::std::any::TypeId::of::<TriggerBlockReportRequestProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TriggerBlockReportRequestProto {
    fn new() -> TriggerBlockReportRequestProto {
        TriggerBlockReportRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<TriggerBlockReportRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "incremental",
                    TriggerBlockReportRequestProto::has_incremental,
                    TriggerBlockReportRequestProto::get_incremental,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TriggerBlockReportRequestProto>(
                    "TriggerBlockReportRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TriggerBlockReportRequestProto {
    fn clear(&mut self) {
        self.clear_incremental();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TriggerBlockReportRequestProto {
    fn eq(&self, other: &TriggerBlockReportRequestProto) -> bool {
        self.incremental == other.incremental &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TriggerBlockReportRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TriggerBlockReportResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl TriggerBlockReportResponseProto {
    pub fn new() -> TriggerBlockReportResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TriggerBlockReportResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<TriggerBlockReportResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TriggerBlockReportResponseProto,
        };
        unsafe {
            instance.get(|| {
                TriggerBlockReportResponseProto {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for TriggerBlockReportResponseProto {
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
        ::std::any::TypeId::of::<TriggerBlockReportResponseProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TriggerBlockReportResponseProto {
    fn new() -> TriggerBlockReportResponseProto {
        TriggerBlockReportResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<TriggerBlockReportResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<TriggerBlockReportResponseProto>(
                    "TriggerBlockReportResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TriggerBlockReportResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TriggerBlockReportResponseProto {
    fn eq(&self, other: &TriggerBlockReportResponseProto) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TriggerBlockReportResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetReconfigurationStatusRequestProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl GetReconfigurationStatusRequestProto {
    pub fn new() -> GetReconfigurationStatusRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetReconfigurationStatusRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetReconfigurationStatusRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetReconfigurationStatusRequestProto,
        };
        unsafe {
            instance.get(|| {
                GetReconfigurationStatusRequestProto {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for GetReconfigurationStatusRequestProto {
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
        ::std::any::TypeId::of::<GetReconfigurationStatusRequestProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetReconfigurationStatusRequestProto {
    fn new() -> GetReconfigurationStatusRequestProto {
        GetReconfigurationStatusRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetReconfigurationStatusRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetReconfigurationStatusRequestProto>(
                    "GetReconfigurationStatusRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetReconfigurationStatusRequestProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetReconfigurationStatusRequestProto {
    fn eq(&self, other: &GetReconfigurationStatusRequestProto) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetReconfigurationStatusRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetReconfigurationStatusConfigChangeProto {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    oldValue: ::protobuf::SingularField<::std::string::String>,
    newValue: ::protobuf::SingularField<::std::string::String>,
    errorMessage: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl GetReconfigurationStatusConfigChangeProto {
    pub fn new() -> GetReconfigurationStatusConfigChangeProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetReconfigurationStatusConfigChangeProto {
        static mut instance: ::protobuf::lazy::Lazy<GetReconfigurationStatusConfigChangeProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetReconfigurationStatusConfigChangeProto,
        };
        unsafe {
            instance.get(|| {
                GetReconfigurationStatusConfigChangeProto {
                    name: ::protobuf::SingularField::none(),
                    oldValue: ::protobuf::SingularField::none(),
                    newValue: ::protobuf::SingularField::none(),
                    errorMessage: ::protobuf::SingularField::none(),
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

    // required string oldValue = 2;

    pub fn clear_oldValue(&mut self) {
        self.oldValue.clear();
    }

    pub fn has_oldValue(&self) -> bool {
        self.oldValue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_oldValue(&mut self, v: ::std::string::String) {
        self.oldValue = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_oldValue<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.oldValue.is_none() {
            self.oldValue.set_default();
        };
        self.oldValue.as_mut().unwrap()
    }

    // Take field
    pub fn take_oldValue(&mut self) -> ::std::string::String {
        self.oldValue.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_oldValue<'a>(&'a self) -> &'a str {
        match self.oldValue.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string newValue = 3;

    pub fn clear_newValue(&mut self) {
        self.newValue.clear();
    }

    pub fn has_newValue(&self) -> bool {
        self.newValue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_newValue(&mut self, v: ::std::string::String) {
        self.newValue = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_newValue<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.newValue.is_none() {
            self.newValue.set_default();
        };
        self.newValue.as_mut().unwrap()
    }

    // Take field
    pub fn take_newValue(&mut self) -> ::std::string::String {
        self.newValue.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_newValue<'a>(&'a self) -> &'a str {
        match self.newValue.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string errorMessage = 4;

    pub fn clear_errorMessage(&mut self) {
        self.errorMessage.clear();
    }

    pub fn has_errorMessage(&self) -> bool {
        self.errorMessage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_errorMessage(&mut self, v: ::std::string::String) {
        self.errorMessage = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_errorMessage<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.errorMessage.is_none() {
            self.errorMessage.set_default();
        };
        self.errorMessage.as_mut().unwrap()
    }

    // Take field
    pub fn take_errorMessage(&mut self) -> ::std::string::String {
        self.errorMessage.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_errorMessage<'a>(&'a self) -> &'a str {
        match self.errorMessage.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for GetReconfigurationStatusConfigChangeProto {
    fn is_initialized(&self) -> bool {
        if self.name.is_none() {
            return false;
        };
        if self.oldValue.is_none() {
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
                    let tmp = self.name.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.oldValue.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.newValue.set_default();
                    try!(is.read_string_into(tmp))
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.errorMessage.set_default();
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
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.oldValue.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.newValue.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in self.errorMessage.iter() {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.oldValue.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.newValue.as_ref() {
            try!(os.write_string(3, &v));
        };
        if let Some(v) = self.errorMessage.as_ref() {
            try!(os.write_string(4, &v));
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
        ::std::any::TypeId::of::<GetReconfigurationStatusConfigChangeProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetReconfigurationStatusConfigChangeProto {
    fn new() -> GetReconfigurationStatusConfigChangeProto {
        GetReconfigurationStatusConfigChangeProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetReconfigurationStatusConfigChangeProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    GetReconfigurationStatusConfigChangeProto::has_name,
                    GetReconfigurationStatusConfigChangeProto::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "oldValue",
                    GetReconfigurationStatusConfigChangeProto::has_oldValue,
                    GetReconfigurationStatusConfigChangeProto::get_oldValue,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "newValue",
                    GetReconfigurationStatusConfigChangeProto::has_newValue,
                    GetReconfigurationStatusConfigChangeProto::get_newValue,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "errorMessage",
                    GetReconfigurationStatusConfigChangeProto::has_errorMessage,
                    GetReconfigurationStatusConfigChangeProto::get_errorMessage,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetReconfigurationStatusConfigChangeProto>(
                    "GetReconfigurationStatusConfigChangeProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetReconfigurationStatusConfigChangeProto {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_oldValue();
        self.clear_newValue();
        self.clear_errorMessage();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetReconfigurationStatusConfigChangeProto {
    fn eq(&self, other: &GetReconfigurationStatusConfigChangeProto) -> bool {
        self.name == other.name &&
        self.oldValue == other.oldValue &&
        self.newValue == other.newValue &&
        self.errorMessage == other.errorMessage &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetReconfigurationStatusConfigChangeProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetReconfigurationStatusResponseProto {
    // message fields
    startTime: ::std::option::Option<i64>,
    endTime: ::std::option::Option<i64>,
    changes: ::protobuf::RepeatedField<GetReconfigurationStatusConfigChangeProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl GetReconfigurationStatusResponseProto {
    pub fn new() -> GetReconfigurationStatusResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetReconfigurationStatusResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetReconfigurationStatusResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetReconfigurationStatusResponseProto,
        };
        unsafe {
            instance.get(|| {
                GetReconfigurationStatusResponseProto {
                    startTime: ::std::option::Option::None,
                    endTime: ::std::option::Option::None,
                    changes: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int64 startTime = 1;

    pub fn clear_startTime(&mut self) {
        self.startTime = ::std::option::Option::None;
    }

    pub fn has_startTime(&self) -> bool {
        self.startTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_startTime(&mut self, v: i64) {
        self.startTime = ::std::option::Option::Some(v);
    }

    pub fn get_startTime<'a>(&self) -> i64 {
        self.startTime.unwrap_or(0)
    }

    // optional int64 endTime = 2;

    pub fn clear_endTime(&mut self) {
        self.endTime = ::std::option::Option::None;
    }

    pub fn has_endTime(&self) -> bool {
        self.endTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_endTime(&mut self, v: i64) {
        self.endTime = ::std::option::Option::Some(v);
    }

    pub fn get_endTime<'a>(&self) -> i64 {
        self.endTime.unwrap_or(0)
    }

    // repeated .hadoop.hdfs.GetReconfigurationStatusConfigChangeProto changes = 3;

    pub fn clear_changes(&mut self) {
        self.changes.clear();
    }

    // Param is passed by value, moved
    pub fn set_changes(&mut self, v: ::protobuf::RepeatedField<GetReconfigurationStatusConfigChangeProto>) {
        self.changes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_changes<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<GetReconfigurationStatusConfigChangeProto> {
        &mut self.changes
    }

    // Take field
    pub fn take_changes(&mut self) -> ::protobuf::RepeatedField<GetReconfigurationStatusConfigChangeProto> {
        ::std::mem::replace(&mut self.changes, ::protobuf::RepeatedField::new())
    }

    pub fn get_changes<'a>(&'a self) -> &'a [GetReconfigurationStatusConfigChangeProto] {
        &self.changes
    }
}

impl ::protobuf::Message for GetReconfigurationStatusResponseProto {
    fn is_initialized(&self) -> bool {
        if self.startTime.is_none() {
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
                    self.startTime = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.endTime = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.changes));
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
        for value in self.startTime.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.endTime.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.changes.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.startTime {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.endTime {
            try!(os.write_int64(2, v));
        };
        for v in self.changes.iter() {
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
        ::std::any::TypeId::of::<GetReconfigurationStatusResponseProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetReconfigurationStatusResponseProto {
    fn new() -> GetReconfigurationStatusResponseProto {
        GetReconfigurationStatusResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetReconfigurationStatusResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "startTime",
                    GetReconfigurationStatusResponseProto::has_startTime,
                    GetReconfigurationStatusResponseProto::get_startTime,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "endTime",
                    GetReconfigurationStatusResponseProto::has_endTime,
                    GetReconfigurationStatusResponseProto::get_endTime,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "changes",
                    GetReconfigurationStatusResponseProto::get_changes,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetReconfigurationStatusResponseProto>(
                    "GetReconfigurationStatusResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetReconfigurationStatusResponseProto {
    fn clear(&mut self) {
        self.clear_startTime();
        self.clear_endTime();
        self.clear_changes();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetReconfigurationStatusResponseProto {
    fn eq(&self, other: &GetReconfigurationStatusResponseProto) -> bool {
        self.startTime == other.startTime &&
        self.endTime == other.endTime &&
        self.changes == other.changes &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetReconfigurationStatusResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x1c, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x44, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b,
    0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x1a, 0x0e, 0x53, 0x65, 0x63,
    0x75, 0x72, 0x69, 0x74, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0a, 0x68, 0x64, 0x66,
    0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x55, 0x0a, 0x23, 0x47, 0x65, 0x74, 0x52, 0x65,
    0x70, 0x6c, 0x69, 0x63, 0x61, 0x56, 0x69, 0x73, 0x69, 0x62, 0x6c, 0x65, 0x4c, 0x65, 0x6e, 0x67,
    0x74, 0x68, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2e,
    0x0a, 0x05, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x1f, 0x2e,
    0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x45, 0x78, 0x74, 0x65,
    0x6e, 0x64, 0x65, 0x64, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x36,
    0x0a, 0x24, 0x47, 0x65, 0x74, 0x52, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x56, 0x69, 0x73, 0x69,
    0x62, 0x6c, 0x65, 0x4c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0e, 0x0a, 0x06, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68,
    0x18, 0x01, 0x20, 0x02, 0x28, 0x04, 0x22, 0x1e, 0x0a, 0x1c, 0x52, 0x65, 0x66, 0x72, 0x65, 0x73,
    0x68, 0x4e, 0x61, 0x6d, 0x65, 0x6e, 0x6f, 0x64, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x1f, 0x0a, 0x1d, 0x52, 0x65, 0x66, 0x72, 0x65, 0x73,
    0x68, 0x4e, 0x61, 0x6d, 0x65, 0x6e, 0x6f, 0x64, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x3f, 0x0a, 0x1b, 0x44, 0x65, 0x6c, 0x65, 0x74,
    0x65, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x50, 0x6f, 0x6f, 0x6c, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x11, 0x0a, 0x09, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x50,
    0x6f, 0x6f, 0x6c, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0d, 0x0a, 0x05, 0x66, 0x6f, 0x72,
    0x63, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x08, 0x22, 0x1e, 0x0a, 0x1c, 0x44, 0x65, 0x6c, 0x65,
    0x74, 0x65, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x50, 0x6f, 0x6f, 0x6c, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x7d, 0x0a, 0x21, 0x47, 0x65, 0x74, 0x42,
    0x6c, 0x6f, 0x63, 0x6b, 0x4c, 0x6f, 0x63, 0x61, 0x6c, 0x50, 0x61, 0x74, 0x68, 0x49, 0x6e, 0x66,
    0x6f, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2e, 0x0a,
    0x05, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x68,
    0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x45, 0x78, 0x74, 0x65, 0x6e,
    0x64, 0x65, 0x64, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x28, 0x0a,
    0x05, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x68,
    0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x54, 0x6f, 0x6b,
    0x65, 0x6e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x7e, 0x0a, 0x22, 0x47, 0x65, 0x74, 0x42, 0x6c,
    0x6f, 0x63, 0x6b, 0x4c, 0x6f, 0x63, 0x61, 0x6c, 0x50, 0x61, 0x74, 0x68, 0x49, 0x6e, 0x66, 0x6f,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2e, 0x0a,
    0x05, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x68,
    0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x45, 0x78, 0x74, 0x65, 0x6e,
    0x64, 0x65, 0x64, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x11, 0x0a,
    0x09, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x50, 0x61, 0x74, 0x68, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09,
    0x12, 0x15, 0x0a, 0x0d, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x4d, 0x65, 0x74, 0x61, 0x50, 0x61, 0x74,
    0x68, 0x18, 0x03, 0x20, 0x02, 0x28, 0x09, 0x22, 0x79, 0x0a, 0x21, 0x47, 0x65, 0x74, 0x48, 0x64,
    0x66, 0x73, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x29, 0x0a, 0x06,
    0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x68,
    0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x54, 0x6f, 0x6b,
    0x65, 0x6e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x13, 0x0a, 0x0b, 0x62, 0x6c, 0x6f, 0x63, 0x6b,
    0x50, 0x6f, 0x6f, 0x6c, 0x49, 0x64, 0x18, 0x03, 0x20, 0x02, 0x28, 0x09, 0x12, 0x14, 0x0a, 0x08,
    0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x49, 0x64, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x10, 0x42, 0x02,
    0x10, 0x01, 0x22, 0x52, 0x0a, 0x22, 0x47, 0x65, 0x74, 0x48, 0x64, 0x66, 0x73, 0x42, 0x6c, 0x6f,
    0x63, 0x6b, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x11, 0x0a, 0x09, 0x76, 0x6f, 0x6c, 0x75,
    0x6d, 0x65, 0x49, 0x64, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0c, 0x12, 0x19, 0x0a, 0x0d, 0x76,
    0x6f, 0x6c, 0x75, 0x6d, 0x65, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03,
    0x28, 0x0d, 0x42, 0x02, 0x10, 0x01, 0x22, 0x32, 0x0a, 0x1c, 0x53, 0x68, 0x75, 0x74, 0x64, 0x6f,
    0x77, 0x6e, 0x44, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x12, 0x0a, 0x0a, 0x66, 0x6f, 0x72, 0x55, 0x70, 0x67,
    0x72, 0x61, 0x64, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x08, 0x22, 0x1f, 0x0a, 0x1d, 0x53, 0x68,
    0x75, 0x74, 0x64, 0x6f, 0x77, 0x6e, 0x44, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x1d, 0x0a, 0x1b, 0x47,
    0x65, 0x74, 0x44, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x56, 0x0a, 0x1c, 0x47, 0x65,
    0x74, 0x44, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x36, 0x0a, 0x09, 0x6c, 0x6f,
    0x63, 0x61, 0x6c, 0x49, 0x6e, 0x66, 0x6f, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x23, 0x2e,
    0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61,
    0x6e, 0x6f, 0x64, 0x65, 0x4c, 0x6f, 0x63, 0x61, 0x6c, 0x49, 0x6e, 0x66, 0x6f, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x22, 0x22, 0x0a, 0x20, 0x53, 0x74, 0x61, 0x72, 0x74, 0x52, 0x65, 0x63, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x23, 0x0a, 0x21, 0x53, 0x74, 0x61, 0x72, 0x74, 0x52,
    0x65, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x35, 0x0a, 0x1e, 0x54,
    0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x52, 0x65, 0x70, 0x6f, 0x72,
    0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x13, 0x0a,
    0x0b, 0x69, 0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x6c, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x08, 0x22, 0x21, 0x0a, 0x1f, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x42, 0x6c, 0x6f,
    0x63, 0x6b, 0x52, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x26, 0x0a, 0x24, 0x47, 0x65, 0x74, 0x52, 0x65, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x73, 0x0a,
    0x29, 0x47, 0x65, 0x74, 0x52, 0x65, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x43,
    0x68, 0x61, 0x6e, 0x67, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0c, 0x0a, 0x04, 0x6e, 0x61,
    0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x10, 0x0a, 0x08, 0x6f, 0x6c, 0x64, 0x56,
    0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x12, 0x10, 0x0a, 0x08, 0x6e, 0x65,
    0x77, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x12, 0x14, 0x0a, 0x0c,
    0x65, 0x72, 0x72, 0x6f, 0x72, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x09, 0x22, 0x94, 0x01, 0x0a, 0x25, 0x47, 0x65, 0x74, 0x52, 0x65, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x11, 0x0a, 0x09,
    0x73, 0x74, 0x61, 0x72, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x03, 0x12,
    0x0f, 0x0a, 0x07, 0x65, 0x6e, 0x64, 0x54, 0x69, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03,
    0x12, 0x47, 0x0a, 0x07, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x36, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e,
    0x47, 0x65, 0x74, 0x52, 0x65, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x43, 0x68,
    0x61, 0x6e, 0x67, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x32, 0xa5, 0x09, 0x0a, 0x1d, 0x43, 0x6c,
    0x69, 0x65, 0x6e, 0x74, 0x44, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x63, 0x6f, 0x6c, 0x53, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x12, 0x7e, 0x0a, 0x17, 0x67,
    0x65, 0x74, 0x52, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x56, 0x69, 0x73, 0x69, 0x62, 0x6c, 0x65,
    0x4c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x12, 0x30, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e,
    0x68, 0x64, 0x66, 0x73, 0x2e, 0x47, 0x65, 0x74, 0x52, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x56,
    0x69, 0x73, 0x69, 0x62, 0x6c, 0x65, 0x4c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x31, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f,
    0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x47, 0x65, 0x74, 0x52, 0x65, 0x70, 0x6c, 0x69, 0x63,
    0x61, 0x56, 0x69, 0x73, 0x69, 0x62, 0x6c, 0x65, 0x4c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x69, 0x0a, 0x10, 0x72,
    0x65, 0x66, 0x72, 0x65, 0x73, 0x68, 0x4e, 0x61, 0x6d, 0x65, 0x6e, 0x6f, 0x64, 0x65, 0x73, 0x12,
    0x29, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x52, 0x65,
    0x66, 0x72, 0x65, 0x73, 0x68, 0x4e, 0x61, 0x6d, 0x65, 0x6e, 0x6f, 0x64, 0x65, 0x73, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x2a, 0x2e, 0x68, 0x61, 0x64,
    0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x52, 0x65, 0x66, 0x72, 0x65, 0x73, 0x68,
    0x4e, 0x61, 0x6d, 0x65, 0x6e, 0x6f, 0x64, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x66, 0x0a, 0x0f, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65,
    0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x50, 0x6f, 0x6f, 0x6c, 0x12, 0x28, 0x2e, 0x68, 0x61, 0x64, 0x6f,
    0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x42, 0x6c,
    0x6f, 0x63, 0x6b, 0x50, 0x6f, 0x6f, 0x6c, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x1a, 0x29, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66,
    0x73, 0x2e, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x50, 0x6f, 0x6f,
    0x6c, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x78,
    0x0a, 0x15, 0x67, 0x65, 0x74, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x4c, 0x6f, 0x63, 0x61, 0x6c, 0x50,
    0x61, 0x74, 0x68, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x2e, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70,
    0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x47, 0x65, 0x74, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x4c, 0x6f,
    0x63, 0x61, 0x6c, 0x50, 0x61, 0x74, 0x68, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x2f, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70,
    0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x47, 0x65, 0x74, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x4c, 0x6f,
    0x63, 0x61, 0x6c, 0x50, 0x61, 0x74, 0x68, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x78, 0x0a, 0x15, 0x67, 0x65, 0x74, 0x48,
    0x64, 0x66, 0x73, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x4c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x12, 0x2e, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e,
    0x47, 0x65, 0x74, 0x48, 0x64, 0x66, 0x73, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x4c, 0x6f, 0x63, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x2f, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e,
    0x47, 0x65, 0x74, 0x48, 0x64, 0x66, 0x73, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x4c, 0x6f, 0x63, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x69, 0x0a, 0x10, 0x73, 0x68, 0x75, 0x74, 0x64, 0x6f, 0x77, 0x6e, 0x44, 0x61,
    0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x12, 0x29, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e,
    0x68, 0x64, 0x66, 0x73, 0x2e, 0x53, 0x68, 0x75, 0x74, 0x64, 0x6f, 0x77, 0x6e, 0x44, 0x61, 0x74,
    0x61, 0x6e, 0x6f, 0x64, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x2a, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e,
    0x53, 0x68, 0x75, 0x74, 0x64, 0x6f, 0x77, 0x6e, 0x44, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x66, 0x0a,
    0x0f, 0x67, 0x65, 0x74, 0x44, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x49, 0x6e, 0x66, 0x6f,
    0x12, 0x28, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x47,
    0x65, 0x74, 0x44, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x29, 0x2e, 0x68, 0x61, 0x64,
    0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x47, 0x65, 0x74, 0x44, 0x61, 0x74, 0x61,
    0x6e, 0x6f, 0x64, 0x65, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x81, 0x01, 0x0a, 0x18, 0x67, 0x65, 0x74, 0x52, 0x65, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x12, 0x31, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73,
    0x2e, 0x47, 0x65, 0x74, 0x52, 0x65, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x32, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68,
    0x64, 0x66, 0x73, 0x2e, 0x47, 0x65, 0x74, 0x52, 0x65, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75,
    0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x75, 0x0a, 0x14, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x52, 0x65, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x2d, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e,
    0x53, 0x74, 0x61, 0x72, 0x74, 0x52, 0x65, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x1a, 0x2e, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x53,
    0x74, 0x61, 0x72, 0x74, 0x52, 0x65, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x6f, 0x0a, 0x12, 0x74, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x42, 0x6c, 0x6f, 0x63, 0x6b,
    0x52, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x12, 0x2b, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e,
    0x68, 0x64, 0x66, 0x73, 0x2e, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x42, 0x6c, 0x6f, 0x63,
    0x6b, 0x52, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x1a, 0x2c, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66,
    0x73, 0x2e, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x52, 0x65,
    0x70, 0x6f, 0x72, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x42, 0x4b, 0x0a, 0x25, 0x6f, 0x72, 0x67, 0x2e, 0x61, 0x70, 0x61, 0x63, 0x68, 0x65, 0x2e,
    0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x63, 0x6f, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x42, 0x1c, 0x43, 0x6c, 0x69, 0x65,
    0x6e, 0x74, 0x44, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63,
    0x6f, 0x6c, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0xa0, 0x01, 0x01, 0x88, 0x01, 0x01, 0x4a, 0xb5,
    0x2c, 0x0a, 0x07, 0x12, 0x05, 0x1b, 0x00, 0xea, 0x01, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x1b, 0x00, 0x3e, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x1b, 0x00,
    0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x1b, 0x07, 0x13, 0x0a,
    0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1b, 0x07, 0x13, 0x0a, 0x0e,
    0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x07, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x03, 0x1b, 0x16, 0x3d, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x1c, 0x00, 0x3d, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01, 0x12, 0x03,
    0x1c, 0x00, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x1c, 0x07,
    0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1c, 0x07, 0x1b,
    0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x07, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x07, 0x12, 0x03, 0x1c, 0x1e, 0x3c, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x1d, 0x00, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x02,
    0x12, 0x03, 0x1d, 0x00, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x02, 0x02, 0x12, 0x03,
    0x1d, 0x07, 0x1c, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x02, 0x02, 0x00, 0x12, 0x03, 0x1d,
    0x07, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d,
    0x07, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x02, 0x03, 0x12, 0x03, 0x1d, 0x1f, 0x23,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x1e, 0x00, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7,
    0x07, 0x03, 0x12, 0x03, 0x1e, 0x00, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x03, 0x02,
    0x12, 0x03, 0x1e, 0x07, 0x24, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x03, 0x02, 0x00, 0x12,
    0x03, 0x1e, 0x07, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x03, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x1e, 0x07, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x03, 0x03, 0x12, 0x03, 0x1e,
    0x27, 0x2b, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x1f, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x00, 0x12, 0x03, 0x21, 0x07, 0x17, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x22,
    0x07, 0x13, 0x0a, 0x43, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x27, 0x00, 0x29, 0x01, 0x1a, 0x37,
    0x2a, 0x0a, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x2d, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b,
    0x20, 0x66, 0x6f, 0x72, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x76, 0x69, 0x73, 0x69, 0x62,
    0x6c, 0x65, 0x20, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x20, 0x69, 0x73, 0x20, 0x72, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x65, 0x64, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x27, 0x08, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x28, 0x02, 0x28,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x28, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x28, 0x0b, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x28, 0x1e, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x28, 0x26, 0x27, 0x0a, 0x34, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x2e, 0x00, 0x30, 0x01, 0x1a, 0x28, 0x2a, 0x0a, 0x20, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x20,
    0x2d, 0x20, 0x76, 0x69, 0x73, 0x69, 0x62, 0x6c, 0x65, 0x20, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68,
    0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x2e, 0x08, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x00, 0x12, 0x03, 0x2f, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x2f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x2f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2f, 0x12,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2f, 0x1b, 0x1c, 0x0a,
    0x1c, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x35, 0x00, 0x36, 0x01, 0x1a, 0x10, 0x2a, 0x0a, 0x20,
    0x76, 0x6f, 0x69, 0x64, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x35, 0x08, 0x24, 0x0a, 0x1d, 0x0a, 0x02, 0x04, 0x03, 0x12,
    0x04, 0x3b, 0x00, 0x3c, 0x01, 0x1a, 0x11, 0x2a, 0x0a, 0x20, 0x76, 0x6f, 0x69, 0x64, 0x20, 0x72,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12,
    0x03, 0x3b, 0x08, 0x25, 0x0a, 0xb2, 0x01, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x43, 0x00, 0x46,
    0x01, 0x1a, 0xa5, 0x01, 0x2a, 0x0a, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x50, 0x6f, 0x6f, 0x6c,
    0x20, 0x2d, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x70, 0x6f, 0x6f, 0x6c, 0x20, 0x74, 0x6f,
    0x20, 0x62, 0x65, 0x20, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x0a, 0x20, 0x66, 0x6f, 0x72,
    0x63, 0x65, 0x20, 0x2d, 0x20, 0x69, 0x66, 0x20, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x2c, 0x20, 0x64,
    0x65, 0x6c, 0x65, 0x74, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20,
    0x70, 0x6f, 0x6f, 0x6c, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x69, 0x66, 0x20, 0x69, 0x74, 0x20,
    0x69, 0x73, 0x20, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x2e, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x69, 0x66, 0x20, 0x74, 0x72, 0x75, 0x65, 0x2c, 0x20, 0x64, 0x65, 0x6c, 0x65,
    0x74, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x70, 0x6f, 0x6f,
    0x6c, 0x20, 0x65, 0x76, 0x65, 0x6e, 0x20, 0x69, 0x66, 0x20, 0x69, 0x74, 0x20, 0x68, 0x61, 0x73,
    0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01,
    0x12, 0x03, 0x43, 0x08, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x44,
    0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x44, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x44, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x44, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x44, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x01, 0x12, 0x03, 0x45, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x45, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x45, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x45, 0x10,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x45, 0x18, 0x19, 0x0a,
    0x1d, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x4b, 0x00, 0x4c, 0x01, 0x1a, 0x11, 0x2a, 0x0a, 0x20,
    0x76, 0x6f, 0x69, 0x64, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x4b, 0x08, 0x24, 0x0a, 0xe5, 0x01, 0x0a, 0x02, 0x04,
    0x06, 0x12, 0x04, 0x55, 0x00, 0x58, 0x01, 0x1a, 0xd8, 0x01, 0x2a, 0x0a, 0x20, 0x47, 0x65, 0x74,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72,
    0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x77, 0x68, 0x65, 0x72, 0x65, 0x20, 0x62, 0x6c, 0x6f,
    0x63, 0x6b, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x69, 0x74, 0x73, 0x20, 0x6d, 0x65, 0x74, 0x61, 0x64,
    0x61, 0x74, 0x61, 0x20, 0x69, 0x73, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x0a, 0x20, 0x62,
    0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x2d, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x70, 0x61, 0x74, 0x68, 0x20, 0x69, 0x6e, 0x66, 0x6f,
    0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x62, 0x65, 0x69, 0x6e, 0x67,
    0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x65, 0x64, 0x0a, 0x20, 0x74, 0x6f, 0x6b, 0x65,
    0x6e, 0x20, 0x2d, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x0a,
    0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x69,
    0x73, 0x20, 0x64, 0x65, 0x70, 0x72, 0x65, 0x63, 0x61, 0x74, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20,
    0x66, 0x61, 0x76, 0x6f, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x64, 0x65,
    0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x20, 0x70, 0x61, 0x73, 0x73, 0x69, 0x6e, 0x67,
    0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x55, 0x08, 0x29, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x56, 0x02, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x56, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x56, 0x0b, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x56, 0x1e, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x56, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x57, 0x02, 0x2e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03, 0x57, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x06, 0x12, 0x03, 0x57, 0x0b, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x57, 0x24, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x57, 0x2c, 0x2d, 0x0a, 0x87, 0x02, 0x0a, 0x02, 0x04, 0x07, 0x12,
    0x04, 0x61, 0x00, 0x65, 0x01, 0x1a, 0xfa, 0x01, 0x2a, 0x0a, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b,
    0x20, 0x2d, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x77, 0x68, 0x69,
    0x63, 0x68, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x70, 0x61, 0x74, 0x68, 0x20, 0x69, 0x6e, 0x66,
    0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x62, 0x65, 0x69, 0x6e,
    0x67, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x65, 0x64, 0x0a, 0x20, 0x6c, 0x6f, 0x63, 0x61,
    0x6c, 0x50, 0x61, 0x74, 0x68, 0x20, 0x2d, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x70, 0x61, 0x74,
    0x68, 0x20, 0x77, 0x68, 0x65, 0x72, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x6c, 0x6f, 0x63,
    0x6b, 0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x69, 0x73, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64,
    0x0a, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x4d, 0x65, 0x74, 0x61, 0x50, 0x61, 0x74, 0x68, 0x20,
    0x2d, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x70, 0x61, 0x74, 0x68, 0x20, 0x77, 0x68, 0x65, 0x72,
    0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x6d, 0x65, 0x74, 0x61,
    0x20, 0x64, 0x61, 0x74, 0x61, 0x20, 0x69, 0x73, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x0a,
    0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x69,
    0x73, 0x20, 0x64, 0x65, 0x70, 0x72, 0x65, 0x63, 0x61, 0x74, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20,
    0x66, 0x61, 0x76, 0x6f, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x64, 0x65,
    0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x20, 0x70, 0x61, 0x73, 0x73, 0x69, 0x6e, 0x67,
    0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x61, 0x08, 0x2a, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x62, 0x02, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x62, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x62, 0x0b, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x62, 0x1e, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x62, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x63, 0x02, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x03, 0x63, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x03, 0x63, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x63, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x63, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x02,
    0x12, 0x03, 0x64, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x64, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x05, 0x12, 0x03, 0x64, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x03, 0x64, 0x12, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x03, 0x64, 0x22, 0x23, 0x0a, 0xda, 0x01,
    0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x6d, 0x00, 0x74, 0x01, 0x1a, 0xcd, 0x01, 0x2a, 0x0a, 0x20,
    0x51, 0x75, 0x65, 0x72, 0x79, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x69,
    0x73, 0x6b, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x6f, 0x66, 0x20,
    0x61, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x62, 0x6c, 0x6f, 0x63,
    0x6b, 0x73, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x44, 0x4e, 0x2e, 0x0a, 0x20,
    0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x50, 0x6f, 0x6f, 0x6c, 0x49, 0x64, 0x20, 0x2d, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x70, 0x6f, 0x6f, 0x6c, 0x20, 0x74, 0x6f, 0x20, 0x71, 0x75, 0x65, 0x72, 0x79, 0x0a,
    0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x49, 0x64, 0x73, 0x20, 0x2d, 0x20, 0x6c, 0x69, 0x73, 0x74,
    0x20, 0x6f, 0x66, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x49, 0x44, 0x73, 0x20, 0x74, 0x6f,
    0x20, 0x71, 0x75, 0x65, 0x72, 0x79, 0x0a, 0x20, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x73, 0x20, 0x2d,
    0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x63, 0x63, 0x65, 0x73, 0x73, 0x20,
    0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x73, 0x20, 0x63, 0x6f, 0x72, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x64, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x6f, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f, 0x66, 0x20,
    0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x49, 0x44, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08,
    0x01, 0x12, 0x03, 0x6d, 0x08, 0x29, 0x0a, 0x4a, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03,
    0x70, 0x02, 0x2f, 0x1a, 0x3d, 0x20, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x64, 0x3a, 0x20, 0x48,
    0x44, 0x46, 0x53, 0x2d, 0x33, 0x39, 0x36, 0x39, 0x0a, 0x20, 0x72, 0x65, 0x70, 0x65, 0x61, 0x74,
    0x65, 0x64, 0x20, 0x45, 0x78, 0x74, 0x65, 0x6e, 0x64, 0x65, 0x64, 0x42, 0x6c, 0x6f, 0x63, 0x6b,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x73, 0x20, 0x3d, 0x20, 0x31,
    0x3b, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x70, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12, 0x03, 0x70, 0x0b, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x70, 0x24, 0x2a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x70, 0x2d, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08,
    0x02, 0x01, 0x12, 0x03, 0x72, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x72, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x72, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x72, 0x12,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x72, 0x20, 0x21, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x03, 0x73, 0x02, 0x33, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x02, 0x04, 0x12, 0x03, 0x73, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x73, 0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x73, 0x14, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x73, 0x1f, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x08, 0x12, 0x03, 0x73,
    0x21, 0x32, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x08, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03,
    0x73, 0x23, 0x30, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x08, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x12, 0x03, 0x73, 0x23, 0x29, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x08, 0x02, 0x02, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x73, 0x23, 0x29, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x08, 0x02, 0x02,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x73, 0x23, 0x29, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x08, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x73, 0x2c, 0x30, 0x0a, 0x8f,
    0x02, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x7c, 0x00, 0x7f, 0x01, 0x1a, 0x82, 0x02, 0x2a, 0x0a,
    0x20, 0x76, 0x6f, 0x6c, 0x75, 0x6d, 0x65, 0x49, 0x64, 0x73, 0x20, 0x2d, 0x20, 0x69, 0x64, 0x20,
    0x6f, 0x66, 0x20, 0x65, 0x61, 0x63, 0x68, 0x20, 0x76, 0x6f, 0x6c, 0x75, 0x6d, 0x65, 0x2c, 0x20,
    0x70, 0x6f, 0x74, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x6d, 0x75, 0x6c, 0x74,
    0x69, 0x70, 0x6c, 0x65, 0x20, 0x62, 0x79, 0x74, 0x65, 0x73, 0x0a, 0x20, 0x76, 0x6f, 0x6c, 0x75,
    0x6d, 0x65, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x65, 0x73, 0x20, 0x2d, 0x20, 0x66, 0x6f, 0x72, 0x20,
    0x65, 0x61, 0x63, 0x68, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x2c, 0x20, 0x61, 0x6e, 0x20, 0x69,
    0x6e, 0x64, 0x65, 0x78, 0x20, 0x69, 0x6e, 0x74, 0x6f, 0x20, 0x76, 0x6f, 0x6c, 0x75, 0x6d, 0x65,
    0x49, 0x64, 0x73, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x79, 0x69, 0x6e, 0x67, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x76, 0x6f, 0x6c, 0x75, 0x6d, 0x65, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x6f, 0x6e, 0x20, 0x77, 0x68, 0x69, 0x63,
    0x68, 0x20, 0x69, 0x74, 0x20, 0x69, 0x73, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x74, 0x65, 0x64, 0x2e,
    0x20, 0x49, 0x66, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74,
    0x20, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x6f, 0x6e, 0x20, 0x61, 0x6e, 0x79, 0x20,
    0x76, 0x6f, 0x6c, 0x75, 0x6d, 0x65, 0x2c, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x20, 0x69, 0x73, 0x20,
    0x73, 0x65, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x4d, 0x41, 0x58, 0x5f, 0x49, 0x4e, 0x54, 0x2e, 0x0a,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x7c, 0x08, 0x2a, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x7d, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x7d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x7d, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x7d, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7d, 0x1d,
    0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x03, 0x7e, 0x02, 0x36, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x04, 0x12, 0x03, 0x7e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x01, 0x05, 0x12, 0x03, 0x7e, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x7e, 0x12, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x7e, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x08, 0x12,
    0x03, 0x7e, 0x24, 0x35, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x09, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00,
    0x12, 0x03, 0x7e, 0x26, 0x33, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x09, 0x02, 0x01, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x12, 0x03, 0x7e, 0x26, 0x2c, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x09, 0x02, 0x01, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x7e, 0x26, 0x2c, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x09,
    0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7e, 0x26, 0x2c, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x09, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x7e, 0x2f, 0x33,
    0x0a, 0xc6, 0x01, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x06, 0x86, 0x01, 0x00, 0x88, 0x01, 0x01, 0x1a,
    0xb7, 0x01, 0x2a, 0x0a, 0x20, 0x66, 0x6f, 0x72, 0x55, 0x70, 0x67, 0x72, 0x61, 0x64, 0x65, 0x20,
    0x2d, 0x20, 0x69, 0x66, 0x20, 0x74, 0x72, 0x75, 0x65, 0x2c, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x61, 0x64, 0x76, 0x69, 0x73, 0x65, 0x64, 0x20, 0x74,
    0x6f, 0x20, 0x77, 0x61, 0x69, 0x74, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x72, 0x65, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x71, 0x75, 0x69, 0x63, 0x6b, 0x0a, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x75, 0x70, 0x67, 0x72, 0x61,
    0x64, 0x65, 0x20, 0x72, 0x65, 0x73, 0x74, 0x61, 0x72, 0x74, 0x20, 0x69, 0x73, 0x20, 0x69, 0x6e,
    0x73, 0x74, 0x72, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x65, 0x64, 0x2e, 0x20, 0x4f, 0x74, 0x68, 0x65,
    0x72, 0x77, 0x69, 0x73, 0x65, 0x2c, 0x20, 0x64, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x20,
    0x64, 0x6f, 0x65, 0x73, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x20, 0x20, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x67, 0x75, 0x6c, 0x61, 0x72, 0x20, 0x73,
    0x68, 0x75, 0x74, 0x64, 0x6f, 0x77, 0x6e, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0a, 0x01,
    0x12, 0x04, 0x86, 0x01, 0x08, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x04,
    0x87, 0x01, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x04, 0x87,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x04, 0x87, 0x01,
    0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x04, 0x87, 0x01, 0x10,
    0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x04, 0x87, 0x01, 0x1d, 0x1e,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x06, 0x8a, 0x01, 0x00, 0x8b, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x04, 0x8a, 0x01, 0x08, 0x25, 0x0a, 0x3b, 0x0a, 0x02, 0x04,
    0x0c, 0x12, 0x06, 0x90, 0x01, 0x00, 0x91, 0x01, 0x01, 0x1a, 0x2d, 0x2a, 0x0a, 0x20, 0x50, 0x69,
    0x6e, 0x67, 0x20, 0x64, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20,
    0x6c, 0x69, 0x76, 0x65, 0x6e, 0x65, 0x73, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x71, 0x75, 0x69,
    0x63, 0x6b, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12,
    0x04, 0x90, 0x01, 0x08, 0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0d, 0x12, 0x06, 0x93, 0x01, 0x00,
    0x95, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12, 0x04, 0x93, 0x01, 0x08, 0x24,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x04, 0x94, 0x01, 0x02, 0x30, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x04, 0x94, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x00, 0x06, 0x12, 0x04, 0x94, 0x01, 0x0b, 0x21, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x04, 0x94, 0x01, 0x22, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x00, 0x03, 0x12, 0x04, 0x94, 0x01, 0x2e, 0x2f, 0x0a, 0x3c, 0x0a, 0x02, 0x04, 0x0e,
    0x12, 0x06, 0x98, 0x01, 0x00, 0x99, 0x01, 0x01, 0x1a, 0x2e, 0x2a, 0x20, 0x41, 0x73, 0x6b, 0x73,
    0x20, 0x44, 0x61, 0x74, 0x61, 0x4e, 0x6f, 0x64, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x6c,
    0x6f, 0x61, 0x64, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x2e, 0x20, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12,
    0x04, 0x98, 0x01, 0x08, 0x28, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x06, 0x9b, 0x01, 0x00,
    0x9c, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x04, 0x9b, 0x01, 0x08, 0x29,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x06, 0x9e, 0x01, 0x00, 0xa0, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x04, 0x9e, 0x01, 0x08, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x10, 0x02, 0x00, 0x12, 0x04, 0x9f, 0x01, 0x02, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x9f, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00,
    0x05, 0x12, 0x04, 0x9f, 0x01, 0x0b, 0x0f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x9f, 0x01, 0x10, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x9f, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x11, 0x12, 0x06, 0xa2, 0x01, 0x00,
    0xa3, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x11, 0x01, 0x12, 0x04, 0xa2, 0x01, 0x08, 0x27,
    0x0a, 0x44, 0x0a, 0x02, 0x04, 0x12, 0x12, 0x06, 0xa6, 0x01, 0x00, 0xa7, 0x01, 0x01, 0x1a, 0x36,
    0x2a, 0x20, 0x51, 0x75, 0x65, 0x72, 0x79, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x75, 0x6e, 0x6e,
    0x69, 0x6e, 0x67, 0x20, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x20, 0x6f, 0x66, 0x20, 0x72, 0x65,
    0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x70, 0x72,
    0x6f, 0x63, 0x65, 0x73, 0x73, 0x20, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x04, 0xa6,
    0x01, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x13, 0x12, 0x06, 0xa9, 0x01, 0x00, 0xae, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x04, 0xa9, 0x01, 0x08, 0x31, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x04, 0xaa, 0x01, 0x02, 0x1b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x04, 0xaa, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x00, 0x05, 0x12, 0x04, 0xaa, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xaa, 0x01, 0x12, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xaa, 0x01, 0x19, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x01,
    0x12, 0x04, 0xab, 0x01, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x04, 0x12,
    0x04, 0xab, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x05, 0x12, 0x04,
    0xab, 0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x01, 0x12, 0x04, 0xab,
    0x01, 0x12, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x03, 0x12, 0x04, 0xab, 0x01,
    0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x02, 0x12, 0x04, 0xac, 0x01, 0x02, 0x1f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x04, 0x12, 0x04, 0xac, 0x01, 0x02, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x05, 0x12, 0x04, 0xac, 0x01, 0x0b, 0x11, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x01, 0x12, 0x04, 0xac, 0x01, 0x12, 0x1a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x02, 0x03, 0x12, 0x04, 0xac, 0x01, 0x1d, 0x1e, 0x0a, 0x27, 0x0a, 0x04,
    0x04, 0x13, 0x02, 0x03, 0x12, 0x04, 0xad, 0x01, 0x02, 0x23, 0x22, 0x19, 0x20, 0x49, 0x74, 0x20,
    0x69, 0x73, 0x20, 0x65, 0x6d, 0x70, 0x74, 0x79, 0x20, 0x69, 0x66, 0x20, 0x73, 0x75, 0x63, 0x63,
    0x65, 0x73, 0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x04, 0x12, 0x04,
    0xad, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x05, 0x12, 0x04, 0xad,
    0x01, 0x0b, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x01, 0x12, 0x04, 0xad, 0x01,
    0x12, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x03, 0x03, 0x12, 0x04, 0xad, 0x01, 0x21,
    0x22, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x06, 0xb0, 0x01, 0x00, 0xb4, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x04, 0xb0, 0x01, 0x08, 0x2d, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x14, 0x02, 0x00, 0x12, 0x04, 0xb1, 0x01, 0x02, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xb1, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xb1, 0x01, 0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xb1, 0x01, 0x11, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xb1, 0x01, 0x1d, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x01, 0x12, 0x04,
    0xb2, 0x01, 0x02, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x04, 0x12, 0x04, 0xb2,
    0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x05, 0x12, 0x04, 0xb2, 0x01,
    0x0b, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x01, 0x12, 0x04, 0xb2, 0x01, 0x11,
    0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x03, 0x12, 0x04, 0xb2, 0x01, 0x1b, 0x1c,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x02, 0x12, 0x04, 0xb3, 0x01, 0x02, 0x41, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x04, 0x12, 0x04, 0xb3, 0x01, 0x02, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x02, 0x06, 0x12, 0x04, 0xb3, 0x01, 0x0b, 0x34, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x02, 0x01, 0x12, 0x04, 0xb3, 0x01, 0x35, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x02, 0x03, 0x12, 0x04, 0xb3, 0x01, 0x3f, 0x40, 0x0a, 0x73, 0x0a, 0x02, 0x06, 0x00,
    0x12, 0x06, 0xba, 0x01, 0x00, 0xea, 0x01, 0x01, 0x1a, 0x65, 0x2a, 0x0a, 0x20, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20,
    0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x44, 0x61,
    0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65, 0x2e, 0x0a, 0x20, 0x53, 0x65, 0x65, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x72, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x64, 0x65, 0x74, 0x61, 0x69, 0x6c,
    0x73, 0x20, 0x6f, 0x66, 0x20, 0x72, 0x70, 0x63, 0x20, 0x63, 0x61, 0x6c, 0x6c, 0x2e, 0x0a, 0x0a,
    0x0b, 0x0a, 0x03, 0x06, 0x00, 0x01, 0x12, 0x04, 0xba, 0x01, 0x08, 0x25, 0x0a, 0x3d, 0x0a, 0x04,
    0x06, 0x00, 0x02, 0x00, 0x12, 0x06, 0xbe, 0x01, 0x02, 0xbf, 0x01, 0x34, 0x1a, 0x2d, 0x2a, 0x0a,
    0x20, 0x52, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x76, 0x69, 0x73,
    0x69, 0x62, 0x6c, 0x65, 0x20, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x20, 0x6f, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbe, 0x01, 0x06, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x00, 0x02, 0x12, 0x04, 0xbe, 0x01, 0x1e, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xbf, 0x01, 0x0e, 0x32, 0x0a, 0x8c, 0x01, 0x0a, 0x04, 0x06, 0x00, 0x02,
    0x01, 0x12, 0x06, 0xc5, 0x01, 0x02, 0xc6, 0x01, 0x2d, 0x1a, 0x7c, 0x2a, 0x0a, 0x20, 0x52, 0x65,
    0x66, 0x72, 0x65, 0x73, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x6f,
    0x66, 0x20, 0x66, 0x65, 0x64, 0x65, 0x72, 0x61, 0x74, 0x65, 0x64, 0x20, 0x6e, 0x61, 0x6d, 0x65,
    0x6e, 0x6f, 0x64, 0x65, 0x73, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74,
    0x65, 0x64, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x2e, 0x0a, 0x20, 0x41, 0x64, 0x64, 0x73, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x6e, 0x61, 0x6d, 0x65,
    0x6e, 0x6f, 0x64, 0x65, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x73, 0x74, 0x6f, 0x70, 0x73, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x64, 0x20, 0x6e, 0x61, 0x6d, 0x65,
    0x6e, 0x6f, 0x64, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xc5, 0x01, 0x06, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x02, 0x12,
    0x04, 0xc5, 0x01, 0x17, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xc6, 0x01, 0x0e, 0x2b, 0x0a, 0x3c, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x02, 0x12, 0x06, 0xcb, 0x01,
    0x02, 0xcc, 0x01, 0x2c, 0x1a, 0x2c, 0x2a, 0x0a, 0x20, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x70, 0x6f, 0x6f, 0x6c, 0x20, 0x66,
    0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x61, 0x74, 0x61, 0x6e, 0x6f, 0x64, 0x65,
    0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x01, 0x12, 0x04, 0xcb, 0x01, 0x06,
    0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x02, 0x12, 0x04, 0xcb, 0x01, 0x16, 0x31,
    0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x02, 0x03, 0x12, 0x04, 0xcc, 0x01, 0x0e, 0x2a, 0x0a,
    0x72, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x03, 0x12, 0x06, 0xd2, 0x01, 0x02, 0xd3, 0x01, 0x32, 0x1a,
    0x62, 0x2a, 0x0a, 0x20, 0x52, 0x65, 0x74, 0x72, 0x69, 0x65, 0x76, 0x65, 0x73, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x70, 0x61, 0x74, 0x68, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x73, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x61,
    0x6e, 0x64, 0x20, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x20, 0x66, 0x69, 0x6c, 0x65,
    0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20,
    0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x73, 0x79, 0x73, 0x74, 0x65,
    0x6d, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x01, 0x12, 0x04, 0xd2, 0x01,
    0x06, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x02, 0x12, 0x04, 0xd2, 0x01, 0x1c,
    0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x03, 0x03, 0x12, 0x04, 0xd3, 0x01, 0x0e, 0x30,
    0x0a, 0x76, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x04, 0x12, 0x06, 0xd9, 0x01, 0x02, 0xda, 0x01, 0x32,
    0x1a, 0x66, 0x2a, 0x0a, 0x20, 0x52, 0x65, 0x74, 0x72, 0x69, 0x65, 0x76, 0x65, 0x20, 0x61, 0x64,
    0x64, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x48, 0x44, 0x46, 0x53, 0x2d, 0x73, 0x70,
    0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x20, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x20,
    0x61, 0x62, 0x6f, 0x75, 0x74, 0x20, 0x61, 0x20, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x62,
    0x6c, 0x6f, 0x63, 0x6b, 0x73, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64, 0x0a, 0x20, 0x6f, 0x6e,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20,
    0x73, 0x79, 0x73, 0x74, 0x65, 0x6d, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x04,
    0x01, 0x12, 0x04, 0xd9, 0x01, 0x06, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x04, 0x02,
    0x12, 0x04, 0xd9, 0x01, 0x1c, 0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x04, 0x03, 0x12,
    0x04, 0xda, 0x01, 0x0e, 0x30, 0x0a, 0x0e, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x05, 0x12, 0x06, 0xdc,
    0x01, 0x02, 0xdd, 0x01, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x05, 0x01, 0x12, 0x04,
    0xdc, 0x01, 0x06, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x05, 0x02, 0x12, 0x04, 0xdc,
    0x01, 0x17, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x05, 0x03, 0x12, 0x04, 0xdd, 0x01,
    0x0e, 0x2b, 0x0a, 0x0e, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x06, 0x12, 0x06, 0xdf, 0x01, 0x02, 0xe0,
    0x01, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x06, 0x01, 0x12, 0x04, 0xdf, 0x01, 0x06,
    0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x06, 0x02, 0x12, 0x04, 0xdf, 0x01, 0x16, 0x31,
    0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x06, 0x03, 0x12, 0x04, 0xe0, 0x01, 0x0e, 0x2a, 0x0a,
    0x0e, 0x0a, 0x04, 0x06, 0x00, 0x02, 0x07, 0x12, 0x06, 0xe2, 0x01, 0x02, 0xe3, 0x01, 0x35, 0x0a,
    0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x07, 0x01, 0x12, 0x04, 0xe2, 0x01, 0x06, 0x1e, 0x0a, 0x0d,
    0x0a, 0x05, 0x06, 0x00, 0x02, 0x07, 0x02, 0x12, 0x04, 0xe2, 0x01, 0x1f, 0x43, 0x0a, 0x0d, 0x0a,
    0x05, 0x06, 0x00, 0x02, 0x07, 0x03, 0x12, 0x04, 0xe3, 0x01, 0x0e, 0x33, 0x0a, 0x0e, 0x0a, 0x04,
    0x06, 0x00, 0x02, 0x08, 0x12, 0x06, 0xe5, 0x01, 0x02, 0xe6, 0x01, 0x31, 0x0a, 0x0d, 0x0a, 0x05,
    0x06, 0x00, 0x02, 0x08, 0x01, 0x12, 0x04, 0xe5, 0x01, 0x06, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x06,
    0x00, 0x02, 0x08, 0x02, 0x12, 0x04, 0xe5, 0x01, 0x1b, 0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00,
    0x02, 0x08, 0x03, 0x12, 0x04, 0xe6, 0x01, 0x0e, 0x2f, 0x0a, 0x0e, 0x0a, 0x04, 0x06, 0x00, 0x02,
    0x09, 0x12, 0x06, 0xe8, 0x01, 0x02, 0xe9, 0x01, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02,
    0x09, 0x01, 0x12, 0x04, 0xe8, 0x01, 0x06, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x09,
    0x02, 0x12, 0x04, 0xe8, 0x01, 0x19, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x06, 0x00, 0x02, 0x09, 0x03,
    0x12, 0x04, 0xe9, 0x01, 0x0e, 0x2d,
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
