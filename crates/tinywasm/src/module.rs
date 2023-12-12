use tinywasm_types::TinyWasmModule;

use crate::{ModuleInstance, Result, Store};

#[derive(Debug)]
/// A WebAssembly Module
///
/// See <https://webassembly.github.io/spec/core/syntax/modules.html#syntax-module>
pub struct Module {
    data: TinyWasmModule,
}

impl From<&TinyWasmModule> for Module {
    fn from(data: &TinyWasmModule) -> Self {
        Self { data: data.clone() }
    }
}

impl From<TinyWasmModule> for Module {
    fn from(data: TinyWasmModule) -> Self {
        Self { data }
    }
}

impl Module {
    #[cfg(feature = "parser")]
    /// Parse a module from bytes. Requires `parser` feature.
    pub fn parse_bytes(wasm: &[u8]) -> Result<Self> {
        let parser = tinywasm_parser::Parser::new();
        let data = parser.parse_module_bytes(wasm)?;
        Ok(data.into())
    }

    #[cfg(all(feature = "parser", feature = "std"))]
    /// Parse a module from a file. Requires `parser` and `std` features.
    pub fn parse_file(path: impl AsRef<crate::std::path::Path> + Clone) -> Result<Self> {
        let parser = tinywasm_parser::Parser::new();
        let data = parser.parse_module_file(path)?;
        Ok(data.into())
    }

    #[cfg(all(feature = "parser", feature = "std"))]
    /// Parse a module from a stream. Requires `parser` and `std` features.
    pub fn parse_stream(stream: impl crate::std::io::Read) -> Result<Self> {
        let parser = tinywasm_parser::Parser::new();
        let data = parser.parse_module_stream(stream)?;
        Ok(data.into())
    }

    /// Instantiate the module in the given store
    ///
    /// Runs the start function if it exists
    /// If you want to run the start function yourself, use `ModuleInstance::new`
    ///
    /// See <https://webassembly.github.io/spec/core/exec/modules.html#exec-instantiation>
    pub fn instantiate(
        self,
        store: &mut Store,
        // imports: Option<()>,
    ) -> Result<ModuleInstance> {
        let idx = store.next_module_instance_idx();

        let func_addrs = store.add_funcs(self.data.funcs.into(), idx);
        let instance = ModuleInstance::new(
            self.data.types,
            self.data.start_func,
            self.data.exports,
            func_addrs,
            idx,
            store.id(),
        );

        store.add_instance(instance.clone())?;
        // let _ = instance.start(store)?;
        Ok(instance)
    }
}
