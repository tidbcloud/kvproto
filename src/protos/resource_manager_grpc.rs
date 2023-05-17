// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

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

const METHOD_RESOURCE_MANAGER_LIST_RESOURCE_GROUPS: ::grpcio::Method<super::resource_manager::ListResourceGroupsRequest, super::resource_manager::ListResourceGroupsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/resource_manager.ResourceManager/ListResourceGroups",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_RESOURCE_MANAGER_GET_RESOURCE_GROUP: ::grpcio::Method<super::resource_manager::GetResourceGroupRequest, super::resource_manager::GetResourceGroupResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/resource_manager.ResourceManager/GetResourceGroup",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_RESOURCE_MANAGER_ADD_RESOURCE_GROUP: ::grpcio::Method<super::resource_manager::PutResourceGroupRequest, super::resource_manager::PutResourceGroupResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/resource_manager.ResourceManager/AddResourceGroup",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_RESOURCE_MANAGER_MODIFY_RESOURCE_GROUP: ::grpcio::Method<super::resource_manager::PutResourceGroupRequest, super::resource_manager::PutResourceGroupResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/resource_manager.ResourceManager/ModifyResourceGroup",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_RESOURCE_MANAGER_DELETE_RESOURCE_GROUP: ::grpcio::Method<super::resource_manager::DeleteResourceGroupRequest, super::resource_manager::DeleteResourceGroupResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/resource_manager.ResourceManager/DeleteResourceGroup",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_RESOURCE_MANAGER_ACQUIRE_TOKEN_BUCKETS: ::grpcio::Method<super::resource_manager::TokenBucketsRequest, super::resource_manager::TokenBucketsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/resource_manager.ResourceManager/AcquireTokenBuckets",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ResourceManagerClient {
    pub client: ::grpcio::Client,
}

