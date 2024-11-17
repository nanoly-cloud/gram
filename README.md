# gram

The idea of gram is to design an application, around user identities, interfacing over a single datascape.

gram, is a server that serves itself, and the files that are needed to build it.

I plan to build out to adding peer & data, connectity and ownership, and then maybe into a more distributed system.

## What?

In short I want to a build super simple universal executable environment, communicating over a single dataspace. Via distributed content addressed data hashing.

Universal modules.
- Interoperability (Wit IDL), for most major language, with any language being able to create bindings
- Execution: WASM, with many major languages being able to compile to wasm. Runs in browsers.
- Standardised: Map HTTP to POSIX.
- Componentised: Wasi's component model, maps well to the directory system designs, and component systems of nester now.
    For one, both are hierarchical, and trepresentable.
    For two, both are compostable.
    For three, they have the same versioning requirements.

Next:
- [ ] Implement process networking
- [ ] Implement users
- [ ] Implement datastore
- [ ] Implement confidence, decaying data lifetimes

