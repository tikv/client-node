use neon::prelude::*;
use once_cell::sync::OnceCell;

// Globally store a static reference to the `TRANSACTION_ERROR` class
pub static TRANSACTION_ERROR: OnceCell<Root<JsFunction>> = OnceCell::new();

pub trait CustomError {
    fn throw<'a, C>(&self, cx: &mut C) -> JsResult<'a, JsObject>
    where
        C: Context<'a>;
}

impl CustomError for OnceCell<Root<JsFunction>> {
    fn throw<'a, C>(&self, cx: &mut C) -> JsResult<'a, JsObject>
    where
        C: Context<'a>,
    {
        let args = Vec::<Handle<JsValue>>::with_capacity(0);
        let error = self
            .get()
            .expect("Expected module to be initialized")
            .to_inner(cx);

        // Use `.construct` to call this as a constructor instead of a normal function
        error.construct(cx, args)
    }
}

pub fn init(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    TRANSACTION_ERROR.get_or_try_init(|| Ok(cx.argument::<JsFunction>(0)?.root(&mut cx)))?;

    Ok(cx.undefined())
}
