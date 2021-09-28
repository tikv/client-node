// Copyright 2021 TiKV Project Authors. Licensed under Apache-2.0.

use neon::prelude::*;
use once_cell::sync::OnceCell;

// Globally store a static reference to the `TRANSACTION_ERROR` class
pub struct ClientErrors {
    pub operation_after_commit_error: OnceCell<Root<JsFunction>>,
    pub undetermined_error: OnceCell<Root<JsFunction>>,
    pub write_conlict_error: OnceCell<Root<JsFunction>>,
    pub already_exist_error: OnceCell<Root<JsFunction>>,
    pub daedlock_error: OnceCell<Root<JsFunction>>,
}

pub static CLIENT_ERRORS: ClientErrors = ClientErrors {
    operation_after_commit_error: OnceCell::new(),
    undetermined_error: OnceCell::new(),
    write_conlict_error: OnceCell::new(),
    already_exist_error: OnceCell::new(),
    daedlock_error: OnceCell::new(),
};

pub trait CustomError {
    fn throw<'a, C>(&self, cx: &mut C, args: Vec<String>) -> JsResult<'a, JsObject>
    where
        C: Context<'a>;
}

impl CustomError for OnceCell<Root<JsFunction>> {
    fn throw<'a, C>(&self, cx: &mut C, args: Vec<String>) -> JsResult<'a, JsObject>
    where
        C: Context<'a>,
    {
        let args: Vec<Handle<JsValue>> = args.into_iter().map(|s| cx.string(s).upcast()).collect();

        let error = self
            .get()
            .expect("Expected module to be initialized")
            .to_inner(cx);

        // Use `.construct` to call this as a constructor instead of a normal function
        error.construct(cx, args)
    }
}

pub fn init(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    CLIENT_ERRORS
        .operation_after_commit_error
        .get_or_try_init(|| Ok(cx.argument::<JsFunction>(0)?.root(&mut cx)))?;
    CLIENT_ERRORS
        .undetermined_error
        .get_or_try_init(|| Ok(cx.argument::<JsFunction>(1)?.root(&mut cx)))?;
    CLIENT_ERRORS
        .write_conlict_error
        .get_or_try_init(|| Ok(cx.argument::<JsFunction>(2)?.root(&mut cx)))?;
    CLIENT_ERRORS
        .already_exist_error
        .get_or_try_init(|| Ok(cx.argument::<JsFunction>(3)?.root(&mut cx)))?;
    CLIENT_ERRORS
        .daedlock_error
        .get_or_try_init(|| Ok(cx.argument::<JsFunction>(4)?.root(&mut cx)))?;
    Ok(cx.undefined())
}
