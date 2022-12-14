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

const METHOD_TIKV_KV_GET: ::grpcio::Method<super::kvrpcpb::GetRequest, super::kvrpcpb::GetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/KvGet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_KV_SCAN: ::grpcio::Method<super::kvrpcpb::ScanRequest, super::kvrpcpb::ScanResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/KvScan",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_KV_PREWRITE: ::grpcio::Method<super::kvrpcpb::PrewriteRequest, super::kvrpcpb::PrewriteResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/KvPrewrite",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_KV_PESSIMISTIC_LOCK: ::grpcio::Method<super::kvrpcpb::PessimisticLockRequest, super::kvrpcpb::PessimisticLockResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/KvPessimisticLock",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_KV_PESSIMISTIC_ROLLBACK: ::grpcio::Method<super::kvrpcpb::PessimisticRollbackRequest, super::kvrpcpb::PessimisticRollbackResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/KVPessimisticRollback",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_KV_TXN_HEART_BEAT: ::grpcio::Method<super::kvrpcpb::TxnHeartBeatRequest, super::kvrpcpb::TxnHeartBeatResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/KvTxnHeartBeat",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_KV_CHECK_TXN_STATUS: ::grpcio::Method<super::kvrpcpb::CheckTxnStatusRequest, super::kvrpcpb::CheckTxnStatusResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/KvCheckTxnStatus",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_KV_CHECK_SECONDARY_LOCKS: ::grpcio::Method<super::kvrpcpb::CheckSecondaryLocksRequest, super::kvrpcpb::CheckSecondaryLocksResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/KvCheckSecondaryLocks",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_KV_COMMIT: ::grpcio::Method<super::kvrpcpb::CommitRequest, super::kvrpcpb::CommitResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/KvCommit",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_KV_IMPORT: ::grpcio::Method<super::kvrpcpb::ImportRequest, super::kvrpcpb::ImportResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/KvImport",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_KV_CLEANUP: ::grpcio::Method<super::kvrpcpb::CleanupRequest, super::kvrpcpb::CleanupResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/KvCleanup",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_KV_BATCH_GET: ::grpcio::Method<super::kvrpcpb::BatchGetRequest, super::kvrpcpb::BatchGetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/KvBatchGet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_KV_BATCH_ROLLBACK: ::grpcio::Method<super::kvrpcpb::BatchRollbackRequest, super::kvrpcpb::BatchRollbackResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/KvBatchRollback",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_KV_SCAN_LOCK: ::grpcio::Method<super::kvrpcpb::ScanLockRequest, super::kvrpcpb::ScanLockResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/KvScanLock",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_KV_RESOLVE_LOCK: ::grpcio::Method<super::kvrpcpb::ResolveLockRequest, super::kvrpcpb::ResolveLockResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/KvResolveLock",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_KV_GC: ::grpcio::Method<super::kvrpcpb::GcRequest, super::kvrpcpb::GcResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/KvGC",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_KV_DELETE_RANGE: ::grpcio::Method<super::kvrpcpb::DeleteRangeRequest, super::kvrpcpb::DeleteRangeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/KvDeleteRange",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_RAW_GET: ::grpcio::Method<super::kvrpcpb::RawGetRequest, super::kvrpcpb::RawGetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/RawGet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_RAW_BATCH_GET: ::grpcio::Method<super::kvrpcpb::RawBatchGetRequest, super::kvrpcpb::RawBatchGetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/RawBatchGet",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_RAW_PUT: ::grpcio::Method<super::kvrpcpb::RawPutRequest, super::kvrpcpb::RawPutResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/RawPut",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_RAW_BATCH_PUT: ::grpcio::Method<super::kvrpcpb::RawBatchPutRequest, super::kvrpcpb::RawBatchPutResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/RawBatchPut",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_RAW_DELETE: ::grpcio::Method<super::kvrpcpb::RawDeleteRequest, super::kvrpcpb::RawDeleteResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/RawDelete",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_RAW_BATCH_DELETE: ::grpcio::Method<super::kvrpcpb::RawBatchDeleteRequest, super::kvrpcpb::RawBatchDeleteResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/RawBatchDelete",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_RAW_SCAN: ::grpcio::Method<super::kvrpcpb::RawScanRequest, super::kvrpcpb::RawScanResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/RawScan",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_RAW_DELETE_RANGE: ::grpcio::Method<super::kvrpcpb::RawDeleteRangeRequest, super::kvrpcpb::RawDeleteRangeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/RawDeleteRange",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_RAW_BATCH_SCAN: ::grpcio::Method<super::kvrpcpb::RawBatchScanRequest, super::kvrpcpb::RawBatchScanResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/RawBatchScan",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_RAW_GET_KEY_TTL: ::grpcio::Method<super::kvrpcpb::RawGetKeyTtlRequest, super::kvrpcpb::RawGetKeyTtlResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/RawGetKeyTTL",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_RAW_COMPARE_AND_SWAP: ::grpcio::Method<super::kvrpcpb::RawCasRequest, super::kvrpcpb::RawCasResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/RawCompareAndSwap",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_RAW_CHECKSUM: ::grpcio::Method<super::kvrpcpb::RawChecksumRequest, super::kvrpcpb::RawChecksumResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/RawChecksum",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_UNSAFE_DESTROY_RANGE: ::grpcio::Method<super::kvrpcpb::UnsafeDestroyRangeRequest, super::kvrpcpb::UnsafeDestroyRangeResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/UnsafeDestroyRange",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_REGISTER_LOCK_OBSERVER: ::grpcio::Method<super::kvrpcpb::RegisterLockObserverRequest, super::kvrpcpb::RegisterLockObserverResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/RegisterLockObserver",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_CHECK_LOCK_OBSERVER: ::grpcio::Method<super::kvrpcpb::CheckLockObserverRequest, super::kvrpcpb::CheckLockObserverResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/CheckLockObserver",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_REMOVE_LOCK_OBSERVER: ::grpcio::Method<super::kvrpcpb::RemoveLockObserverRequest, super::kvrpcpb::RemoveLockObserverResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/RemoveLockObserver",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_PHYSICAL_SCAN_LOCK: ::grpcio::Method<super::kvrpcpb::PhysicalScanLockRequest, super::kvrpcpb::PhysicalScanLockResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/PhysicalScanLock",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_COPROCESSOR: ::grpcio::Method<super::coprocessor::Request, super::coprocessor::Response> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/Coprocessor",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_COPROCESSOR_STREAM: ::grpcio::Method<super::coprocessor::Request, super::coprocessor::Response> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/tikvpb.Tikv/CoprocessorStream",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_BATCH_COPROCESSOR: ::grpcio::Method<super::coprocessor::BatchRequest, super::coprocessor::BatchResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/tikvpb.Tikv/BatchCoprocessor",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_RAW_COPROCESSOR: ::grpcio::Method<super::kvrpcpb::RawCoprocessorRequest, super::kvrpcpb::RawCoprocessorResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/RawCoprocessor",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_RAFT: ::grpcio::Method<super::raft_serverpb::RaftMessage, super::raft_serverpb::Done> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/tikvpb.Tikv/Raft",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_BATCH_RAFT: ::grpcio::Method<super::tikvpb::BatchRaftMessage, super::raft_serverpb::Done> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/tikvpb.Tikv/BatchRaft",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_SNAPSHOT: ::grpcio::Method<super::raft_serverpb::SnapshotChunk, super::raft_serverpb::Done> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ClientStreaming,
    name: "/tikvpb.Tikv/Snapshot",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_SPLIT_REGION: ::grpcio::Method<super::kvrpcpb::SplitRegionRequest, super::kvrpcpb::SplitRegionResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/SplitRegion",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_READ_INDEX: ::grpcio::Method<super::kvrpcpb::ReadIndexRequest, super::kvrpcpb::ReadIndexResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/ReadIndex",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_MVCC_GET_BY_KEY: ::grpcio::Method<super::kvrpcpb::MvccGetByKeyRequest, super::kvrpcpb::MvccGetByKeyResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/MvccGetByKey",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_MVCC_GET_BY_START_TS: ::grpcio::Method<super::kvrpcpb::MvccGetByStartTsRequest, super::kvrpcpb::MvccGetByStartTsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/MvccGetByStartTs",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_BATCH_COMMANDS: ::grpcio::Method<super::tikvpb::BatchCommandsRequest, super::tikvpb::BatchCommandsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/tikvpb.Tikv/BatchCommands",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_DISPATCH_MPP_TASK: ::grpcio::Method<super::mpp::DispatchTaskRequest, super::mpp::DispatchTaskResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/DispatchMPPTask",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_CANCEL_MPP_TASK: ::grpcio::Method<super::mpp::CancelTaskRequest, super::mpp::CancelTaskResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/CancelMPPTask",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_ESTABLISH_MPP_CONNECTION: ::grpcio::Method<super::mpp::EstablishMppConnectionRequest, super::mpp::MppDataPacket> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/tikvpb.Tikv/EstablishMPPConnection",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_IS_ALIVE: ::grpcio::Method<super::mpp::IsAliveRequest, super::mpp::IsAliveResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/IsAlive",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_CHECK_LEADER: ::grpcio::Method<super::kvrpcpb::CheckLeaderRequest, super::kvrpcpb::CheckLeaderResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/CheckLeader",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_GET_STORE_SAFE_TS: ::grpcio::Method<super::kvrpcpb::StoreSafeTsRequest, super::kvrpcpb::StoreSafeTsResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/GetStoreSafeTS",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_GET_LOCK_WAIT_INFO: ::grpcio::Method<super::kvrpcpb::GetLockWaitInfoRequest, super::kvrpcpb::GetLockWaitInfoResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/GetLockWaitInfo",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_TIKV_COMPACT: ::grpcio::Method<super::kvrpcpb::CompactRequest, super::kvrpcpb::CompactResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/tikvpb.Tikv/Compact",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct TikvClient {
    client: ::grpcio::Client,
}

