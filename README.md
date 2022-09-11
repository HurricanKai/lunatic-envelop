# Lunatic-Envelop

A simple library providing a few helper functions for offloading large binary blobs onto another process, reducing overhead on long message chains.

Useful when transfering typical binary content without touching it, for example images, encrypted data, or WASM bytecode.

This _will_ be inefficient if the envelop is only passed across a process boundary once, as creating & opening the process will copy the data once.

Also extremely useful for cases where it's unclear how often a message will be passed, replacing a somehow scaling message count with a constant number. For example in binary-tree balancing process structures.
