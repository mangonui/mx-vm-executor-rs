macro_rules! return_if_ptr_null {
    ($ptr_var:ident, $err_msg:expr, $err_return_val:expr) => {
        if $ptr_var.is_null() {
            with_service(|service| service.update_last_error_str($err_msg.to_string()));
            return $err_return_val;
        }
    };
    ($ptr_var:ident, $err_msg:expr) => {
        return_if_ptr_null!($ptr_var, $err_msg, vm_exec_result_t::VM_EXEC_ERROR)
    };
}

macro_rules! cast_input_ptr {
    ($ptr_var:ident, $expected_ty:ty, $err_msg:expr, $err_return_val:expr) => {
        if $ptr_var.is_null() {
            with_service(|service| service.update_last_error_str($err_msg.to_string()));
            return $err_return_val;
        } else if ($ptr_var as usize) % std::mem::align_of::<$expected_ty>() != 0 {
            with_service(|service| {
                service.update_last_error_str("input ptr is misaligned".to_string())
            });
            return $err_return_val;
        } else {
            unsafe { &mut *($ptr_var as *mut $expected_ty) }
        }
    };
    ($ptr_var:ident, $expected_ty:ty, $err_msg:expr) => {
        cast_input_ptr!(
            $ptr_var,
            $expected_ty,
            $err_msg,
            vm_exec_result_t::VM_EXEC_ERROR
        )
    };
}

/// Cast a `*mut vm_exec_instance_t` to `&mut CapiInstance` after
/// checking null + alignment AND verifying the instance has not been
/// destroyed.
///
/// The audit flagged that the bare `cast_input_ptr!` accepts stale
/// pointers — a freed-and-reused heap slot satisfies the null +
/// alignment checks but dereferences into the wrong type. Combined
/// with the AtomicBool destroy-poison on `CapiInstance`, this macro
/// closes the practical exploitability of the staleness window: the
/// first destroy flips the poison flag; any subsequent C-API call
/// that lands on the same (now-stale) pointer observes the flag and
/// errors out cleanly rather than reading random heap bytes as a
/// trait object.
///
/// Caveats:
///   - Only protects pointers whose underlying allocation is *still*
///     a CapiInstance (i.e. Box::from_raw has not run yet, OR the
///     allocator hasn't reused the slot for a different type).
///   - Does not protect against use-after-free where the allocator
///     has handed the memory to a different Box of a different type.
///     Closing that window requires the full handle-based API
///     redesign (audit finding #3, multi-day refactor) which is out
///     of scope for this batch.
macro_rules! cast_capi_instance_ptr {
    ($ptr_var:ident, $err_return_val:expr) => {{
        let inst = cast_input_ptr!(
            $ptr_var,
            $crate::capi_instance::CapiInstance,
            "instance ptr is null",
            $err_return_val
        );
        if inst
            .destroyed
            .load(std::sync::atomic::Ordering::Acquire)
        {
            with_service(|service| {
                service.update_last_error_str("instance pointer is destroyed (stale)".to_string())
            });
            return $err_return_val;
        }
        inst
    }};
    ($ptr_var:ident) => {
        cast_capi_instance_ptr!($ptr_var, vm_exec_result_t::VM_EXEC_ERROR)
    };
}

macro_rules! cast_input_const_ptr {
    ($ptr_var:ident, $expected_ty:ty, $err_msg:expr, $err_return_val:expr) => {
        if $ptr_var.is_null() {
            with_service(|service| service.update_last_error_str($err_msg.to_string()));
            return $err_return_val;
        } else if ($ptr_var as usize) % std::mem::align_of::<$expected_ty>() != 0 {
            with_service(|service| {
                service.update_last_error_str("input ptr is misaligned".to_string())
            });
            return $err_return_val;
        } else {
            unsafe { &*($ptr_var as *const $expected_ty) }
        }
    };
    ($ptr_var:ident, $expected_ty:ty, $err_msg:expr) => {
        cast_input_const_ptr!(
            $ptr_var,
            $expected_ty,
            $err_msg,
            vm_exec_result_t::VM_EXEC_ERROR
        )
    };
}
