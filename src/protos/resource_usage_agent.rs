// This file is generated by rust-protobuf 2.8.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `resource_usage_agent.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_8_0;

#[derive(PartialEq,Clone,Default)]
pub struct ResourceMeteringRequest {
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ResourceMeteringRequest {
    fn default() -> &'a ResourceMeteringRequest {
        <ResourceMeteringRequest as ::protobuf::Message>::default_instance()
    }
}

impl ResourceMeteringRequest {
    pub fn new() -> ResourceMeteringRequest {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for ResourceMeteringRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> ResourceMeteringRequest {
        ResourceMeteringRequest::new()
    }

    fn default_instance() -> &'static ResourceMeteringRequest {
        static mut instance: ::protobuf::lazy::Lazy<ResourceMeteringRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResourceMeteringRequest,
        };
        unsafe {
            instance.get(ResourceMeteringRequest::new)
        }
    }
}

impl ::protobuf::Clear for ResourceMeteringRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::protobuf::PbPrint for ResourceMeteringRequest {
    #[allow(unused_variables)]
    fn fmt(&self, name: &str, buf: &mut String) {
    }
}
impl ::std::fmt::Debug for ResourceMeteringRequest {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        Ok(())
    }
}

impl ::protobuf::reflect::ProtobufValue for ResourceMeteringRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EmptyResponse {
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a EmptyResponse {
    fn default() -> &'a EmptyResponse {
        <EmptyResponse as ::protobuf::Message>::default_instance()
    }
}

impl EmptyResponse {
    pub fn new() -> EmptyResponse {
        ::std::default::Default::default()
    }
}

impl ::protobuf::Message for EmptyResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> EmptyResponse {
        EmptyResponse::new()
    }

    fn default_instance() -> &'static EmptyResponse {
        static mut instance: ::protobuf::lazy::Lazy<EmptyResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EmptyResponse,
        };
        unsafe {
            instance.get(EmptyResponse::new)
        }
    }
}

impl ::protobuf::Clear for EmptyResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::protobuf::PbPrint for EmptyResponse {
    #[allow(unused_variables)]
    fn fmt(&self, name: &str, buf: &mut String) {
    }
}
impl ::std::fmt::Debug for EmptyResponse {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        Ok(())
    }
}

impl ::protobuf::reflect::ProtobufValue for EmptyResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResourceUsageRecord {
    // message oneof groups
    pub record_oneof: ::std::option::Option<ResourceUsageRecord_oneof_record_oneof>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a ResourceUsageRecord {
    fn default() -> &'a ResourceUsageRecord {
        <ResourceUsageRecord as ::protobuf::Message>::default_instance()
    }
}

#[derive(Clone,PartialEq)]
pub enum ResourceUsageRecord_oneof_record_oneof {
    Record(GroupTagRecord),
}

impl ::protobuf::PbPrint for ResourceUsageRecord_oneof_record_oneof {
    fn fmt(&self, name: &str, buf: &mut String) {
        match self {
            ResourceUsageRecord_oneof_record_oneof::Record(v) => ::protobuf::PbPrint::fmt(v, name, buf),
        }
    }
}

impl ResourceUsageRecord {
    pub fn new() -> ResourceUsageRecord {
        ::std::default::Default::default()
    }

    // .resource_usage_agent.GroupTagRecord record = 1;


    pub fn get_record(&self) -> &GroupTagRecord {
        match self.record_oneof {
            ::std::option::Option::Some(ResourceUsageRecord_oneof_record_oneof::Record(ref v)) => v,
            _ => GroupTagRecord::default_instance(),
        }
    }
    pub fn clear_record(&mut self) {
        self.record_oneof = ::std::option::Option::None;
    }

    pub fn has_record(&self) -> bool {
        match self.record_oneof {
            ::std::option::Option::Some(ResourceUsageRecord_oneof_record_oneof::Record(..)) => true,
            _ => false,
        }
    }

    // Param is passed by value, moved
    pub fn set_record(&mut self, v: GroupTagRecord) {
        self.record_oneof = ::std::option::Option::Some(ResourceUsageRecord_oneof_record_oneof::Record(v))
    }

    // Mutable pointer to the field.
    pub fn mut_record(&mut self) -> &mut GroupTagRecord {
        if let ::std::option::Option::Some(ResourceUsageRecord_oneof_record_oneof::Record(_)) = self.record_oneof {
        } else {
            self.record_oneof = ::std::option::Option::Some(ResourceUsageRecord_oneof_record_oneof::Record(GroupTagRecord::new()));
        }
        match self.record_oneof {
            ::std::option::Option::Some(ResourceUsageRecord_oneof_record_oneof::Record(ref mut v)) => v,
            _ => panic!(),
        }
    }

    // Take field
    pub fn take_record(&mut self) -> GroupTagRecord {
        if self.has_record() {
            match self.record_oneof.take() {
                ::std::option::Option::Some(ResourceUsageRecord_oneof_record_oneof::Record(v)) => v,
                _ => panic!(),
            }
        } else {
            GroupTagRecord::new()
        }
    }
}

