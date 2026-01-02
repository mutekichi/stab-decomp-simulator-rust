use pyo3::prelude::*;
use pyo3::types::PyBytes;

pub fn parse_py_seed(seed: Option<Bound<'_, PyAny>>) -> PyResult<Option<[u8; 32]>> {
    match seed {
        None => Ok(None),
        Some(obj) => {
            if let Ok(val) = obj.extract::<u64>() {
                // Handle small integers efficiently
                let mut s = [0u8; 32];
                s[0..8].copy_from_slice(&val.to_le_bytes());
                Ok(Some(s))
            } else if obj.get_type().name()? == "int" {
                // Handle large Python integers using to_bytes method
                let bytes_obj = obj.call_method1("to_bytes", (32, "little"))?;
                let slice: &[u8] = bytes_obj.extract()?;
                let mut s = [0u8; 32];
                s.copy_from_slice(slice);
                Ok(Some(s))
            } else if let Ok(bytes) = obj.downcast::<PyBytes>() {
                // Handle raw bytes
                let slice = bytes.as_bytes();
                if slice.len() != 32 {
                    return Err(pyo3::exceptions::PyValueError::new_err(format!(
                        "Seed must be 32 bytes, got {}",
                        slice.len()
                    )));
                }
                let mut s = [0u8; 32];
                s.copy_from_slice(slice);
                Ok(Some(s))
            } else {
                Err(pyo3::exceptions::PyTypeError::new_err(
                    "Seed must be an integer or 32 bytes",
                ))
            }
        }
    }
}
