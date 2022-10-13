use crate::io::{console::Console, fs::File};
use core::marker::PhantomData;
use rhai::{EvalAltResult, Module, ModuleResolver, Scope, Shared};
use std::collections::BTreeMap;
use std::prelude::v1::*;
use std::rc::Rc;

#[derive(Clone)]
pub struct InimModuleResolver<C, F>
where
    C: Console,
    F: File,
{
    em_modules: BTreeMap<String, Rc<Module>>,

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
            em_modules: BTreeMap::new(),

            console_phantom: PhantomData,
            f_phantom: PhantomData,
        }
    }

    pub fn register_module(&mut self, path: impl Into<String>, mut module: Module) -> &mut Self {
        let path = path.into();

        if module.id().is_none() {
            module.set_id(path.clone());
        }

        module.build_index();
        self.em_modules.insert(path, module.into());

        self
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
        if let Some(em) = self.em_modules.get(path).cloned() {
            return Ok(em);
        }

        let path = format!("{}.rhai", path);
        let path = path.as_str();

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
