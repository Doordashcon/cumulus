window.SIDEBAR_ITEMS = {"enum":[["AccountKeyring","Set of test accounts."],["BlocksPruning","Block pruning settings."],["Ed25519Keyring","Set of test accounts."],["ExecutionStrategy","Strategy for executing a call into the runtime."],["Sr25519Keyring","Set of test accounts."],["WasmExecutionMethod","Specification of different methods of executing the runtime Wasm code."]],"fn":[["generate_extrinsic","Generate an extrinsic from the provided function call, origin and [`Client`]."],["transfer","Transfer some token from one account to another using a provided test [`Client`]."],["validate_block","Call `validate_block` in the given `wasm_blob`."]],"mod":[["client","Substrate Client and associated logic."],["client_ext","Client extension for tests."],["sc_client_db","Client backend that is backed by a database."],["sc_executor","A crate that provides means of executing/dispatching calls into the runtime."],["sp_consensus","Common utilities for building and using consensus engines in substrate."]],"struct":[["BlockData","Parachain block data."],["ExecutionExtensions","A producer of execution extensions for offchain calls."],["ExecutionStrategies","Execution strategies settings."],["GenesisParameters","Parameters of test-client builder with test-runtime."],["HeadData","Parachain head data included in the chain."],["LocalExecutor","Native executor used for tests. Native executor instance."],["NativeElseWasmExecutor","A generic `CodeExecutor` implementation that uses a delegate to determine wasm code equivalence and dispatch to native code when possible, falling back on `WasmExecutor` when not."],["RpcHandlers","RPC handlers that can perform RPC queries."],["RpcTransactionError","An error for when the RPC call fails."],["RpcTransactionOutput","The output of an RPC transaction."],["Storage","Struct containing data needed for a storage."],["StorageChild","Child trie storage data."],["ValidationParams","Validation parameters for evaluating the parachain validity function."],["ValidationResult","The result of parachain validation."],["WasmExecutor","An abstraction over Wasm code executor. Supports selecting execution backend and manages runtime cache."]],"trait":[["BlockchainEventsExt","An extension trait for `BlockchainEvents`."],["BuildParachainBlockData","Extension trait for the `BlockBuilder` to build directly a [`ParachainBlockData`]."],["ClientBlockImportExt","Extension trait for a test client around block importing."],["ClientExt","Extension trait for a test client."],["DefaultTestClientBuilderExt","A `TestClientBuilder` with default backend and executor."],["GenesisInit","A genesis storage initialization trait."],["InitBlockBuilder","An extension for the Cumulus test client to init a block builder."],["Keystore","Something that generates, stores and provides access to secret keys."],["RpcHandlersExt","An extension trait for `RpcHandlers`."],["TestClientBuilderExt","A `test-runtime` extensions to `TestClientBuilder`."]],"type":[["Backend","Test client database backend."],["BadBlocks","Known bad block hashes."],["Client","Test client type with `LocalExecutor` and generic Backend."],["Executor","Test client executor."],["ExecutorResult","Result type alias."],["ForkBlocks","Expected hashes of blocks at given heights."],["KeystorePtr","A shared pointer to a keystore implementation."],["LongestChain","LongestChain type for the test runtime/client."],["ParachainBlockData",""],["TestClientBuilder","Test client builder for Cumulus"]]};