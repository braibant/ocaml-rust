static BOXROOT_SETUP: std::sync::Once = std::sync::Once::new();

pub struct RootedValue<T>
where
    T: 'static,
{
    root: ocaml_boxroot_sys::BoxRoot,
    phantom_data: std::marker::PhantomData<T>,
}

impl<T> RootedValue<T> {
    pub fn create(v: ocaml_sys::Value) -> RootedValue<T> {
        BOXROOT_SETUP.call_once(|| unsafe {
            ocaml_boxroot_sys::boxroot_setup();
        });

        RootedValue {
            root: unsafe { ocaml_boxroot_sys::boxroot_create(v) },
            phantom_data: std::marker::PhantomData,
        }
    }

    pub fn value(&self) -> crate::Value<T> {
        let value = unsafe { ocaml_boxroot_sys::boxroot_get(self.root) };
        unsafe { crate::Value::new(value) }
    }
}

impl<T> Drop for RootedValue<T> {
    fn drop(&mut self) {
        unsafe { ocaml_boxroot_sys::boxroot_delete(self.root) }
    }
}
