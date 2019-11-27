//! The codegen module provides common functions and data structures used by multiple backends
//! during the code generation process.
#[cfg(unix)]
use crate::fault::FaultInfo;
use crate::{
    backend::RunnableModule,
    backend::{Backend, CacheGen, Compiler, CompilerConfig, Features, Token},
    cache::{Artifact, Error as CacheError},
    error::{CompileError, CompileResult},
    module::{ModuleInfo, ModuleInner},
    structures::Map,
    types::{FuncIndex, FuncSig, SigIndex},
};
use smallvec::SmallVec;
use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};
use wasmparser::{self, WasmDecoder};
use wasmparser::{Operator, Type as WpType};

/// A type that defines a function pointer, which is called when breakpoints occur.
pub type BreakpointHandler =
    Box<dyn Fn(BreakpointInfo) -> Result<(), Box<dyn Any>> + Send + Sync + 'static>;

/// Maps instruction pointers to their breakpoint handlers.
pub type BreakpointMap = Arc<HashMap<usize, BreakpointHandler>>;

/// An event generated during parsing of a wasm binary
#[derive(Debug)]
pub enum Event<'a, 'b> {
    /// An internal event created by the parser used to provide hooks during code generation.
    Internal(InternalEvent),
    /// An event generated by parsing a wasm operator
    Wasm(&'b Operator<'a>),
    /// An event generated by parsing a wasm operator that contains an owned `Operator`
    WasmOwned(Operator<'a>),
}

/// Kinds of `InternalEvent`s created during parsing.
pub enum InternalEvent {
    /// A function parse is about to begin.
    FunctionBegin(u32),
    /// A function parsing has just completed.
    FunctionEnd,
    /// A breakpoint emitted during parsing.
    Breakpoint(BreakpointHandler),
    /// Indicates setting an internal field.
    SetInternal(u32),
    /// Indicates getting an internal field.
    GetInternal(u32),
}

impl fmt::Debug for InternalEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InternalEvent::FunctionBegin(_) => write!(f, "FunctionBegin"),
            InternalEvent::FunctionEnd => write!(f, "FunctionEnd"),
            InternalEvent::Breakpoint(_) => write!(f, "Breakpoint"),
            InternalEvent::SetInternal(_) => write!(f, "SetInternal"),
            InternalEvent::GetInternal(_) => write!(f, "GetInternal"),
        }
    }
}

/// Information for a breakpoint
#[cfg(unix)]
pub struct BreakpointInfo<'a> {
    /// Fault.
    pub fault: Option<&'a FaultInfo>,
}

/// Information for a breakpoint
#[cfg(not(unix))]
pub struct BreakpointInfo {
    /// Fault placeholder.
    pub fault: Option<()>,
}

/// A trait that represents the functions needed to be implemented to generate code for a module.
pub trait ModuleCodeGenerator<FCG: FunctionCodeGenerator<E>, RM: RunnableModule, E: Debug> {
    /// Creates a new module code generator.
    fn new() -> Self;

    /// Creates a new module code generator for specified target.
    fn new_with_target(
        triple: Option<String>,
        cpu_name: Option<String>,
        cpu_features: Option<String>,
    ) -> Self;

    /// Returns the backend id associated with this MCG.
    fn backend_id() -> Backend;

    /// Feeds the compiler config.
    fn feed_compiler_config(&mut self, _config: &CompilerConfig) -> Result<(), E> {
        Ok(())
    }
    /// Adds an import function.
    fn feed_import_function(&mut self) -> Result<(), E>;
    /// Sets the signatures.
    fn feed_signatures(&mut self, signatures: Map<SigIndex, FuncSig>) -> Result<(), E>;
    /// Sets function signatures.
    fn feed_function_signatures(&mut self, assoc: Map<FuncIndex, SigIndex>) -> Result<(), E>;
    /// Checks the precondition for a module.
    fn check_precondition(&mut self, module_info: &ModuleInfo) -> Result<(), E>;
    /// Creates a new function and returns the function-scope code generator for it.
    fn next_function(&mut self, module_info: Arc<RwLock<ModuleInfo>>) -> Result<&mut FCG, E>;
    /// Finalizes this module.
    fn finalize(self, module_info: &ModuleInfo) -> Result<(RM, Box<dyn CacheGen>), E>;

    /// Creates a module from cache.
    unsafe fn from_cache(cache: Artifact, _: Token) -> Result<ModuleInner, CacheError>;
}

/// A streaming compiler which is designed to generated code for a module based on a stream
/// of wasm parser events.
pub struct StreamingCompiler<
    'a,
    'b,
    MCG: ModuleCodeGenerator<FCG, RM, E>,
    FCG: FunctionCodeGenerator<E>,
    RM: RunnableModule + 'static,
    E: Debug,
    CGEN: Fn() -> MiddlewareChain<'a, 'b>,
