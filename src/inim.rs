use rhai::{packages::Package, Engine, Module, Scope};
use rhai_rand::RandomPackage;
use rhai_sci::SciPackage;

use crate::{
    io::{
        console::{register_console, Console},
        dummy::Dummy,
        fs::{register_file, File},
        net::{register_socket, Net},
        sys::{register_sys, Sys},
    },
    module_resolver::InimModuleResolver,
};

use core::marker::PhantomData;

use std::prelude::v1::*;

pub struct Inim<'a, C = Dummy, F = Dummy, S = Dummy, N = Dummy>
where
    C: Console,
    F: File,
    S: Sys,
    N: Net,
{
    engine: Engine,

    mod_resolver: InimModuleResolver<C, F>,

    scopes: Vec<Scope<'a>>,
    current_scope: usize,

    path: &'a str,

    _console_phantom: PhantomData<C>,
    _f_phantom: PhantomData<F>,
    _sys_phantom: PhantomData<S>,
    _net_phantom: PhantomData<N>,
}

impl<'a, C, F, S, N> Inim<'a, C, F, S, N>
where
    C: Console,
    F: File,
    S: Sys,
    N: Net,
{
    pub fn new() -> Self {
        let mut inim = Inim {
            engine: Engine::new(),

            mod_resolver: InimModuleResolver::<C, F>::new(),

            scopes: Vec::new(),
            current_scope: 0,

            path: "repl",

            _console_phantom: PhantomData,
            _f_phantom: PhantomData,
            _sys_phantom: PhantomData,
            _net_phantom: PhantomData,
        };

        // Setup packages
        inim.engine
            .register_global_module(SciPackage::new().as_shared_module())
            .register_global_module(RandomPackage::new().as_shared_module())
            .set_module_resolver(inim.mod_resolver.clone());

        register_console::<C>(&mut inim.engine);
        register_file::<F>(&mut inim.engine);
        register_sys::<S>(&mut inim.engine);
        register_socket::<N>(&mut inim.engine);

        // Add default scope
        inim.scopes.push(Scope::<'a>::new());

        inim
    }

    pub fn register_module(&mut self, path: impl Into<String>, module: Module) -> &mut Self {
        self.mod_resolver.register_module(path, module);
        self
    }

    pub fn update_modules(&mut self) -> &mut Self {
        self.engine.set_module_resolver(self.mod_resolver.clone());
        self
    }

    pub fn set_scope(&mut self, scope_num: usize) -> &mut Self {
        self.current_scope = scope_num;

        self
    }

    pub fn run_file(&mut self, path: &'a str) -> &mut Self {
        let mut file = F::open(path, "r");
        let prog = file.read_string_all();
        file.close();

        self.path = path;
        self.run(prog.as_str());
        self.path = "repl";

        self
    }

    pub fn run(&mut self, prog: &str) -> &mut Self {
        let mut ast = match self
            .engine
            .compile_with_scope(&mut self.scopes[self.current_scope], prog)
        {
            Ok(ast) => ast,
            Err(error) => {
                C::debug(
                    format!(
                        "{}:{}:{} Compile error: {:#?}",
                        self.path,
                        error.position().line().unwrap_or(0),
                        error.position().position().unwrap_or(0),
                        error.err_type()
                    )
                    .as_str(),
                );
                return self;
            }
        };

        ast.set_source(self.path);

        match self
            .engine
            .run_ast_with_scope(&mut self.scopes[self.current_scope], &ast)
        {
            Ok(_) => {}
            Err(error) => C::debug(
                format!(
                    "{}:{}:{} Compile error: {:#?}",
                    self.path,
                    error.position().line().unwrap_or(0),
                    error.position().position().unwrap_or(0),
                    error
                )
                .as_str(),
            ),
        };

        self
    }
}
