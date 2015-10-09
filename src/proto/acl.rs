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
pub struct AclEntryProto {
    // message fields
    field_type: ::std::option::Option<AclEntryProto_AclEntryTypeProto>,
    scope: ::std::option::Option<AclEntryProto_AclEntryScopeProto>,
    permissions: ::std::option::Option<AclEntryProto_FsActionProto>,
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl AclEntryProto {
    pub fn new() -> AclEntryProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AclEntryProto {
        static mut instance: ::protobuf::lazy::Lazy<AclEntryProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AclEntryProto,
        };
        unsafe {
            instance.get(|| {
                AclEntryProto {
                    field_type: ::std::option::Option::None,
                    scope: ::std::option::Option::None,
                    permissions: ::std::option::Option::None,
                    name: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.AclEntryProto.AclEntryTypeProto type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: AclEntryProto_AclEntryTypeProto) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type<'a>(&self) -> AclEntryProto_AclEntryTypeProto {
        self.field_type.unwrap_or(AclEntryProto_AclEntryTypeProto::USER)
    }

    // required .hadoop.hdfs.AclEntryProto.AclEntryScopeProto scope = 2;

    pub fn clear_scope(&mut self) {
        self.scope = ::std::option::Option::None;
    }

    pub fn has_scope(&self) -> bool {
        self.scope.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scope(&mut self, v: AclEntryProto_AclEntryScopeProto) {
        self.scope = ::std::option::Option::Some(v);
    }

    pub fn get_scope<'a>(&self) -> AclEntryProto_AclEntryScopeProto {
        self.scope.unwrap_or(AclEntryProto_AclEntryScopeProto::ACCESS)
    }

    // required .hadoop.hdfs.AclEntryProto.FsActionProto permissions = 3;

    pub fn clear_permissions(&mut self) {
        self.permissions = ::std::option::Option::None;
    }

    pub fn has_permissions(&self) -> bool {
        self.permissions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_permissions(&mut self, v: AclEntryProto_FsActionProto) {
        self.permissions = ::std::option::Option::Some(v);
    }

    pub fn get_permissions<'a>(&self) -> AclEntryProto_FsActionProto {
        self.permissions.unwrap_or(AclEntryProto_FsActionProto::NONE)
    }

    // optional string name = 4;

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
}

impl ::protobuf::Message for AclEntryProto {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        };
        if self.scope.is_none() {
            return false;
        };
        if self.permissions.is_none() {
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
                    let tmp = try!(is.read_enum());
                    self.scope = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.permissions = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.name.set_default();
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
        for value in self.field_type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.scope.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in self.permissions.iter() {
            my_size += ::protobuf::rt::enum_size(3, *value);
        };
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.scope {
            try!(os.write_enum(2, v as i32));
        };
        if let Some(v) = self.permissions {
            try!(os.write_enum(3, v as i32));
        };
        if let Some(v) = self.name.as_ref() {
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
        ::std::any::TypeId::of::<AclEntryProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AclEntryProto {
    fn new() -> AclEntryProto {
        AclEntryProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<AclEntryProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "field_type",
                    AclEntryProto::has_field_type,
                    AclEntryProto::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "scope",
                    AclEntryProto::has_scope,
                    AclEntryProto::get_scope,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "permissions",
                    AclEntryProto::has_permissions,
                    AclEntryProto::get_permissions,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    AclEntryProto::has_name,
                    AclEntryProto::get_name,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AclEntryProto>(
                    "AclEntryProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AclEntryProto {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_scope();
        self.clear_permissions();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AclEntryProto {
    fn eq(&self, other: &AclEntryProto) -> bool {
        self.field_type == other.field_type &&
        self.scope == other.scope &&
        self.permissions == other.permissions &&
        self.name == other.name &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AclEntryProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AclEntryProto_AclEntryScopeProto {
    ACCESS = 0,
    DEFAULT = 1,
}

impl ::protobuf::ProtobufEnum for AclEntryProto_AclEntryScopeProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AclEntryProto_AclEntryScopeProto> {
        match value {
            0 => ::std::option::Option::Some(AclEntryProto_AclEntryScopeProto::ACCESS),
            1 => ::std::option::Option::Some(AclEntryProto_AclEntryScopeProto::DEFAULT),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<AclEntryProto_AclEntryScopeProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("AclEntryProto_AclEntryScopeProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for AclEntryProto_AclEntryScopeProto {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AclEntryProto_AclEntryTypeProto {
    USER = 0,
    GROUP = 1,
    MASK = 2,
    OTHER = 3,
}

impl ::protobuf::ProtobufEnum for AclEntryProto_AclEntryTypeProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AclEntryProto_AclEntryTypeProto> {
        match value {
            0 => ::std::option::Option::Some(AclEntryProto_AclEntryTypeProto::USER),
            1 => ::std::option::Option::Some(AclEntryProto_AclEntryTypeProto::GROUP),
            2 => ::std::option::Option::Some(AclEntryProto_AclEntryTypeProto::MASK),
            3 => ::std::option::Option::Some(AclEntryProto_AclEntryTypeProto::OTHER),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<AclEntryProto_AclEntryTypeProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("AclEntryProto_AclEntryTypeProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for AclEntryProto_AclEntryTypeProto {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AclEntryProto_FsActionProto {
    NONE = 0,
    EXECUTE = 1,
    WRITE = 2,
    WRITE_EXECUTE = 3,
    READ = 4,
    READ_EXECUTE = 5,
    READ_WRITE = 6,
    PERM_ALL = 7,
}

impl ::protobuf::ProtobufEnum for AclEntryProto_FsActionProto {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AclEntryProto_FsActionProto> {
        match value {
            0 => ::std::option::Option::Some(AclEntryProto_FsActionProto::NONE),
            1 => ::std::option::Option::Some(AclEntryProto_FsActionProto::EXECUTE),
            2 => ::std::option::Option::Some(AclEntryProto_FsActionProto::WRITE),
            3 => ::std::option::Option::Some(AclEntryProto_FsActionProto::WRITE_EXECUTE),
            4 => ::std::option::Option::Some(AclEntryProto_FsActionProto::READ),
            5 => ::std::option::Option::Some(AclEntryProto_FsActionProto::READ_EXECUTE),
            6 => ::std::option::Option::Some(AclEntryProto_FsActionProto::READ_WRITE),
            7 => ::std::option::Option::Some(AclEntryProto_FsActionProto::PERM_ALL),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<AclEntryProto_FsActionProto>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("AclEntryProto_FsActionProto", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for AclEntryProto_FsActionProto {
}

#[derive(Clone,Default)]
pub struct AclStatusProto {
    // message fields
    owner: ::protobuf::SingularField<::std::string::String>,
    group: ::protobuf::SingularField<::std::string::String>,
    sticky: ::std::option::Option<bool>,
    entries: ::protobuf::RepeatedField<AclEntryProto>,
    permission: ::protobuf::SingularPtrField<FsPermissionProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl AclStatusProto {
    pub fn new() -> AclStatusProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AclStatusProto {
        static mut instance: ::protobuf::lazy::Lazy<AclStatusProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AclStatusProto,
        };
        unsafe {
            instance.get(|| {
                AclStatusProto {
                    owner: ::protobuf::SingularField::none(),
                    group: ::protobuf::SingularField::none(),
                    sticky: ::std::option::Option::None,
                    entries: ::protobuf::RepeatedField::new(),
                    permission: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string owner = 1;

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

    // required string group = 2;

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

    // required bool sticky = 3;

    pub fn clear_sticky(&mut self) {
        self.sticky = ::std::option::Option::None;
    }

    pub fn has_sticky(&self) -> bool {
        self.sticky.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sticky(&mut self, v: bool) {
        self.sticky = ::std::option::Option::Some(v);
    }

    pub fn get_sticky<'a>(&self) -> bool {
        self.sticky.unwrap_or(false)
    }

    // repeated .hadoop.hdfs.AclEntryProto entries = 4;

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_entries(&mut self, v: ::protobuf::RepeatedField<AclEntryProto>) {
        self.entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entries<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<AclEntryProto> {
        &mut self.entries
    }

    // Take field
    pub fn take_entries(&mut self) -> ::protobuf::RepeatedField<AclEntryProto> {
        ::std::mem::replace(&mut self.entries, ::protobuf::RepeatedField::new())
    }

    pub fn get_entries<'a>(&'a self) -> &'a [AclEntryProto] {
        &self.entries
    }

    // optional .hadoop.hdfs.FsPermissionProto permission = 5;

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
}

impl ::protobuf::Message for AclStatusProto {
    fn is_initialized(&self) -> bool {
        if self.owner.is_none() {
            return false;
        };
        if self.group.is_none() {
            return false;
        };
        if self.sticky.is_none() {
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
                    let tmp = self.owner.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.group.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.sticky = ::std::option::Option::Some(tmp);
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.entries));
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.permission.set_default();
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
        for value in self.owner.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.group.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        if self.sticky.is_some() {
            my_size += 2;
        };
        for value in self.entries.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.permission.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.owner.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.group.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.sticky {
            try!(os.write_bool(3, v));
        };
        for v in self.entries.iter() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.permission.as_ref() {
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
        ::std::any::TypeId::of::<AclStatusProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AclStatusProto {
    fn new() -> AclStatusProto {
        AclStatusProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<AclStatusProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "owner",
                    AclStatusProto::has_owner,
                    AclStatusProto::get_owner,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "group",
                    AclStatusProto::has_group,
                    AclStatusProto::get_group,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "sticky",
                    AclStatusProto::has_sticky,
                    AclStatusProto::get_sticky,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "entries",
                    AclStatusProto::get_entries,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "permission",
                    AclStatusProto::has_permission,
                    AclStatusProto::get_permission,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AclStatusProto>(
                    "AclStatusProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AclStatusProto {
    fn clear(&mut self) {
        self.clear_owner();
        self.clear_group();
        self.clear_sticky();
        self.clear_entries();
        self.clear_permission();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AclStatusProto {
    fn eq(&self, other: &AclStatusProto) -> bool {
        self.owner == other.owner &&
        self.group == other.group &&
        self.sticky == other.sticky &&
        self.entries == other.entries &&
        self.permission == other.permission &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AclStatusProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AclEditLogProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    entries: ::protobuf::RepeatedField<AclEntryProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl AclEditLogProto {
    pub fn new() -> AclEditLogProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AclEditLogProto {
        static mut instance: ::protobuf::lazy::Lazy<AclEditLogProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AclEditLogProto,
        };
        unsafe {
            instance.get(|| {
                AclEditLogProto {
                    src: ::protobuf::SingularField::none(),
                    entries: ::protobuf::RepeatedField::new(),
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

    // repeated .hadoop.hdfs.AclEntryProto entries = 2;

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_entries(&mut self, v: ::protobuf::RepeatedField<AclEntryProto>) {
        self.entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entries<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<AclEntryProto> {
        &mut self.entries
    }

    // Take field
    pub fn take_entries(&mut self) -> ::protobuf::RepeatedField<AclEntryProto> {
        ::std::mem::replace(&mut self.entries, ::protobuf::RepeatedField::new())
    }

    pub fn get_entries<'a>(&'a self) -> &'a [AclEntryProto] {
        &self.entries
    }
}

impl ::protobuf::Message for AclEditLogProto {
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
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.entries));
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
        for value in self.entries.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.src.as_ref() {
            try!(os.write_string(1, &v));
        };
        for v in self.entries.iter() {
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
        ::std::any::TypeId::of::<AclEditLogProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AclEditLogProto {
    fn new() -> AclEditLogProto {
        AclEditLogProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<AclEditLogProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "src",
                    AclEditLogProto::has_src,
                    AclEditLogProto::get_src,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "entries",
                    AclEditLogProto::get_entries,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AclEditLogProto>(
                    "AclEditLogProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AclEditLogProto {
    fn clear(&mut self) {
        self.clear_src();
        self.clear_entries();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AclEditLogProto {
    fn eq(&self, other: &AclEditLogProto) -> bool {
        self.src == other.src &&
        self.entries == other.entries &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AclEditLogProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ModifyAclEntriesRequestProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    aclSpec: ::protobuf::RepeatedField<AclEntryProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ModifyAclEntriesRequestProto {
    pub fn new() -> ModifyAclEntriesRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ModifyAclEntriesRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<ModifyAclEntriesRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ModifyAclEntriesRequestProto,
        };
        unsafe {
            instance.get(|| {
                ModifyAclEntriesRequestProto {
                    src: ::protobuf::SingularField::none(),
                    aclSpec: ::protobuf::RepeatedField::new(),
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

    // repeated .hadoop.hdfs.AclEntryProto aclSpec = 2;

    pub fn clear_aclSpec(&mut self) {
        self.aclSpec.clear();
    }

    // Param is passed by value, moved
    pub fn set_aclSpec(&mut self, v: ::protobuf::RepeatedField<AclEntryProto>) {
        self.aclSpec = v;
    }

    // Mutable pointer to the field.
    pub fn mut_aclSpec<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<AclEntryProto> {
        &mut self.aclSpec
    }

    // Take field
    pub fn take_aclSpec(&mut self) -> ::protobuf::RepeatedField<AclEntryProto> {
        ::std::mem::replace(&mut self.aclSpec, ::protobuf::RepeatedField::new())
    }

    pub fn get_aclSpec<'a>(&'a self) -> &'a [AclEntryProto] {
        &self.aclSpec
    }
}

impl ::protobuf::Message for ModifyAclEntriesRequestProto {
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
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.aclSpec));
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
        for value in self.aclSpec.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.src.as_ref() {
            try!(os.write_string(1, &v));
        };
        for v in self.aclSpec.iter() {
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
        ::std::any::TypeId::of::<ModifyAclEntriesRequestProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ModifyAclEntriesRequestProto {
    fn new() -> ModifyAclEntriesRequestProto {
        ModifyAclEntriesRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ModifyAclEntriesRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "src",
                    ModifyAclEntriesRequestProto::has_src,
                    ModifyAclEntriesRequestProto::get_src,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "aclSpec",
                    ModifyAclEntriesRequestProto::get_aclSpec,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ModifyAclEntriesRequestProto>(
                    "ModifyAclEntriesRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ModifyAclEntriesRequestProto {
    fn clear(&mut self) {
        self.clear_src();
        self.clear_aclSpec();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ModifyAclEntriesRequestProto {
    fn eq(&self, other: &ModifyAclEntriesRequestProto) -> bool {
        self.src == other.src &&
        self.aclSpec == other.aclSpec &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ModifyAclEntriesRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ModifyAclEntriesResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl ModifyAclEntriesResponseProto {
    pub fn new() -> ModifyAclEntriesResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ModifyAclEntriesResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<ModifyAclEntriesResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ModifyAclEntriesResponseProto,
        };
        unsafe {
            instance.get(|| {
                ModifyAclEntriesResponseProto {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for ModifyAclEntriesResponseProto {
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
        ::std::any::TypeId::of::<ModifyAclEntriesResponseProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ModifyAclEntriesResponseProto {
    fn new() -> ModifyAclEntriesResponseProto {
        ModifyAclEntriesResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<ModifyAclEntriesResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ModifyAclEntriesResponseProto>(
                    "ModifyAclEntriesResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ModifyAclEntriesResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ModifyAclEntriesResponseProto {
    fn eq(&self, other: &ModifyAclEntriesResponseProto) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ModifyAclEntriesResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RemoveAclRequestProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RemoveAclRequestProto {
    pub fn new() -> RemoveAclRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RemoveAclRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<RemoveAclRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RemoveAclRequestProto,
        };
        unsafe {
            instance.get(|| {
                RemoveAclRequestProto {
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

impl ::protobuf::Message for RemoveAclRequestProto {
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
        ::std::any::TypeId::of::<RemoveAclRequestProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RemoveAclRequestProto {
    fn new() -> RemoveAclRequestProto {
        RemoveAclRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RemoveAclRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "src",
                    RemoveAclRequestProto::has_src,
                    RemoveAclRequestProto::get_src,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RemoveAclRequestProto>(
                    "RemoveAclRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RemoveAclRequestProto {
    fn clear(&mut self) {
        self.clear_src();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RemoveAclRequestProto {
    fn eq(&self, other: &RemoveAclRequestProto) -> bool {
        self.src == other.src &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RemoveAclRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RemoveAclResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RemoveAclResponseProto {
    pub fn new() -> RemoveAclResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RemoveAclResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<RemoveAclResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RemoveAclResponseProto,
        };
        unsafe {
            instance.get(|| {
                RemoveAclResponseProto {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for RemoveAclResponseProto {
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
        ::std::any::TypeId::of::<RemoveAclResponseProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RemoveAclResponseProto {
    fn new() -> RemoveAclResponseProto {
        RemoveAclResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RemoveAclResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RemoveAclResponseProto>(
                    "RemoveAclResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RemoveAclResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RemoveAclResponseProto {
    fn eq(&self, other: &RemoveAclResponseProto) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RemoveAclResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RemoveAclEntriesRequestProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    aclSpec: ::protobuf::RepeatedField<AclEntryProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RemoveAclEntriesRequestProto {
    pub fn new() -> RemoveAclEntriesRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RemoveAclEntriesRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<RemoveAclEntriesRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RemoveAclEntriesRequestProto,
        };
        unsafe {
            instance.get(|| {
                RemoveAclEntriesRequestProto {
                    src: ::protobuf::SingularField::none(),
                    aclSpec: ::protobuf::RepeatedField::new(),
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

    // repeated .hadoop.hdfs.AclEntryProto aclSpec = 2;

    pub fn clear_aclSpec(&mut self) {
        self.aclSpec.clear();
    }

    // Param is passed by value, moved
    pub fn set_aclSpec(&mut self, v: ::protobuf::RepeatedField<AclEntryProto>) {
        self.aclSpec = v;
    }

    // Mutable pointer to the field.
    pub fn mut_aclSpec<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<AclEntryProto> {
        &mut self.aclSpec
    }

    // Take field
    pub fn take_aclSpec(&mut self) -> ::protobuf::RepeatedField<AclEntryProto> {
        ::std::mem::replace(&mut self.aclSpec, ::protobuf::RepeatedField::new())
    }

    pub fn get_aclSpec<'a>(&'a self) -> &'a [AclEntryProto] {
        &self.aclSpec
    }
}

impl ::protobuf::Message for RemoveAclEntriesRequestProto {
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
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.aclSpec));
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
        for value in self.aclSpec.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.src.as_ref() {
            try!(os.write_string(1, &v));
        };
        for v in self.aclSpec.iter() {
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
        ::std::any::TypeId::of::<RemoveAclEntriesRequestProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RemoveAclEntriesRequestProto {
    fn new() -> RemoveAclEntriesRequestProto {
        RemoveAclEntriesRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RemoveAclEntriesRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "src",
                    RemoveAclEntriesRequestProto::has_src,
                    RemoveAclEntriesRequestProto::get_src,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "aclSpec",
                    RemoveAclEntriesRequestProto::get_aclSpec,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RemoveAclEntriesRequestProto>(
                    "RemoveAclEntriesRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RemoveAclEntriesRequestProto {
    fn clear(&mut self) {
        self.clear_src();
        self.clear_aclSpec();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RemoveAclEntriesRequestProto {
    fn eq(&self, other: &RemoveAclEntriesRequestProto) -> bool {
        self.src == other.src &&
        self.aclSpec == other.aclSpec &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RemoveAclEntriesRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RemoveAclEntriesResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RemoveAclEntriesResponseProto {
    pub fn new() -> RemoveAclEntriesResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RemoveAclEntriesResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<RemoveAclEntriesResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RemoveAclEntriesResponseProto,
        };
        unsafe {
            instance.get(|| {
                RemoveAclEntriesResponseProto {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for RemoveAclEntriesResponseProto {
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
        ::std::any::TypeId::of::<RemoveAclEntriesResponseProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RemoveAclEntriesResponseProto {
    fn new() -> RemoveAclEntriesResponseProto {
        RemoveAclEntriesResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RemoveAclEntriesResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RemoveAclEntriesResponseProto>(
                    "RemoveAclEntriesResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RemoveAclEntriesResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RemoveAclEntriesResponseProto {
    fn eq(&self, other: &RemoveAclEntriesResponseProto) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RemoveAclEntriesResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RemoveDefaultAclRequestProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RemoveDefaultAclRequestProto {
    pub fn new() -> RemoveDefaultAclRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RemoveDefaultAclRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<RemoveDefaultAclRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RemoveDefaultAclRequestProto,
        };
        unsafe {
            instance.get(|| {
                RemoveDefaultAclRequestProto {
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

impl ::protobuf::Message for RemoveDefaultAclRequestProto {
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
        ::std::any::TypeId::of::<RemoveDefaultAclRequestProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RemoveDefaultAclRequestProto {
    fn new() -> RemoveDefaultAclRequestProto {
        RemoveDefaultAclRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RemoveDefaultAclRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "src",
                    RemoveDefaultAclRequestProto::has_src,
                    RemoveDefaultAclRequestProto::get_src,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RemoveDefaultAclRequestProto>(
                    "RemoveDefaultAclRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RemoveDefaultAclRequestProto {
    fn clear(&mut self) {
        self.clear_src();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RemoveDefaultAclRequestProto {
    fn eq(&self, other: &RemoveDefaultAclRequestProto) -> bool {
        self.src == other.src &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RemoveDefaultAclRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RemoveDefaultAclResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RemoveDefaultAclResponseProto {
    pub fn new() -> RemoveDefaultAclResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RemoveDefaultAclResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<RemoveDefaultAclResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RemoveDefaultAclResponseProto,
        };
        unsafe {
            instance.get(|| {
                RemoveDefaultAclResponseProto {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for RemoveDefaultAclResponseProto {
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
        ::std::any::TypeId::of::<RemoveDefaultAclResponseProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RemoveDefaultAclResponseProto {
    fn new() -> RemoveDefaultAclResponseProto {
        RemoveDefaultAclResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RemoveDefaultAclResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RemoveDefaultAclResponseProto>(
                    "RemoveDefaultAclResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RemoveDefaultAclResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RemoveDefaultAclResponseProto {
    fn eq(&self, other: &RemoveDefaultAclResponseProto) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RemoveDefaultAclResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SetAclRequestProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    aclSpec: ::protobuf::RepeatedField<AclEntryProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl SetAclRequestProto {
    pub fn new() -> SetAclRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetAclRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<SetAclRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetAclRequestProto,
        };
        unsafe {
            instance.get(|| {
                SetAclRequestProto {
                    src: ::protobuf::SingularField::none(),
                    aclSpec: ::protobuf::RepeatedField::new(),
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

    // repeated .hadoop.hdfs.AclEntryProto aclSpec = 2;

    pub fn clear_aclSpec(&mut self) {
        self.aclSpec.clear();
    }

    // Param is passed by value, moved
    pub fn set_aclSpec(&mut self, v: ::protobuf::RepeatedField<AclEntryProto>) {
        self.aclSpec = v;
    }

    // Mutable pointer to the field.
    pub fn mut_aclSpec<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<AclEntryProto> {
        &mut self.aclSpec
    }

    // Take field
    pub fn take_aclSpec(&mut self) -> ::protobuf::RepeatedField<AclEntryProto> {
        ::std::mem::replace(&mut self.aclSpec, ::protobuf::RepeatedField::new())
    }

    pub fn get_aclSpec<'a>(&'a self) -> &'a [AclEntryProto] {
        &self.aclSpec
    }
}

impl ::protobuf::Message for SetAclRequestProto {
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
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.aclSpec));
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
        for value in self.aclSpec.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.src.as_ref() {
            try!(os.write_string(1, &v));
        };
        for v in self.aclSpec.iter() {
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
        ::std::any::TypeId::of::<SetAclRequestProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetAclRequestProto {
    fn new() -> SetAclRequestProto {
        SetAclRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetAclRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "src",
                    SetAclRequestProto::has_src,
                    SetAclRequestProto::get_src,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "aclSpec",
                    SetAclRequestProto::get_aclSpec,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SetAclRequestProto>(
                    "SetAclRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetAclRequestProto {
    fn clear(&mut self) {
        self.clear_src();
        self.clear_aclSpec();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetAclRequestProto {
    fn eq(&self, other: &SetAclRequestProto) -> bool {
        self.src == other.src &&
        self.aclSpec == other.aclSpec &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SetAclRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct SetAclResponseProto {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl SetAclResponseProto {
    pub fn new() -> SetAclResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SetAclResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<SetAclResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SetAclResponseProto,
        };
        unsafe {
            instance.get(|| {
                SetAclResponseProto {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for SetAclResponseProto {
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
        ::std::any::TypeId::of::<SetAclResponseProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for SetAclResponseProto {
    fn new() -> SetAclResponseProto {
        SetAclResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<SetAclResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<SetAclResponseProto>(
                    "SetAclResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SetAclResponseProto {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for SetAclResponseProto {
    fn eq(&self, other: &SetAclResponseProto) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for SetAclResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetAclStatusRequestProto {
    // message fields
    src: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl GetAclStatusRequestProto {
    pub fn new() -> GetAclStatusRequestProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetAclStatusRequestProto {
        static mut instance: ::protobuf::lazy::Lazy<GetAclStatusRequestProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetAclStatusRequestProto,
        };
        unsafe {
            instance.get(|| {
                GetAclStatusRequestProto {
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

impl ::protobuf::Message for GetAclStatusRequestProto {
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
        ::std::any::TypeId::of::<GetAclStatusRequestProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetAclStatusRequestProto {
    fn new() -> GetAclStatusRequestProto {
        GetAclStatusRequestProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetAclStatusRequestProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "src",
                    GetAclStatusRequestProto::has_src,
                    GetAclStatusRequestProto::get_src,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetAclStatusRequestProto>(
                    "GetAclStatusRequestProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetAclStatusRequestProto {
    fn clear(&mut self) {
        self.clear_src();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetAclStatusRequestProto {
    fn eq(&self, other: &GetAclStatusRequestProto) -> bool {
        self.src == other.src &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetAclStatusRequestProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetAclStatusResponseProto {
    // message fields
    result: ::protobuf::SingularPtrField<AclStatusProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl GetAclStatusResponseProto {
    pub fn new() -> GetAclStatusResponseProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetAclStatusResponseProto {
        static mut instance: ::protobuf::lazy::Lazy<GetAclStatusResponseProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetAclStatusResponseProto,
        };
        unsafe {
            instance.get(|| {
                GetAclStatusResponseProto {
                    result: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.AclStatusProto result = 1;

    pub fn clear_result(&mut self) {
        self.result.clear();
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: AclStatusProto) {
        self.result = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_result<'a>(&'a mut self) -> &'a mut AclStatusProto {
        if self.result.is_none() {
            self.result.set_default();
        };
        self.result.as_mut().unwrap()
    }

    // Take field
    pub fn take_result(&mut self) -> AclStatusProto {
        self.result.take().unwrap_or_else(|| AclStatusProto::new())
    }

    pub fn get_result<'a>(&'a self) -> &'a AclStatusProto {
        self.result.as_ref().unwrap_or_else(|| AclStatusProto::default_instance())
    }
}

impl ::protobuf::Message for GetAclStatusResponseProto {
    fn is_initialized(&self) -> bool {
        if self.result.is_none() {
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
                    let tmp = self.result.set_default();
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
        for value in self.result.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result.as_ref() {
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
        ::std::any::TypeId::of::<GetAclStatusResponseProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetAclStatusResponseProto {
    fn new() -> GetAclStatusResponseProto {
        GetAclStatusResponseProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetAclStatusResponseProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "result",
                    GetAclStatusResponseProto::has_result,
                    GetAclStatusResponseProto::get_result,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetAclStatusResponseProto>(
                    "GetAclStatusResponseProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetAclStatusResponseProto {
    fn clear(&mut self) {
        self.clear_result();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetAclStatusResponseProto {
    fn eq(&self, other: &GetAclStatusResponseProto) -> bool {
        self.result == other.result &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetAclStatusResponseProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x09, 0x61, 0x63, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x68, 0x61, 0x64,
    0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x1a, 0x0a, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x22, 0xc4, 0x03, 0x0a, 0x0d, 0x41, 0x63, 0x6c, 0x45, 0x6e, 0x74, 0x72,
    0x79, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x3a, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01,
    0x20, 0x02, 0x28, 0x0e, 0x32, 0x2c, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64,
    0x66, 0x73, 0x2e, 0x41, 0x63, 0x6c, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x2e, 0x41, 0x63, 0x6c, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x54, 0x79, 0x70, 0x65, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x3c, 0x0a, 0x05, 0x73, 0x63, 0x6f, 0x70, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28,
    0x0e, 0x32, 0x2d, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e,
    0x41, 0x63, 0x6c, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x41, 0x63,
    0x6c, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x53, 0x63, 0x6f, 0x70, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x3d, 0x0a, 0x0b, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x18,
    0x03, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x28, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68,
    0x64, 0x66, 0x73, 0x2e, 0x41, 0x63, 0x6c, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x2e, 0x46, 0x73, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x0c, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x22, 0x2d, 0x0a,
    0x12, 0x41, 0x63, 0x6c, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x53, 0x63, 0x6f, 0x70, 0x65, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x0a, 0x0a, 0x06, 0x41, 0x43, 0x43, 0x45, 0x53, 0x53, 0x10, 0x00, 0x12,
    0x0b, 0x0a, 0x07, 0x44, 0x45, 0x46, 0x41, 0x55, 0x4c, 0x54, 0x10, 0x01, 0x22, 0x3d, 0x0a, 0x11,
    0x41, 0x63, 0x6c, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x54, 0x79, 0x70, 0x65, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x08, 0x0a, 0x04, 0x55, 0x53, 0x45, 0x52, 0x10, 0x00, 0x12, 0x09, 0x0a, 0x05, 0x47,
    0x52, 0x4f, 0x55, 0x50, 0x10, 0x01, 0x12, 0x08, 0x0a, 0x04, 0x4d, 0x41, 0x53, 0x4b, 0x10, 0x02,
    0x12, 0x09, 0x0a, 0x05, 0x4f, 0x54, 0x48, 0x45, 0x52, 0x10, 0x03, 0x22, 0x7e, 0x0a, 0x0d, 0x46,
    0x73, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x08, 0x0a, 0x04,
    0x4e, 0x4f, 0x4e, 0x45, 0x10, 0x00, 0x12, 0x0b, 0x0a, 0x07, 0x45, 0x58, 0x45, 0x43, 0x55, 0x54,
    0x45, 0x10, 0x01, 0x12, 0x09, 0x0a, 0x05, 0x57, 0x52, 0x49, 0x54, 0x45, 0x10, 0x02, 0x12, 0x11,
    0x0a, 0x0d, 0x57, 0x52, 0x49, 0x54, 0x45, 0x5f, 0x45, 0x58, 0x45, 0x43, 0x55, 0x54, 0x45, 0x10,
    0x03, 0x12, 0x08, 0x0a, 0x04, 0x52, 0x45, 0x41, 0x44, 0x10, 0x04, 0x12, 0x10, 0x0a, 0x0c, 0x52,
    0x45, 0x41, 0x44, 0x5f, 0x45, 0x58, 0x45, 0x43, 0x55, 0x54, 0x45, 0x10, 0x05, 0x12, 0x0e, 0x0a,
    0x0a, 0x52, 0x45, 0x41, 0x44, 0x5f, 0x57, 0x52, 0x49, 0x54, 0x45, 0x10, 0x06, 0x12, 0x0c, 0x0a,
    0x08, 0x50, 0x45, 0x52, 0x4d, 0x5f, 0x41, 0x4c, 0x4c, 0x10, 0x07, 0x22, 0x9f, 0x01, 0x0a, 0x0e,
    0x41, 0x63, 0x6c, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0d,
    0x0a, 0x05, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0d, 0x0a,
    0x05, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0e, 0x0a, 0x06,
    0x73, 0x74, 0x69, 0x63, 0x6b, 0x79, 0x18, 0x03, 0x20, 0x02, 0x28, 0x08, 0x12, 0x2b, 0x0a, 0x07,
    0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1a, 0x2e,
    0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x41, 0x63, 0x6c, 0x45,
    0x6e, 0x74, 0x72, 0x79, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x32, 0x0a, 0x0a, 0x70, 0x65, 0x72,
    0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e,
    0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x46, 0x73, 0x50, 0x65,
    0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x4b, 0x0a,
    0x0f, 0x41, 0x63, 0x6c, 0x45, 0x64, 0x69, 0x74, 0x4c, 0x6f, 0x67, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x0b, 0x0a, 0x03, 0x73, 0x72, 0x63, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x2b, 0x0a,
    0x07, 0x65, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1a,
    0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x41, 0x63, 0x6c,
    0x45, 0x6e, 0x74, 0x72, 0x79, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x58, 0x0a, 0x1c, 0x4d, 0x6f,
    0x64, 0x69, 0x66, 0x79, 0x41, 0x63, 0x6c, 0x45, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x0a, 0x03, 0x73, 0x72,
    0x63, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x2b, 0x0a, 0x07, 0x61, 0x63, 0x6c, 0x53, 0x70,
    0x65, 0x63, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f,
    0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x41, 0x63, 0x6c, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x22, 0x1f, 0x0a, 0x1d, 0x4d, 0x6f, 0x64, 0x69, 0x66, 0x79, 0x41, 0x63,
    0x6c, 0x45, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x24, 0x0a, 0x15, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x41,
    0x63, 0x6c, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b,
    0x0a, 0x03, 0x73, 0x72, 0x63, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x22, 0x18, 0x0a, 0x16, 0x52,
    0x65, 0x6d, 0x6f, 0x76, 0x65, 0x41, 0x63, 0x6c, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x58, 0x0a, 0x1c, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x41,
    0x63, 0x6c, 0x45, 0x6e, 0x74, 0x72, 0x69, 0x65, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x0a, 0x03, 0x73, 0x72, 0x63, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x09, 0x12, 0x2b, 0x0a, 0x07, 0x61, 0x63, 0x6c, 0x53, 0x70, 0x65, 0x63, 0x18, 0x02, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66,
    0x73, 0x2e, 0x41, 0x63, 0x6c, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22,
    0x1f, 0x0a, 0x1d, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x41, 0x63, 0x6c, 0x45, 0x6e, 0x74, 0x72,
    0x69, 0x65, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x22, 0x2b, 0x0a, 0x1c, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c,
    0x74, 0x41, 0x63, 0x6c, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x0b, 0x0a, 0x03, 0x73, 0x72, 0x63, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x22, 0x1f, 0x0a,
    0x1d, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x41, 0x63,
    0x6c, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x4e,
    0x0a, 0x12, 0x53, 0x65, 0x74, 0x41, 0x63, 0x6c, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x0a, 0x03, 0x73, 0x72, 0x63, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x09, 0x12, 0x2b, 0x0a, 0x07, 0x61, 0x63, 0x6c, 0x53, 0x70, 0x65, 0x63, 0x18, 0x02, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73,
    0x2e, 0x41, 0x63, 0x6c, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x15,
    0x0a, 0x13, 0x53, 0x65, 0x74, 0x41, 0x63, 0x6c, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x27, 0x0a, 0x18, 0x47, 0x65, 0x74, 0x41, 0x63, 0x6c, 0x53,
    0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x0b, 0x0a, 0x03, 0x73, 0x72, 0x63, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x22, 0x48,
    0x0a, 0x19, 0x47, 0x65, 0x74, 0x41, 0x63, 0x6c, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2b, 0x0a, 0x06, 0x72,
    0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x68, 0x61,
    0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x41, 0x63, 0x6c, 0x53, 0x74, 0x61,
    0x74, 0x75, 0x73, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x42, 0x35, 0x0a, 0x25, 0x6f, 0x72, 0x67, 0x2e,
    0x61, 0x70, 0x61, 0x63, 0x68, 0x65, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64,
    0x66, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x42, 0x09, 0x41, 0x63, 0x6c, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0xa0, 0x01, 0x01, 0x4a,
    0x8a, 0x16, 0x0a, 0x06, 0x12, 0x04, 0x12, 0x00, 0x70, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x12, 0x00, 0x3e, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x12, 0x00,
    0x3e, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x12, 0x07, 0x13, 0x0a,
    0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x12, 0x07, 0x13, 0x0a, 0x0e,
    0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x12, 0x07, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x03, 0x12, 0x16, 0x3d, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x13, 0x00, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01, 0x12, 0x03,
    0x13, 0x00, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x13, 0x07,
    0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x13, 0x07, 0x1b,
    0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x13, 0x07, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x07, 0x12, 0x03, 0x13, 0x1e, 0x29, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x14, 0x00, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x02,
    0x12, 0x03, 0x14, 0x00, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x02, 0x02, 0x12, 0x03,
    0x14, 0x07, 0x24, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x02, 0x02, 0x00, 0x12, 0x03, 0x14,
    0x07, 0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x14,
    0x07, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x02, 0x03, 0x12, 0x03, 0x14, 0x27, 0x2b,
    0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x15, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00,
    0x12, 0x03, 0x17, 0x07, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x19, 0x00, 0x35,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x19, 0x08, 0x15, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x00, 0x04, 0x00, 0x12, 0x04, 0x1a, 0x02, 0x1d, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x04, 0x00, 0x01, 0x12, 0x03, 0x1a, 0x07, 0x19, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x1b, 0x04, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x04, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00,
    0x02, 0x00, 0x02, 0x12, 0x03, 0x1b, 0x0e, 0x11, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x1c, 0x04, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x1c, 0x04, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02,
    0x01, 0x02, 0x12, 0x03, 0x1c, 0x0e, 0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x04, 0x01, 0x12,
    0x04, 0x1f, 0x02, 0x24, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x04, 0x01, 0x01, 0x12, 0x03,
    0x1f, 0x07, 0x18, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x20,
    0x04, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x20,
    0x04, 0x08, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x20,
    0x0c, 0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x21, 0x04,
    0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x21, 0x04,
    0x09, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x01, 0x02, 0x01, 0x02, 0x12, 0x03, 0x21, 0x0c,
    0x0f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x22, 0x04, 0x10,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x22, 0x04, 0x08,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x01, 0x02, 0x02, 0x02, 0x12, 0x03, 0x22, 0x0c, 0x0f,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x23, 0x04, 0x10, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x23, 0x04, 0x09, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x01, 0x02, 0x03, 0x02, 0x12, 0x03, 0x23, 0x0c, 0x0f, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x00, 0x04, 0x02, 0x12, 0x04, 0x26, 0x02, 0x2f, 0x03, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x04, 0x02, 0x01, 0x12, 0x03, 0x26, 0x07, 0x14, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x00, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x27, 0x04, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x27, 0x04, 0x08, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x04, 0x02, 0x02, 0x00, 0x02, 0x12, 0x03, 0x27, 0x14, 0x17, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00,
    0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x28, 0x04, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04,
    0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x28, 0x04, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04,
    0x02, 0x02, 0x01, 0x02, 0x12, 0x03, 0x28, 0x14, 0x17, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04,
    0x02, 0x02, 0x02, 0x12, 0x03, 0x29, 0x04, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x02,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x29, 0x04, 0x09, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x02,
    0x02, 0x02, 0x02, 0x12, 0x03, 0x29, 0x14, 0x17, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x2a, 0x04, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x02, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x2a, 0x04, 0x11, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x02, 0x02,
    0x03, 0x02, 0x12, 0x03, 0x2a, 0x14, 0x17, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x2b, 0x04, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x02, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x2b, 0x04, 0x08, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x02, 0x02, 0x04,
    0x02, 0x12, 0x03, 0x2b, 0x14, 0x17, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x02, 0x02, 0x05,
    0x12, 0x03, 0x2c, 0x04, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x02, 0x02, 0x05, 0x01,
    0x12, 0x03, 0x2c, 0x04, 0x10, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x02, 0x02, 0x05, 0x02,
    0x12, 0x03, 0x2c, 0x14, 0x17, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x02, 0x02, 0x06, 0x12,
    0x03, 0x2d, 0x04, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x02, 0x02, 0x06, 0x01, 0x12,
    0x03, 0x2d, 0x04, 0x0e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x02, 0x02, 0x06, 0x02, 0x12,
    0x03, 0x2d, 0x14, 0x17, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x02, 0x02, 0x07, 0x12, 0x03,
    0x2e, 0x04, 0x18, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x02, 0x02, 0x07, 0x01, 0x12, 0x03,
    0x2e, 0x04, 0x0c, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x02, 0x02, 0x07, 0x02, 0x12, 0x03,
    0x2e, 0x14, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x31, 0x02, 0x29,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x31, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x31, 0x0b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x31, 0x1d, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x31, 0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x32, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x32, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x32, 0x0b,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x32, 0x1e, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x32, 0x27, 0x28, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x33, 0x02, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x33, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x06, 0x12, 0x03, 0x33, 0x0b, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x33, 0x19, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x33,
    0x27, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x34, 0x02, 0x29, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03, 0x34, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x34, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x34, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x34, 0x27, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x37,
    0x00, 0x3d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x37, 0x08, 0x16, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x38, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x38, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x38, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x38, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x38, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x39, 0x02,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x39, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x39, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x39, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x39, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02,
    0x02, 0x12, 0x03, 0x3a, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12,
    0x03, 0x3a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x3a,
    0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3a, 0x10, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x3a, 0x23, 0x24, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x3b, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x03, 0x04, 0x12, 0x03, 0x3b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x03, 0x06, 0x12, 0x03, 0x3b, 0x0b, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x3b, 0x19, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x3b, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x3c, 0x02, 0x2c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x04, 0x12, 0x03, 0x3c, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x06, 0x12, 0x03, 0x3c, 0x0b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x3c, 0x1d, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x3c, 0x2a, 0x2b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04,
    0x3f, 0x00, 0x42, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x3f, 0x08, 0x17,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x40, 0x02, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x40, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x40, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x40, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x40, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x41,
    0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x41, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x03, 0x41, 0x0b, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x41, 0x19, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x41, 0x23, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03,
    0x12, 0x04, 0x44, 0x00, 0x47, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x44,
    0x08, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x45, 0x02, 0x25, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x45, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x45, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x45, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x45, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12,
    0x03, 0x46, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x46,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12, 0x03, 0x46, 0x0b, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x46, 0x19, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x46, 0x23, 0x24, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x04, 0x12, 0x04, 0x49, 0x00, 0x4a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12,
    0x03, 0x49, 0x08, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x4c, 0x00, 0x4e, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x4c, 0x08, 0x1d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x4d, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x4d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x4d, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x4d, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4d, 0x18,
    0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x50, 0x00, 0x51, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x50, 0x08, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12,
    0x04, 0x53, 0x00, 0x56, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x53, 0x08,
    0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x54, 0x02, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x54, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x54, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x54, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x54, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03,
    0x55, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x03, 0x55, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x06, 0x12, 0x03, 0x55, 0x0b, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x55, 0x19, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x55, 0x23, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x08, 0x12, 0x04, 0x58, 0x00, 0x59, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03,
    0x58, 0x08, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x5b, 0x00, 0x5d, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x5b, 0x08, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x09, 0x02, 0x00, 0x12, 0x03, 0x5c, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x5c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x5c, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5c,
    0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5c, 0x18, 0x19,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x5f, 0x00, 0x60, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x0a, 0x01, 0x12, 0x03, 0x5f, 0x08, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04,
    0x62, 0x00, 0x65, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x62, 0x08, 0x1a,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x63, 0x02, 0x25, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x03, 0x63, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x00, 0x05, 0x12, 0x03, 0x63, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x63, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x63, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12, 0x03, 0x64,
    0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x03, 0x64, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x06, 0x12, 0x03, 0x64, 0x0b, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x03, 0x64, 0x19, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x03, 0x64, 0x23, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0c,
    0x12, 0x04, 0x67, 0x00, 0x68, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x67,
    0x08, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0d, 0x12, 0x04, 0x6a, 0x00, 0x6c, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12, 0x03, 0x6a, 0x08, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d,
    0x02, 0x00, 0x12, 0x03, 0x6b, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x6b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x6b, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6b, 0x12,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6b, 0x18, 0x19, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x04, 0x6e, 0x00, 0x70, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x0e, 0x01, 0x12, 0x03, 0x6e, 0x08, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12,
    0x03, 0x6f, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x03, 0x6f,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x06, 0x12, 0x03, 0x6f, 0x0b, 0x19,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6f, 0x1a, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6f, 0x23, 0x24,
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