> {
    middleware_chain_generator: CGEN,
    _phantom_mcg: PhantomData<MCG>,
    _phantom_fcg: PhantomData<FCG>,
    _phantom_rm: PhantomData<RM>,
    _phantom_e: PhantomData<E>,
}

/// A simple generator for a `StreamingCompiler`.
pub struct SimpleStreamingCompilerGen<
    MCG: ModuleCodeGenerator<FCG, RM, E>,
    FCG: FunctionCodeGenerator<E>,
    RM: RunnableModule + 'static,
    E: Debug,
> {
    _phantom_mcg: PhantomData<MCG>,
    _phantom_fcg: PhantomData<FCG>,
    _phantom_rm: PhantomData<RM>,
    _phantom_e: PhantomData<E>,
}

impl<
        MCG: ModuleCodeGenerator<FCG, RM, E>,
        FCG: FunctionCodeGenerator<E>,
        RM: RunnableModule + 'static,
        E: Debug,
    > SimpleStreamingCompilerGen<MCG, FCG, RM, E>
{
    /// Create a new `StreamingCompiler`.
    pub fn new<'a, 'b>(
    ) -> StreamingCompiler<'a, 'b, MCG, FCG, RM, E, impl Fn() -> MiddlewareChain<'a, 'b>> {
        StreamingCompiler::new(|| MiddlewareChain::new())
    }
}

impl<
        'a,
        'b,
        MCG: ModuleCodeGenerator<FCG, RM, E>,
        FCG: FunctionCodeGenerator<E>,
        RM: RunnableModule + 'static,
        E: Debug,
        CGEN: Fn() -> MiddlewareChain<'a, 'b>,
    > StreamingCompiler<'a, 'b, MCG, FCG, RM, E, CGEN>
{
    /// Create a new `StreamingCompiler` with the given `MiddlewareChain`.
    pub fn new(chain_gen: CGEN) -> Self {
        Self {
            middleware_chain_generator: chain_gen,
            _phantom_mcg: PhantomData,
            _phantom_fcg: PhantomData,
            _phantom_rm: PhantomData,
            _phantom_e: PhantomData,
        }
    }
}

/// Create a new `ValidatingParserConfig` with the given features.
pub fn validating_parser_config(features: &Features) -> wasmparser::ValidatingParserConfig {
    wasmparser::ValidatingParserConfig {
        operator_config: wasmparser::OperatorValidatorConfig {
            enable_threads: features.threads,
            enable_reference_types: false,
            enable_simd: features.simd,
            enable_bulk_memory: false,
            enable_multi_value: false,
        },
    }
}

fn validate_with_features(bytes: &[u8], features: &Features) -> CompileResult<()> {
    let mut parser =
        wasmparser::ValidatingParser::new(bytes, Some(validating_parser_config(features)));
    loop {
        let state = parser.read();
        match *state {
            wasmparser::ParserState::EndWasm => break Ok(()),
            wasmparser::ParserState::Error(err) => Err(CompileError::ValidationError {
                msg: err.message.to_string(),
            })?,
            _ => {}
        }
    }
}

impl<
        'a,
        'b,
        MCG: ModuleCodeGenerator<FCG, RM, E>,
        FCG: FunctionCodeGenerator<E>,
        RM: RunnableModule + 'static,
        E: Debug,
        CGEN: Fn() -> MiddlewareChain<'a, 'b>,
    > Compiler for StreamingCompiler<'a, 'b, MCG, FCG, RM, E, CGEN>
{
    fn compile(
        &self,
        wasm: &[u8],
        compiler_config: CompilerConfig,
        _: Token,
    ) -> CompileResult<ModuleInner> {
        if requires_pre_validation(MCG::backend_id()) {
            validate_with_features(wasm, &compiler_config.features)?;
        }

        let mut mcg = match MCG::backend_id() {
            Backend::LLVM => MCG::new_with_target(
                compiler_config.triple.clone(),
                compiler_config.cpu_name.clone(),
                compiler_config.cpu_features.clone(),
            ),
            _ => MCG::new(),
        };
        let mut chain = (self.middleware_chain_generator)();
        let info = crate::parse::read_module(
            wasm,
            MCG::backend_id(),
            &mut mcg,
            &mut chain,
            &compiler_config,
        )?;
        let (exec_context, cache_gen) =
            mcg.finalize(&info.read().unwrap())
                .map_err(|x| CompileError::InternalError {
                    msg: format!("{:?}", x),
                })?;
        Ok(ModuleInner {
            cache_gen,
            runnable_module: Box::new(exec_context),
            info: Arc::try_unwrap(info).unwrap().into_inner().unwrap(),
        })
    }

    unsafe fn from_cache(
        &self,
        artifact: Artifact,
        token: Token,
    ) -> Result<ModuleInner, CacheError> {
        MCG::from_cache(artifact, token)
    }
}

