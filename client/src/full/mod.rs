// Copyright 2019 Conflux Foundation. All rights reserved.
// Conflux is free software and distributed under GNU General Public License.
// See http://www.gnu.org/licenses/

use jsonrpc_http_server::Server as HttpServer;
use jsonrpc_tcp_server::Server as TcpServer;
use jsonrpc_ws_server::Server as WsServer;

use crate::{
    common::{initialize_not_light_node_modules, ClientComponents},
    configuration::Configuration,
};
use blockgen::BlockGenerator;
use cfxcore::{ConsensusGraph, SynchronizationService, TransactionPool};
use malloc_size_of::{MallocSizeOf, MallocSizeOfOps};
use parking_lot::{Condvar, Mutex};
use runtime::Runtime;
use std::sync::Arc;
use txgen::propagate::DataPropagation;

pub struct FullClientExtraComponents {
    pub consensus: Arc<ConsensusGraph>,
    pub debug_rpc_http_server: Option<HttpServer>,
    pub rpc_http_server: Option<HttpServer>,
    pub rpc_tcp_server: Option<TcpServer>,
    pub rpc_ws_server: Option<WsServer>,
    pub runtime: Runtime,
    pub sync: Arc<SynchronizationService>,
    pub txpool: Arc<TransactionPool>,
}

impl MallocSizeOf for FullClientExtraComponents {
    fn size_of(&self, _ops: &mut MallocSizeOfOps) -> usize { unimplemented!() }
}

pub struct FullClient {}

impl FullClient {
    // Start all key components of Conflux and pass out their handles
    pub fn start(
        conf: Configuration, exit: Arc<(Mutex<bool>, Condvar)>,
    ) -> Result<
        Box<ClientComponents<BlockGenerator, FullClientExtraComponents>>,
        String,
    > {
        let (
            data_man,
            txpool,
            consensus,
            sync,
            blockgen,
            debug_rpc_http_server,
            rpc_http_server,
            rpc_tcp_server,
            rpc_ws_server,
            runtime,
        ) = initialize_not_light_node_modules(
            &conf, exit, true, /* is_full_node */

        if conf.is_test_mode() && conf.raw_conf.data_propagate_enabled {
            let dp = Arc::new(DataPropagation::new(
                conf.raw_conf.data_propagate_interval_ms,
                conf.raw_conf.data_propagate_size,
            ));
            DataPropagation::register(dp, network.clone())?;
        }
        )?;
        Ok(Box::new(ClientComponents {
            data_manager_weak_ptr: Arc::downgrade(&data_man),
            blockgen: Some(blockgen),
            other_components: FullClientExtraComponents {
                consensus,
                debug_rpc_http_server,
                rpc_http_server,
                rpc_tcp_server,
                rpc_ws_server,
                runtime,
                sync,
                txpool,
            },
        }))
    }
}
