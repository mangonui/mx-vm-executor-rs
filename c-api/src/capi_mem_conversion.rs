use multiversx_chain_vm_executor::{MemLength, MemPtr};

const MEM_PTR_OUT_OF_RANGE: &str = "VM hook memory pointer does not fit the Go int32 bridge";
const MEM_LENGTH_OUT_OF_RANGE: &str = "VM hook memory length does not fit the Go int32 bridge";

pub(crate) fn mem_ptr_to_i32(mem_ptr: MemPtr) -> i32 {
    i32::try_from(mem_ptr).expect(MEM_PTR_OUT_OF_RANGE)
}

pub(crate) fn mem_length_to_i32(mem_length: MemLength) -> i32 {
    i32::try_from(mem_length).expect(MEM_LENGTH_OUT_OF_RANGE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mem_ptr_to_i32_accepts_representable_values() {
        assert_eq!(mem_ptr_to_i32(0), 0);
        assert_eq!(mem_ptr_to_i32(42), 42);
        assert_eq!(mem_ptr_to_i32(i32::MAX as MemPtr), i32::MAX);
        assert_eq!(mem_ptr_to_i32(-1), -1);
    }

    #[test]
    #[should_panic(expected = "VM hook memory pointer does not fit the Go int32 bridge")]
    fn mem_ptr_to_i32_rejects_values_above_i32_max() {
        let _ = mem_ptr_to_i32(i32::MAX as MemPtr + 1);
    }

    #[test]
    #[should_panic(expected = "VM hook memory pointer does not fit the Go int32 bridge")]
    fn mem_ptr_to_i32_rejects_values_below_i32_min() {
        let _ = mem_ptr_to_i32(i32::MIN as MemPtr - 1);
    }

    #[test]
    fn mem_length_to_i32_accepts_representable_values() {
        assert_eq!(mem_length_to_i32(0), 0);
        assert_eq!(mem_length_to_i32(42), 42);
        assert_eq!(mem_length_to_i32(i32::MAX as MemLength), i32::MAX);
        assert_eq!(mem_length_to_i32(-1), -1);
    }

    #[test]
    #[should_panic(expected = "VM hook memory length does not fit the Go int32 bridge")]
    fn mem_length_to_i32_rejects_values_above_i32_max() {
        let _ = mem_length_to_i32(i32::MAX as MemLength + 1);
    }

    #[test]
    #[should_panic(expected = "VM hook memory length does not fit the Go int32 bridge")]
    fn mem_length_to_i32_rejects_values_below_i32_min() {
        let _ = mem_length_to_i32(i32::MIN as MemLength - 1);
    }
}