fn requires_pre_validation(backend: Backend) -> bool {
    match backend {
        Backend::Cranelift => true,
        Backend::LLVM => false,
        Backend::Singlepass => false,
    }
}

/// A sink for parse events.
pub struct EventSink<'a, 'b> {
    buffer: SmallVec<[Event<'a, 'b>; 2]>,
}

impl<'a, 'b> EventSink<'a, 'b> {
    /// Push a new `Event` to this sink.
    pub fn push(&mut self, ev: Event<'a, 'b>) {
        self.buffer.push(ev);
    }
}

/// A container for a chain of middlewares.
pub struct MiddlewareChain<'a, 'b> {
    chain: Vec<Box<dyn Fn(EventIter<'a, 'b>, &ModuleInfo) -> EventIter<'a, 'b>>>,
}

impl<'a, 'b> MiddlewareChain<'a, 'b> {
    /// Create a new empty `MiddlewareChain`.
    pub fn new() -> Self {
        MiddlewareChain { chain: vec![] }
    }

    /// Push a new `FunctionMiddleware` to this `MiddlewareChain`.
    pub fn push<F>(&mut self, constructor: F)
    where
        F: Fn(EventIter<'a, 'b>, &ModuleInfo) -> EventIter<'a, 'b>,
    {
        self.chain.push(Box::new(constructor));
    }

    /// Run this chain with the provided function code generator, event and module info.
    pub(crate) fn run<E: Debug, FCG: FunctionCodeGenerator<E>>(
        self,
        fcg: Option<&mut FCG>,
        source: EventIter<'a, 'b>,
        module_info: &ModuleInfo,
    ) -> Result<(), String> {
        let event_iter = {
            let mut cur_iter = source;
            for f in self.chain {
                cur_iter = f(cur_iter);
            }
            cur_iter
        };

        if let Some(fcg) = fcg {
            for ev in event_iter {
                fcg.feed_event(ev, module_info)
                    .map_err(|x| format!("{:?}", x))?;
            }
        }

        Ok(())
    }
}

/// A trait that represents the signature required to implement middleware for a function.
pub trait FunctionMiddleware {
    /// Processes the given event, module info and sink.
    fn feed_event<'a, 'b: 'a>(
        &mut self,
        source: EventIter<'a, 'b>,
        //module_info: &ModuleInfo,
    ) -> EventIter<'a, 'b>;
}

///abc
pub struct EventIter<'a, 'b> {
    inner: Box<dyn Iterator<Item = Result<Event<'a, 'b>, Box<dyn Debug>>>>,
}

impl<'a, 'b> EventIter<'a, 'b> {
    /// abc
    pub fn new<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Result<Event<'a, 'b>, Box<dyn Debug>>> + 'static,
    {
        Self {
            inner: Box::new(iter.into_iter()),
        }
    }
}

impl<'a, 'b> Iterator for EventIter<'a, 'b> {
    type Item = Result<Event<'a, 'b>, Box<dyn Debug>>;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

pub(crate) trait GenericFunctionMiddleware {
    fn feed_event<'a, 'b: 'a>(
        &mut self,
        source: EventIter<'a, 'b>,
        //module_info: &ModuleInfo,
    ) -> EventIter<'a, 'b>;
}

impl<T: FunctionMiddleware> GenericFunctionMiddleware for T {
    fn feed_event<'a, 'b: 'a>(
        &mut self,
        source: EventIter<'a, 'b>,
        //module_info: &ModuleInfo,
    ) -> EventIter<'a, 'b> {
        <Self as FunctionMiddleware>::feed_event(self, source /*, module_info*/)
    }
}

/// The function-scope code generator trait.
pub trait FunctionCodeGenerator<E: Debug> {
    /// Sets the return type.
    fn feed_return(&mut self, ty: WpType) -> Result<(), E>;

    /// Adds a parameter to the function.
    fn feed_param(&mut self, ty: WpType) -> Result<(), E>;

    /// Adds `n` locals to the function.
    fn feed_local(&mut self, ty: WpType, n: usize) -> Result<(), E>;

    /// Called before the first call to `feed_opcode`.
    fn begin_body(&mut self, module_info: &ModuleInfo) -> Result<(), E>;

    /// Called for each operator.
    fn feed_event(&mut self, op: Event, module_info: &ModuleInfo) -> Result<(), E>;

    /// Finalizes the function.
    fn finalize(&mut self) -> Result<(), E>;
}