impl TikvClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        TikvClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn kv_get_opt(&self, req: &super::kvrpcpb::GetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::GetResponse> {
        self.client.unary_call(&METHOD_TIKV_KV_GET, req, opt)
    }

    pub fn kv_get(&self, req: &super::kvrpcpb::GetRequest) -> ::grpcio::Result<super::kvrpcpb::GetResponse> {
        self.kv_get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_get_async_opt(&self, req: &super::kvrpcpb::GetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::GetResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_KV_GET, req, opt)
    }

    pub fn kv_get_async(&self, req: &super::kvrpcpb::GetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::GetResponse>> {
        self.kv_get_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_scan_opt(&self, req: &super::kvrpcpb::ScanRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::ScanResponse> {
        self.client.unary_call(&METHOD_TIKV_KV_SCAN, req, opt)
    }

    pub fn kv_scan(&self, req: &super::kvrpcpb::ScanRequest) -> ::grpcio::Result<super::kvrpcpb::ScanResponse> {
        self.kv_scan_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_scan_async_opt(&self, req: &super::kvrpcpb::ScanRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::ScanResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_KV_SCAN, req, opt)
    }

    pub fn kv_scan_async(&self, req: &super::kvrpcpb::ScanRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::ScanResponse>> {
        self.kv_scan_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_prewrite_opt(&self, req: &super::kvrpcpb::PrewriteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::PrewriteResponse> {
        self.client.unary_call(&METHOD_TIKV_KV_PREWRITE, req, opt)
    }

    pub fn kv_prewrite(&self, req: &super::kvrpcpb::PrewriteRequest) -> ::grpcio::Result<super::kvrpcpb::PrewriteResponse> {
        self.kv_prewrite_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_prewrite_async_opt(&self, req: &super::kvrpcpb::PrewriteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::PrewriteResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_KV_PREWRITE, req, opt)
    }

    pub fn kv_prewrite_async(&self, req: &super::kvrpcpb::PrewriteRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::PrewriteResponse>> {
        self.kv_prewrite_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_pessimistic_lock_opt(&self, req: &super::kvrpcpb::PessimisticLockRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::PessimisticLockResponse> {
        self.client.unary_call(&METHOD_TIKV_KV_PESSIMISTIC_LOCK, req, opt)
    }

    pub fn kv_pessimistic_lock(&self, req: &super::kvrpcpb::PessimisticLockRequest) -> ::grpcio::Result<super::kvrpcpb::PessimisticLockResponse> {
        self.kv_pessimistic_lock_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_pessimistic_lock_async_opt(&self, req: &super::kvrpcpb::PessimisticLockRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::PessimisticLockResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_KV_PESSIMISTIC_LOCK, req, opt)
    }

    pub fn kv_pessimistic_lock_async(&self, req: &super::kvrpcpb::PessimisticLockRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::PessimisticLockResponse>> {
        self.kv_pessimistic_lock_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_pessimistic_rollback_opt(&self, req: &super::kvrpcpb::PessimisticRollbackRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::PessimisticRollbackResponse> {
        self.client.unary_call(&METHOD_TIKV_KV_PESSIMISTIC_ROLLBACK, req, opt)
    }

    pub fn kv_pessimistic_rollback(&self, req: &super::kvrpcpb::PessimisticRollbackRequest) -> ::grpcio::Result<super::kvrpcpb::PessimisticRollbackResponse> {
        self.kv_pessimistic_rollback_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_pessimistic_rollback_async_opt(&self, req: &super::kvrpcpb::PessimisticRollbackRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::PessimisticRollbackResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_KV_PESSIMISTIC_ROLLBACK, req, opt)
    }

    pub fn kv_pessimistic_rollback_async(&self, req: &super::kvrpcpb::PessimisticRollbackRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::PessimisticRollbackResponse>> {
        self.kv_pessimistic_rollback_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_txn_heart_beat_opt(&self, req: &super::kvrpcpb::TxnHeartBeatRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::TxnHeartBeatResponse> {
        self.client.unary_call(&METHOD_TIKV_KV_TXN_HEART_BEAT, req, opt)
    }

    pub fn kv_txn_heart_beat(&self, req: &super::kvrpcpb::TxnHeartBeatRequest) -> ::grpcio::Result<super::kvrpcpb::TxnHeartBeatResponse> {
        self.kv_txn_heart_beat_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_txn_heart_beat_async_opt(&self, req: &super::kvrpcpb::TxnHeartBeatRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::TxnHeartBeatResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_KV_TXN_HEART_BEAT, req, opt)
    }

    pub fn kv_txn_heart_beat_async(&self, req: &super::kvrpcpb::TxnHeartBeatRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::TxnHeartBeatResponse>> {
        self.kv_txn_heart_beat_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_check_txn_status_opt(&self, req: &super::kvrpcpb::CheckTxnStatusRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::CheckTxnStatusResponse> {
        self.client.unary_call(&METHOD_TIKV_KV_CHECK_TXN_STATUS, req, opt)
    }

    pub fn kv_check_txn_status(&self, req: &super::kvrpcpb::CheckTxnStatusRequest) -> ::grpcio::Result<super::kvrpcpb::CheckTxnStatusResponse> {
        self.kv_check_txn_status_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_check_txn_status_async_opt(&self, req: &super::kvrpcpb::CheckTxnStatusRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::CheckTxnStatusResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_KV_CHECK_TXN_STATUS, req, opt)
    }

    pub fn kv_check_txn_status_async(&self, req: &super::kvrpcpb::CheckTxnStatusRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::CheckTxnStatusResponse>> {
        self.kv_check_txn_status_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_check_secondary_locks_opt(&self, req: &super::kvrpcpb::CheckSecondaryLocksRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::CheckSecondaryLocksResponse> {
        self.client.unary_call(&METHOD_TIKV_KV_CHECK_SECONDARY_LOCKS, req, opt)
    }

    pub fn kv_check_secondary_locks(&self, req: &super::kvrpcpb::CheckSecondaryLocksRequest) -> ::grpcio::Result<super::kvrpcpb::CheckSecondaryLocksResponse> {
        self.kv_check_secondary_locks_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_check_secondary_locks_async_opt(&self, req: &super::kvrpcpb::CheckSecondaryLocksRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::CheckSecondaryLocksResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_KV_CHECK_SECONDARY_LOCKS, req, opt)
    }

    pub fn kv_check_secondary_locks_async(&self, req: &super::kvrpcpb::CheckSecondaryLocksRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::CheckSecondaryLocksResponse>> {
        self.kv_check_secondary_locks_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_commit_opt(&self, req: &super::kvrpcpb::CommitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::CommitResponse> {
        self.client.unary_call(&METHOD_TIKV_KV_COMMIT, req, opt)
    }

    pub fn kv_commit(&self, req: &super::kvrpcpb::CommitRequest) -> ::grpcio::Result<super::kvrpcpb::CommitResponse> {
        self.kv_commit_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_commit_async_opt(&self, req: &super::kvrpcpb::CommitRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::CommitResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_KV_COMMIT, req, opt)
    }

    pub fn kv_commit_async(&self, req: &super::kvrpcpb::CommitRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::CommitResponse>> {
        self.kv_commit_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_import_opt(&self, req: &super::kvrpcpb::ImportRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::ImportResponse> {
        self.client.unary_call(&METHOD_TIKV_KV_IMPORT, req, opt)
    }

    pub fn kv_import(&self, req: &super::kvrpcpb::ImportRequest) -> ::grpcio::Result<super::kvrpcpb::ImportResponse> {
        self.kv_import_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_import_async_opt(&self, req: &super::kvrpcpb::ImportRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::ImportResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_KV_IMPORT, req, opt)
    }

    pub fn kv_import_async(&self, req: &super::kvrpcpb::ImportRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::ImportResponse>> {
        self.kv_import_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_cleanup_opt(&self, req: &super::kvrpcpb::CleanupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::CleanupResponse> {
        self.client.unary_call(&METHOD_TIKV_KV_CLEANUP, req, opt)
    }

    pub fn kv_cleanup(&self, req: &super::kvrpcpb::CleanupRequest) -> ::grpcio::Result<super::kvrpcpb::CleanupResponse> {
        self.kv_cleanup_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_cleanup_async_opt(&self, req: &super::kvrpcpb::CleanupRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::CleanupResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_KV_CLEANUP, req, opt)
    }

    pub fn kv_cleanup_async(&self, req: &super::kvrpcpb::CleanupRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::CleanupResponse>> {
        self.kv_cleanup_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_batch_get_opt(&self, req: &super::kvrpcpb::BatchGetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::BatchGetResponse> {
        self.client.unary_call(&METHOD_TIKV_KV_BATCH_GET, req, opt)
    }

    pub fn kv_batch_get(&self, req: &super::kvrpcpb::BatchGetRequest) -> ::grpcio::Result<super::kvrpcpb::BatchGetResponse> {
        self.kv_batch_get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_batch_get_async_opt(&self, req: &super::kvrpcpb::BatchGetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::BatchGetResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_KV_BATCH_GET, req, opt)
    }

    pub fn kv_batch_get_async(&self, req: &super::kvrpcpb::BatchGetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::BatchGetResponse>> {
        self.kv_batch_get_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_batch_rollback_opt(&self, req: &super::kvrpcpb::BatchRollbackRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::BatchRollbackResponse> {
        self.client.unary_call(&METHOD_TIKV_KV_BATCH_ROLLBACK, req, opt)
    }

    pub fn kv_batch_rollback(&self, req: &super::kvrpcpb::BatchRollbackRequest) -> ::grpcio::Result<super::kvrpcpb::BatchRollbackResponse> {
        self.kv_batch_rollback_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_batch_rollback_async_opt(&self, req: &super::kvrpcpb::BatchRollbackRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::BatchRollbackResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_KV_BATCH_ROLLBACK, req, opt)
    }

    pub fn kv_batch_rollback_async(&self, req: &super::kvrpcpb::BatchRollbackRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::BatchRollbackResponse>> {
        self.kv_batch_rollback_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_scan_lock_opt(&self, req: &super::kvrpcpb::ScanLockRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::ScanLockResponse> {
        self.client.unary_call(&METHOD_TIKV_KV_SCAN_LOCK, req, opt)
    }

    pub fn kv_scan_lock(&self, req: &super::kvrpcpb::ScanLockRequest) -> ::grpcio::Result<super::kvrpcpb::ScanLockResponse> {
        self.kv_scan_lock_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_scan_lock_async_opt(&self, req: &super::kvrpcpb::ScanLockRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::ScanLockResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_KV_SCAN_LOCK, req, opt)
    }

    pub fn kv_scan_lock_async(&self, req: &super::kvrpcpb::ScanLockRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::ScanLockResponse>> {
        self.kv_scan_lock_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_resolve_lock_opt(&self, req: &super::kvrpcpb::ResolveLockRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::ResolveLockResponse> {
        self.client.unary_call(&METHOD_TIKV_KV_RESOLVE_LOCK, req, opt)
    }

    pub fn kv_resolve_lock(&self, req: &super::kvrpcpb::ResolveLockRequest) -> ::grpcio::Result<super::kvrpcpb::ResolveLockResponse> {
        self.kv_resolve_lock_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_resolve_lock_async_opt(&self, req: &super::kvrpcpb::ResolveLockRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::ResolveLockResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_KV_RESOLVE_LOCK, req, opt)
    }

    pub fn kv_resolve_lock_async(&self, req: &super::kvrpcpb::ResolveLockRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::ResolveLockResponse>> {
        self.kv_resolve_lock_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_gc_opt(&self, req: &super::kvrpcpb::GcRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::GcResponse> {
        self.client.unary_call(&METHOD_TIKV_KV_GC, req, opt)
    }

    pub fn kv_gc(&self, req: &super::kvrpcpb::GcRequest) -> ::grpcio::Result<super::kvrpcpb::GcResponse> {
        self.kv_gc_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_gc_async_opt(&self, req: &super::kvrpcpb::GcRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::GcResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_KV_GC, req, opt)
    }

    pub fn kv_gc_async(&self, req: &super::kvrpcpb::GcRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::GcResponse>> {
        self.kv_gc_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_delete_range_opt(&self, req: &super::kvrpcpb::DeleteRangeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::DeleteRangeResponse> {
        self.client.unary_call(&METHOD_TIKV_KV_DELETE_RANGE, req, opt)
    }

    pub fn kv_delete_range(&self, req: &super::kvrpcpb::DeleteRangeRequest) -> ::grpcio::Result<super::kvrpcpb::DeleteRangeResponse> {
        self.kv_delete_range_opt(req, ::grpcio::CallOption::default())
    }

    pub fn kv_delete_range_async_opt(&self, req: &super::kvrpcpb::DeleteRangeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::DeleteRangeResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_KV_DELETE_RANGE, req, opt)
    }

    pub fn kv_delete_range_async(&self, req: &super::kvrpcpb::DeleteRangeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::DeleteRangeResponse>> {
        self.kv_delete_range_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raw_get_opt(&self, req: &super::kvrpcpb::RawGetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::RawGetResponse> {
        self.client.unary_call(&METHOD_TIKV_RAW_GET, req, opt)
    }

    pub fn raw_get(&self, req: &super::kvrpcpb::RawGetRequest) -> ::grpcio::Result<super::kvrpcpb::RawGetResponse> {
        self.raw_get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raw_get_async_opt(&self, req: &super::kvrpcpb::RawGetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RawGetResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_RAW_GET, req, opt)
    }

    pub fn raw_get_async(&self, req: &super::kvrpcpb::RawGetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RawGetResponse>> {
        self.raw_get_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raw_batch_get_opt(&self, req: &super::kvrpcpb::RawBatchGetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::RawBatchGetResponse> {
        self.client.unary_call(&METHOD_TIKV_RAW_BATCH_GET, req, opt)
    }

    pub fn raw_batch_get(&self, req: &super::kvrpcpb::RawBatchGetRequest) -> ::grpcio::Result<super::kvrpcpb::RawBatchGetResponse> {
        self.raw_batch_get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raw_batch_get_async_opt(&self, req: &super::kvrpcpb::RawBatchGetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RawBatchGetResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_RAW_BATCH_GET, req, opt)
    }

    pub fn raw_batch_get_async(&self, req: &super::kvrpcpb::RawBatchGetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RawBatchGetResponse>> {
        self.raw_batch_get_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raw_put_opt(&self, req: &super::kvrpcpb::RawPutRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::RawPutResponse> {
        self.client.unary_call(&METHOD_TIKV_RAW_PUT, req, opt)
    }

    pub fn raw_put(&self, req: &super::kvrpcpb::RawPutRequest) -> ::grpcio::Result<super::kvrpcpb::RawPutResponse> {
        self.raw_put_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raw_put_async_opt(&self, req: &super::kvrpcpb::RawPutRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RawPutResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_RAW_PUT, req, opt)
    }

    pub fn raw_put_async(&self, req: &super::kvrpcpb::RawPutRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RawPutResponse>> {
        self.raw_put_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raw_batch_put_opt(&self, req: &super::kvrpcpb::RawBatchPutRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::RawBatchPutResponse> {
        self.client.unary_call(&METHOD_TIKV_RAW_BATCH_PUT, req, opt)
    }

    pub fn raw_batch_put(&self, req: &super::kvrpcpb::RawBatchPutRequest) -> ::grpcio::Result<super::kvrpcpb::RawBatchPutResponse> {
        self.raw_batch_put_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raw_batch_put_async_opt(&self, req: &super::kvrpcpb::RawBatchPutRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RawBatchPutResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_RAW_BATCH_PUT, req, opt)
    }

    pub fn raw_batch_put_async(&self, req: &super::kvrpcpb::RawBatchPutRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RawBatchPutResponse>> {
        self.raw_batch_put_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raw_delete_opt(&self, req: &super::kvrpcpb::RawDeleteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::RawDeleteResponse> {
        self.client.unary_call(&METHOD_TIKV_RAW_DELETE, req, opt)
    }

    pub fn raw_delete(&self, req: &super::kvrpcpb::RawDeleteRequest) -> ::grpcio::Result<super::kvrpcpb::RawDeleteResponse> {
        self.raw_delete_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raw_delete_async_opt(&self, req: &super::kvrpcpb::RawDeleteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RawDeleteResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_RAW_DELETE, req, opt)
    }

    pub fn raw_delete_async(&self, req: &super::kvrpcpb::RawDeleteRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RawDeleteResponse>> {
        self.raw_delete_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raw_batch_delete_opt(&self, req: &super::kvrpcpb::RawBatchDeleteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::RawBatchDeleteResponse> {
        self.client.unary_call(&METHOD_TIKV_RAW_BATCH_DELETE, req, opt)
    }

    pub fn raw_batch_delete(&self, req: &super::kvrpcpb::RawBatchDeleteRequest) -> ::grpcio::Result<super::kvrpcpb::RawBatchDeleteResponse> {
        self.raw_batch_delete_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raw_batch_delete_async_opt(&self, req: &super::kvrpcpb::RawBatchDeleteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RawBatchDeleteResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_RAW_BATCH_DELETE, req, opt)
    }

    pub fn raw_batch_delete_async(&self, req: &super::kvrpcpb::RawBatchDeleteRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RawBatchDeleteResponse>> {
        self.raw_batch_delete_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raw_scan_opt(&self, req: &super::kvrpcpb::RawScanRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::RawScanResponse> {
        self.client.unary_call(&METHOD_TIKV_RAW_SCAN, req, opt)
    }

    pub fn raw_scan(&self, req: &super::kvrpcpb::RawScanRequest) -> ::grpcio::Result<super::kvrpcpb::RawScanResponse> {
        self.raw_scan_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raw_scan_async_opt(&self, req: &super::kvrpcpb::RawScanRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RawScanResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_RAW_SCAN, req, opt)
    }

    pub fn raw_scan_async(&self, req: &super::kvrpcpb::RawScanRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RawScanResponse>> {
        self.raw_scan_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raw_delete_range_opt(&self, req: &super::kvrpcpb::RawDeleteRangeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::RawDeleteRangeResponse> {
        self.client.unary_call(&METHOD_TIKV_RAW_DELETE_RANGE, req, opt)
    }

    pub fn raw_delete_range(&self, req: &super::kvrpcpb::RawDeleteRangeRequest) -> ::grpcio::Result<super::kvrpcpb::RawDeleteRangeResponse> {
        self.raw_delete_range_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raw_delete_range_async_opt(&self, req: &super::kvrpcpb::RawDeleteRangeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RawDeleteRangeResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_RAW_DELETE_RANGE, req, opt)
    }

    pub fn raw_delete_range_async(&self, req: &super::kvrpcpb::RawDeleteRangeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RawDeleteRangeResponse>> {
        self.raw_delete_range_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raw_batch_scan_opt(&self, req: &super::kvrpcpb::RawBatchScanRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::RawBatchScanResponse> {
        self.client.unary_call(&METHOD_TIKV_RAW_BATCH_SCAN, req, opt)
    }

    pub fn raw_batch_scan(&self, req: &super::kvrpcpb::RawBatchScanRequest) -> ::grpcio::Result<super::kvrpcpb::RawBatchScanResponse> {
        self.raw_batch_scan_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raw_batch_scan_async_opt(&self, req: &super::kvrpcpb::RawBatchScanRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RawBatchScanResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_RAW_BATCH_SCAN, req, opt)
    }

    pub fn raw_batch_scan_async(&self, req: &super::kvrpcpb::RawBatchScanRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RawBatchScanResponse>> {
        self.raw_batch_scan_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raw_get_key_ttl_opt(&self, req: &super::kvrpcpb::RawGetKeyTtlRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::RawGetKeyTtlResponse> {
        self.client.unary_call(&METHOD_TIKV_RAW_GET_KEY_TTL, req, opt)
    }

    pub fn raw_get_key_ttl(&self, req: &super::kvrpcpb::RawGetKeyTtlRequest) -> ::grpcio::Result<super::kvrpcpb::RawGetKeyTtlResponse> {
        self.raw_get_key_ttl_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raw_get_key_ttl_async_opt(&self, req: &super::kvrpcpb::RawGetKeyTtlRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RawGetKeyTtlResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_RAW_GET_KEY_TTL, req, opt)
    }

    pub fn raw_get_key_ttl_async(&self, req: &super::kvrpcpb::RawGetKeyTtlRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RawGetKeyTtlResponse>> {
        self.raw_get_key_ttl_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raw_compare_and_swap_opt(&self, req: &super::kvrpcpb::RawCasRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::RawCasResponse> {
        self.client.unary_call(&METHOD_TIKV_RAW_COMPARE_AND_SWAP, req, opt)
    }

    pub fn raw_compare_and_swap(&self, req: &super::kvrpcpb::RawCasRequest) -> ::grpcio::Result<super::kvrpcpb::RawCasResponse> {
        self.raw_compare_and_swap_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raw_compare_and_swap_async_opt(&self, req: &super::kvrpcpb::RawCasRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RawCasResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_RAW_COMPARE_AND_SWAP, req, opt)
    }

    pub fn raw_compare_and_swap_async(&self, req: &super::kvrpcpb::RawCasRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RawCasResponse>> {
        self.raw_compare_and_swap_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raw_checksum_opt(&self, req: &super::kvrpcpb::RawChecksumRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::RawChecksumResponse> {
        self.client.unary_call(&METHOD_TIKV_RAW_CHECKSUM, req, opt)
    }

    pub fn raw_checksum(&self, req: &super::kvrpcpb::RawChecksumRequest) -> ::grpcio::Result<super::kvrpcpb::RawChecksumResponse> {
        self.raw_checksum_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raw_checksum_async_opt(&self, req: &super::kvrpcpb::RawChecksumRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RawChecksumResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_RAW_CHECKSUM, req, opt)
    }

    pub fn raw_checksum_async(&self, req: &super::kvrpcpb::RawChecksumRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RawChecksumResponse>> {
        self.raw_checksum_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn unsafe_destroy_range_opt(&self, req: &super::kvrpcpb::UnsafeDestroyRangeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::UnsafeDestroyRangeResponse> {
        self.client.unary_call(&METHOD_TIKV_UNSAFE_DESTROY_RANGE, req, opt)
    }

    pub fn unsafe_destroy_range(&self, req: &super::kvrpcpb::UnsafeDestroyRangeRequest) -> ::grpcio::Result<super::kvrpcpb::UnsafeDestroyRangeResponse> {
        self.unsafe_destroy_range_opt(req, ::grpcio::CallOption::default())
    }

    pub fn unsafe_destroy_range_async_opt(&self, req: &super::kvrpcpb::UnsafeDestroyRangeRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::UnsafeDestroyRangeResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_UNSAFE_DESTROY_RANGE, req, opt)
    }

    pub fn unsafe_destroy_range_async(&self, req: &super::kvrpcpb::UnsafeDestroyRangeRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::UnsafeDestroyRangeResponse>> {
        self.unsafe_destroy_range_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn register_lock_observer_opt(&self, req: &super::kvrpcpb::RegisterLockObserverRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::RegisterLockObserverResponse> {
        self.client.unary_call(&METHOD_TIKV_REGISTER_LOCK_OBSERVER, req, opt)
    }

    pub fn register_lock_observer(&self, req: &super::kvrpcpb::RegisterLockObserverRequest) -> ::grpcio::Result<super::kvrpcpb::RegisterLockObserverResponse> {
        self.register_lock_observer_opt(req, ::grpcio::CallOption::default())
    }

    pub fn register_lock_observer_async_opt(&self, req: &super::kvrpcpb::RegisterLockObserverRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RegisterLockObserverResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_REGISTER_LOCK_OBSERVER, req, opt)
    }

    pub fn register_lock_observer_async(&self, req: &super::kvrpcpb::RegisterLockObserverRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RegisterLockObserverResponse>> {
        self.register_lock_observer_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn check_lock_observer_opt(&self, req: &super::kvrpcpb::CheckLockObserverRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::CheckLockObserverResponse> {
        self.client.unary_call(&METHOD_TIKV_CHECK_LOCK_OBSERVER, req, opt)
    }

    pub fn check_lock_observer(&self, req: &super::kvrpcpb::CheckLockObserverRequest) -> ::grpcio::Result<super::kvrpcpb::CheckLockObserverResponse> {
        self.check_lock_observer_opt(req, ::grpcio::CallOption::default())
    }

    pub fn check_lock_observer_async_opt(&self, req: &super::kvrpcpb::CheckLockObserverRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::CheckLockObserverResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_CHECK_LOCK_OBSERVER, req, opt)
    }

    pub fn check_lock_observer_async(&self, req: &super::kvrpcpb::CheckLockObserverRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::CheckLockObserverResponse>> {
        self.check_lock_observer_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn remove_lock_observer_opt(&self, req: &super::kvrpcpb::RemoveLockObserverRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::RemoveLockObserverResponse> {
        self.client.unary_call(&METHOD_TIKV_REMOVE_LOCK_OBSERVER, req, opt)
    }

    pub fn remove_lock_observer(&self, req: &super::kvrpcpb::RemoveLockObserverRequest) -> ::grpcio::Result<super::kvrpcpb::RemoveLockObserverResponse> {
        self.remove_lock_observer_opt(req, ::grpcio::CallOption::default())
    }

    pub fn remove_lock_observer_async_opt(&self, req: &super::kvrpcpb::RemoveLockObserverRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RemoveLockObserverResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_REMOVE_LOCK_OBSERVER, req, opt)
    }

    pub fn remove_lock_observer_async(&self, req: &super::kvrpcpb::RemoveLockObserverRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RemoveLockObserverResponse>> {
        self.remove_lock_observer_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn physical_scan_lock_opt(&self, req: &super::kvrpcpb::PhysicalScanLockRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::PhysicalScanLockResponse> {
        self.client.unary_call(&METHOD_TIKV_PHYSICAL_SCAN_LOCK, req, opt)
    }

    pub fn physical_scan_lock(&self, req: &super::kvrpcpb::PhysicalScanLockRequest) -> ::grpcio::Result<super::kvrpcpb::PhysicalScanLockResponse> {
        self.physical_scan_lock_opt(req, ::grpcio::CallOption::default())
    }

    pub fn physical_scan_lock_async_opt(&self, req: &super::kvrpcpb::PhysicalScanLockRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::PhysicalScanLockResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_PHYSICAL_SCAN_LOCK, req, opt)
    }

    pub fn physical_scan_lock_async(&self, req: &super::kvrpcpb::PhysicalScanLockRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::PhysicalScanLockResponse>> {
        self.physical_scan_lock_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn coprocessor_opt(&self, req: &super::coprocessor::Request, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::coprocessor::Response> {
        self.client.unary_call(&METHOD_TIKV_COPROCESSOR, req, opt)
    }

    pub fn coprocessor(&self, req: &super::coprocessor::Request) -> ::grpcio::Result<super::coprocessor::Response> {
        self.coprocessor_opt(req, ::grpcio::CallOption::default())
    }

    pub fn coprocessor_async_opt(&self, req: &super::coprocessor::Request, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::coprocessor::Response>> {
        self.client.unary_call_async(&METHOD_TIKV_COPROCESSOR, req, opt)
    }

    pub fn coprocessor_async(&self, req: &super::coprocessor::Request) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::coprocessor::Response>> {
        self.coprocessor_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn coprocessor_stream_opt(&self, req: &super::coprocessor::Request, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::coprocessor::Response>> {
        self.client.server_streaming(&METHOD_TIKV_COPROCESSOR_STREAM, req, opt)
    }

    pub fn coprocessor_stream(&self, req: &super::coprocessor::Request) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::coprocessor::Response>> {
        self.coprocessor_stream_opt(req, ::grpcio::CallOption::default())
    }

    pub fn batch_coprocessor_opt(&self, req: &super::coprocessor::BatchRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::coprocessor::BatchResponse>> {
        self.client.server_streaming(&METHOD_TIKV_BATCH_COPROCESSOR, req, opt)
    }

    pub fn batch_coprocessor(&self, req: &super::coprocessor::BatchRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::coprocessor::BatchResponse>> {
        self.batch_coprocessor_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raw_coprocessor_opt(&self, req: &super::kvrpcpb::RawCoprocessorRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::RawCoprocessorResponse> {
        self.client.unary_call(&METHOD_TIKV_RAW_COPROCESSOR, req, opt)
    }

    pub fn raw_coprocessor(&self, req: &super::kvrpcpb::RawCoprocessorRequest) -> ::grpcio::Result<super::kvrpcpb::RawCoprocessorResponse> {
        self.raw_coprocessor_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raw_coprocessor_async_opt(&self, req: &super::kvrpcpb::RawCoprocessorRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RawCoprocessorResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_RAW_COPROCESSOR, req, opt)
    }

    pub fn raw_coprocessor_async(&self, req: &super::kvrpcpb::RawCoprocessorRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::RawCoprocessorResponse>> {
        self.raw_coprocessor_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn raft_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::raft_serverpb::RaftMessage>, ::grpcio::ClientCStreamReceiver<super::raft_serverpb::Done>)> {
        self.client.client_streaming(&METHOD_TIKV_RAFT, opt)
    }

    pub fn raft(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::raft_serverpb::RaftMessage>, ::grpcio::ClientCStreamReceiver<super::raft_serverpb::Done>)> {
        self.raft_opt(::grpcio::CallOption::default())
    }

    pub fn batch_raft_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::tikvpb::BatchRaftMessage>, ::grpcio::ClientCStreamReceiver<super::raft_serverpb::Done>)> {
        self.client.client_streaming(&METHOD_TIKV_BATCH_RAFT, opt)
    }

    pub fn batch_raft(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::tikvpb::BatchRaftMessage>, ::grpcio::ClientCStreamReceiver<super::raft_serverpb::Done>)> {
        self.batch_raft_opt(::grpcio::CallOption::default())
    }

    pub fn snapshot_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::raft_serverpb::SnapshotChunk>, ::grpcio::ClientCStreamReceiver<super::raft_serverpb::Done>)> {
        self.client.client_streaming(&METHOD_TIKV_SNAPSHOT, opt)
    }

    pub fn snapshot(&self) -> ::grpcio::Result<(::grpcio::ClientCStreamSender<super::raft_serverpb::SnapshotChunk>, ::grpcio::ClientCStreamReceiver<super::raft_serverpb::Done>)> {
        self.snapshot_opt(::grpcio::CallOption::default())
    }

    pub fn split_region_opt(&self, req: &super::kvrpcpb::SplitRegionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::SplitRegionResponse> {
        self.client.unary_call(&METHOD_TIKV_SPLIT_REGION, req, opt)
    }

    pub fn split_region(&self, req: &super::kvrpcpb::SplitRegionRequest) -> ::grpcio::Result<super::kvrpcpb::SplitRegionResponse> {
        self.split_region_opt(req, ::grpcio::CallOption::default())
    }

    pub fn split_region_async_opt(&self, req: &super::kvrpcpb::SplitRegionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::SplitRegionResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_SPLIT_REGION, req, opt)
    }

    pub fn split_region_async(&self, req: &super::kvrpcpb::SplitRegionRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::SplitRegionResponse>> {
        self.split_region_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn read_index_opt(&self, req: &super::kvrpcpb::ReadIndexRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::ReadIndexResponse> {
        self.client.unary_call(&METHOD_TIKV_READ_INDEX, req, opt)
    }

    pub fn read_index(&self, req: &super::kvrpcpb::ReadIndexRequest) -> ::grpcio::Result<super::kvrpcpb::ReadIndexResponse> {
        self.read_index_opt(req, ::grpcio::CallOption::default())
    }

    pub fn read_index_async_opt(&self, req: &super::kvrpcpb::ReadIndexRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::ReadIndexResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_READ_INDEX, req, opt)
    }

    pub fn read_index_async(&self, req: &super::kvrpcpb::ReadIndexRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::ReadIndexResponse>> {
        self.read_index_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn mvcc_get_by_key_opt(&self, req: &super::kvrpcpb::MvccGetByKeyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::MvccGetByKeyResponse> {
        self.client.unary_call(&METHOD_TIKV_MVCC_GET_BY_KEY, req, opt)
    }

    pub fn mvcc_get_by_key(&self, req: &super::kvrpcpb::MvccGetByKeyRequest) -> ::grpcio::Result<super::kvrpcpb::MvccGetByKeyResponse> {
        self.mvcc_get_by_key_opt(req, ::grpcio::CallOption::default())
    }

    pub fn mvcc_get_by_key_async_opt(&self, req: &super::kvrpcpb::MvccGetByKeyRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::MvccGetByKeyResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_MVCC_GET_BY_KEY, req, opt)
    }

    pub fn mvcc_get_by_key_async(&self, req: &super::kvrpcpb::MvccGetByKeyRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::MvccGetByKeyResponse>> {
        self.mvcc_get_by_key_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn mvcc_get_by_start_ts_opt(&self, req: &super::kvrpcpb::MvccGetByStartTsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::MvccGetByStartTsResponse> {
        self.client.unary_call(&METHOD_TIKV_MVCC_GET_BY_START_TS, req, opt)
    }

    pub fn mvcc_get_by_start_ts(&self, req: &super::kvrpcpb::MvccGetByStartTsRequest) -> ::grpcio::Result<super::kvrpcpb::MvccGetByStartTsResponse> {
        self.mvcc_get_by_start_ts_opt(req, ::grpcio::CallOption::default())
    }

    pub fn mvcc_get_by_start_ts_async_opt(&self, req: &super::kvrpcpb::MvccGetByStartTsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::MvccGetByStartTsResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_MVCC_GET_BY_START_TS, req, opt)
    }

    pub fn mvcc_get_by_start_ts_async(&self, req: &super::kvrpcpb::MvccGetByStartTsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::MvccGetByStartTsResponse>> {
        self.mvcc_get_by_start_ts_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn batch_commands_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::tikvpb::BatchCommandsRequest>, ::grpcio::ClientDuplexReceiver<super::tikvpb::BatchCommandsResponse>)> {
        self.client.duplex_streaming(&METHOD_TIKV_BATCH_COMMANDS, opt)
    }

    pub fn batch_commands(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::tikvpb::BatchCommandsRequest>, ::grpcio::ClientDuplexReceiver<super::tikvpb::BatchCommandsResponse>)> {
        self.batch_commands_opt(::grpcio::CallOption::default())
    }

    pub fn dispatch_mpp_task_opt(&self, req: &super::mpp::DispatchTaskRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mpp::DispatchTaskResponse> {
        self.client.unary_call(&METHOD_TIKV_DISPATCH_MPP_TASK, req, opt)
    }

    pub fn dispatch_mpp_task(&self, req: &super::mpp::DispatchTaskRequest) -> ::grpcio::Result<super::mpp::DispatchTaskResponse> {
        self.dispatch_mpp_task_opt(req, ::grpcio::CallOption::default())
    }

    pub fn dispatch_mpp_task_async_opt(&self, req: &super::mpp::DispatchTaskRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mpp::DispatchTaskResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_DISPATCH_MPP_TASK, req, opt)
    }

    pub fn dispatch_mpp_task_async(&self, req: &super::mpp::DispatchTaskRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mpp::DispatchTaskResponse>> {
        self.dispatch_mpp_task_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cancel_mpp_task_opt(&self, req: &super::mpp::CancelTaskRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mpp::CancelTaskResponse> {
        self.client.unary_call(&METHOD_TIKV_CANCEL_MPP_TASK, req, opt)
    }

    pub fn cancel_mpp_task(&self, req: &super::mpp::CancelTaskRequest) -> ::grpcio::Result<super::mpp::CancelTaskResponse> {
        self.cancel_mpp_task_opt(req, ::grpcio::CallOption::default())
    }

    pub fn cancel_mpp_task_async_opt(&self, req: &super::mpp::CancelTaskRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mpp::CancelTaskResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_CANCEL_MPP_TASK, req, opt)
    }

    pub fn cancel_mpp_task_async(&self, req: &super::mpp::CancelTaskRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mpp::CancelTaskResponse>> {
        self.cancel_mpp_task_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn establish_mpp_connection_opt(&self, req: &super::mpp::EstablishMppConnectionRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::mpp::MppDataPacket>> {
        self.client.server_streaming(&METHOD_TIKV_ESTABLISH_MPP_CONNECTION, req, opt)
    }

    pub fn establish_mpp_connection(&self, req: &super::mpp::EstablishMppConnectionRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::mpp::MppDataPacket>> {
        self.establish_mpp_connection_opt(req, ::grpcio::CallOption::default())
    }

    pub fn is_alive_opt(&self, req: &super::mpp::IsAliveRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::mpp::IsAliveResponse> {
        self.client.unary_call(&METHOD_TIKV_IS_ALIVE, req, opt)
    }

    pub fn is_alive(&self, req: &super::mpp::IsAliveRequest) -> ::grpcio::Result<super::mpp::IsAliveResponse> {
        self.is_alive_opt(req, ::grpcio::CallOption::default())
    }

    pub fn is_alive_async_opt(&self, req: &super::mpp::IsAliveRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mpp::IsAliveResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_IS_ALIVE, req, opt)
    }

    pub fn is_alive_async(&self, req: &super::mpp::IsAliveRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::mpp::IsAliveResponse>> {
        self.is_alive_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn check_leader_opt(&self, req: &super::kvrpcpb::CheckLeaderRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::CheckLeaderResponse> {
        self.client.unary_call(&METHOD_TIKV_CHECK_LEADER, req, opt)
    }

    pub fn check_leader(&self, req: &super::kvrpcpb::CheckLeaderRequest) -> ::grpcio::Result<super::kvrpcpb::CheckLeaderResponse> {
        self.check_leader_opt(req, ::grpcio::CallOption::default())
    }

    pub fn check_leader_async_opt(&self, req: &super::kvrpcpb::CheckLeaderRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::CheckLeaderResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_CHECK_LEADER, req, opt)
    }

    pub fn check_leader_async(&self, req: &super::kvrpcpb::CheckLeaderRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::CheckLeaderResponse>> {
        self.check_leader_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_store_safe_ts_opt(&self, req: &super::kvrpcpb::StoreSafeTsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::StoreSafeTsResponse> {
        self.client.unary_call(&METHOD_TIKV_GET_STORE_SAFE_TS, req, opt)
    }

    pub fn get_store_safe_ts(&self, req: &super::kvrpcpb::StoreSafeTsRequest) -> ::grpcio::Result<super::kvrpcpb::StoreSafeTsResponse> {
        self.get_store_safe_ts_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_store_safe_ts_async_opt(&self, req: &super::kvrpcpb::StoreSafeTsRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::StoreSafeTsResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_GET_STORE_SAFE_TS, req, opt)
    }

    pub fn get_store_safe_ts_async(&self, req: &super::kvrpcpb::StoreSafeTsRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::StoreSafeTsResponse>> {
        self.get_store_safe_ts_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_lock_wait_info_opt(&self, req: &super::kvrpcpb::GetLockWaitInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::GetLockWaitInfoResponse> {
        self.client.unary_call(&METHOD_TIKV_GET_LOCK_WAIT_INFO, req, opt)
    }

    pub fn get_lock_wait_info(&self, req: &super::kvrpcpb::GetLockWaitInfoRequest) -> ::grpcio::Result<super::kvrpcpb::GetLockWaitInfoResponse> {
        self.get_lock_wait_info_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_lock_wait_info_async_opt(&self, req: &super::kvrpcpb::GetLockWaitInfoRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::GetLockWaitInfoResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_GET_LOCK_WAIT_INFO, req, opt)
    }

    pub fn get_lock_wait_info_async(&self, req: &super::kvrpcpb::GetLockWaitInfoRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::GetLockWaitInfoResponse>> {
        self.get_lock_wait_info_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn compact_opt(&self, req: &super::kvrpcpb::CompactRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvrpcpb::CompactResponse> {
        self.client.unary_call(&METHOD_TIKV_COMPACT, req, opt)
    }

    pub fn compact(&self, req: &super::kvrpcpb::CompactRequest) -> ::grpcio::Result<super::kvrpcpb::CompactResponse> {
        self.compact_opt(req, ::grpcio::CallOption::default())
    }

    pub fn compact_async_opt(&self, req: &super::kvrpcpb::CompactRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::CompactResponse>> {
        self.client.unary_call_async(&METHOD_TIKV_COMPACT, req, opt)
    }

    pub fn compact_async(&self, req: &super::kvrpcpb::CompactRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvrpcpb::CompactResponse>> {
        self.compact_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::std::future::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Tikv {
    fn kv_get(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::GetRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::GetResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn kv_scan(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::ScanRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::ScanResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn kv_prewrite(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::PrewriteRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::PrewriteResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn kv_pessimistic_lock(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::PessimisticLockRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::PessimisticLockResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn kv_pessimistic_rollback(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::PessimisticRollbackRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::PessimisticRollbackResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn kv_txn_heart_beat(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::TxnHeartBeatRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::TxnHeartBeatResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn kv_check_txn_status(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::CheckTxnStatusRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::CheckTxnStatusResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn kv_check_secondary_locks(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::CheckSecondaryLocksRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::CheckSecondaryLocksResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn kv_commit(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::CommitRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::CommitResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn kv_import(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::ImportRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::ImportResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn kv_cleanup(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::CleanupRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::CleanupResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn kv_batch_get(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::BatchGetRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::BatchGetResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn kv_batch_rollback(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::BatchRollbackRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::BatchRollbackResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn kv_scan_lock(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::ScanLockRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::ScanLockResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn kv_resolve_lock(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::ResolveLockRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::ResolveLockResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn kv_gc(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::GcRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::GcResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn kv_delete_range(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::DeleteRangeRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::DeleteRangeResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn raw_get(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::RawGetRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::RawGetResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn raw_batch_get(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::RawBatchGetRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::RawBatchGetResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn raw_put(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::RawPutRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::RawPutResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn raw_batch_put(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::RawBatchPutRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::RawBatchPutResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn raw_delete(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::RawDeleteRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::RawDeleteResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn raw_batch_delete(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::RawBatchDeleteRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::RawBatchDeleteResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn raw_scan(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::RawScanRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::RawScanResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn raw_delete_range(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::RawDeleteRangeRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::RawDeleteRangeResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn raw_batch_scan(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::RawBatchScanRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::RawBatchScanResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn raw_get_key_ttl(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::RawGetKeyTtlRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::RawGetKeyTtlResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn raw_compare_and_swap(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::RawCasRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::RawCasResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn raw_checksum(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::RawChecksumRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::RawChecksumResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn unsafe_destroy_range(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::UnsafeDestroyRangeRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::UnsafeDestroyRangeResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn register_lock_observer(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::RegisterLockObserverRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::RegisterLockObserverResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn check_lock_observer(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::CheckLockObserverRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::CheckLockObserverResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn remove_lock_observer(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::RemoveLockObserverRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::RemoveLockObserverResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn physical_scan_lock(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::PhysicalScanLockRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::PhysicalScanLockResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn coprocessor(&mut self, ctx: ::grpcio::RpcContext, _req: super::coprocessor::Request, sink: ::grpcio::UnarySink<super::coprocessor::Response>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn coprocessor_stream(&mut self, ctx: ::grpcio::RpcContext, _req: super::coprocessor::Request, sink: ::grpcio::ServerStreamingSink<super::coprocessor::Response>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn batch_coprocessor(&mut self, ctx: ::grpcio::RpcContext, _req: super::coprocessor::BatchRequest, sink: ::grpcio::ServerStreamingSink<super::coprocessor::BatchResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn raw_coprocessor(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::RawCoprocessorRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::RawCoprocessorResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn raft(&mut self, ctx: ::grpcio::RpcContext, _stream: ::grpcio::RequestStream<super::raft_serverpb::RaftMessage>, sink: ::grpcio::ClientStreamingSink<super::raft_serverpb::Done>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn batch_raft(&mut self, ctx: ::grpcio::RpcContext, _stream: ::grpcio::RequestStream<super::tikvpb::BatchRaftMessage>, sink: ::grpcio::ClientStreamingSink<super::raft_serverpb::Done>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn snapshot(&mut self, ctx: ::grpcio::RpcContext, _stream: ::grpcio::RequestStream<super::raft_serverpb::SnapshotChunk>, sink: ::grpcio::ClientStreamingSink<super::raft_serverpb::Done>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn split_region(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::SplitRegionRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::SplitRegionResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn read_index(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::ReadIndexRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::ReadIndexResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn mvcc_get_by_key(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::MvccGetByKeyRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::MvccGetByKeyResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn mvcc_get_by_start_ts(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::MvccGetByStartTsRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::MvccGetByStartTsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn batch_commands(&mut self, ctx: ::grpcio::RpcContext, _stream: ::grpcio::RequestStream<super::tikvpb::BatchCommandsRequest>, sink: ::grpcio::DuplexSink<super::tikvpb::BatchCommandsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn dispatch_mpp_task(&mut self, ctx: ::grpcio::RpcContext, _req: super::mpp::DispatchTaskRequest, sink: ::grpcio::UnarySink<super::mpp::DispatchTaskResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn cancel_mpp_task(&mut self, ctx: ::grpcio::RpcContext, _req: super::mpp::CancelTaskRequest, sink: ::grpcio::UnarySink<super::mpp::CancelTaskResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn establish_mpp_connection(&mut self, ctx: ::grpcio::RpcContext, _req: super::mpp::EstablishMppConnectionRequest, sink: ::grpcio::ServerStreamingSink<super::mpp::MppDataPacket>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn is_alive(&mut self, ctx: ::grpcio::RpcContext, _req: super::mpp::IsAliveRequest, sink: ::grpcio::UnarySink<super::mpp::IsAliveResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn check_leader(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::CheckLeaderRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::CheckLeaderResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_store_safe_ts(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::StoreSafeTsRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::StoreSafeTsResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn get_lock_wait_info(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::GetLockWaitInfoRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::GetLockWaitInfoResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
    fn compact(&mut self, ctx: ::grpcio::RpcContext, _req: super::kvrpcpb::CompactRequest, sink: ::grpcio::UnarySink<super::kvrpcpb::CompactResponse>) {
        grpcio::unimplemented_call!(ctx, sink)
    }
}

pub fn create_tikv<S: Tikv + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_KV_GET, move |ctx, req, resp| {
        instance.kv_get(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_KV_SCAN, move |ctx, req, resp| {
        instance.kv_scan(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_KV_PREWRITE, move |ctx, req, resp| {
        instance.kv_prewrite(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_KV_PESSIMISTIC_LOCK, move |ctx, req, resp| {
        instance.kv_pessimistic_lock(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_KV_PESSIMISTIC_ROLLBACK, move |ctx, req, resp| {
        instance.kv_pessimistic_rollback(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_KV_TXN_HEART_BEAT, move |ctx, req, resp| {
        instance.kv_txn_heart_beat(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_KV_CHECK_TXN_STATUS, move |ctx, req, resp| {
        instance.kv_check_txn_status(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_KV_CHECK_SECONDARY_LOCKS, move |ctx, req, resp| {
        instance.kv_check_secondary_locks(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_KV_COMMIT, move |ctx, req, resp| {
        instance.kv_commit(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_KV_IMPORT, move |ctx, req, resp| {
        instance.kv_import(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_KV_CLEANUP, move |ctx, req, resp| {
        instance.kv_cleanup(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_KV_BATCH_GET, move |ctx, req, resp| {
        instance.kv_batch_get(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_KV_BATCH_ROLLBACK, move |ctx, req, resp| {
        instance.kv_batch_rollback(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_KV_SCAN_LOCK, move |ctx, req, resp| {
        instance.kv_scan_lock(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_KV_RESOLVE_LOCK, move |ctx, req, resp| {
        instance.kv_resolve_lock(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_KV_GC, move |ctx, req, resp| {
        instance.kv_gc(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_KV_DELETE_RANGE, move |ctx, req, resp| {
        instance.kv_delete_range(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_RAW_GET, move |ctx, req, resp| {
        instance.raw_get(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_RAW_BATCH_GET, move |ctx, req, resp| {
        instance.raw_batch_get(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_RAW_PUT, move |ctx, req, resp| {
        instance.raw_put(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_RAW_BATCH_PUT, move |ctx, req, resp| {
        instance.raw_batch_put(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_RAW_DELETE, move |ctx, req, resp| {
        instance.raw_delete(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_RAW_BATCH_DELETE, move |ctx, req, resp| {
        instance.raw_batch_delete(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_RAW_SCAN, move |ctx, req, resp| {
        instance.raw_scan(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_RAW_DELETE_RANGE, move |ctx, req, resp| {
        instance.raw_delete_range(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_RAW_BATCH_SCAN, move |ctx, req, resp| {
        instance.raw_batch_scan(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_RAW_GET_KEY_TTL, move |ctx, req, resp| {
        instance.raw_get_key_ttl(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_RAW_COMPARE_AND_SWAP, move |ctx, req, resp| {
        instance.raw_compare_and_swap(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_RAW_CHECKSUM, move |ctx, req, resp| {
        instance.raw_checksum(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_UNSAFE_DESTROY_RANGE, move |ctx, req, resp| {
        instance.unsafe_destroy_range(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_REGISTER_LOCK_OBSERVER, move |ctx, req, resp| {
        instance.register_lock_observer(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_CHECK_LOCK_OBSERVER, move |ctx, req, resp| {
        instance.check_lock_observer(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_REMOVE_LOCK_OBSERVER, move |ctx, req, resp| {
        instance.remove_lock_observer(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_PHYSICAL_SCAN_LOCK, move |ctx, req, resp| {
        instance.physical_scan_lock(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_COPROCESSOR, move |ctx, req, resp| {
        instance.coprocessor(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_TIKV_COPROCESSOR_STREAM, move |ctx, req, resp| {
        instance.coprocessor_stream(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_TIKV_BATCH_COPROCESSOR, move |ctx, req, resp| {
        instance.batch_coprocessor(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_RAW_COPROCESSOR, move |ctx, req, resp| {
        instance.raw_coprocessor(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_TIKV_RAFT, move |ctx, req, resp| {
        instance.raft(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_TIKV_BATCH_RAFT, move |ctx, req, resp| {
        instance.batch_raft(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_client_streaming_handler(&METHOD_TIKV_SNAPSHOT, move |ctx, req, resp| {
        instance.snapshot(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_SPLIT_REGION, move |ctx, req, resp| {
        instance.split_region(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_READ_INDEX, move |ctx, req, resp| {
        instance.read_index(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_MVCC_GET_BY_KEY, move |ctx, req, resp| {
        instance.mvcc_get_by_key(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_MVCC_GET_BY_START_TS, move |ctx, req, resp| {
        instance.mvcc_get_by_start_ts(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_TIKV_BATCH_COMMANDS, move |ctx, req, resp| {
        instance.batch_commands(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_DISPATCH_MPP_TASK, move |ctx, req, resp| {
        instance.dispatch_mpp_task(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_CANCEL_MPP_TASK, move |ctx, req, resp| {
        instance.cancel_mpp_task(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_TIKV_ESTABLISH_MPP_CONNECTION, move |ctx, req, resp| {
        instance.establish_mpp_connection(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_IS_ALIVE, move |ctx, req, resp| {
        instance.is_alive(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_CHECK_LEADER, move |ctx, req, resp| {
        instance.check_leader(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_GET_STORE_SAFE_TS, move |ctx, req, resp| {
        instance.get_store_safe_ts(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_TIKV_GET_LOCK_WAIT_INFO, move |ctx, req, resp| {
        instance.get_lock_wait_info(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_unary_handler(&METHOD_TIKV_COMPACT, move |ctx, req, resp| {
        instance.compact(ctx, req, resp)
    });
    builder.build()
}
