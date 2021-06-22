use crate::proof::ProvingContext;

#[no_mangle]
pub extern "C" fn librust_proving_ctx_init() -> *mut ProvingContext {
    let ctx = Box::new(ProvingContext::new());

    Box::into_raw(ctx)
}

#[no_mangle]
pub extern "C" fn librust_proof() -> bool {
    true
}

#[no_mangle]
pub extern "C" fn librust_proving_ctx_free(_ctx: *mut ProvingContext) {

}