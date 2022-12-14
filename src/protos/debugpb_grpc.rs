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

const METHOD_DEBUG_GET: ::grpcio::Method<super::debugpb::GetRequest, super::debugpb::GetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/debugpb.Debug/Get",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DEBUG_RAFT_LOG: ::grpcio::Method<super::debugpb::RaftLogRequest, super::debugpb::RaftLogResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/debugpb.Debug/RaftLog",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DEBUG_REGION_INFO: ::grpcio::Method<super::debugpb::RegionInfoRequest, super::debugpb::RegionInfoResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/debugpb.Debug/RegionInfo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DEBUG_REGION_SIZE: ::grpcio::Method<super::debugpb::RegionSizeRequest, super::debugpb::RegionSizeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/debugpb.Debug/RegionSize",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DEBUG_SCAN_MVCC: ::grpcio::Method<super::debugpb::ScanMvccRequest, super::debugpb::ScanMvccResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/debugpb.Debug/ScanMvcc",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DEBUG_COMPACT: ::grpcio::Method<super::debugpb::CompactRequest, super::debugpb::CompactResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/debugpb.Debug/Compact",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DEBUG_INJECT_FAIL_POINT: ::grpcio::Method<super::debugpb::InjectFailPointRequest, super::debugpb::InjectFailPointResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/debugpb.Debug/InjectFailPoint",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DEBUG_RECOVER_FAIL_POINT: ::grpcio::Method<super::debugpb::RecoverFailPointRequest, super::debugpb::RecoverFailPointResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/debugpb.Debug/RecoverFailPoint",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DEBUG_LIST_FAIL_POINTS: ::grpcio::Method<super::debugpb::ListFailPointsRequest, super::debugpb::ListFailPointsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/debugpb.Debug/ListFailPoints",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DEBUG_GET_METRICS: ::grpcio::Method<super::debugpb::GetMetricsRequest, super::debugpb::GetMetricsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/debugpb.Debug/GetMetrics",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DEBUG_CHECK_REGION_CONSISTENCY: ::grpcio::Method<super::debugpb::RegionConsistencyCheckRequest, super::debugpb::RegionConsistencyCheckResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/debugpb.Debug/CheckRegionConsistency",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DEBUG_MODIFY_TIKV_CONFIG: ::grpcio::Method<super::debugpb::ModifyTikvConfigRequest, super::debugpb::ModifyTikvConfigResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/debugpb.Debug/ModifyTikvConfig",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DEBUG_GET_REGION_PROPERTIES: ::grpcio::Method<super::debugpb::GetRegionPropertiesRequest, super::debugpb::GetRegionPropertiesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/debugpb.Debug/GetRegionProperties",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DEBUG_GET_STORE_INFO: ::grpcio::Method<super::debugpb::GetStoreInfoRequest, super::debugpb::GetStoreInfoResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/debugpb.Debug/GetStoreInfo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DEBUG_GET_CLUSTER_INFO: ::grpcio::Method<super::debugpb::GetClusterInfoRequest, super::debugpb::GetClusterInfoResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/debugpb.Debug/GetClusterInfo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DEBUG_GET_ALL_REGIONS_IN_STORE: ::grpcio::Method<super::debugpb::GetAllRegionsInStoreRequest, super::debugpb::GetAllRegionsInStoreResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/debugpb.Debug/GetAllRegionsInStore",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_DEBUG_RESET_TO_VERSION: ::grpcio::Method<super::debugpb::ResetToVersionRequest, super::debugpb::ResetToVersionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/debugpb.Debug/ResetToVersion",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct DebugClient {
    client: ::grpcio::Client,
}