impl ::protobuf::Message for ResourceUsageRecord {
    fn is_initialized(&self) -> bool {
        if let Some(ResourceUsageRecord_oneof_record_oneof::Record(ref v)) = self.record_oneof {
            if !v.is_initialized() {
                return false;
            }
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeLengthDelimited {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.record_oneof = ::std::option::Option::Some(ResourceUsageRecord_oneof_record_oneof::Record(is.read_message()?));
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
        if let ::std::option::Option::Some(ref v) = self.record_oneof {
            match v {
                &ResourceUsageRecord_oneof_record_oneof::Record(ref v) => {
                    let len = v.compute_size();
                    my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
                },
            };
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let ::std::option::Option::Some(ref v) = self.record_oneof {
            match v {
                &ResourceUsageRecord_oneof_record_oneof::Record(ref v) => {
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
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> ResourceUsageRecord {
        ResourceUsageRecord::new()
    }

    fn default_instance() -> &'static ResourceUsageRecord {
        static mut instance: ::protobuf::lazy::Lazy<ResourceUsageRecord> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResourceUsageRecord,
        };
        unsafe {
            instance.get(ResourceUsageRecord::new)
        }
    }
}

impl ::protobuf::Clear for ResourceUsageRecord {
    fn clear(&mut self) {
        self.record_oneof = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::protobuf::PbPrint for ResourceUsageRecord {
    #[allow(unused_variables)]
    fn fmt(&self, name: &str, buf: &mut String) {
        ::protobuf::push_message_start(name, buf);
        let old_len = buf.len();
        ::protobuf::PbPrint::fmt(&self.record_oneof, "record_oneof", buf);
        if old_len < buf.len() {
          buf.push(' ');
        }
        buf.push('}');
    }
}
impl ::std::fmt::Debug for ResourceUsageRecord {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut s = String::new();
        ::protobuf::PbPrint::fmt(&self.record_oneof, "record_oneof", &mut s);
        write!(f, "{}", s)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResourceUsageRecord {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GroupTagRecord {
    // message fields
    pub resource_group_tag: ::std::vec::Vec<u8>,
    pub items: ::protobuf::RepeatedField<GroupTagRecordItem>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a GroupTagRecord {
    fn default() -> &'a GroupTagRecord {
        <GroupTagRecord as ::protobuf::Message>::default_instance()
    }
}

impl GroupTagRecord {
    pub fn new() -> GroupTagRecord {
        ::std::default::Default::default()
    }

    // bytes resource_group_tag = 1;


    pub fn get_resource_group_tag(&self) -> &[u8] {
        &self.resource_group_tag
    }
    pub fn clear_resource_group_tag(&mut self) {
        self.resource_group_tag.clear();
    }

    // Param is passed by value, moved
    pub fn set_resource_group_tag(&mut self, v: ::std::vec::Vec<u8>) {
        self.resource_group_tag = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_resource_group_tag(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.resource_group_tag
    }

    // Take field
    pub fn take_resource_group_tag(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.resource_group_tag, ::std::vec::Vec::new())
    }

    // repeated .resource_usage_agent.GroupTagRecordItem items = 2;


    pub fn get_items(&self) -> &[GroupTagRecordItem] {
        &self.items
    }
    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<GroupTagRecordItem>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<GroupTagRecordItem> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<GroupTagRecordItem> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for GroupTagRecord {
    fn is_initialized(&self) -> bool {
        for v in &self.items {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.resource_group_tag)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.items)?;
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
        if !self.resource_group_tag.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.resource_group_tag);
        }
        for value in &self.items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.resource_group_tag.is_empty() {
            os.write_bytes(1, &self.resource_group_tag)?;
        }
        for v in &self.items {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
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
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> GroupTagRecord {
        GroupTagRecord::new()
    }

    fn default_instance() -> &'static GroupTagRecord {
        static mut instance: ::protobuf::lazy::Lazy<GroupTagRecord> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GroupTagRecord,
        };
        unsafe {
            instance.get(GroupTagRecord::new)
        }
    }
}

impl ::protobuf::Clear for GroupTagRecord {
    fn clear(&mut self) {
        self.resource_group_tag.clear();
        self.items.clear();
        self.unknown_fields.clear();
    }
}

impl ::protobuf::PbPrint for GroupTagRecord {
    #[allow(unused_variables)]
    fn fmt(&self, name: &str, buf: &mut String) {
        ::protobuf::push_message_start(name, buf);
        let old_len = buf.len();
        ::protobuf::PbPrint::fmt(&self.resource_group_tag, "resource_group_tag", buf);
        ::protobuf::PbPrint::fmt(&self.items, "items", buf);
        if old_len < buf.len() {
          buf.push(' ');
        }
        buf.push('}');
    }
}
impl ::std::fmt::Debug for GroupTagRecord {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut s = String::new();
        ::protobuf::PbPrint::fmt(&self.resource_group_tag, "resource_group_tag", &mut s);
        ::protobuf::PbPrint::fmt(&self.items, "items", &mut s);
        write!(f, "{}", s)
    }
}

impl ::protobuf::reflect::ProtobufValue for GroupTagRecord {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GroupTagRecordItem {
    // message fields
    pub timestamp_sec: u64,
    pub cpu_time_ms: u32,
    pub read_keys: u32,
    pub write_keys: u32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a GroupTagRecordItem {
    fn default() -> &'a GroupTagRecordItem {
        <GroupTagRecordItem as ::protobuf::Message>::default_instance()
    }
}

impl GroupTagRecordItem {
    pub fn new() -> GroupTagRecordItem {
        ::std::default::Default::default()
    }

    // uint64 timestamp_sec = 1;


    pub fn get_timestamp_sec(&self) -> u64 {
        self.timestamp_sec
    }
    pub fn clear_timestamp_sec(&mut self) {
        self.timestamp_sec = 0;
    }

    // Param is passed by value, moved
    pub fn set_timestamp_sec(&mut self, v: u64) {
        self.timestamp_sec = v;
    }

    // uint32 cpu_time_ms = 2;


    pub fn get_cpu_time_ms(&self) -> u32 {
        self.cpu_time_ms
    }
    pub fn clear_cpu_time_ms(&mut self) {
        self.cpu_time_ms = 0;
    }

    // Param is passed by value, moved
    pub fn set_cpu_time_ms(&mut self, v: u32) {
        self.cpu_time_ms = v;
    }

    // uint32 read_keys = 3;


    pub fn get_read_keys(&self) -> u32 {
        self.read_keys
    }
    pub fn clear_read_keys(&mut self) {
        self.read_keys = 0;
    }

    // Param is passed by value, moved
    pub fn set_read_keys(&mut self, v: u32) {
        self.read_keys = v;
    }

    // uint32 write_keys = 4;


    pub fn get_write_keys(&self) -> u32 {
        self.write_keys
    }
    pub fn clear_write_keys(&mut self) {
        self.write_keys = 0;
    }

    // Param is passed by value, moved
    pub fn set_write_keys(&mut self, v: u32) {
        self.write_keys = v;
    }
}

impl ::protobuf::Message for GroupTagRecordItem {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.timestamp_sec = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.cpu_time_ms = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.read_keys = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.write_keys = tmp;
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
        if self.timestamp_sec != 0 {
            my_size += ::protobuf::rt::value_size(1, self.timestamp_sec, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.cpu_time_ms != 0 {
            my_size += ::protobuf::rt::value_size(2, self.cpu_time_ms, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.read_keys != 0 {
            my_size += ::protobuf::rt::value_size(3, self.read_keys, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.write_keys != 0 {
            my_size += ::protobuf::rt::value_size(4, self.write_keys, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.timestamp_sec != 0 {
            os.write_uint64(1, self.timestamp_sec)?;
        }
        if self.cpu_time_ms != 0 {
            os.write_uint32(2, self.cpu_time_ms)?;
        }
        if self.read_keys != 0 {
            os.write_uint32(3, self.read_keys)?;
        }
        if self.write_keys != 0 {
            os.write_uint32(4, self.write_keys)?;
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
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> GroupTagRecordItem {
        GroupTagRecordItem::new()
    }

    fn default_instance() -> &'static GroupTagRecordItem {
        static mut instance: ::protobuf::lazy::Lazy<GroupTagRecordItem> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GroupTagRecordItem,
        };
        unsafe {
            instance.get(GroupTagRecordItem::new)
        }
    }
}

impl ::protobuf::Clear for GroupTagRecordItem {
    fn clear(&mut self) {
        self.timestamp_sec = 0;
        self.cpu_time_ms = 0;
        self.read_keys = 0;
        self.write_keys = 0;
        self.unknown_fields.clear();
    }
}

impl ::protobuf::PbPrint for GroupTagRecordItem {
    #[allow(unused_variables)]
    fn fmt(&self, name: &str, buf: &mut String) {
        ::protobuf::push_message_start(name, buf);
        let old_len = buf.len();
        ::protobuf::PbPrint::fmt(&self.timestamp_sec, "timestamp_sec", buf);
        ::protobuf::PbPrint::fmt(&self.cpu_time_ms, "cpu_time_ms", buf);
        ::protobuf::PbPrint::fmt(&self.read_keys, "read_keys", buf);
        ::protobuf::PbPrint::fmt(&self.write_keys, "write_keys", buf);
        if old_len < buf.len() {
          buf.push(' ');
        }
        buf.push('}');
    }
}
impl ::std::fmt::Debug for GroupTagRecordItem {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let mut s = String::new();
        ::protobuf::PbPrint::fmt(&self.timestamp_sec, "timestamp_sec", &mut s);
        ::protobuf::PbPrint::fmt(&self.cpu_time_ms, "cpu_time_ms", &mut s);
        ::protobuf::PbPrint::fmt(&self.read_keys, "read_keys", &mut s);
        ::protobuf::PbPrint::fmt(&self.write_keys, "write_keys", &mut s);
        write!(f, "{}", s)
    }
}

impl ::protobuf::reflect::ProtobufValue for GroupTagRecordItem {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}
pub use super::resource_usage_agent_grpc::*;
