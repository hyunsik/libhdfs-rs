// This file is generated. Do not edit
// @generated

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;
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
pub struct CreateEncryptionZoneRequestProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    keyName: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CreateEncryptionZoneRequestProto {
    pub fn new() -> CreateEncryptionZoneRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreateEncryptionZoneRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<CreateEncryptionZoneRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreateEncryptionZoneRequestProto,
        };
        unsafe {
            instance.get(|| {
                CreateEncryptionZoneRequestProto {
                    src: ::protobuf::SingularField::none(),
                    keyName: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string src = 1;

    pub fn clear_src(&mut self) {
        self.src.clear();
    }

    pub fn has_src(&self) -> bool {
        self.src.is_some()
    }

    // Param is passed by value, moved
    pub fn set_src(&mut self, v: ::std::string::String) {
        self.src = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_src<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.src.is_none() {
            self.src.set_default();
        };
        self.src.as_mut().unwrap()
    }

    // Take field
    pub fn take_src(&mut self) -> ::std::string::String {
        self.src.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_src<'a>(&'a self) -> &'a str {
        match self.src.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string keyName = 2;

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

impl ::protobuf::Message for CreateEncryptionZoneRequestProto {
    fn is_initialized(&self) -> bool {
        if self.src.is_none() {
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
                    let tmp = self.src.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
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
        for value in self.src.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.keyName.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.src.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.keyName.as_ref() {
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
        ::std::any::TypeId::of::<CreateEncryptionZoneRequestProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CreateEncryptionZoneRequestProto {
    fn new() -> CreateEncryptionZoneRequestProto {
        CreateEncryptionZoneRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreateEncryptionZoneRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "src",
                    CreateEncryptionZoneRequestProto::has_src,
                    CreateEncryptionZoneRequestProto::get_src,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "keyName",
                    CreateEncryptionZoneRequestProto::has_keyName,
                    CreateEncryptionZoneRequestProto::get_keyName,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CreateEncryptionZoneRequestProto>(
                    "CreateEncryptionZoneRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreateEncryptionZoneRequestProto {
    fn clear(&mut self) {
        self.clear_src();
        self.clear_keyName();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CreateEncryptionZoneRequestProto {
    fn eq(&self, other: &CreateEncryptionZoneRequestProto) -> bool {
        self.src == other.src &&
        self.keyName == other.keyName &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CreateEncryptionZoneRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CreateEncryptionZoneResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CreateEncryptionZoneResponseProto {
    pub fn new() -> CreateEncryptionZoneResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreateEncryptionZoneResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<CreateEncryptionZoneResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreateEncryptionZoneResponseProto,
        };
        unsafe {
            instance.get(|| {
                CreateEncryptionZoneResponseProto {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for CreateEncryptionZoneResponseProto {
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
        ::std::any::TypeId::of::<CreateEncryptionZoneResponseProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CreateEncryptionZoneResponseProto {
    fn new() -> CreateEncryptionZoneResponseProto {
        CreateEncryptionZoneResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreateEncryptionZoneResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CreateEncryptionZoneResponseProto>(
                    "CreateEncryptionZoneResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreateEncryptionZoneResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CreateEncryptionZoneResponseProto {
    fn eq(&self, other: &CreateEncryptionZoneResponseProto) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CreateEncryptionZoneResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ListEncryptionZonesRequestProto {
    // message fields
    id: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ListEncryptionZonesRequestProto {
    pub fn new() -> ListEncryptionZonesRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListEncryptionZonesRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<ListEncryptionZonesRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListEncryptionZonesRequestProto,
        };
        unsafe {
            instance.get(|| {
                ListEncryptionZonesRequestProto {
                    id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id<'a>(&self) -> i64 {
        self.id.unwrap_or(0)
    }
}

impl ::protobuf::Message for ListEncryptionZonesRequestProto {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.id = ::std::option::Option::Some(tmp);
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
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_int64(1, v));
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
        ::std::any::TypeId::of::<ListEncryptionZonesRequestProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ListEncryptionZonesRequestProto {
    fn new() -> ListEncryptionZonesRequestProto {
        ListEncryptionZonesRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListEncryptionZonesRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "id",
                    ListEncryptionZonesRequestProto::has_id,
                    ListEncryptionZonesRequestProto::get_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListEncryptionZonesRequestProto>(
                    "ListEncryptionZonesRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListEncryptionZonesRequestProto {
    fn clear(&mut self) {
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ListEncryptionZonesRequestProto {
    fn eq(&self, other: &ListEncryptionZonesRequestProto) -> bool {
        self.id == other.id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ListEncryptionZonesRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct EncryptionZoneProto {
    // message fields
    id: ::std::option::Option<i64>,
    path: ::protobuf::SingularField<::std::string::String>,
    suite: ::std::option::Option<CipherSuiteProto>,
    cryptoProtocolVersion: ::std::option::Option<CryptoProtocolVersionProto>,
    keyName: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl EncryptionZoneProto {
    pub fn new() -> EncryptionZoneProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EncryptionZoneProto {
        static mut instance: ::protobuf::lazy::Lazy<EncryptionZoneProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EncryptionZoneProto,
        };
        unsafe {
            instance.get(|| {
                EncryptionZoneProto {
                    id: ::std::option::Option::None,
                    path: ::protobuf::SingularField::none(),
                    suite: ::std::option::Option::None,
                    cryptoProtocolVersion: ::std::option::Option::None,
                    keyName: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: i64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id<'a>(&self) -> i64 {
        self.id.unwrap_or(0)
    }

    // required string path = 2;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.path.is_none() {
            self.path.set_default();
        };
        self.path.as_mut().unwrap()
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        self.path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path<'a>(&'a self) -> &'a str {
        match self.path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required .hadoop.hdfs.CipherSuiteProto suite = 3;

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

    // required .hadoop.hdfs.CryptoProtocolVersionProto cryptoProtocolVersion = 4;

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
}

impl ::protobuf::Message for EncryptionZoneProto {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        };
        if self.path.is_none() {
            return false;
        };
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
                    let tmp = try!(is.read_int64());
                    self.id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.path.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.suite = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.cryptoProtocolVersion = ::std::option::Option::Some(tmp);
                },
                5 => {
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
        for value in self.id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.path.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.suite.iter() {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        for value in self.cryptoProtocolVersion.iter() {
            my_size += ::protobuf::rt::enum_size(4, *value);
        };
        for value in self.keyName.iter() {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.path.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.suite {
            try!(os.write_enum(3, v as i32));
        };
        if let Some(v) = self.cryptoProtocolVersion {
            try!(os.write_enum(4, v as i32));
        };
        if let Some(v) = self.keyName.as_ref() {
            try!(os.write_string(5, &v));
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
        ::std::any::TypeId::of::<EncryptionZoneProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EncryptionZoneProto {
    fn new() -> EncryptionZoneProto {
        EncryptionZoneProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<EncryptionZoneProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "id",
                    EncryptionZoneProto::has_id,
                    EncryptionZoneProto::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path",
                    EncryptionZoneProto::has_path,
                    EncryptionZoneProto::get_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "suite",
                    EncryptionZoneProto::has_suite,
                    EncryptionZoneProto::get_suite,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "cryptoProtocolVersion",
                    EncryptionZoneProto::has_cryptoProtocolVersion,
                    EncryptionZoneProto::get_cryptoProtocolVersion,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "keyName",
                    EncryptionZoneProto::has_keyName,
                    EncryptionZoneProto::get_keyName,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EncryptionZoneProto>(
                    "EncryptionZoneProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EncryptionZoneProto {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_path();
        self.clear_suite();
        self.clear_cryptoProtocolVersion();
        self.clear_keyName();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EncryptionZoneProto {
    fn eq(&self, other: &EncryptionZoneProto) -> bool {
        self.id == other.id &&
        self.path == other.path &&
        self.suite == other.suite &&
        self.cryptoProtocolVersion == other.cryptoProtocolVersion &&
        self.keyName == other.keyName &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EncryptionZoneProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ListEncryptionZonesResponseProto {
    // message fields
    zones: ::protobuf::RepeatedField<EncryptionZoneProto>,
    hasMore: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ListEncryptionZonesResponseProto {
    pub fn new() -> ListEncryptionZonesResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ListEncryptionZonesResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<ListEncryptionZonesResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ListEncryptionZonesResponseProto,
        };
        unsafe {
            instance.get(|| {
                ListEncryptionZonesResponseProto {
                    zones: ::protobuf::RepeatedField::new(),
                    hasMore: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .hadoop.hdfs.EncryptionZoneProto zones = 1;

    pub fn clear_zones(&mut self) {
        self.zones.clear();
    }

    // Param is passed by value, moved
    pub fn set_zones(&mut self, v: ::protobuf::RepeatedField<EncryptionZoneProto>) {
        self.zones = v;
    }

    // Mutable pointer to the field.
    pub fn mut_zones<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<EncryptionZoneProto> {
        &mut self.zones
    }

    // Take field
    pub fn take_zones(&mut self) -> ::protobuf::RepeatedField<EncryptionZoneProto> {
        ::std::mem::replace(&mut self.zones, ::protobuf::RepeatedField::new())
    }

    pub fn get_zones<'a>(&'a self) -> &'a [EncryptionZoneProto] {
        &self.zones
    }

    // required bool hasMore = 2;

    pub fn clear_hasMore(&mut self) {
        self.hasMore = ::std::option::Option::None;
    }

    pub fn has_hasMore(&self) -> bool {
        self.hasMore.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hasMore(&mut self, v: bool) {
        self.hasMore = ::std::option::Option::Some(v);
    }

    pub fn get_hasMore<'a>(&self) -> bool {
        self.hasMore.unwrap_or(false)
    }
}

impl ::protobuf::Message for ListEncryptionZonesResponseProto {
    fn is_initialized(&self) -> bool {
        if self.hasMore.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.zones));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.hasMore = ::std::option::Option::Some(tmp);
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
        for value in self.zones.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.hasMore.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.zones.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.hasMore {
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
        ::std::any::TypeId::of::<ListEncryptionZonesResponseProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ListEncryptionZonesResponseProto {
    fn new() -> ListEncryptionZonesResponseProto {
        ListEncryptionZonesResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ListEncryptionZonesResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "zones",
                    ListEncryptionZonesResponseProto::get_zones,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "hasMore",
                    ListEncryptionZonesResponseProto::has_hasMore,
                    ListEncryptionZonesResponseProto::get_hasMore,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ListEncryptionZonesResponseProto>(
                    "ListEncryptionZonesResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ListEncryptionZonesResponseProto {
    fn clear(&mut self) {
        self.clear_zones();
        self.clear_hasMore();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ListEncryptionZonesResponseProto {
    fn eq(&self, other: &ListEncryptionZonesResponseProto) -> bool {
        self.zones == other.zones &&
        self.hasMore == other.hasMore &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ListEncryptionZonesResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetEZForPathRequestProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl GetEZForPathRequestProto {
    pub fn new() -> GetEZForPathRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetEZForPathRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetEZForPathRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetEZForPathRequestProto,
        };
        unsafe {
            instance.get(|| {
                GetEZForPathRequestProto {
                    src: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string src = 1;

    pub fn clear_src(&mut self) {
        self.src.clear();
    }

    pub fn has_src(&self) -> bool {
        self.src.is_some()
    }

    // Param is passed by value, moved
    pub fn set_src(&mut self, v: ::std::string::String) {
        self.src = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_src<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.src.is_none() {
            self.src.set_default();
        };
        self.src.as_mut().unwrap()
    }

    // Take field
    pub fn take_src(&mut self) -> ::std::string::String {
        self.src.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_src<'a>(&'a self) -> &'a str {
        match self.src.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for GetEZForPathRequestProto {
    fn is_initialized(&self) -> bool {
        if self.src.is_none() {
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
                    let tmp = self.src.set_default();
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
        for value in self.src.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.src.as_ref() {
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
        ::std::any::TypeId::of::<GetEZForPathRequestProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetEZForPathRequestProto {
    fn new() -> GetEZForPathRequestProto {
        GetEZForPathRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetEZForPathRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "src",
                    GetEZForPathRequestProto::has_src,
                    GetEZForPathRequestProto::get_src,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetEZForPathRequestProto>(
                    "GetEZForPathRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetEZForPathRequestProto {
    fn clear(&mut self) {
        self.clear_src();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetEZForPathRequestProto {
    fn eq(&self, other: &GetEZForPathRequestProto) -> bool {
        self.src == other.src &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetEZForPathRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetEZForPathResponseProto {
    // message fields
    zone: ::protobuf::SingularPtrField<EncryptionZoneProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl GetEZForPathResponseProto {
    pub fn new() -> GetEZForPathResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetEZForPathResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetEZForPathResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetEZForPathResponseProto,
        };
        unsafe {
            instance.get(|| {
                GetEZForPathResponseProto {
                    zone: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .hadoop.hdfs.EncryptionZoneProto zone = 1;

    pub fn clear_zone(&mut self) {
        self.zone.clear();
    }

    pub fn has_zone(&self) -> bool {
        self.zone.is_some()
    }

    // Param is passed by value, moved
    pub fn set_zone(&mut self, v: EncryptionZoneProto) {
        self.zone = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_zone<'a>(&'a mut self) -> &'a mut EncryptionZoneProto {
        if self.zone.is_none() {
            self.zone.set_default();
        };
        self.zone.as_mut().unwrap()
    }

    // Take field
    pub fn take_zone(&mut self) -> EncryptionZoneProto {
        self.zone.take().unwrap_or_else(|| EncryptionZoneProto::new())
    }

    pub fn get_zone<'a>(&'a self) -> &'a EncryptionZoneProto {
        self.zone.as_ref().unwrap_or_else(|| EncryptionZoneProto::default_instance())
    }
}

impl ::protobuf::Message for GetEZForPathResponseProto {
    fn is_initialized(&self) -> bool {
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
                    let tmp = self.zone.set_default();
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
        for value in self.zone.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.zone.as_ref() {
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
        ::std::any::TypeId::of::<GetEZForPathResponseProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetEZForPathResponseProto {
    fn new() -> GetEZForPathResponseProto {
        GetEZForPathResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetEZForPathResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "zone",
                    GetEZForPathResponseProto::has_zone,
                    GetEZForPathResponseProto::get_zone,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetEZForPathResponseProto>(
                    "GetEZForPathResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetEZForPathResponseProto {
    fn clear(&mut self) {
        self.clear_zone();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetEZForPathResponseProto {
    fn eq(&self, other: &GetEZForPathResponseProto) -> bool {
        self.zone == other.zone &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetEZForPathResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x10, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x0b, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x1a,
    0x0a, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x40, 0x0a, 0x20, 0x43,
    0x72, 0x65, 0x61, 0x74, 0x65, 0x45, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x5a,
    0x6f, 0x6e, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x0b, 0x0a, 0x03, 0x73, 0x72, 0x63, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0f, 0x0a, 0x07,
    0x6b, 0x65, 0x79, 0x4e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x22, 0x23, 0x0a,
    0x21, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x45, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f,
    0x6e, 0x5a, 0x6f, 0x6e, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x22, 0x2d, 0x0a, 0x1f, 0x4c, 0x69, 0x73, 0x74, 0x45, 0x6e, 0x63, 0x72, 0x79, 0x70,
    0x74, 0x69, 0x6f, 0x6e, 0x5a, 0x6f, 0x6e, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x03, 0x22, 0xb6, 0x01, 0x0a, 0x13, 0x45, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e,
    0x5a, 0x6f, 0x6e, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0a, 0x0a, 0x02, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x02, 0x28, 0x03, 0x12, 0x0c, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x02, 0x20,
    0x02, 0x28, 0x09, 0x12, 0x2c, 0x0a, 0x05, 0x73, 0x75, 0x69, 0x74, 0x65, 0x18, 0x03, 0x20, 0x02,
    0x28, 0x0e, 0x32, 0x1d, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73,
    0x2e, 0x43, 0x69, 0x70, 0x68, 0x65, 0x72, 0x53, 0x75, 0x69, 0x74, 0x65, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x46, 0x0a, 0x15, 0x63, 0x72, 0x79, 0x70, 0x74, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x63, 0x6f, 0x6c, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x04, 0x20, 0x02, 0x28, 0x0e,
    0x32, 0x27, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x43,
    0x72, 0x79, 0x70, 0x74, 0x6f, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x56, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0f, 0x0a, 0x07, 0x6b, 0x65, 0x79,
    0x4e, 0x61, 0x6d, 0x65, 0x18, 0x05, 0x20, 0x02, 0x28, 0x09, 0x22, 0x64, 0x0a, 0x20, 0x4c, 0x69,
    0x73, 0x74, 0x45, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x5a, 0x6f, 0x6e, 0x65,
    0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2f,
    0x0a, 0x05, 0x7a, 0x6f, 0x6e, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x20, 0x2e,
    0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x45, 0x6e, 0x63, 0x72,
    0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x5a, 0x6f, 0x6e, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x0f, 0x0a, 0x07, 0x68, 0x61, 0x73, 0x4d, 0x6f, 0x72, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x08,
    0x22, 0x27, 0x0a, 0x18, 0x47, 0x65, 0x74, 0x45, 0x5a, 0x46, 0x6f, 0x72, 0x50, 0x61, 0x74, 0x68,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x0a, 0x03,
    0x73, 0x72, 0x63, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x22, 0x4b, 0x0a, 0x19, 0x47, 0x65, 0x74,
    0x45, 0x5a, 0x46, 0x6f, 0x72, 0x50, 0x61, 0x74, 0x68, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2e, 0x0a, 0x04, 0x7a, 0x6f, 0x6e, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64,
    0x66, 0x73, 0x2e, 0x45, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x5a, 0x6f, 0x6e,
    0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x42, 0x41, 0x0a, 0x25, 0x6f, 0x72, 0x67, 0x2e, 0x61, 0x70,
    0x61, 0x63, 0x68, 0x65, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x42,
    0x15, 0x45, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x5a, 0x6f, 0x6e, 0x65, 0x73,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0xa0, 0x01, 0x01, 0x4a, 0xf7, 0x09, 0x0a, 0x06, 0x12, 0x04,
    0x1c, 0x00, 0x42, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x1c, 0x00, 0x3e, 0x0a, 0x0b,
    0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x1c, 0x00, 0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x1c, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x1c, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00,
    0x07, 0x12, 0x03, 0x1c, 0x16, 0x3d, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x1d, 0x00, 0x36,
    0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01, 0x12, 0x03, 0x1d, 0x00, 0x36, 0x0a, 0x0c, 0x0a,
    0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x1d, 0x07, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x08,
    0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x07, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7,
    0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x07, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7,
    0x07, 0x01, 0x07, 0x12, 0x03, 0x1d, 0x1e, 0x35, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x1e,
    0x00, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x02, 0x12, 0x03, 0x1e, 0x00, 0x2c, 0x0a,
    0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x02, 0x02, 0x12, 0x03, 0x1e, 0x07, 0x24, 0x0a, 0x0d, 0x0a,
    0x06, 0x08, 0xe7, 0x07, 0x02, 0x02, 0x00, 0x12, 0x03, 0x1e, 0x07, 0x24, 0x0a, 0x0e, 0x0a, 0x07,
    0x08, 0xe7, 0x07, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x07, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x08, 0xe7, 0x07, 0x02, 0x03, 0x12, 0x03, 0x1e, 0x27, 0x2b, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12,
    0x03, 0x1f, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x21, 0x07, 0x13, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x23, 0x00, 0x26, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x23, 0x08, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x24, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x24,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x24, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x24, 0x12, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x24, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x25, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x25, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x25, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x25, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x25, 0x1c,
    0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x28, 0x00, 0x29, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x28, 0x08, 0x29, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12,
    0x04, 0x2b, 0x00, 0x2d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x2b, 0x08,
    0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x2c, 0x02, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2c, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x2c, 0x11, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x2c, 0x16, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x2f, 0x00,
    0x35, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x2f, 0x08, 0x1b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x30, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x30, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x30, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x30, 0x11, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x30, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x31, 0x02, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x31, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x31, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x31, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x31, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02,
    0x12, 0x03, 0x32, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x32, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x06, 0x12, 0x03, 0x32, 0x0b,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x32, 0x1c, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x32, 0x24, 0x25, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x03, 0x12, 0x03, 0x33, 0x02, 0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x33, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03,
    0x06, 0x12, 0x03, 0x33, 0x0b, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x33, 0x26, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x33,
    0x3e, 0x3f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x04, 0x12, 0x03, 0x34, 0x02, 0x1e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x04, 0x12, 0x03, 0x34, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x04, 0x05, 0x12, 0x03, 0x34, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x04, 0x01, 0x12, 0x03, 0x34, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x04, 0x03, 0x12, 0x03, 0x34, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x37,
    0x00, 0x3a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x37, 0x08, 0x28, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x38, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x38, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x38, 0x0b, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x38, 0x1f, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x38, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x39, 0x02,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x39, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x39, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x39, 0x10, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x39, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12,
    0x04, 0x3c, 0x00, 0x3e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x3c, 0x08,
    0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x3d, 0x04, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3d, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x3d, 0x14, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x3d, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x40, 0x00,
    0x42, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x40, 0x08, 0x21, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x41, 0x04, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x41, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x41, 0x0d, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x41, 0x21, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x41, 0x28, 0x29,
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
