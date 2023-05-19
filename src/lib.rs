#[allow(dead_code)]
#[allow(unknown_lints)]
#[allow(clippy::all)]
#[allow(renamed_and_removed_lints)]
#[allow(bare_trait_objects)]
#[allow(deprecated)]
mod protos {
    include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

    use raft_proto::eraftpb;
}

pub use protos::*;

pub mod cdc_adapt {
    #[cfg(not(feature = "prost-codec"))]
    pub mod pb {
        impl ::std::fmt::Debug for crate::cdcpb::Event_oneof_event {
            #[allow(unused_variables)]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let mut buf = String::new();
                match self {
                    crate::cdcpb::Event_oneof_event::Entries(v) => {
                        ::protobuf::PbPrint::fmt(v, "Entries", &mut buf)
                    }
                    crate::cdcpb::Event_oneof_event::Admin(v) => {
                        ::protobuf::PbPrint::fmt(v, "Admin", &mut buf)
                    }
                    crate::cdcpb::Event_oneof_event::Error(v) => {
                        ::protobuf::PbPrint::fmt(v, "Error", &mut buf)
                    }
                    crate::cdcpb::Event_oneof_event::ResolvedTs(v) => {
                        ::protobuf::PbPrint::fmt(v, "ResolvedTs", &mut buf)
                    }
                    crate::cdcpb::Event_oneof_event::LongTxn(v) => {
                        ::protobuf::PbPrint::fmt(v, "Long", &mut buf)
                    }
                }
                write!(f, "{}", buf)
            }
        }

        #[allow(dead_code)]
        fn assert_fmt_debug() {
            fn require_impl_debug<T: ::std::fmt::Debug>(_: T) {}
            require_impl_debug(crate::cdcpb::Event_oneof_event::Entries(
                ::std::default::Default::default(),
            ));
            require_impl_debug(crate::cdcpb::ChangeDataEvent::default());
        }
    }

    #[cfg(feature = "prost-codec")]
    pub mod prost {
        #[allow(dead_code)]
        fn assert_fmt_debug() {
            fn require_impl_debug<T: ::std::fmt::Debug>(_: T) {}
            require_impl_debug(crate::cdcpb::event::Event::Entries(
                ::std::default::Default::default(),
            ));
            require_impl_debug(crate::cdcpb::ChangeDataEvent::default());
        }
    }
}