impl DebugClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        DebugClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_opt(&self, req: &super::debugpb::GetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::debugpb::GetResponse> {
        self.client.unary_call(&METHOD_DEBUG_GET, req, opt)
    }

    pub fn get(&self, req: &super::debugpb::GetRequest) -> ::grpcio::Result<super::debugpb::GetResponse> {
        self.get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_async_opt(&self, req: &super::debugpb::GetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::GetResponse>> {
        self.client.unary_call_async(&METHOD_DEBUG_GET, req, opt)
    }

    pub fn get_async(&self, req: &super::debugpb::GetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::GetResponse>> {
        self.get_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raft_log_opt(&self, req: &super::debugpb::RaftLogRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::debugpb::RaftLogResponse> {
        self.client.unary_call(&METHOD_DEBUG_RAFT_LOG, req, opt)
    }

    pub fn raft_log(&self, req: &super::debugpb::RaftLogRequest) -> ::grpcio::Result<super::debugpb::RaftLogResponse> {
        self.raft_log_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raft_log_async_opt(&self, req: &super::debugpb::RaftLogRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::RaftLogResponse>> {
        self.client.unary_call_async(&METHOD_DEBUG_RAFT_LOG, req, opt)
    }

    pub fn raft_log_async(&self, req: &super::debugpb::RaftLogRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::RaftLogResponse>> {
        self.raft_log_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn region_info_opt(&self, req: &super::debugpb::RegionInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::debugpb::RegionInfoResponse> {
        self.client.unary_call(&METHOD_DEBUG_REGION_INFO, req, opt)
    }

    pub fn region_info(&self, req: &super::debugpb::RegionInfoRequest) -> ::grpcio::Result<super::debugpb::RegionInfoResponse> {
        self.region_info_opt(req, ::grpcio::CallOption::default())
    }

    pub fn region_info_async_opt(&self, req: &super::debugpb::RegionInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::RegionInfoResponse>> {
        self.client.unary_call_async(&METHOD_DEBUG_REGION_INFO, req, opt)
    }

    pub fn region_info_async(&self, req: &super::debugpb::RegionInfoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::RegionInfoResponse>> {
        self.region_info_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn region_size_opt(&self, req: &super::debugpb::RegionSizeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::debugpb::RegionSizeResponse> {
        self.client.unary_call(&METHOD_DEBUG_REGION_SIZE, req, opt)
    }

    pub fn region_size(&self, req: &super::debugpb::RegionSizeRequest) -> ::grpcio::Result<super::debugpb::RegionSizeResponse> {
        self.region_size_opt(req, ::grpcio::CallOption::default())
    }

    pub fn region_size_async_opt(&self, req: &super::debugpb::RegionSizeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::RegionSizeResponse>> {
        self.client.unary_call_async(&METHOD_DEBUG_REGION_SIZE, req, opt)
    }

    pub fn region_size_async(&self, req: &super::debugpb::RegionSizeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::RegionSizeResponse>> {
        self.region_size_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn scan_mvcc_opt(&self, req: &super::debugpb::ScanMvccRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::debugpb::ScanMvccResponse>> {
        self.client.server_streaming(&METHOD_DEBUG_SCAN_MVCC, req, opt)
    }

    pub fn scan_mvcc(&self, req: &super::debugpb::ScanMvccRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::debugpb::ScanMvccResponse>> {
        self.scan_mvcc_opt(req, ::grpcio::CallOption::default())
    }

    pub fn compact_opt(&self, req: &super::debugpb::CompactRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::debugpb::CompactResponse> {
        self.client.unary_call(&METHOD_DEBUG_COMPACT, req, opt)
    }

    pub fn compact(&self, req: &super::debugpb::CompactRequest) -> ::grpcio::Result<super::debugpb::CompactResponse> {
        self.compact_opt(req, ::grpcio::CallOption::default())
    }

    pub fn compact_async_opt(&self, req: &super::debugpb::CompactRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::CompactResponse>> {
        self.client.unary_call_async(&METHOD_DEBUG_COMPACT, req, opt)
    }

    pub fn compact_async(&self, req: &super::debugpb::CompactRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::CompactResponse>> {
        self.compact_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn inject_fail_point_opt(&self, req: &super::debugpb::InjectFailPointRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::debugpb::InjectFailPointResponse> {
        self.client.unary_call(&METHOD_DEBUG_INJECT_FAIL_POINT, req, opt)
    }

    pub fn inject_fail_point(&self, req: &super::debugpb::InjectFailPointRequest) -> ::grpcio::Result<super::debugpb::InjectFailPointResponse> {
        self.inject_fail_point_opt(req, ::grpcio::CallOption::default())
    }

    pub fn inject_fail_point_async_opt(&self, req: &super::debugpb::InjectFailPointRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::InjectFailPointResponse>> {
        self.client.unary_call_async(&METHOD_DEBUG_INJECT_FAIL_POINT, req, opt)
    }

    pub fn inject_fail_point_async(&self, req: &super::debugpb::InjectFailPointRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::InjectFailPointResponse>> {
        self.inject_fail_point_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn recover_fail_point_opt(&self, req: &super::debugpb::RecoverFailPointRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::debugpb::RecoverFailPointResponse> {
        self.client.unary_call(&METHOD_DEBUG_RECOVER_FAIL_POINT, req, opt)
    }

    pub fn recover_fail_point(&self, req: &super::debugpb::RecoverFailPointRequest) -> ::grpcio::Result<super::debugpb::RecoverFailPointResponse> {
        self.recover_fail_point_opt(req, ::grpcio::CallOption::default())
    }

    pub fn recover_fail_point_async_opt(&self, req: &super::debugpb::RecoverFailPointRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::RecoverFailPointResponse>> {
        self.client.unary_call_async(&METHOD_DEBUG_RECOVER_FAIL_POINT, req, opt)
    }

    pub fn recover_fail_point_async(&self, req: &super::debugpb::RecoverFailPointRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::RecoverFailPointResponse>> {
        self.recover_fail_point_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_fail_points_opt(&self, req: &super::debugpb::ListFailPointsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::debugpb::ListFailPointsResponse> {
        self.client.unary_call(&METHOD_DEBUG_LIST_FAIL_POINTS, req, opt)
    }

    pub fn list_fail_points(&self, req: &super::debugpb::ListFailPointsRequest) -> ::grpcio::Result<super::debugpb::ListFailPointsResponse> {
        self.list_fail_points_opt(req, ::grpcio::CallOption::default())
    }

    pub fn list_fail_points_async_opt(&self, req: &super::debugpb::ListFailPointsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::ListFailPointsResponse>> {
        self.client.unary_call_async(&METHOD_DEBUG_LIST_FAIL_POINTS, req, opt)
    }

    pub fn list_fail_points_async(&self, req: &super::debugpb::ListFailPointsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::ListFailPointsResponse>> {
        self.list_fail_points_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_metrics_opt(&self, req: &super::debugpb::GetMetricsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::debugpb::GetMetricsResponse> {
        self.client.unary_call(&METHOD_DEBUG_GET_METRICS, req, opt)
    }

    pub fn get_metrics(&self, req: &super::debugpb::GetMetricsRequest) -> ::grpcio::Result<super::debugpb::GetMetricsResponse> {
        self.get_metrics_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_metrics_async_opt(&self, req: &super::debugpb::GetMetricsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::GetMetricsResponse>> {
        self.client.unary_call_async(&METHOD_DEBUG_GET_METRICS, req, opt)
    }

    pub fn get_metrics_async(&self, req: &super::debugpb::GetMetricsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::GetMetricsResponse>> {
        self.get_metrics_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn check_region_consistency_opt(&self, req: &super::debugpb::RegionConsistencyCheckRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::debugpb::RegionConsistencyCheckResponse> {
        self.client.unary_call(&METHOD_DEBUG_CHECK_REGION_CONSISTENCY, req, opt)
    }

    pub fn check_region_consistency(&self, req: &super::debugpb::RegionConsistencyCheckRequest) -> ::grpcio::Result<super::debugpb::RegionConsistencyCheckResponse> {
        self.check_region_consistency_opt(req, ::grpcio::CallOption::default())
    }

    pub fn check_region_consistency_async_opt(&self, req: &super::debugpb::RegionConsistencyCheckRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::RegionConsistencyCheckResponse>> {
        self.client.unary_call_async(&METHOD_DEBUG_CHECK_REGION_CONSISTENCY, req, opt)
    }

    pub fn check_region_consistency_async(&self, req: &super::debugpb::RegionConsistencyCheckRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::RegionConsistencyCheckResponse>> {
        self.check_region_consistency_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn modify_tikv_config_opt(&self, req: &super::debugpb::ModifyTikvConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::debugpb::ModifyTikvConfigResponse> {
        self.client.unary_call(&METHOD_DEBUG_MODIFY_TIKV_CONFIG, req, opt)
    }

    pub fn modify_tikv_config(&self, req: &super::debugpb::ModifyTikvConfigRequest) -> ::grpcio::Result<super::debugpb::ModifyTikvConfigResponse> {
        self.modify_tikv_config_opt(req, ::grpcio::CallOption::default())
    }

    pub fn modify_tikv_config_async_opt(&self, req: &super::debugpb::ModifyTikvConfigRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::ModifyTikvConfigResponse>> {
        self.client.unary_call_async(&METHOD_DEBUG_MODIFY_TIKV_CONFIG, req, opt)
    }

    pub fn modify_tikv_config_async(&self, req: &super::debugpb::ModifyTikvConfigRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::ModifyTikvConfigResponse>> {
        self.modify_tikv_config_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_region_properties_opt(&self, req: &super::debugpb::GetRegionPropertiesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::debugpb::GetRegionPropertiesResponse> {
        self.client.unary_call(&METHOD_DEBUG_GET_REGION_PROPERTIES, req, opt)
    }

    pub fn get_region_properties(&self, req: &super::debugpb::GetRegionPropertiesRequest) -> ::grpcio::Result<super::debugpb::GetRegionPropertiesResponse> {
        self.get_region_properties_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_region_properties_async_opt(&self, req: &super::debugpb::GetRegionPropertiesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::GetRegionPropertiesResponse>> {
        self.client.unary_call_async(&METHOD_DEBUG_GET_REGION_PROPERTIES, req, opt)
    }

    pub fn get_region_properties_async(&self, req: &super::debugpb::GetRegionPropertiesRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::GetRegionPropertiesResponse>> {
        self.get_region_properties_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_store_info_opt(&self, req: &super::debugpb::GetStoreInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::debugpb::GetStoreInfoResponse> {
        self.client.unary_call(&METHOD_DEBUG_GET_STORE_INFO, req, opt)
    }

    pub fn get_store_info(&self, req: &super::debugpb::GetStoreInfoRequest) -> ::grpcio::Result<super::debugpb::GetStoreInfoResponse> {
        self.get_store_info_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_store_info_async_opt(&self, req: &super::debugpb::GetStoreInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::GetStoreInfoResponse>> {
        self.client.unary_call_async(&METHOD_DEBUG_GET_STORE_INFO, req, opt)
    }

    pub fn get_store_info_async(&self, req: &super::debugpb::GetStoreInfoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::GetStoreInfoResponse>> {
        self.get_store_info_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_cluster_info_opt(&self, req: &super::debugpb::GetClusterInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::debugpb::GetClusterInfoResponse> {
        self.client.unary_call(&METHOD_DEBUG_GET_CLUSTER_INFO, req, opt)
    }

    pub fn get_cluster_info(&self, req: &super::debugpb::GetClusterInfoRequest) -> ::grpcio::Result<super::debugpb::GetClusterInfoResponse> {
        self.get_cluster_info_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_cluster_info_async_opt(&self, req: &super::debugpb::GetClusterInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::GetClusterInfoResponse>> {
        self.client.unary_call_async(&METHOD_DEBUG_GET_CLUSTER_INFO, req, opt)
    }

    pub fn get_cluster_info_async(&self, req: &super::debugpb::GetClusterInfoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::GetClusterInfoResponse>> {
        self.get_cluster_info_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_all_regions_in_store_opt(&self, req: &super::debugpb::GetAllRegionsInStoreRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::debugpb::GetAllRegionsInStoreResponse> {
        self.client.unary_call(&METHOD_DEBUG_GET_ALL_REGIONS_IN_STORE, req, opt)
    }

    pub fn get_all_regions_in_store(&self, req: &super::debugpb::GetAllRegionsInStoreRequest) -> ::grpcio::Result<super::debugpb::GetAllRegionsInStoreResponse> {
        self.get_all_regions_in_store_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_all_regions_in_store_async_opt(&self, req: &super::debugpb::GetAllRegionsInStoreRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::GetAllRegionsInStoreResponse>> {
        self.client.unary_call_async(&METHOD_DEBUG_GET_ALL_REGIONS_IN_STORE, req, opt)
    }

    pub fn get_all_regions_in_store_async(&self, req: &super::debugpb::GetAllRegionsInStoreRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::GetAllRegionsInStoreResponse>> {
        self.get_all_regions_in_store_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn reset_to_version_opt(&self, req: &super::debugpb::ResetToVersionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::debugpb::ResetToVersionResponse> {
        self.client.unary_call(&METHOD_DEBUG_RESET_TO_VERSION, req, opt)
    }

    pub fn reset_to_version(&self, req: &super::debugpb::ResetToVersionRequest) -> ::grpcio::Result<super::debugpb::ResetToVersionResponse> {
        self.reset_to_version_opt(req, ::grpcio::CallOption::default())
    }

    pub fn reset_to_version_async_opt(&self, req: &super::debugpb::ResetToVersionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::ResetToVersionResponse>> {
        self.client.unary_call_async(&METHOD_DEBUG_RESET_TO_VERSION, req, opt)
    }

    pub fn reset_to_version_async(&self, req: &super::debugpb::ResetToVersionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::debugpb::ResetToVersionResponse>> {
        self.reset_to_version_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Debug {
    fn get(&mut self, ctx: ::grpcio::RpcContext, _req: super::debugpb::GetRequest, sink: ::grpcio::UnarySink<super::debugpb::GetResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn raft_log(&mut self, ctx: ::grpcio::RpcContext, _req: super::debugpb::RaftLogRequest, sink: ::grpcio::UnarySink<super::debugpb::RaftLogResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn region_info(&mut self, ctx: ::grpcio::RpcContext, _req: super::debugpb::RegionInfoRequest, sink: ::grpcio::UnarySink<super::debugpb::RegionInfoResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn region_size(&mut self, ctx: ::grpcio::RpcContext, _req: super::debugpb::RegionSizeRequest, sink: ::grpcio::UnarySink<super::debugpb::RegionSizeResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn scan_mvcc(&mut self, ctx: ::grpcio::RpcContext, _req: super::debugpb::ScanMvccRequest, sink: ::grpcio::ServerStreamingSink<super::debugpb::ScanMvccResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn compact(&mut self, ctx: ::grpcio::RpcContext, _req: super::debugpb::CompactRequest, sink: ::grpcio::UnarySink<super::debugpb::CompactResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn inject_fail_point(&mut self, ctx: ::grpcio::RpcContext, _req: super::debugpb::InjectFailPointRequest, sink: ::grpcio::UnarySink<super::debugpb::InjectFailPointResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn recover_fail_point(&mut self, ctx: ::grpcio::RpcContext, _req: super::debugpb::RecoverFailPointRequest, sink: ::grpcio::UnarySink<super::debugpb::RecoverFailPointResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn list_fail_points(&mut self, ctx: ::grpcio::RpcContext, _req: super::debugpb::ListFailPointsRequest, sink: ::grpcio::UnarySink<super::debugpb::ListFailPointsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_metrics(&mut self, ctx: ::grpcio::RpcContext, _req: super::debugpb::GetMetricsRequest, sink: ::grpcio::UnarySink<super::debugpb::GetMetricsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn check_region_consistency(&mut self, ctx: ::grpcio::RpcContext, _req: super::debugpb::RegionConsistencyCheckRequest, sink: ::grpcio::UnarySink<super::debugpb::RegionConsistencyCheckResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn modify_tikv_config(&mut self, ctx: ::grpcio::RpcContext, _req: super::debugpb::ModifyTikvConfigRequest, sink: ::grpcio::UnarySink<super::debugpb::ModifyTikvConfigResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_region_properties(&mut self, ctx: ::grpcio::RpcContext, _req: super::debugpb::GetRegionPropertiesRequest, sink: ::grpcio::UnarySink<super::debugpb::GetRegionPropertiesResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_store_info(&mut self, ctx: ::grpcio::RpcContext, _req: super::debugpb::GetStoreInfoRequest, sink: ::grpcio::UnarySink<super::debugpb::GetStoreInfoResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_cluster_info(&mut self, ctx: ::grpcio::RpcContext, _req: super::debugpb::GetClusterInfoRequest, sink: ::grpcio::UnarySink<super::debugpb::GetClusterInfoResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_all_regions_in_store(&mut self, ctx: ::grpcio::RpcContext, _req: super::debugpb::GetAllRegionsInStoreRequest, sink: ::grpcio::UnarySink<super::debugpb::GetAllRegionsInStoreResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn reset_to_version(&mut self, ctx: ::grpcio::RpcContext, _req: super::debugpb::ResetToVersionRequest, sink: ::grpcio::UnarySink<super::debugpb::ResetToVersionResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_debug<S: Debug + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DEBUG_GET, move |ctx, req, resp| {
        instance.get(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DEBUG_RAFT_LOG, move |ctx, req, resp| {
        instance.raft_log(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DEBUG_REGION_INFO, move |ctx, req, resp| {
        instance.region_info(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DEBUG_REGION_SIZE, move |ctx, req, resp| {
        instance.region_size(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_DEBUG_SCAN_MVCC, move |ctx, req, resp| {
        instance.scan_mvcc(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DEBUG_COMPACT, move |ctx, req, resp| {
        instance.compact(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DEBUG_INJECT_FAIL_POINT, move |ctx, req, resp| {
        instance.inject_fail_point(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DEBUG_RECOVER_FAIL_POINT, move |ctx, req, resp| {
        instance.recover_fail_point(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DEBUG_LIST_FAIL_POINTS, move |ctx, req, resp| {
        instance.list_fail_points(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DEBUG_GET_METRICS, move |ctx, req, resp| {
        instance.get_metrics(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DEBUG_CHECK_REGION_CONSISTENCY, move |ctx, req, resp| {
        instance.check_region_consistency(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DEBUG_MODIFY_TIKV_CONFIG, move |ctx, req, resp| {
        instance.modify_tikv_config(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DEBUG_GET_REGION_PROPERTIES, move |ctx, req, resp| {
        instance.get_region_properties(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DEBUG_GET_STORE_INFO, move |ctx, req, resp| {
        instance.get_store_info(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DEBUG_GET_CLUSTER_INFO, move |ctx, req, resp| {
        instance.get_cluster_info(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_DEBUG_GET_ALL_REGIONS_IN_STORE, move |ctx, req, resp| {
        instance.get_all_regions_in_store(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_DEBUG_RESET_TO_VERSION, move |ctx, req, resp| {
        instance.reset_to_version(ctx, req, resp)
    });
    builder.build()
}
