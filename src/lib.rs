#![no_std]

extern crate no_std_compat as std;

use std::prelude::v1::*;

use core::marker::PhantomData;

use io::{console::Console, fs::File, sys::Sys};
use module_resolver::InimModuleResolver;
use rhai::{packages::Package, Engine, Module, Scope};
use rhai_rand::RandomPackage;
use rhai_sci::SciPackage;

pub mod io;
mod module_resolver;

pub struct InimFactory<C, F, S>
where
    C: Console,
    F: File,
    S: Sys,
{
    mod_resolver: InimModuleResolver<C, F>,

    console_phantom: PhantomData<C>,
    f_phantom: PhantomData<F>,
    sys_phantom: PhantomData<S>,
}

impl<C, F, S> InimFactory<C, F, S>
where
    C: Console,
    F: File,
    S: Sys,
{
    pub fn new() -> Self {
        Self {
            mod_resolver: InimModuleResolver::new(),

            console_phantom: PhantomData,
            f_phantom: PhantomData,
            sys_phantom: PhantomData,
        }
    }

    pub fn register_module(&mut self, path: impl Into<String>, module: Module) -> &mut Self {
        self.mod_resolver.register_module(path, module);
        self
    }

    pub fn build(&mut self) -> Inim<C, F, S> {
        Inim::new(self.mod_resolver.clone())
    }
}

pub struct Inim<'a, C, F, S>
where
    C: Console,
    F: File,
    S: Sys,
{
    engine: Engine,

    scopes: Vec<Scope<'a>>,
    current_scope: usize,

    path: &'a str,

    console_phantom: PhantomData<C>,
    f_phantom: PhantomData<F>,
    sys_phantom: PhantomData<S>,
}

impl<'a, C, F, S> Inim<'a, C, F, S>
where
    C: Console,
    F: File,
    S: Sys,
{
    pub fn new(mod_resolver: InimModuleResolver<C, F>) -> Self {
        let mut inim = Inim {
            engine: Engine::new(),

            scopes: Vec::new(),
            current_scope: 0,

            path: "repl",

            console_phantom: PhantomData,
            f_phantom: PhantomData,
            sys_phantom: PhantomData,
        };

        // Setup packages
        inim.engine
            .register_global_module(SciPackage::new().as_shared_module())
            .register_global_module(RandomPackage::new().as_shared_module())
            .set_module_resolver(mod_resolver);

        // Register Console
        inim.engine.on_print(C::print);
        inim.engine.on_debug(|text, source, position| {
            C::debug(
                format!(
                    "{}:{}:{} {}",
                    source.unwrap(),
                    position.line().unwrap_or(0),
                    position.position().unwrap_or(0),
                    text
                )
                .as_str(),
            )
        });

        // Registering FS
        inim.engine
            .register_type_with_name::<F>("File")
            .register_fn("open", F::open)
            .register_fn("close", F::close)
            .register_fn("seek", F::seek)
            .register_fn("step", F::step)
            .register_fn("read", F::read_all)
            .register_fn("read", F::read_until)
            .register_fn("read", F::read_amount)
            .register_fn("write", F::write);

        // Register Sys
        inim.engine
            .register_fn("ls", S::ls)
            .register_fn("rm", S::rm)
            .register_fn("mkdir", S::mkdir)
            .register_fn("rmdir", S::rmdir)
            .register_fn("time", S::time)
            .register_fn("path", S::path);

        // Register Net

        // Add default scope
        inim.scopes.push(Scope::<'a>::new());

        inim
    }

    pub fn set_scope(&mut self, scope_num: usize) -> &mut Self {
        self.current_scope = scope_num;

        self
    }

    pub fn run_file(&mut self, path: &'a str) -> &mut Self {
        let mut file = F::open(path, "r");
        let prog = file.read_all();
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