impl ResourceManagerClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ResourceManagerClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn list_resource_groups_opt(&self, req: &super::resource_manager::ListResourceGroupsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::resource_manager::ListResourceGroupsResponse> {
        self.client.unary_call(&METHOD_RESOURCE_MANAGER_LIST_RESOURCE_GROUPS, req, opt)
    }

    pub fn list_resource_groups(&self, req: &super::resource_manager::ListResourceGroupsRequest) -> ::grpcio::Result<super::resource_manager::ListResourceGroupsResponse> {
        self.list_resource_groups_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_resource_groups_async_opt(&self, req: &super::resource_manager::ListResourceGroupsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::resource_manager::ListResourceGroupsResponse>> {
        self.client.unary_call_async(&METHOD_RESOURCE_MANAGER_LIST_RESOURCE_GROUPS, req, opt)
    }

    pub fn list_resource_groups_async(&self, req: &super::resource_manager::ListResourceGroupsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::resource_manager::ListResourceGroupsResponse>> {
        self.list_resource_groups_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_resource_group_opt(&self, req: &super::resource_manager::GetResourceGroupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::resource_manager::GetResourceGroupResponse> {
        self.client.unary_call(&METHOD_RESOURCE_MANAGER_GET_RESOURCE_GROUP, req, opt)
    }

    pub fn get_resource_group(&self, req: &super::resource_manager::GetResourceGroupRequest) -> ::grpcio::Result<super::resource_manager::GetResourceGroupResponse> {
        self.get_resource_group_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_resource_group_async_opt(&self, req: &super::resource_manager::GetResourceGroupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::resource_manager::GetResourceGroupResponse>> {
        self.client.unary_call_async(&METHOD_RESOURCE_MANAGER_GET_RESOURCE_GROUP, req, opt)
    }

    pub fn get_resource_group_async(&self, req: &super::resource_manager::GetResourceGroupRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::resource_manager::GetResourceGroupResponse>> {
        self.get_resource_group_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn add_resource_group_opt(&self, req: &super::resource_manager::PutResourceGroupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::resource_manager::PutResourceGroupResponse> {
        self.client.unary_call(&METHOD_RESOURCE_MANAGER_ADD_RESOURCE_GROUP, req, opt)
    }

    pub fn add_resource_group(&self, req: &super::resource_manager::PutResourceGroupRequest) -> ::grpcio::Result<super::resource_manager::PutResourceGroupResponse> {
        self.add_resource_group_opt(req, ::grpcio::CallOption::default())
    }

    pub fn add_resource_group_async_opt(&self, req: &super::resource_manager::PutResourceGroupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::resource_manager::PutResourceGroupResponse>> {
        self.client.unary_call_async(&METHOD_RESOURCE_MANAGER_ADD_RESOURCE_GROUP, req, opt)
    }

    pub fn add_resource_group_async(&self, req: &super::resource_manager::PutResourceGroupRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::resource_manager::PutResourceGroupResponse>> {
        self.add_resource_group_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn modify_resource_group_opt(&self, req: &super::resource_manager::PutResourceGroupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::resource_manager::PutResourceGroupResponse> {
        self.client.unary_call(&METHOD_RESOURCE_MANAGER_MODIFY_RESOURCE_GROUP, req, opt)
    }

    pub fn modify_resource_group(&self, req: &super::resource_manager::PutResourceGroupRequest) -> ::grpcio::Result<super::resource_manager::PutResourceGroupResponse> {
        self.modify_resource_group_opt(req, ::grpcio::CallOption::default())
    }

    pub fn modify_resource_group_async_opt(&self, req: &super::resource_manager::PutResourceGroupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::resource_manager::PutResourceGroupResponse>> {
        self.client.unary_call_async(&METHOD_RESOURCE_MANAGER_MODIFY_RESOURCE_GROUP, req, opt)
    }

    pub fn modify_resource_group_async(&self, req: &super::resource_manager::PutResourceGroupRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::resource_manager::PutResourceGroupResponse>> {
        self.modify_resource_group_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_resource_group_opt(&self, req: &super::resource_manager::DeleteResourceGroupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::resource_manager::DeleteResourceGroupResponse> {
        self.client.unary_call(&METHOD_RESOURCE_MANAGER_DELETE_RESOURCE_GROUP, req, opt)
    }

    pub fn delete_resource_group(&self, req: &super::resource_manager::DeleteResourceGroupRequest) -> ::grpcio::Result<super::resource_manager::DeleteResourceGroupResponse> {
        self.delete_resource_group_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_resource_group_async_opt(&self, req: &super::resource_manager::DeleteResourceGroupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::resource_manager::DeleteResourceGroupResponse>> {
        self.client.unary_call_async(&METHOD_RESOURCE_MANAGER_DELETE_RESOURCE_GROUP, req, opt)
    }

    pub fn delete_resource_group_async(&self, req: &super::resource_manager::DeleteResourceGroupRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::resource_manager::DeleteResourceGroupResponse>> {
        self.delete_resource_group_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn acquire_token_buckets_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::resource_manager::TokenBucketsRequest>, ::grpcio::ClientDuplexReceiver<super::resource_manager::TokenBucketsResponse>)> {
        self.client.duplex_streaming(&METHOD_RESOURCE_MANAGER_ACQUIRE_TOKEN_BUCKETS, opt)
    }

    pub fn acquire_token_buckets(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::resource_manager::TokenBucketsRequest>, ::grpcio::ClientDuplexReceiver<super::resource_manager::TokenBucketsResponse>)> {
        self.acquire_token_buckets_opt(::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait ResourceManager {
    fn list_resource_groups(&mut self, ctx: ::grpcio::RpcContext, _req: super::resource_manager::ListResourceGroupsRequest, sink: ::grpcio::UnarySink<super::resource_manager::ListResourceGroupsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_resource_group(&mut self, ctx: ::grpcio::RpcContext, _req: super::resource_manager::GetResourceGroupRequest, sink: ::grpcio::UnarySink<super::resource_manager::GetResourceGroupResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn add_resource_group(&mut self, ctx: ::grpcio::RpcContext, _req: super::resource_manager::PutResourceGroupRequest, sink: ::grpcio::UnarySink<super::resource_manager::PutResourceGroupResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn modify_resource_group(&mut self, ctx: ::grpcio::RpcContext, _req: super::resource_manager::PutResourceGroupRequest, sink: ::grpcio::UnarySink<super::resource_manager::PutResourceGroupResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn delete_resource_group(&mut self, ctx: ::grpcio::RpcContext, _req: super::resource_manager::DeleteResourceGroupRequest, sink: ::grpcio::UnarySink<super::resource_manager::DeleteResourceGroupResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn acquire_token_buckets(&mut self, ctx: ::grpcio::RpcContext, _stream: ::grpcio::RequestStream<super::resource_manager::TokenBucketsRequest>, sink: ::grpcio::DuplexSink<super::resource_manager::TokenBucketsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_resource_manager<S: ResourceManager + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_RESOURCE_MANAGER_LIST_RESOURCE_GROUPS, move |ctx, req, resp| {
        instance.list_resource_groups(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_RESOURCE_MANAGER_GET_RESOURCE_GROUP, move |ctx, req, resp| {
        instance.get_resource_group(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_RESOURCE_MANAGER_ADD_RESOURCE_GROUP, move |ctx, req, resp| {
        instance.add_resource_group(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_RESOURCE_MANAGER_MODIFY_RESOURCE_GROUP, move |ctx, req, resp| {
        instance.modify_resource_group(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_RESOURCE_MANAGER_DELETE_RESOURCE_GROUP, move |ctx, req, resp| {
        instance.delete_resource_group(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_duplex_streaming_handler(&METHOD_RESOURCE_MANAGER_ACQUIRE_TOKEN_BUCKETS, move |ctx, req, resp| {
        instance.acquire_token_buckets(ctx, req, resp)
    });
    builder.build()
}
