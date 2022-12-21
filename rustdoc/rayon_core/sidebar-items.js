window.SIDEBAR_ITEMS = {"fn":[["broadcast","Executes `op` within every thread in the current threadpool. If this is called from a non-Rayon thread, it will execute in the global threadpool. Any attempts to use `join`, `scope`, or parallel iterators will then operate within that threadpool. When the call has completed on each thread, returns a vector containing all of their return values."],["current_num_threads","Returns the number of threads in the current registry. If this code is executing within a Rayon thread-pool, then this will be the number of threads for the thread-pool of the current thread. Otherwise, it will be the number of threads for the global thread-pool."],["current_thread_has_pending_tasks","If called from a Rayon worker thread, indicates whether that thread’s local deque still has pending tasks. Otherwise, returns `None`. For more information, see the `ThreadPool::current_thread_has_pending_tasks()` method."],["current_thread_index","If called from a Rayon worker thread, returns the index of that thread within its current pool; if not called from a Rayon thread, returns `None`."],["in_place_scope","Creates a “fork-join” scope `s` and invokes the closure with a reference to `s`. This closure can then spawn asynchronous tasks into `s`. Those tasks may run asynchronously with respect to the closure; they may themselves spawn additional tasks into `s`. When the closure returns, it will block until all tasks that have been spawned into `s` complete."],["in_place_scope_fifo","Creates a “fork-join” scope `s` with FIFO order, and invokes the closure with a reference to `s`. This closure can then spawn asynchronous tasks into `s`. Those tasks may run asynchronously with respect to the closure; they may themselves spawn additional tasks into `s`. When the closure returns, it will block until all tasks that have been spawned into `s` complete."],["initialize","Deprecated in favor of `ThreadPoolBuilder::build_global`."],["join","Takes two closures and potentially runs them in parallel. It returns a pair of the results from those closures."],["join_context","Identical to `join`, except that the closures have a parameter that provides context for the way the closure has been called, especially indicating whether they’re executing on a different thread than where `join_context` was called.  This will occur if the second job is stolen by a different thread, or if `join_context` was called from outside the thread pool to begin with."],["max_num_threads","Returns the maximum number of threads that Rayon supports in a single thread-pool."],["scope","Creates a “fork-join” scope `s` and invokes the closure with a reference to `s`. This closure can then spawn asynchronous tasks into `s`. Those tasks may run asynchronously with respect to the closure; they may themselves spawn additional tasks into `s`. When the closure returns, it will block until all tasks that have been spawned into `s` complete."],["scope_fifo","Creates a “fork-join” scope `s` with FIFO order, and invokes the closure with a reference to `s`. This closure can then spawn asynchronous tasks into `s`. Those tasks may run asynchronously with respect to the closure; they may themselves spawn additional tasks into `s`. When the closure returns, it will block until all tasks that have been spawned into `s` complete."],["spawn","Fires off a task into the Rayon threadpool in the “static” or “global” scope.  Just like a standard thread, this task is not tied to the current stack frame, and hence it cannot hold any references other than those with `'static` lifetime. If you want to spawn a task that references stack data, use the `scope()` function to create a scope."],["spawn_broadcast","Spawns an asynchronous task on every thread in this thread-pool. This task will run in the implicit, global scope, which means that it may outlast the current stack frame – therefore, it cannot capture any references onto the stack (you will likely need a `move` closure)."],["spawn_fifo","Fires off a task into the Rayon threadpool in the “static” or “global” scope.  Just like a standard thread, this task is not tied to the current stack frame, and hence it cannot hold any references other than those with `'static` lifetime. If you want to spawn a task that references stack data, use the `scope_fifo()` function to create a scope."]],"struct":[["BroadcastContext","Provides context to a closure called by `broadcast`."],["Configuration","Contains the rayon thread pool configuration. Use `ThreadPoolBuilder` instead."],["FnContext","Provides the calling context to a closure called by `join_context`."],["Scope","Represents a fork-join scope which can be used to spawn any number of tasks. See `scope()` for more information."],["ScopeFifo","Represents a fork-join scope which can be used to spawn any number of tasks. Those spawned from the same thread are prioritized in relative FIFO order. See `scope_fifo()` for more information."],["ThreadBuilder","Thread builder used for customization via `ThreadPoolBuilder::spawn_handler`."],["ThreadPool","Represents a user created thread-pool."],["ThreadPoolBuildError","Error when initializing a thread pool."],["ThreadPoolBuilder","Used to create a new `ThreadPool` or to configure the global rayon thread pool."]]};