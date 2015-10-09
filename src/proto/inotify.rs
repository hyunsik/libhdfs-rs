// This file is generated. Do not edit
// @generated

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;
use super::acl::AclEntryProto;
use super::acl::AclStatusProto;
use super::acl::AclEditLogProto;
use super::acl::ModifyAclEntriesRequestProto;
use super::acl::ModifyAclEntriesResponseProto;
use super::acl::RemoveAclRequestProto;
use super::acl::RemoveAclResponseProto;
use super::acl::RemoveAclEntriesRequestProto;
use super::acl::RemoveAclEntriesResponseProto;
use super::acl::RemoveDefaultAclRequestProto;
use super::acl::RemoveDefaultAclResponseProto;
use super::acl::SetAclRequestProto;
use super::acl::SetAclResponseProto;
use super::acl::GetAclStatusRequestProto;
use super::acl::GetAclStatusResponseProto;
use super::xattr::XAttrProto;
use super::xattr::XAttrEditLogProto;
use super::xattr::SetXAttrRequestProto;
use super::xattr::SetXAttrResponseProto;
use super::xattr::GetXAttrsRequestProto;
use super::xattr::GetXAttrsResponseProto;
use super::xattr::ListXAttrsRequestProto;
use super::xattr::ListXAttrsResponseProto;
use super::xattr::RemoveXAttrRequestProto;
use super::xattr::RemoveXAttrResponseProto;
use super::xattr::XAttrSetFlagProto;
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
pub struct EventProto {
    // message fields
    field_type: ::std::option::Option<EventType>,
    contents: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl EventProto {
    pub fn new() -> EventProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EventProto {
        static mut instance: ::protobuf::lazy::Lazy<EventProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EventProto,
        };
        unsafe {
            instance.get(|| {
                EventProto {
                    field_type: ::std::option::Option::None,
                    contents: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.EventType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: EventType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type<'a>(&self) -> EventType {
        self.field_type.unwrap_or(EventType::EVENT_CREATE)
    }

    // required bytes contents = 2;

    pub fn clear_contents(&mut self) {
        self.contents.clear();
    }

    pub fn has_contents(&self) -> bool {
        self.contents.is_some()
    }

    // Param is passed by value, moved
    pub fn set_contents(&mut self, v: ::std::vec::Vec<u8>) {
        self.contents = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_contents<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.contents.is_none() {
            self.contents.set_default();
        };
        self.contents.as_mut().unwrap()
    }

    // Take field
    pub fn take_contents(&mut self) -> ::std::vec::Vec<u8> {
        self.contents.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_contents<'a>(&'a self) -> &'a [u8] {
        match self.contents.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for EventProto {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        };
        if self.contents.is_none() {
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
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.contents.set_default();
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
        for value in self.field_type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.contents.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.contents.as_ref() {
            try!(os.write_bytes(2, &v));
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
        ::std::any::TypeId::of::<EventProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EventProto {
    fn new() -> EventProto {
        EventProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<EventProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "field_type",
                    EventProto::has_field_type,
                    EventProto::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "contents",
                    EventProto::has_contents,
                    EventProto::get_contents,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EventProto>(
                    "EventProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EventProto {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_contents();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EventProto {
    fn eq(&self, other: &EventProto) -> bool {
        self.field_type == other.field_type &&
        self.contents == other.contents &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EventProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct EventBatchProto {
    // message fields
    txid: ::std::option::Option<i64>,
    events: ::protobuf::RepeatedField<EventProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl EventBatchProto {
    pub fn new() -> EventBatchProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EventBatchProto {
        static mut instance: ::protobuf::lazy::Lazy<EventBatchProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EventBatchProto,
        };
        unsafe {
            instance.get(|| {
                EventBatchProto {
                    txid: ::std::option::Option::None,
                    events: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required int64 txid = 1;

    pub fn clear_txid(&mut self) {
        self.txid = ::std::option::Option::None;
    }

    pub fn has_txid(&self) -> bool {
        self.txid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_txid(&mut self, v: i64) {
        self.txid = ::std::option::Option::Some(v);
    }

    pub fn get_txid<'a>(&self) -> i64 {
        self.txid.unwrap_or(0)
    }

    // repeated .hadoop.hdfs.EventProto events = 2;

    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    // Param is passed by value, moved
    pub fn set_events(&mut self, v: ::protobuf::RepeatedField<EventProto>) {
        self.events = v;
    }

    // Mutable pointer to the field.
    pub fn mut_events<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<EventProto> {
        &mut self.events
    }

    // Take field
    pub fn take_events(&mut self) -> ::protobuf::RepeatedField<EventProto> {
        ::std::mem::replace(&mut self.events, ::protobuf::RepeatedField::new())
    }

    pub fn get_events<'a>(&'a self) -> &'a [EventProto] {
        &self.events
    }
}

impl ::protobuf::Message for EventBatchProto {
    fn is_initialized(&self) -> bool {
        if self.txid.is_none() {
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
                    self.txid = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.events));
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
        for value in self.txid.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.events.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.txid {
            try!(os.write_int64(1, v));
        };
        for v in self.events.iter() {
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
        ::std::any::TypeId::of::<EventBatchProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EventBatchProto {
    fn new() -> EventBatchProto {
        EventBatchProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<EventBatchProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "txid",
                    EventBatchProto::has_txid,
                    EventBatchProto::get_txid,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "events",
                    EventBatchProto::get_events,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EventBatchProto>(
                    "EventBatchProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EventBatchProto {
    fn clear(&mut self) {
        self.clear_txid();
        self.clear_events();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EventBatchProto {
    fn eq(&self, other: &EventBatchProto) -> bool {
        self.txid == other.txid &&
        self.events == other.events &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EventBatchProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CreateEventProto {
    // message fields
    field_type: ::std::option::Option<INodeType>,
    path: ::protobuf::SingularField<::std::string::String>,
    ctime: ::std::option::Option<i64>,
    ownerName: ::protobuf::SingularField<::std::string::String>,
    groupName: ::protobuf::SingularField<::std::string::String>,
    perms: ::protobuf::SingularPtrField<FsPermissionProto>,
    replication: ::std::option::Option<i32>,
    symlinkTarget: ::protobuf::SingularField<::std::string::String>,
    overwrite: ::std::option::Option<bool>,
    defaultBlockSize: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CreateEventProto {
    pub fn new() -> CreateEventProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CreateEventProto {
        static mut instance: ::protobuf::lazy::Lazy<CreateEventProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CreateEventProto,
        };
        unsafe {
            instance.get(|| {
                CreateEventProto {
                    field_type: ::std::option::Option::None,
                    path: ::protobuf::SingularField::none(),
                    ctime: ::std::option::Option::None,
                    ownerName: ::protobuf::SingularField::none(),
                    groupName: ::protobuf::SingularField::none(),
                    perms: ::protobuf::SingularPtrField::none(),
                    replication: ::std::option::Option::None,
                    symlinkTarget: ::protobuf::SingularField::none(),
                    overwrite: ::std::option::Option::None,
                    defaultBlockSize: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required .hadoop.hdfs.INodeType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: INodeType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type<'a>(&self) -> INodeType {
        self.field_type.unwrap_or(INodeType::I_TYPE_FILE)
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

    // required int64 ctime = 3;

    pub fn clear_ctime(&mut self) {
        self.ctime = ::std::option::Option::None;
    }

    pub fn has_ctime(&self) -> bool {
        self.ctime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ctime(&mut self, v: i64) {
        self.ctime = ::std::option::Option::Some(v);
    }

    pub fn get_ctime<'a>(&self) -> i64 {
        self.ctime.unwrap_or(0)
    }

    // required string ownerName = 4;

    pub fn clear_ownerName(&mut self) {
        self.ownerName.clear();
    }

    pub fn has_ownerName(&self) -> bool {
        self.ownerName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ownerName(&mut self, v: ::std::string::String) {
        self.ownerName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ownerName<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.ownerName.is_none() {
            self.ownerName.set_default();
        };
        self.ownerName.as_mut().unwrap()
    }

    // Take field
    pub fn take_ownerName(&mut self) -> ::std::string::String {
        self.ownerName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ownerName<'a>(&'a self) -> &'a str {
        match self.ownerName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string groupName = 5;

    pub fn clear_groupName(&mut self) {
        self.groupName.clear();
    }

    pub fn has_groupName(&self) -> bool {
        self.groupName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_groupName(&mut self, v: ::std::string::String) {
        self.groupName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_groupName<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.groupName.is_none() {
            self.groupName.set_default();
        };
        self.groupName.as_mut().unwrap()
    }

    // Take field
    pub fn take_groupName(&mut self) -> ::std::string::String {
        self.groupName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_groupName<'a>(&'a self) -> &'a str {
        match self.groupName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required .hadoop.hdfs.FsPermissionProto perms = 6;

    pub fn clear_perms(&mut self) {
        self.perms.clear();
    }

    pub fn has_perms(&self) -> bool {
        self.perms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_perms(&mut self, v: FsPermissionProto) {
        self.perms = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_perms<'a>(&'a mut self) -> &'a mut FsPermissionProto {
        if self.perms.is_none() {
            self.perms.set_default();
        };
        self.perms.as_mut().unwrap()
    }

    // Take field
    pub fn take_perms(&mut self) -> FsPermissionProto {
        self.perms.take().unwrap_or_else(|| FsPermissionProto::new())
    }

    pub fn get_perms<'a>(&'a self) -> &'a FsPermissionProto {
        self.perms.as_ref().unwrap_or_else(|| FsPermissionProto::default_instance())
    }

    // optional int32 replication = 7;

    pub fn clear_replication(&mut self) {
        self.replication = ::std::option::Option::None;
    }

    pub fn has_replication(&self) -> bool {
        self.replication.is_some()
    }

    // Param is passed by value, moved
    pub fn set_replication(&mut self, v: i32) {
        self.replication = ::std::option::Option::Some(v);
    }

    pub fn get_replication<'a>(&self) -> i32 {
        self.replication.unwrap_or(0)
    }

    // optional string symlinkTarget = 8;

    pub fn clear_symlinkTarget(&mut self) {
        self.symlinkTarget.clear();
    }

    pub fn has_symlinkTarget(&self) -> bool {
        self.symlinkTarget.is_some()
    }

    // Param is passed by value, moved
    pub fn set_symlinkTarget(&mut self, v: ::std::string::String) {
        self.symlinkTarget = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_symlinkTarget<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.symlinkTarget.is_none() {
            self.symlinkTarget.set_default();
        };
        self.symlinkTarget.as_mut().unwrap()
    }

    // Take field
    pub fn take_symlinkTarget(&mut self) -> ::std::string::String {
        self.symlinkTarget.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_symlinkTarget<'a>(&'a self) -> &'a str {
        match self.symlinkTarget.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional bool overwrite = 9;

    pub fn clear_overwrite(&mut self) {
        self.overwrite = ::std::option::Option::None;
    }

    pub fn has_overwrite(&self) -> bool {
        self.overwrite.is_some()
    }

    // Param is passed by value, moved
    pub fn set_overwrite(&mut self, v: bool) {
        self.overwrite = ::std::option::Option::Some(v);
    }

    pub fn get_overwrite<'a>(&self) -> bool {
        self.overwrite.unwrap_or(false)
    }

    // optional int64 defaultBlockSize = 10;

    pub fn clear_defaultBlockSize(&mut self) {
        self.defaultBlockSize = ::std::option::Option::None;
    }

    pub fn has_defaultBlockSize(&self) -> bool {
        self.defaultBlockSize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_defaultBlockSize(&mut self, v: i64) {
        self.defaultBlockSize = ::std::option::Option::Some(v);
    }

    pub fn get_defaultBlockSize<'a>(&self) -> i64 {
        self.defaultBlockSize.unwrap_or(0i64)
    }
}

impl ::protobuf::Message for CreateEventProto {
    fn is_initialized(&self) -> bool {
        if self.field_type.is_none() {
            return false;
        };
        if self.path.is_none() {
            return false;
        };
        if self.ctime.is_none() {
            return false;
        };
        if self.ownerName.is_none() {
            return false;
        };
        if self.groupName.is_none() {
            return false;
        };
        if self.perms.is_none() {
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
                    let tmp = try!(is.read_int64());
                    self.ctime = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.ownerName.set_default();
                    try!(is.read_string_into(tmp))
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.groupName.set_default();
                    try!(is.read_string_into(tmp))
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.perms.set_default();
                    try!(is.merge_message(tmp))
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.replication = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.symlinkTarget.set_default();
                    try!(is.read_string_into(tmp))
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.overwrite = ::std::option::Option::Some(tmp);
                },
                10 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.defaultBlockSize = ::std::option::Option::Some(tmp);
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
        for value in self.path.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.ctime.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.ownerName.iter() {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in self.groupName.iter() {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        for value in self.perms.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.replication.iter() {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.symlinkTarget.iter() {
            my_size += ::protobuf::rt::string_size(8, &value);
        };
        if self.overwrite.is_some() {
            my_size += 2;
        };
        for value in self.defaultBlockSize.iter() {
            my_size += ::protobuf::rt::value_size(10, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_enum(1, v as i32));
        };
        if let Some(v) = self.path.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.ctime {
            try!(os.write_int64(3, v));
        };
        if let Some(v) = self.ownerName.as_ref() {
            try!(os.write_string(4, &v));
        };
        if let Some(v) = self.groupName.as_ref() {
            try!(os.write_string(5, &v));
        };
        if let Some(v) = self.perms.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.replication {
            try!(os.write_int32(7, v));
        };
        if let Some(v) = self.symlinkTarget.as_ref() {
            try!(os.write_string(8, &v));
        };
        if let Some(v) = self.overwrite {
            try!(os.write_bool(9, v));
        };
        if let Some(v) = self.defaultBlockSize {
            try!(os.write_int64(10, v));
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
        ::std::any::TypeId::of::<CreateEventProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CreateEventProto {
    fn new() -> CreateEventProto {
        CreateEventProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CreateEventProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "field_type",
                    CreateEventProto::has_field_type,
                    CreateEventProto::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path",
                    CreateEventProto::has_path,
                    CreateEventProto::get_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "ctime",
                    CreateEventProto::has_ctime,
                    CreateEventProto::get_ctime,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "ownerName",
                    CreateEventProto::has_ownerName,
                    CreateEventProto::get_ownerName,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "groupName",
                    CreateEventProto::has_groupName,
                    CreateEventProto::get_groupName,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "perms",
                    CreateEventProto::has_perms,
                    CreateEventProto::get_perms,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "replication",
                    CreateEventProto::has_replication,
                    CreateEventProto::get_replication,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "symlinkTarget",
                    CreateEventProto::has_symlinkTarget,
                    CreateEventProto::get_symlinkTarget,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "overwrite",
                    CreateEventProto::has_overwrite,
                    CreateEventProto::get_overwrite,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "defaultBlockSize",
                    CreateEventProto::has_defaultBlockSize,
                    CreateEventProto::get_defaultBlockSize,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CreateEventProto>(
                    "CreateEventProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CreateEventProto {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_path();
        self.clear_ctime();
        self.clear_ownerName();
        self.clear_groupName();
        self.clear_perms();
        self.clear_replication();
        self.clear_symlinkTarget();
        self.clear_overwrite();
        self.clear_defaultBlockSize();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CreateEventProto {
    fn eq(&self, other: &CreateEventProto) -> bool {
        self.field_type == other.field_type &&
        self.path == other.path &&
        self.ctime == other.ctime &&
        self.ownerName == other.ownerName &&
        self.groupName == other.groupName &&
        self.perms == other.perms &&
        self.replication == other.replication &&
        self.symlinkTarget == other.symlinkTarget &&
        self.overwrite == other.overwrite &&
        self.defaultBlockSize == other.defaultBlockSize &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CreateEventProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CloseEventProto {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    fileSize: ::std::option::Option<i64>,
    timestamp: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl CloseEventProto {
    pub fn new() -> CloseEventProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CloseEventProto {
        static mut instance: ::protobuf::lazy::Lazy<CloseEventProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CloseEventProto,
        };
        unsafe {
            instance.get(|| {
                CloseEventProto {
                    path: ::protobuf::SingularField::none(),
                    fileSize: ::std::option::Option::None,
                    timestamp: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string path = 1;

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

    // required int64 fileSize = 2;

    pub fn clear_fileSize(&mut self) {
        self.fileSize = ::std::option::Option::None;
    }

    pub fn has_fileSize(&self) -> bool {
        self.fileSize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fileSize(&mut self, v: i64) {
        self.fileSize = ::std::option::Option::Some(v);
    }

    pub fn get_fileSize<'a>(&self) -> i64 {
        self.fileSize.unwrap_or(0)
    }

    // required int64 timestamp = 3;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: i64) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp<'a>(&self) -> i64 {
        self.timestamp.unwrap_or(0)
    }
}

impl ::protobuf::Message for CloseEventProto {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        };
        if self.fileSize.is_none() {
            return false;
        };
        if self.timestamp.is_none() {
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
                    let tmp = self.path.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.fileSize = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.timestamp = ::std::option::Option::Some(tmp);
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
        for value in self.path.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.fileSize.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.timestamp.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.fileSize {
            try!(os.write_int64(2, v));
        };
        if let Some(v) = self.timestamp {
            try!(os.write_int64(3, v));
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
        ::std::any::TypeId::of::<CloseEventProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CloseEventProto {
    fn new() -> CloseEventProto {
        CloseEventProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<CloseEventProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path",
                    CloseEventProto::has_path,
                    CloseEventProto::get_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "fileSize",
                    CloseEventProto::has_fileSize,
                    CloseEventProto::get_fileSize,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "timestamp",
                    CloseEventProto::has_timestamp,
                    CloseEventProto::get_timestamp,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CloseEventProto>(
                    "CloseEventProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CloseEventProto {
    fn clear(&mut self) {
        self.clear_path();
        self.clear_fileSize();
        self.clear_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CloseEventProto {
    fn eq(&self, other: &CloseEventProto) -> bool {
        self.path == other.path &&
        self.fileSize == other.fileSize &&
        self.timestamp == other.timestamp &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CloseEventProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AppendEventProto {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    newBlock: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl AppendEventProto {
    pub fn new() -> AppendEventProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AppendEventProto {
        static mut instance: ::protobuf::lazy::Lazy<AppendEventProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AppendEventProto,
        };
        unsafe {
            instance.get(|| {
                AppendEventProto {
                    path: ::protobuf::SingularField::none(),
                    newBlock: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string path = 1;

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

    // optional bool newBlock = 2;

    pub fn clear_newBlock(&mut self) {
        self.newBlock = ::std::option::Option::None;
    }

    pub fn has_newBlock(&self) -> bool {
        self.newBlock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_newBlock(&mut self, v: bool) {
        self.newBlock = ::std::option::Option::Some(v);
    }

    pub fn get_newBlock<'a>(&self) -> bool {
        self.newBlock.unwrap_or(false)
    }
}

impl ::protobuf::Message for AppendEventProto {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
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
                    let tmp = self.path.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.newBlock = ::std::option::Option::Some(tmp);
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
        for value in self.path.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if self.newBlock.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.newBlock {
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
        ::std::any::TypeId::of::<AppendEventProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AppendEventProto {
    fn new() -> AppendEventProto {
        AppendEventProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<AppendEventProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path",
                    AppendEventProto::has_path,
                    AppendEventProto::get_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "newBlock",
                    AppendEventProto::has_newBlock,
                    AppendEventProto::get_newBlock,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AppendEventProto>(
                    "AppendEventProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AppendEventProto {
    fn clear(&mut self) {
        self.clear_path();
        self.clear_newBlock();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AppendEventProto {
    fn eq(&self, other: &AppendEventProto) -> bool {
        self.path == other.path &&
        self.newBlock == other.newBlock &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AppendEventProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RenameEventProto {
    // message fields
    srcPath: ::protobuf::SingularField<::std::string::String>,
    destPath: ::protobuf::SingularField<::std::string::String>,
    timestamp: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl RenameEventProto {
    pub fn new() -> RenameEventProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RenameEventProto {
        static mut instance: ::protobuf::lazy::Lazy<RenameEventProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RenameEventProto,
        };
        unsafe {
            instance.get(|| {
                RenameEventProto {
                    srcPath: ::protobuf::SingularField::none(),
                    destPath: ::protobuf::SingularField::none(),
                    timestamp: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string srcPath = 1;

    pub fn clear_srcPath(&mut self) {
        self.srcPath.clear();
    }

    pub fn has_srcPath(&self) -> bool {
        self.srcPath.is_some()
    }

    // Param is passed by value, moved
    pub fn set_srcPath(&mut self, v: ::std::string::String) {
        self.srcPath = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_srcPath<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.srcPath.is_none() {
            self.srcPath.set_default();
        };
        self.srcPath.as_mut().unwrap()
    }

    // Take field
    pub fn take_srcPath(&mut self) -> ::std::string::String {
        self.srcPath.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_srcPath<'a>(&'a self) -> &'a str {
        match self.srcPath.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required string destPath = 2;

    pub fn clear_destPath(&mut self) {
        self.destPath.clear();
    }

    pub fn has_destPath(&self) -> bool {
        self.destPath.is_some()
    }

    // Param is passed by value, moved
    pub fn set_destPath(&mut self, v: ::std::string::String) {
        self.destPath = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_destPath<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.destPath.is_none() {
            self.destPath.set_default();
        };
        self.destPath.as_mut().unwrap()
    }

    // Take field
    pub fn take_destPath(&mut self) -> ::std::string::String {
        self.destPath.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_destPath<'a>(&'a self) -> &'a str {
        match self.destPath.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // required int64 timestamp = 3;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: i64) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp<'a>(&self) -> i64 {
        self.timestamp.unwrap_or(0)
    }
}

impl ::protobuf::Message for RenameEventProto {
    fn is_initialized(&self) -> bool {
        if self.srcPath.is_none() {
            return false;
        };
        if self.destPath.is_none() {
            return false;
        };
        if self.timestamp.is_none() {
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
                    let tmp = self.srcPath.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.destPath.set_default();
                    try!(is.read_string_into(tmp))
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.timestamp = ::std::option::Option::Some(tmp);
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
        for value in self.srcPath.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.destPath.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.timestamp.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.srcPath.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.destPath.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.timestamp {
            try!(os.write_int64(3, v));
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
        ::std::any::TypeId::of::<RenameEventProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RenameEventProto {
    fn new() -> RenameEventProto {
        RenameEventProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<RenameEventProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "srcPath",
                    RenameEventProto::has_srcPath,
                    RenameEventProto::get_srcPath,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "destPath",
                    RenameEventProto::has_destPath,
                    RenameEventProto::get_destPath,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "timestamp",
                    RenameEventProto::has_timestamp,
                    RenameEventProto::get_timestamp,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RenameEventProto>(
                    "RenameEventProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RenameEventProto {
    fn clear(&mut self) {
        self.clear_srcPath();
        self.clear_destPath();
        self.clear_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RenameEventProto {
    fn eq(&self, other: &RenameEventProto) -> bool {
        self.srcPath == other.srcPath &&
        self.destPath == other.destPath &&
        self.timestamp == other.timestamp &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RenameEventProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct MetadataUpdateEventProto {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    field_type: ::std::option::Option<MetadataUpdateType>,
    mtime: ::std::option::Option<i64>,
    atime: ::std::option::Option<i64>,
    replication: ::std::option::Option<i32>,
    ownerName: ::protobuf::SingularField<::std::string::String>,
    groupName: ::protobuf::SingularField<::std::string::String>,
    perms: ::protobuf::SingularPtrField<FsPermissionProto>,
    acls: ::protobuf::RepeatedField<AclEntryProto>,
    xAttrs: ::protobuf::RepeatedField<XAttrProto>,
    xAttrsRemoved: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl MetadataUpdateEventProto {
    pub fn new() -> MetadataUpdateEventProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MetadataUpdateEventProto {
        static mut instance: ::protobuf::lazy::Lazy<MetadataUpdateEventProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MetadataUpdateEventProto,
        };
        unsafe {
            instance.get(|| {
                MetadataUpdateEventProto {
                    path: ::protobuf::SingularField::none(),
                    field_type: ::std::option::Option::None,
                    mtime: ::std::option::Option::None,
                    atime: ::std::option::Option::None,
                    replication: ::std::option::Option::None,
                    ownerName: ::protobuf::SingularField::none(),
                    groupName: ::protobuf::SingularField::none(),
                    perms: ::protobuf::SingularPtrField::none(),
                    acls: ::protobuf::RepeatedField::new(),
                    xAttrs: ::protobuf::RepeatedField::new(),
                    xAttrsRemoved: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string path = 1;

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

    // required .hadoop.hdfs.MetadataUpdateType type = 2;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: MetadataUpdateType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type<'a>(&self) -> MetadataUpdateType {
        self.field_type.unwrap_or(MetadataUpdateType::META_TYPE_TIMES)
    }

    // optional int64 mtime = 3;

    pub fn clear_mtime(&mut self) {
        self.mtime = ::std::option::Option::None;
    }

    pub fn has_mtime(&self) -> bool {
        self.mtime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_mtime(&mut self, v: i64) {
        self.mtime = ::std::option::Option::Some(v);
    }

    pub fn get_mtime<'a>(&self) -> i64 {
        self.mtime.unwrap_or(0)
    }

    // optional int64 atime = 4;

    pub fn clear_atime(&mut self) {
        self.atime = ::std::option::Option::None;
    }

    pub fn has_atime(&self) -> bool {
        self.atime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_atime(&mut self, v: i64) {
        self.atime = ::std::option::Option::Some(v);
    }

    pub fn get_atime<'a>(&self) -> i64 {
        self.atime.unwrap_or(0)
    }

    // optional int32 replication = 5;

    pub fn clear_replication(&mut self) {
        self.replication = ::std::option::Option::None;
    }

    pub fn has_replication(&self) -> bool {
        self.replication.is_some()
    }

    // Param is passed by value, moved
    pub fn set_replication(&mut self, v: i32) {
        self.replication = ::std::option::Option::Some(v);
    }

    pub fn get_replication<'a>(&self) -> i32 {
        self.replication.unwrap_or(0)
    }

    // optional string ownerName = 6;

    pub fn clear_ownerName(&mut self) {
        self.ownerName.clear();
    }

    pub fn has_ownerName(&self) -> bool {
        self.ownerName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ownerName(&mut self, v: ::std::string::String) {
        self.ownerName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ownerName<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.ownerName.is_none() {
            self.ownerName.set_default();
        };
        self.ownerName.as_mut().unwrap()
    }

    // Take field
    pub fn take_ownerName(&mut self) -> ::std::string::String {
        self.ownerName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_ownerName<'a>(&'a self) -> &'a str {
        match self.ownerName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string groupName = 7;

    pub fn clear_groupName(&mut self) {
        self.groupName.clear();
    }

    pub fn has_groupName(&self) -> bool {
        self.groupName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_groupName(&mut self, v: ::std::string::String) {
        self.groupName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_groupName<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.groupName.is_none() {
            self.groupName.set_default();
        };
        self.groupName.as_mut().unwrap()
    }

    // Take field
    pub fn take_groupName(&mut self) -> ::std::string::String {
        self.groupName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_groupName<'a>(&'a self) -> &'a str {
        match self.groupName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .hadoop.hdfs.FsPermissionProto perms = 8;

    pub fn clear_perms(&mut self) {
        self.perms.clear();
    }

    pub fn has_perms(&self) -> bool {
        self.perms.is_some()
    }

    // Param is passed by value, moved
    pub fn set_perms(&mut self, v: FsPermissionProto) {
        self.perms = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_perms<'a>(&'a mut self) -> &'a mut FsPermissionProto {
        if self.perms.is_none() {
            self.perms.set_default();
        };
        self.perms.as_mut().unwrap()
    }

    // Take field
    pub fn take_perms(&mut self) -> FsPermissionProto {
        self.perms.take().unwrap_or_else(|| FsPermissionProto::new())
    }

    pub fn get_perms<'a>(&'a self) -> &'a FsPermissionProto {
        self.perms.as_ref().unwrap_or_else(|| FsPermissionProto::default_instance())
    }

    // repeated .hadoop.hdfs.AclEntryProto acls = 9;

    pub fn clear_acls(&mut self) {
        self.acls.clear();
    }

    // Param is passed by value, moved
    pub fn set_acls(&mut self, v: ::protobuf::RepeatedField<AclEntryProto>) {
        self.acls = v;
    }

    // Mutable pointer to the field.
    pub fn mut_acls<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<AclEntryProto> {
        &mut self.acls
    }

    // Take field
    pub fn take_acls(&mut self) -> ::protobuf::RepeatedField<AclEntryProto> {
        ::std::mem::replace(&mut self.acls, ::protobuf::RepeatedField::new())
    }

    pub fn get_acls<'a>(&'a self) -> &'a [AclEntryProto] {
        &self.acls
    }

    // repeated .hadoop.hdfs.XAttrProto xAttrs = 10;

    pub fn clear_xAttrs(&mut self) {
        self.xAttrs.clear();
    }

    // Param is passed by value, moved
    pub fn set_xAttrs(&mut self, v: ::protobuf::RepeatedField<XAttrProto>) {
        self.xAttrs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_xAttrs<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<XAttrProto> {
        &mut self.xAttrs
    }

    // Take field
    pub fn take_xAttrs(&mut self) -> ::protobuf::RepeatedField<XAttrProto> {
        ::std::mem::replace(&mut self.xAttrs, ::protobuf::RepeatedField::new())
    }

    pub fn get_xAttrs<'a>(&'a self) -> &'a [XAttrProto] {
        &self.xAttrs
    }

    // optional bool xAttrsRemoved = 11;

    pub fn clear_xAttrsRemoved(&mut self) {
        self.xAttrsRemoved = ::std::option::Option::None;
    }

    pub fn has_xAttrsRemoved(&self) -> bool {
        self.xAttrsRemoved.is_some()
    }

    // Param is passed by value, moved
    pub fn set_xAttrsRemoved(&mut self, v: bool) {
        self.xAttrsRemoved = ::std::option::Option::Some(v);
    }

    pub fn get_xAttrsRemoved<'a>(&self) -> bool {
        self.xAttrsRemoved.unwrap_or(false)
    }
}

impl ::protobuf::Message for MetadataUpdateEventProto {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
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
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.path.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.mtime = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.atime = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int32());
                    self.replication = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.ownerName.set_default();
                    try!(is.read_string_into(tmp))
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.groupName.set_default();
                    try!(is.read_string_into(tmp))
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = self.perms.set_default();
                    try!(is.merge_message(tmp))
                },
                9 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.acls));
                },
                10 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.xAttrs));
                },
                11 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_bool());
                    self.xAttrsRemoved = ::std::option::Option::Some(tmp);
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
        for value in self.path.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.field_type.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in self.mtime.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.atime.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.replication.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.ownerName.iter() {
            my_size += ::protobuf::rt::string_size(6, &value);
        };
        for value in self.groupName.iter() {
            my_size += ::protobuf::rt::string_size(7, &value);
        };
        for value in self.perms.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.acls.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.xAttrs.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.xAttrsRemoved.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.field_type {
            try!(os.write_enum(2, v as i32));
        };
        if let Some(v) = self.mtime {
            try!(os.write_int64(3, v));
        };
        if let Some(v) = self.atime {
            try!(os.write_int64(4, v));
        };
        if let Some(v) = self.replication {
            try!(os.write_int32(5, v));
        };
        if let Some(v) = self.ownerName.as_ref() {
            try!(os.write_string(6, &v));
        };
        if let Some(v) = self.groupName.as_ref() {
            try!(os.write_string(7, &v));
        };
        if let Some(v) = self.perms.as_ref() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.acls.iter() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.xAttrs.iter() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.xAttrsRemoved {
            try!(os.write_bool(11, v));
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
        ::std::any::TypeId::of::<MetadataUpdateEventProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MetadataUpdateEventProto {
    fn new() -> MetadataUpdateEventProto {
        MetadataUpdateEventProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<MetadataUpdateEventProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path",
                    MetadataUpdateEventProto::has_path,
                    MetadataUpdateEventProto::get_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "field_type",
                    MetadataUpdateEventProto::has_field_type,
                    MetadataUpdateEventProto::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "mtime",
                    MetadataUpdateEventProto::has_mtime,
                    MetadataUpdateEventProto::get_mtime,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "atime",
                    MetadataUpdateEventProto::has_atime,
                    MetadataUpdateEventProto::get_atime,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i32_accessor(
                    "replication",
                    MetadataUpdateEventProto::has_replication,
                    MetadataUpdateEventProto::get_replication,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "ownerName",
                    MetadataUpdateEventProto::has_ownerName,
                    MetadataUpdateEventProto::get_ownerName,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "groupName",
                    MetadataUpdateEventProto::has_groupName,
                    MetadataUpdateEventProto::get_groupName,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "perms",
                    MetadataUpdateEventProto::has_perms,
                    MetadataUpdateEventProto::get_perms,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "acls",
                    MetadataUpdateEventProto::get_acls,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "xAttrs",
                    MetadataUpdateEventProto::get_xAttrs,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "xAttrsRemoved",
                    MetadataUpdateEventProto::has_xAttrsRemoved,
                    MetadataUpdateEventProto::get_xAttrsRemoved,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MetadataUpdateEventProto>(
                    "MetadataUpdateEventProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MetadataUpdateEventProto {
    fn clear(&mut self) {
        self.clear_path();
        self.clear_field_type();
        self.clear_mtime();
        self.clear_atime();
        self.clear_replication();
        self.clear_ownerName();
        self.clear_groupName();
        self.clear_perms();
        self.clear_acls();
        self.clear_xAttrs();
        self.clear_xAttrsRemoved();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for MetadataUpdateEventProto {
    fn eq(&self, other: &MetadataUpdateEventProto) -> bool {
        self.path == other.path &&
        self.field_type == other.field_type &&
        self.mtime == other.mtime &&
        self.atime == other.atime &&
        self.replication == other.replication &&
        self.ownerName == other.ownerName &&
        self.groupName == other.groupName &&
        self.perms == other.perms &&
        self.acls == other.acls &&
        self.xAttrs == other.xAttrs &&
        self.xAttrsRemoved == other.xAttrsRemoved &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for MetadataUpdateEventProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct UnlinkEventProto {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    timestamp: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl UnlinkEventProto {
    pub fn new() -> UnlinkEventProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnlinkEventProto {
        static mut instance: ::protobuf::lazy::Lazy<UnlinkEventProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnlinkEventProto,
        };
        unsafe {
            instance.get(|| {
                UnlinkEventProto {
                    path: ::protobuf::SingularField::none(),
                    timestamp: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // required string path = 1;

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

    // required int64 timestamp = 2;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: i64) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp<'a>(&self) -> i64 {
        self.timestamp.unwrap_or(0)
    }
}

impl ::protobuf::Message for UnlinkEventProto {
    fn is_initialized(&self) -> bool {
        if self.path.is_none() {
            return false;
        };
        if self.timestamp.is_none() {
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
                    let tmp = self.path.set_default();
                    try!(is.read_string_into(tmp))
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.timestamp = ::std::option::Option::Some(tmp);
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
        for value in self.path.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.timestamp.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.path.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.timestamp {
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
        ::std::any::TypeId::of::<UnlinkEventProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UnlinkEventProto {
    fn new() -> UnlinkEventProto {
        UnlinkEventProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnlinkEventProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "path",
                    UnlinkEventProto::has_path,
                    UnlinkEventProto::get_path,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "timestamp",
                    UnlinkEventProto::has_timestamp,
                    UnlinkEventProto::get_timestamp,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UnlinkEventProto>(
                    "UnlinkEventProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnlinkEventProto {
    fn clear(&mut self) {
        self.clear_path();
        self.clear_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for UnlinkEventProto {
    fn eq(&self, other: &UnlinkEventProto) -> bool {
        self.path == other.path &&
        self.timestamp == other.timestamp &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for UnlinkEventProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct EventsListProto {
    // message fields
    events: ::protobuf::RepeatedField<EventProto>,
    firstTxid: ::std::option::Option<i64>,
    lastTxid: ::std::option::Option<i64>,
    syncTxid: ::std::option::Option<i64>,
    batch: ::protobuf::RepeatedField<EventBatchProto>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

impl EventsListProto {
    pub fn new() -> EventsListProto {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EventsListProto {
        static mut instance: ::protobuf::lazy::Lazy<EventsListProto> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EventsListProto,
        };
        unsafe {
            instance.get(|| {
                EventsListProto {
                    events: ::protobuf::RepeatedField::new(),
                    firstTxid: ::std::option::Option::None,
                    lastTxid: ::std::option::Option::None,
                    syncTxid: ::std::option::Option::None,
                    batch: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .hadoop.hdfs.EventProto events = 1;

    pub fn clear_events(&mut self) {
        self.events.clear();
    }

    // Param is passed by value, moved
    pub fn set_events(&mut self, v: ::protobuf::RepeatedField<EventProto>) {
        self.events = v;
    }

    // Mutable pointer to the field.
    pub fn mut_events<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<EventProto> {
        &mut self.events
    }

    // Take field
    pub fn take_events(&mut self) -> ::protobuf::RepeatedField<EventProto> {
        ::std::mem::replace(&mut self.events, ::protobuf::RepeatedField::new())
    }

    pub fn get_events<'a>(&'a self) -> &'a [EventProto] {
        &self.events
    }

    // required int64 firstTxid = 2;

    pub fn clear_firstTxid(&mut self) {
        self.firstTxid = ::std::option::Option::None;
    }

    pub fn has_firstTxid(&self) -> bool {
        self.firstTxid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_firstTxid(&mut self, v: i64) {
        self.firstTxid = ::std::option::Option::Some(v);
    }

    pub fn get_firstTxid<'a>(&self) -> i64 {
        self.firstTxid.unwrap_or(0)
    }

    // required int64 lastTxid = 3;

    pub fn clear_lastTxid(&mut self) {
        self.lastTxid = ::std::option::Option::None;
    }

    pub fn has_lastTxid(&self) -> bool {
        self.lastTxid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastTxid(&mut self, v: i64) {
        self.lastTxid = ::std::option::Option::Some(v);
    }

    pub fn get_lastTxid<'a>(&self) -> i64 {
        self.lastTxid.unwrap_or(0)
    }

    // required int64 syncTxid = 4;

    pub fn clear_syncTxid(&mut self) {
        self.syncTxid = ::std::option::Option::None;
    }

    pub fn has_syncTxid(&self) -> bool {
        self.syncTxid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_syncTxid(&mut self, v: i64) {
        self.syncTxid = ::std::option::Option::Some(v);
    }

    pub fn get_syncTxid<'a>(&self) -> i64 {
        self.syncTxid.unwrap_or(0)
    }

    // repeated .hadoop.hdfs.EventBatchProto batch = 5;

    pub fn clear_batch(&mut self) {
        self.batch.clear();
    }

    // Param is passed by value, moved
    pub fn set_batch(&mut self, v: ::protobuf::RepeatedField<EventBatchProto>) {
        self.batch = v;
    }

    // Mutable pointer to the field.
    pub fn mut_batch<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<EventBatchProto> {
        &mut self.batch
    }

    // Take field
    pub fn take_batch(&mut self) -> ::protobuf::RepeatedField<EventBatchProto> {
        ::std::mem::replace(&mut self.batch, ::protobuf::RepeatedField::new())
    }

    pub fn get_batch<'a>(&'a self) -> &'a [EventBatchProto] {
        &self.batch
    }
}

impl ::protobuf::Message for EventsListProto {
    fn is_initialized(&self) -> bool {
        if self.firstTxid.is_none() {
            return false;
        };
        if self.lastTxid.is_none() {
            return false;
        };
        if self.syncTxid.is_none() {
            return false;
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.events));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.firstTxid = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.lastTxid = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::ProtobufError::WireError("unexpected wire type".to_string()));
                    };
                    let tmp = try!(is.read_int64());
                    self.syncTxid = ::std::option::Option::Some(tmp);
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.batch));
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
        for value in self.events.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.firstTxid.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.lastTxid.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.syncTxid.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.batch.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.events.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.firstTxid {
            try!(os.write_int64(2, v));
        };
        if let Some(v) = self.lastTxid {
            try!(os.write_int64(3, v));
        };
        if let Some(v) = self.syncTxid {
            try!(os.write_int64(4, v));
        };
        for v in self.batch.iter() {
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
        ::std::any::TypeId::of::<EventsListProto>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EventsListProto {
    fn new() -> EventsListProto {
        EventsListProto::new()
    }

    fn descriptor_static(_: ::std::option::Option<EventsListProto>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "events",
                    EventsListProto::get_events,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "firstTxid",
                    EventsListProto::has_firstTxid,
                    EventsListProto::get_firstTxid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "lastTxid",
                    EventsListProto::has_lastTxid,
                    EventsListProto::get_lastTxid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "syncTxid",
                    EventsListProto::has_syncTxid,
                    EventsListProto::get_syncTxid,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "batch",
                    EventsListProto::get_batch,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EventsListProto>(
                    "EventsListProto",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EventsListProto {
    fn clear(&mut self) {
        self.clear_events();
        self.clear_firstTxid();
        self.clear_lastTxid();
        self.clear_syncTxid();
        self.clear_batch();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for EventsListProto {
    fn eq(&self, other: &EventsListProto) -> bool {
        self.events == other.events &&
        self.firstTxid == other.firstTxid &&
        self.lastTxid == other.lastTxid &&
        self.syncTxid == other.syncTxid &&
        self.batch == other.batch &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for EventsListProto {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum EventType {
    EVENT_CREATE = 0,
    EVENT_CLOSE = 1,
    EVENT_APPEND = 2,
    EVENT_RENAME = 3,
    EVENT_METADATA = 4,
    EVENT_UNLINK = 5,
}

impl ::protobuf::ProtobufEnum for EventType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<EventType> {
        match value {
            0 => ::std::option::Option::Some(EventType::EVENT_CREATE),
            1 => ::std::option::Option::Some(EventType::EVENT_CLOSE),
            2 => ::std::option::Option::Some(EventType::EVENT_APPEND),
            3 => ::std::option::Option::Some(EventType::EVENT_RENAME),
            4 => ::std::option::Option::Some(EventType::EVENT_METADATA),
            5 => ::std::option::Option::Some(EventType::EVENT_UNLINK),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<EventType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("EventType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for EventType {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum INodeType {
    I_TYPE_FILE = 0,
    I_TYPE_DIRECTORY = 1,
    I_TYPE_SYMLINK = 2,
}

impl ::protobuf::ProtobufEnum for INodeType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<INodeType> {
        match value {
            0 => ::std::option::Option::Some(INodeType::I_TYPE_FILE),
            1 => ::std::option::Option::Some(INodeType::I_TYPE_DIRECTORY),
            2 => ::std::option::Option::Some(INodeType::I_TYPE_SYMLINK),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<INodeType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("INodeType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for INodeType {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MetadataUpdateType {
    META_TYPE_TIMES = 0,
    META_TYPE_REPLICATION = 1,
    META_TYPE_OWNER = 2,
    META_TYPE_PERMS = 3,
    META_TYPE_ACLS = 4,
    META_TYPE_XATTRS = 5,
}

impl ::protobuf::ProtobufEnum for MetadataUpdateType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MetadataUpdateType> {
        match value {
            0 => ::std::option::Option::Some(MetadataUpdateType::META_TYPE_TIMES),
            1 => ::std::option::Option::Some(MetadataUpdateType::META_TYPE_REPLICATION),
            2 => ::std::option::Option::Some(MetadataUpdateType::META_TYPE_OWNER),
            3 => ::std::option::Option::Some(MetadataUpdateType::META_TYPE_PERMS),
            4 => ::std::option::Option::Some(MetadataUpdateType::META_TYPE_ACLS),
            5 => ::std::option::Option::Some(MetadataUpdateType::META_TYPE_XATTRS),
            _ => ::std::option::Option::None
        }
    }

    fn enum_descriptor_static(_: Option<MetadataUpdateType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MetadataUpdateType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MetadataUpdateType {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0d, 0x69, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x0b, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x1a, 0x09, 0x61, 0x63,
    0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0b, 0x78, 0x61, 0x74, 0x74, 0x72, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0a, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x22, 0x44, 0x0a, 0x0a, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x24,
    0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x16, 0x2e, 0x68,
    0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x45, 0x76, 0x65, 0x6e, 0x74,
    0x54, 0x79, 0x70, 0x65, 0x12, 0x10, 0x0a, 0x08, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73,
    0x18, 0x02, 0x20, 0x02, 0x28, 0x0c, 0x22, 0x48, 0x0a, 0x0f, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x42,
    0x61, 0x74, 0x63, 0x68, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0c, 0x0a, 0x04, 0x74, 0x78, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x02, 0x28, 0x03, 0x12, 0x27, 0x0a, 0x06, 0x65, 0x76, 0x65, 0x6e, 0x74,
    0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70,
    0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x22, 0x86, 0x02, 0x0a, 0x10, 0x43, 0x72, 0x65, 0x61, 0x74, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74,
    0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x24, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20,
    0x02, 0x28, 0x0e, 0x32, 0x16, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66,
    0x73, 0x2e, 0x49, 0x4e, 0x6f, 0x64, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0c, 0x0a, 0x04, 0x70,
    0x61, 0x74, 0x68, 0x18, 0x02, 0x20, 0x02, 0x28, 0x09, 0x12, 0x0d, 0x0a, 0x05, 0x63, 0x74, 0x69,
    0x6d, 0x65, 0x18, 0x03, 0x20, 0x02, 0x28, 0x03, 0x12, 0x11, 0x0a, 0x09, 0x6f, 0x77, 0x6e, 0x65,
    0x72, 0x4e, 0x61, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x02, 0x28, 0x09, 0x12, 0x11, 0x0a, 0x09, 0x67,
    0x72, 0x6f, 0x75, 0x70, 0x4e, 0x61, 0x6d, 0x65, 0x18, 0x05, 0x20, 0x02, 0x28, 0x09, 0x12, 0x2d,
    0x0a, 0x05, 0x70, 0x65, 0x72, 0x6d, 0x73, 0x18, 0x06, 0x20, 0x02, 0x28, 0x0b, 0x32, 0x1e, 0x2e,
    0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x46, 0x73, 0x50, 0x65,
    0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x13, 0x0a,
    0x0b, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x07, 0x20, 0x01,
    0x28, 0x05, 0x12, 0x15, 0x0a, 0x0d, 0x73, 0x79, 0x6d, 0x6c, 0x69, 0x6e, 0x6b, 0x54, 0x61, 0x72,
    0x67, 0x65, 0x74, 0x18, 0x08, 0x20, 0x01, 0x28, 0x09, 0x12, 0x11, 0x0a, 0x09, 0x6f, 0x76, 0x65,
    0x72, 0x77, 0x72, 0x69, 0x74, 0x65, 0x18, 0x09, 0x20, 0x01, 0x28, 0x08, 0x12, 0x1b, 0x0a, 0x10,
    0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x53, 0x69, 0x7a, 0x65,
    0x18, 0x0a, 0x20, 0x01, 0x28, 0x03, 0x3a, 0x01, 0x30, 0x22, 0x44, 0x0a, 0x0f, 0x43, 0x6c, 0x6f,
    0x73, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0c, 0x0a, 0x04,
    0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12, 0x10, 0x0a, 0x08, 0x66, 0x69,
    0x6c, 0x65, 0x53, 0x69, 0x7a, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x03, 0x12, 0x11, 0x0a, 0x09,
    0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x03, 0x20, 0x02, 0x28, 0x03, 0x22,
    0x39, 0x0a, 0x10, 0x41, 0x70, 0x70, 0x65, 0x6e, 0x64, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x0c, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28,
    0x09, 0x12, 0x17, 0x0a, 0x08, 0x6e, 0x65, 0x77, 0x42, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x08, 0x3a, 0x05, 0x66, 0x61, 0x6c, 0x73, 0x65, 0x22, 0x48, 0x0a, 0x10, 0x52, 0x65,
    0x6e, 0x61, 0x6d, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0f,
    0x0a, 0x07, 0x73, 0x72, 0x63, 0x50, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12,
    0x10, 0x0a, 0x08, 0x64, 0x65, 0x73, 0x74, 0x50, 0x61, 0x74, 0x68, 0x18, 0x02, 0x20, 0x02, 0x28,
    0x09, 0x12, 0x11, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x03,
    0x20, 0x02, 0x28, 0x03, 0x22, 0xc9, 0x02, 0x0a, 0x18, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74,
    0x61, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x50, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x0c, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02, 0x28, 0x09, 0x12,
    0x2d, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x02, 0x28, 0x0e, 0x32, 0x1f, 0x2e,
    0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x4d, 0x65, 0x74, 0x61,
    0x64, 0x61, 0x74, 0x61, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0d,
    0x0a, 0x05, 0x6d, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x03, 0x12, 0x0d, 0x0a,
    0x05, 0x61, 0x74, 0x69, 0x6d, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x03, 0x12, 0x13, 0x0a, 0x0b,
    0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x05, 0x20, 0x01, 0x28,
    0x05, 0x12, 0x11, 0x0a, 0x09, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x4e, 0x61, 0x6d, 0x65, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x09, 0x12, 0x11, 0x0a, 0x09, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x4e, 0x61, 0x6d,
    0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x09, 0x12, 0x2d, 0x0a, 0x05, 0x70, 0x65, 0x72, 0x6d, 0x73,
    0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e,
    0x68, 0x64, 0x66, 0x73, 0x2e, 0x46, 0x73, 0x50, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f,
    0x6e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x28, 0x0a, 0x04, 0x61, 0x63, 0x6c, 0x73, 0x18, 0x09,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64,
    0x66, 0x73, 0x2e, 0x41, 0x63, 0x6c, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x12, 0x27, 0x0a, 0x06, 0x78, 0x41, 0x74, 0x74, 0x72, 0x73, 0x18, 0x0a, 0x20, 0x03, 0x28, 0x0b,
    0x32, 0x17, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x58,
    0x41, 0x74, 0x74, 0x72, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x15, 0x0a, 0x0d, 0x78, 0x41, 0x74,
    0x74, 0x72, 0x73, 0x52, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x64, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x08,
    0x22, 0x33, 0x0a, 0x10, 0x55, 0x6e, 0x6c, 0x69, 0x6e, 0x6b, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0c, 0x0a, 0x04, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x02,
    0x28, 0x09, 0x12, 0x11, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x03, 0x22, 0x9e, 0x01, 0x0a, 0x0f, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73,
    0x4c, 0x69, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x27, 0x0a, 0x06, 0x65, 0x76, 0x65,
    0x6e, 0x74, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x68, 0x61, 0x64, 0x6f,
    0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x11, 0x0a, 0x09, 0x66, 0x69, 0x72, 0x73, 0x74, 0x54, 0x78, 0x69, 0x64, 0x18,
    0x02, 0x20, 0x02, 0x28, 0x03, 0x12, 0x10, 0x0a, 0x08, 0x6c, 0x61, 0x73, 0x74, 0x54, 0x78, 0x69,
    0x64, 0x18, 0x03, 0x20, 0x02, 0x28, 0x03, 0x12, 0x10, 0x0a, 0x08, 0x73, 0x79, 0x6e, 0x63, 0x54,
    0x78, 0x69, 0x64, 0x18, 0x04, 0x20, 0x02, 0x28, 0x03, 0x12, 0x2b, 0x0a, 0x05, 0x62, 0x61, 0x74,
    0x63, 0x68, 0x18, 0x05, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f,
    0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x42, 0x61, 0x74, 0x63,
    0x68, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x2a, 0x78, 0x0a, 0x09, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x54,
    0x79, 0x70, 0x65, 0x12, 0x10, 0x0a, 0x0c, 0x45, 0x56, 0x45, 0x4e, 0x54, 0x5f, 0x43, 0x52, 0x45,
    0x41, 0x54, 0x45, 0x10, 0x00, 0x12, 0x0f, 0x0a, 0x0b, 0x45, 0x56, 0x45, 0x4e, 0x54, 0x5f, 0x43,
    0x4c, 0x4f, 0x53, 0x45, 0x10, 0x01, 0x12, 0x10, 0x0a, 0x0c, 0x45, 0x56, 0x45, 0x4e, 0x54, 0x5f,
    0x41, 0x50, 0x50, 0x45, 0x4e, 0x44, 0x10, 0x02, 0x12, 0x10, 0x0a, 0x0c, 0x45, 0x56, 0x45, 0x4e,
    0x54, 0x5f, 0x52, 0x45, 0x4e, 0x41, 0x4d, 0x45, 0x10, 0x03, 0x12, 0x12, 0x0a, 0x0e, 0x45, 0x56,
    0x45, 0x4e, 0x54, 0x5f, 0x4d, 0x45, 0x54, 0x41, 0x44, 0x41, 0x54, 0x41, 0x10, 0x04, 0x12, 0x10,
    0x0a, 0x0c, 0x45, 0x56, 0x45, 0x4e, 0x54, 0x5f, 0x55, 0x4e, 0x4c, 0x49, 0x4e, 0x4b, 0x10, 0x05,
    0x2a, 0x46, 0x0a, 0x09, 0x49, 0x4e, 0x6f, 0x64, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0f, 0x0a,
    0x0b, 0x49, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x46, 0x49, 0x4c, 0x45, 0x10, 0x00, 0x12, 0x14,
    0x0a, 0x10, 0x49, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x44, 0x49, 0x52, 0x45, 0x43, 0x54, 0x4f,
    0x52, 0x59, 0x10, 0x01, 0x12, 0x12, 0x0a, 0x0e, 0x49, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x53,
    0x59, 0x4d, 0x4c, 0x49, 0x4e, 0x4b, 0x10, 0x02, 0x2a, 0x98, 0x01, 0x0a, 0x12, 0x4d, 0x65, 0x74,
    0x61, 0x64, 0x61, 0x74, 0x61, 0x55, 0x70, 0x64, 0x61, 0x74, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12,
    0x13, 0x0a, 0x0f, 0x4d, 0x45, 0x54, 0x41, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x54, 0x49, 0x4d,
    0x45, 0x53, 0x10, 0x00, 0x12, 0x19, 0x0a, 0x15, 0x4d, 0x45, 0x54, 0x41, 0x5f, 0x54, 0x59, 0x50,
    0x45, 0x5f, 0x52, 0x45, 0x50, 0x4c, 0x49, 0x43, 0x41, 0x54, 0x49, 0x4f, 0x4e, 0x10, 0x01, 0x12,
    0x13, 0x0a, 0x0f, 0x4d, 0x45, 0x54, 0x41, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x4f, 0x57, 0x4e,
    0x45, 0x52, 0x10, 0x02, 0x12, 0x13, 0x0a, 0x0f, 0x4d, 0x45, 0x54, 0x41, 0x5f, 0x54, 0x59, 0x50,
    0x45, 0x5f, 0x50, 0x45, 0x52, 0x4d, 0x53, 0x10, 0x03, 0x12, 0x12, 0x0a, 0x0e, 0x4d, 0x45, 0x54,
    0x41, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x41, 0x43, 0x4c, 0x53, 0x10, 0x04, 0x12, 0x14, 0x0a,
    0x10, 0x4d, 0x45, 0x54, 0x41, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x58, 0x41, 0x54, 0x54, 0x52,
    0x53, 0x10, 0x05, 0x42, 0x39, 0x0a, 0x25, 0x6f, 0x72, 0x67, 0x2e, 0x61, 0x70, 0x61, 0x63, 0x68,
    0x65, 0x2e, 0x68, 0x61, 0x64, 0x6f, 0x6f, 0x70, 0x2e, 0x68, 0x64, 0x66, 0x73, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x42, 0x0d, 0x49, 0x6e,
    0x6f, 0x74, 0x69, 0x66, 0x79, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x73, 0xa0, 0x01, 0x01, 0x4a, 0xbe,
    0x1f, 0x0a, 0x06, 0x12, 0x04, 0x1b, 0x00, 0x7d, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x1b, 0x00, 0x3e, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x1b, 0x00, 0x3e,
    0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x1b, 0x07, 0x13, 0x0a, 0x0d,
    0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1b, 0x07, 0x13, 0x0a, 0x0e, 0x0a,
    0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x07, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x08, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x03, 0x1b, 0x16, 0x3d, 0x0a, 0x08, 0x0a, 0x01, 0x08,
    0x12, 0x03, 0x1c, 0x00, 0x2e, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01, 0x12, 0x03, 0x1c,
    0x00, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x1c, 0x07, 0x1b,
    0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1c, 0x07, 0x1b, 0x0a,
    0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x07, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x07, 0x12, 0x03, 0x1c, 0x1e, 0x2d, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x1d, 0x00, 0x2c, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x02, 0x12,
    0x03, 0x1d, 0x00, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x02, 0x02, 0x12, 0x03, 0x1d,
    0x07, 0x24, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x02, 0x02, 0x00, 0x12, 0x03, 0x1d, 0x07,
    0x24, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x07,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x02, 0x03, 0x12, 0x03, 0x1d, 0x27, 0x2b, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x1e, 0x08, 0x13, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12,
    0x03, 0x20, 0x07, 0x12, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x21, 0x07, 0x14, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x22, 0x07, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00,
    0x12, 0x04, 0x24, 0x00, 0x2b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x24,
    0x05, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x25, 0x02, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x25, 0x02, 0x0e, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x25, 0x11, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x26, 0x02, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x26, 0x02, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12,
    0x03, 0x26, 0x10, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x27, 0x02,
    0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x27, 0x02, 0x0e, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x27, 0x11, 0x14, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x28, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x28, 0x02, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03,
    0x02, 0x12, 0x03, 0x28, 0x11, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03,
    0x29, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x29, 0x02,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x29, 0x13, 0x16, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x05, 0x12, 0x03, 0x2a, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x2a, 0x02, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x05, 0x02, 0x12, 0x03, 0x2a, 0x11, 0x14, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x2d, 0x00, 0x30, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x2d, 0x08, 0x12,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x2e, 0x02, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2e, 0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x2e, 0x15, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x2e, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x2f,
    0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2f, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2f, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2f, 0x11, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2f, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01,
    0x12, 0x04, 0x32, 0x00, 0x35, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x32,
    0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x33, 0x02, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x33, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x33, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x33, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x33, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12,
    0x03, 0x34, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x34,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x06, 0x12, 0x03, 0x34, 0x0b, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x34, 0x16, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x34, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02,
    0x05, 0x01, 0x12, 0x04, 0x37, 0x00, 0x3b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12,
    0x03, 0x37, 0x05, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00, 0x12, 0x03, 0x38, 0x02,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x38, 0x02, 0x0d, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x38, 0x10, 0x13, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x01, 0x02, 0x01, 0x12, 0x03, 0x39, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x39, 0x02, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01,
    0x02, 0x12, 0x03, 0x39, 0x15, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x02, 0x12, 0x03,
    0x3a, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3a, 0x02,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x02, 0x12, 0x03, 0x3a, 0x13, 0x16, 0x0a,
    0x0a, 0x0a, 0x02, 0x05, 0x02, 0x12, 0x04, 0x3d, 0x00, 0x44, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05,
    0x02, 0x01, 0x12, 0x03, 0x3d, 0x05, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x00, 0x12,
    0x03, 0x3e, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3e,
    0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x00, 0x02, 0x12, 0x03, 0x3e, 0x14, 0x17,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x3f, 0x02, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3f, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x02, 0x02, 0x01, 0x02, 0x12, 0x03, 0x3f, 0x1a, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02,
    0x02, 0x12, 0x03, 0x40, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x40, 0x02, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x02, 0x02, 0x12, 0x03, 0x40,
    0x14, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x41, 0x02, 0x18, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x41, 0x02, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x02, 0x02, 0x03, 0x02, 0x12, 0x03, 0x41, 0x14, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x42, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x04,
    0x01, 0x12, 0x03, 0x42, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x04, 0x02, 0x12,
    0x03, 0x42, 0x13, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x43, 0x02,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x05, 0x01, 0x12, 0x03, 0x43, 0x02, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x05, 0x02, 0x12, 0x03, 0x43, 0x15, 0x18, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x02, 0x12, 0x04, 0x46, 0x00, 0x51, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01,
    0x12, 0x03, 0x46, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x47,
    0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x47, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x47, 0x0b, 0x14, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x47, 0x15, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x47, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x48, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x48, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x48, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x48, 0x12,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x48, 0x19, 0x1a, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x49, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x49, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x49, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x49, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x49, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x4a, 0x02,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x4a, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x4a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x4a, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x4a, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x4b, 0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x04, 0x12,
    0x03, 0x4b, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x05, 0x12, 0x03, 0x4b,
    0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03, 0x4b, 0x12, 0x1b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x4b, 0x1e, 0x1f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x4c, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x05, 0x04, 0x12, 0x03, 0x4c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x05, 0x06, 0x12, 0x03, 0x4c, 0x0b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x01,
    0x12, 0x03, 0x4c, 0x1d, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x03, 0x12, 0x03,
    0x4c, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x06, 0x12, 0x03, 0x4d, 0x02, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x04, 0x12, 0x03, 0x4d, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x06, 0x05, 0x12, 0x03, 0x4d, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x06, 0x01, 0x12, 0x03, 0x4d, 0x11, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x06, 0x03, 0x12, 0x03, 0x4d, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x07,
    0x12, 0x03, 0x4e, 0x02, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x04, 0x12, 0x03,
    0x4e, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x05, 0x12, 0x03, 0x4e, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x01, 0x12, 0x03, 0x4e, 0x12, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x07, 0x03, 0x12, 0x03, 0x4e, 0x22, 0x23, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x08, 0x12, 0x03, 0x4f, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x08, 0x04, 0x12, 0x03, 0x4f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08,
    0x05, 0x12, 0x03, 0x4f, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x01, 0x12,
    0x03, 0x4f, 0x10, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x08, 0x03, 0x12, 0x03, 0x4f,
    0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x09, 0x12, 0x03, 0x50, 0x02, 0x33, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x04, 0x12, 0x03, 0x50, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x09, 0x05, 0x12, 0x03, 0x50, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x09, 0x01, 0x12, 0x03, 0x50, 0x11, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x09, 0x03, 0x12, 0x03, 0x50, 0x24, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x08,
    0x12, 0x03, 0x50, 0x27, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x09, 0x07, 0x12, 0x03,
    0x50, 0x30, 0x31, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x53, 0x00, 0x57, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x53, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x03, 0x02, 0x00, 0x12, 0x03, 0x54, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x54, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x54, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x54,
    0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x54, 0x19, 0x1a,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x55, 0x02, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x55, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x55, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x55, 0x11, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x55, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x56,
    0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x56, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x56, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x56, 0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x56, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04,
    0x12, 0x04, 0x59, 0x00, 0x5c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x59,
    0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x5a, 0x02, 0x1b, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x5a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5a, 0x12, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x5a, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12,
    0x03, 0x5b, 0x02, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x5b,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x5b, 0x0b, 0x0f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5b, 0x10, 0x18, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5b, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x08, 0x12, 0x03, 0x5b, 0x1d, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x07, 0x12, 0x03, 0x5b, 0x28, 0x2d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04,
    0x5e, 0x00, 0x62, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x5e, 0x08, 0x18,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x5f, 0x02, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x5f, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x5f, 0x12, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x5f, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x60,
    0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x60, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x60, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x60, 0x12, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x60, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x02, 0x12, 0x03, 0x61, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x61, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x61, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x61, 0x11,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x61, 0x1d, 0x1e, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x64, 0x00, 0x70, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x06, 0x01, 0x12, 0x03, 0x64, 0x08, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12,
    0x03, 0x65, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x65,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05, 0x12, 0x03, 0x65, 0x0b, 0x11,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x65, 0x12, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x65, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x66, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x66, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x66, 0x0b, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x66, 0x1e, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x66, 0x25,
    0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x67, 0x02, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x04, 0x12, 0x03, 0x67, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x02, 0x05, 0x12, 0x03, 0x67, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x67, 0x11, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x67, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x03, 0x12, 0x03,
    0x68, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x04, 0x12, 0x03, 0x68, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x05, 0x12, 0x03, 0x68, 0x0b, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x03, 0x01, 0x12, 0x03, 0x68, 0x11, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x03, 0x03, 0x12, 0x03, 0x68, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x04, 0x12, 0x03, 0x69, 0x02, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04,
    0x04, 0x12, 0x03, 0x69, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x05, 0x12,
    0x03, 0x69, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x01, 0x12, 0x03, 0x69,
    0x11, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x04, 0x03, 0x12, 0x03, 0x69, 0x1f, 0x20,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x05, 0x12, 0x03, 0x6a, 0x02, 0x20, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x05, 0x04, 0x12, 0x03, 0x6a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x05, 0x05, 0x12, 0x03, 0x6a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x05, 0x01, 0x12, 0x03, 0x6a, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x05, 0x03,
    0x12, 0x03, 0x6a, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x06, 0x12, 0x03, 0x6b,
    0x02, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x04, 0x12, 0x03, 0x6b, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x05, 0x12, 0x03, 0x6b, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x06, 0x01, 0x12, 0x03, 0x6b, 0x12, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x06, 0x03, 0x12, 0x03, 0x6b, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06,
    0x02, 0x07, 0x12, 0x03, 0x6c, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x04,
    0x12, 0x03, 0x6c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x06, 0x12, 0x03,
    0x6c, 0x0b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x01, 0x12, 0x03, 0x6c, 0x1d,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x07, 0x03, 0x12, 0x03, 0x6c, 0x25, 0x26, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x08, 0x12, 0x03, 0x6d, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x08, 0x04, 0x12, 0x03, 0x6d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x08, 0x06, 0x12, 0x03, 0x6d, 0x0b, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08,
    0x01, 0x12, 0x03, 0x6d, 0x19, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x08, 0x03, 0x12,
    0x03, 0x6d, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x09, 0x12, 0x03, 0x6e, 0x02,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x09, 0x04, 0x12, 0x03, 0x6e, 0x02, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x09, 0x06, 0x12, 0x03, 0x6e, 0x0b, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x09, 0x01, 0x12, 0x03, 0x6e, 0x16, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x09, 0x03, 0x12, 0x03, 0x6e, 0x1f, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02,
    0x0a, 0x12, 0x03, 0x6f, 0x02, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0a, 0x04, 0x12,
    0x03, 0x6f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0a, 0x05, 0x12, 0x03, 0x6f,
    0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x6f, 0x10, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x0a, 0x03, 0x12, 0x03, 0x6f, 0x20, 0x22, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x72, 0x00, 0x75, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07,
    0x01, 0x12, 0x03, 0x72, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03,
    0x73, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x73, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x73, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x73, 0x12, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x73, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x07, 0x02, 0x01, 0x12, 0x03, 0x74, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x74, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x74, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x74,
    0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x74, 0x1d, 0x1e,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x77, 0x00, 0x7d, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x08, 0x01, 0x12, 0x03, 0x77, 0x08, 0x17, 0x0a, 0x19, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00,
    0x12, 0x03, 0x78, 0x02, 0x21, 0x22, 0x0c, 0x20, 0x64, 0x65, 0x70, 0x72, 0x65, 0x63, 0x61, 0x74,
    0x65, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x78, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x06, 0x12, 0x03, 0x78, 0x0b, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x78, 0x16, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x78, 0x1f, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x08, 0x02, 0x01, 0x12, 0x03, 0x79, 0x02, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x79, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x79, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x79,
    0x11, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x79, 0x1d, 0x1e,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x03, 0x7a, 0x02, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x02, 0x04, 0x12, 0x03, 0x7a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x02, 0x05, 0x12, 0x03, 0x7a, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x7a, 0x11, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x7a, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x03, 0x12, 0x03, 0x7b,
    0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x04, 0x12, 0x03, 0x7b, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x05, 0x12, 0x03, 0x7b, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x03, 0x01, 0x12, 0x03, 0x7b, 0x11, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x03, 0x03, 0x12, 0x03, 0x7b, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08,
    0x02, 0x04, 0x12, 0x03, 0x7c, 0x02, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x04,
    0x12, 0x03, 0x7c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x06, 0x12, 0x03,
    0x7c, 0x0b, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x01, 0x12, 0x03, 0x7c, 0x1b,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x04, 0x03, 0x12, 0x03, 0x7c, 0x23, 0x24,
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
