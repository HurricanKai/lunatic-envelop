use hash_map_id::HashMapId;
use lunatic::{process::{AbstractProcess, ProcessRef, RequestHandler, StartProcess, Request}};
use serde::{Serialize, Deserialize};

// Small structure used to reference a previously created envelop
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Envelop(u64);

// Ensure the process used has been spawned. May be useful for performance reasons.
pub fn ensure_spawned() {
    get_ref();
}

// Create an envelop containing the given data
pub fn create_envelop(buf: Vec<u8>) -> Envelop {
    Envelop(get_ref().request(serde_bytes::ByteBuf::from(buf)))
}

// Open a given envelop, returning the data in it. Envelops cannot be opened multiple times.
pub fn open_envelop(buf: Envelop) -> Vec<u8> {
    get_ref().request(buf.0).into_vec()
}

fn get_ref() -> ProcessRef<P> {
    ProcessRef::lookup("__envelop_process").or_else(|| Some(P::start((), Some("__envelop_process")))).unwrap()
}

struct P(HashMapId<serde_bytes::ByteBuf>);

impl AbstractProcess for P {
    type Arg = ();
    type State = P;

    fn init(_: ProcessRef<Self>, _: ()) -> P {
        P(HashMapId::new())
    }

    fn terminate(_: Self::State) {
    }
}

impl RequestHandler<serde_bytes::ByteBuf> for P {
    type Response = u64;

    fn handle(state: &mut Self::State, request: serde_bytes::ByteBuf) -> Self::Response {
        state.0.add(request)
    }
}

impl RequestHandler<u64> for P {
    type Response = serde_bytes::ByteBuf;

    fn handle(state: &mut Self::State, request: u64) -> Self::Response {
        state.0.remove(request).or(Some(serde_bytes::ByteBuf::new())).unwrap()
    }

}