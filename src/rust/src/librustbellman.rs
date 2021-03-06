use bellman::groth16;
use bls12_381::Bls12;
use libc::c_uchar;
use crate::proof::VerificationContext;
use crate::proof::ProvingContext;
use crate::utils::timelapse::TimeElapse;

static mut PVK: Option<groth16::PreparedVerifyingKey<Bls12>> = None;

const GROTH_PROOF_SIZE: usize = 
      48  // π_A
    + 96  // π_B
    + 48; // π_C

#[no_mangle]
pub extern "C" fn librust_proving_ctx_init() -> *mut ProvingContext {
    let ctx = Box::new(ProvingContext::new());
    Box::into_raw(ctx)
}

#[no_mangle]
pub extern "C" fn librust_proof(ctx: *mut ProvingContext, inputs: *const [c_uchar; 32], zkproof: *mut [c_uchar; GROTH_PROOF_SIZE]) -> bool {
    println!("\nstarted build proof...");

    let now = TimeElapse::new();

    let (proof, pvk) = unsafe { &mut *ctx }.spend_proof(unsafe { *inputs });

    let elapsed = now.elapsed();
    println!("build proof consume time : {} secs", elapsed);
 
    // Write proof out to caller
    proof
    .write(&mut (unsafe { &mut *zkproof })[..])
    .expect("should be able to serialize a proof");

    // Write pvk out to caller
    // use of mutable static is unsafe and requires unsafe function or block
    unsafe {
        PVK = Some(pvk);
    }

    true
}

// https://doc.rust-lang.org/std/primitive.pointer.html#:~:text=unsafe%20%7B%0A%20%20%20%20drop(Box%3A%3Afrom_raw(my_speed))%3B%0A%7D
#[no_mangle]
pub extern "C" fn librust_proving_ctx_free(ctx: *mut ProvingContext) {
    unsafe {
        drop(Box::from_raw(ctx));
    }
}

#[no_mangle]
pub extern "C" fn librust_verification_ctx_init() -> *mut VerificationContext {
    let ctx = Box::new(VerificationContext::new());
    Box::into_raw(ctx)
}

#[no_mangle]
pub extern "C" fn librust_verification_check(ctx: *mut VerificationContext, zkproof: *const [c_uchar; GROTH_PROOF_SIZE], inputs: *const [c_uchar; 32]) -> bool {
    println!("\nstarted verification check...");

    let pvk = unsafe { PVK.as_ref() }.unwrap();

    // Deserialize the proof
    let zkproof = match groth16::Proof::read(&(unsafe { &*zkproof })[..]) {
        Ok(p) => p,
        Err(_) => return false,
    };

    //https://doc.rust-lang.org/std/time/struct.SystemTime.html
    let now = TimeElapse::new();
    let res = unsafe { &mut *ctx }.verify_proof(pvk, zkproof, unsafe { &*inputs });
    let elapsed = now.elapsed();
    println!("verify proof consume time : {} secs", elapsed);

    res
}

#[no_mangle]
pub extern "C" fn librust_verification_ctx_free(ctx: *mut VerificationContext) {
    unsafe {
        drop(Box::from_raw(ctx));
    }
}

#[no_mangle]
pub extern "C" fn hello_world() {
    println!("Hello world from Bellman.");
}
