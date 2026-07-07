use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct VolatileRamCache {
    pub internal_registers: Mutex<HashMap<String, String>>,
}

pub struct LogigateSandbox {
    pub memory_bank: Arc<VolatileRamCache>,
    pub frt_armed: Arc<AtomicBool>,
}

impl LogigateSandbox {
    pub fn initialize_isolated_sandbox() -> Self {
        Self {
            memory_bank: Arc::new(VolatileRamCache {
                internal_registers: Mutex::new(HashMap::new()),
            }),
            frt_armed: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn secure_write(&self, key: &str, value: &str) -> Result<(), String> {
        if self.frt_armed.load(Ordering::SeqCst) {
            return Err("Execution Blocked: Forced Reset Active.".to_string());
        }
        let mut cache = self.memory_bank.internal_registers.lock().unwrap();
        cache.insert(key.to_string(), value.to_string());
        Ok(())
    }

    pub fn execute_forced_reset_trigger(&self) {
        self.frt_armed.store(true, Ordering::SeqCst);
        
        let mut cache = self.memory_bank.internal_registers.lock().unwrap();
        for (k, v) in cache.iter_mut() {
            *k = String::from("00000000");
            *v = String::from("00000000");
        }
        cache.clear();
        
        self.frt_armed.store(false, Ordering::SeqCst);
    }
}
