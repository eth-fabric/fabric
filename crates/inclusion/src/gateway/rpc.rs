/// Reference implementation handler that uses your existing `CommitmentsServerState<T>`.
///
/// Other implementations can ignore this type completely and define their own
/// handler structs and state, as long as they implement `CommitmentsRpcServer`.
#[derive(Clone)]
pub struct DefaultCommitmentsRpc<T> {
    pub state: Arc<CommitmentsServerState<T>>,
}

impl<T> DefaultCommitmentsRpc<T> {
    pub fn new(state: Arc<CommitmentsServerState<T>>) -> Self {
        Self { state }
    }
}

/// Implement the generated server trait for the reference handler.
///
/// The bodies are left as `todo!()` for now, so you can wire your existing logic
/// into them later.
#[async_trait]
impl<T> CommitmentsRpcServer for DefaultCommitmentsRpc<T>
where
    T: GatewayConfig + Send + Sync + 'static,
{
    async fn commitment_request(
        &self,
        _params: CommitmentRequestParams,
    ) -> RpcResult<CommitmentRequestResponse> {
        // Use `self.state` and fill in your logic here.
        // Map your internal error to `RpcResult` as needed.
        todo!()
    }

    async fn commitment_result(
        &self,
        _params: CommitmentResultParams,
    ) -> RpcResult<CommitmentResultResponse> {
        todo!()
    }

    async fn slots(&self) -> RpcResult<SlotsResponse> {
        todo!()
    }

    async fn fee(
        &self,
        _params: FeeParams,
    ) -> RpcResult<FeeResponse> {
        todo!()
    }

    async fn generate_proxy_key(
        &self,
        _params: GenerateProxyKeyParams,
    ) -> RpcResult<GenerateProxyKeyResponse> {
        todo!()
    }
}