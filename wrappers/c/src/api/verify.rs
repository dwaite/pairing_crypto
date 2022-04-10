use crate::dtos::{BbsVerifyRequestDto, ByteArray, PairingCryptoFfiError};
use ffi_support::{ConcurrentHandleMap, ErrorCode, ExternError};
use pairing_crypto::bls12_381::bbs::*;

lazy_static! {
    pub static ref BBS_VERIFY_CONTEXT: ConcurrentHandleMap<BbsVerifyRequestDto> =
        ConcurrentHandleMap::new();
}

#[no_mangle]
pub extern "C" fn bls12381_bbs_verify_context_init(err: &mut ExternError) -> u64 {
    BBS_VERIFY_CONTEXT.insert_with_output(err, || BbsVerifyRequestDto {
        messages: Vec::new(),
        public_key: Vec::new(),
        signature: Vec::new(),
    })
}

set_byte_array_impl!(
    bls12381_bbs_verify_context_set_public_key,
    BBS_VERIFY_CONTEXT,
    public_key
);

add_byte_array_impl!(
    bls12381_bbs_verify_context_set_message,
    BBS_VERIFY_CONTEXT,
    messages
);

set_byte_array_impl!(
    bls12381_bbs_verify_context_set_signature,
    BBS_VERIFY_CONTEXT,
    signature
);

#[no_mangle]
pub extern "C" fn bls12381_bbs_verify_context_finish(handle: u64, err: &mut ExternError) -> i32 {
    BBS_VERIFY_CONTEXT.call_with_result(
        err,
        handle,
        move |ctx| -> Result<i32, PairingCryptoFfiError> {
            if ctx.public_key.is_empty() {
                return Err(PairingCryptoFfiError::new("public_key must be set"));
            }
            if ctx.messages.is_empty() {
                return Err(PairingCryptoFfiError::new("messages cannot be empty"));
            }
            if ctx.signature.is_empty() {
                return Err(PairingCryptoFfiError::new("signature must be set"));
            }

            match verify(BbsVerifyRequest {
                public_key: ctx.public_key.clone(),
                messages: ctx.messages.clone(),
                signature: ctx.signature.clone(),
            })
            .unwrap()
            {
                true => Ok(0),
                false => Ok(1),
            }
        },
    )
}

define_handle_map_deleter!(BBS_VERIFY_CONTEXT, bls12381_bbs_verify_free);