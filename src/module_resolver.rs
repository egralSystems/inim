use core::marker::PhantomData;

use alloc::{boxed::Box, format, string::ToString};
use rhai::{EvalAltResult, Module, ModuleResolver, Scope, Shared};

use crate::io::{console::Console, fs::File};

pub struct InimModuleResolver<C, F>
where
    C: Console + 'static,
    F: File + Clone + 'static,
{
    console_phantom: PhantomData<C>,
    f_phantom: PhantomData<F>,
}

impl<C, F> InimModuleResolver<C, F>
where
    C: Console + 'static,
    F: File + Clone + 'static,
{
    pub fn new() -> Self {
        Self {
            console_phantom: PhantomData,
            f_phantom: PhantomData,
        }
    }
}

impl<C, F> ModuleResolver for InimModuleResolver<C, F>
where
    C: Console + 'static,
    F: File + Clone + 'static,
{
    fn resolve(
        &self,
        engine: &rhai::Engine,
        _source: Option<&str>,
        path: &str,
        pos: rhai::Position,
    ) -> Result<Shared<Module>, Box<EvalAltResult>> {
        let mut src = F::open(path, "r");
        let mut ast = match engine.compile(src.read_all()) {
            Ok(ast) => ast,
            Err(error) => {
                C::debug(
                    format!(
                        "{}:{}:{} Compile error: {:#?}",
                        path,
                        error.position().line().unwrap_or(0),
                        error.position().position().unwrap_or(0),
                        error.err_type()
                    )
                    .as_str(),
                );
                return Err(Box::new(EvalAltResult::ErrorInModule(
                    path.to_string(),
                    error.into(),
                    pos,
                )));
            }
        };

        ast.set_source(path);

        let scope = Scope::new();

        let mut m = match Module::eval_ast_as_new(scope, &ast, &engine) {
            Ok(m) => m,
            Err(error) => {
                C::debug(
                    format!(
                        "{}:{}:{} Compile error: {:#?}",
                        path,
                        error.position().line().unwrap_or(0),
                        error.position().position().unwrap_or(0),
                        error
                    )
                    .as_str(),
                );
                return Err(Box::new(EvalAltResult::ErrorInModule(
                    path.to_string(),
                    error.into(),
                    pos,
                )));
            }
        };
        m.build_index();

        Ok(Shared::new(m))
    }
}
