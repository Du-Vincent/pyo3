#![no_implicit_prelude]
#![allow(unused_variables, clippy::unnecessary_wraps)]

#[crate::pyclass]
#[pyo3(crate = "crate")]
pub struct Dummy;

#[crate::pyclass]
#[pyo3(crate = "crate")]
pub struct DummyIter;

#[cfg(Py_3_8)]
#[crate::pymethods]
#[pyo3(crate = "crate")]
impl Dummy {
    //////////////////////
    // Basic customization
    //////////////////////
    fn __repr__(&self) -> &'static str {
        "Dummy"
    }

    fn __str__(&self) -> &'static str {
        "Dummy"
    }

    fn __bytes__<'py>(&self, py: crate::Python<'py>) -> &'py crate::types::PyBytes {
        crate::types::PyBytes::new(py, &[0])
    }

    fn __format__(&self, format_spec: ::std::string::String) -> ::std::string::String {
        ::std::panic!("unimplemented isn't hygienic before 1.50")
    }

    fn __lt__(&self, other: &Self) -> bool {
        false
    }

    fn __le__(&self, other: &Self) -> bool {
        false
    }
    fn __eq__(&self, other: &Self) -> bool {
        false
    }
    fn __ne__(&self, other: &Self) -> bool {
        false
    }
    fn __gt__(&self, other: &Self) -> bool {
        false
    }
    fn __ge__(&self, other: &Self) -> bool {
        false
    }

    fn __hash__(&self) -> u64 {
        42
    }

    fn __bool__(&self) -> bool {
        true
    }

    //////////////////////
    // Customizing attribute access
    //////////////////////

    fn __getattr__(&self, name: ::std::string::String) -> &crate::PyAny {
        ::std::panic!("unimplemented isn't hygienic before 1.50")
    }

    fn __getattribute__(&self, name: ::std::string::String) -> &crate::PyAny {
        ::std::panic!("unimplemented isn't hygienic before 1.50")
    }

    fn __setattr__(&mut self, name: ::std::string::String, value: ::std::string::String) {}

    fn __delattr__(&mut self, name: ::std::string::String) {}

    fn __dir__<'py>(&self, py: crate::Python<'py>) -> &'py crate::types::PyList {
        crate::types::PyList::new(py, ::std::vec![0_u8])
    }

    //////////////////////
    // Implementing Descriptors
    //////////////////////

    fn __get__(
        &self,
        instance: &crate::PyAny,
        owner: &crate::PyAny,
    ) -> crate::PyResult<&crate::PyAny> {
        ::std::panic!("unimplemented isn't hygienic before 1.50")
    }

    fn __set__(&self, instance: &crate::PyAny, owner: &crate::PyAny) {}

    fn __delete__(&self, instance: &crate::PyAny) {}

    fn __set_name__(&self, owner: &crate::PyAny, name: &crate::PyAny) {}

    //////////////////////
    // Implementing Descriptors
    //////////////////////

    fn __len__(&self) -> usize {
        0
    }

    fn __getitem__(&self, key: u32) -> crate::PyResult<u32> {
        ::std::result::Result::Err(crate::exceptions::PyKeyError::new_err("boo"))
    }

    fn __setitem__(&self, key: u32, value: u32) {}

    fn __delitem__(&self, key: u32) {}

    fn __iter__(_: crate::pycell::PyRef<Self>, py: crate::Python) -> crate::Py<DummyIter> {
        crate::Py::new(py, DummyIter {}).unwrap()
    }

    fn __next__(&mut self) -> ::std::option::Option<()> {
        ::std::option::Option::None
    }

    fn __reversed__(slf: crate::pycell::PyRef<Self>, py: crate::Python) -> crate::Py<DummyIter> {
        crate::Py::new(py, DummyIter {}).unwrap()
    }

    fn __contains__(&self, item: u32) -> bool {
        false
    }

    //////////////////////
    // Emulating numeric types
    //////////////////////

    fn __add__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __sub__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __mul__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __truediv__(&self, _other: &Self) -> crate::PyResult<()> {
        ::std::result::Result::Err(crate::exceptions::PyZeroDivisionError::new_err("boo"))
    }

    fn __floordiv__(&self, _other: &Self) -> crate::PyResult<()> {
        ::std::result::Result::Err(crate::exceptions::PyZeroDivisionError::new_err("boo"))
    }

    fn __mod__(&self, _other: &Self) -> u32 {
        0
    }

    fn __divmod__(&self, _other: &Self) -> (u32, u32) {
        (0, 0)
    }

    fn __pow__(&self, _other: &Self, modulo: ::std::option::Option<i32>) -> Dummy {
        Dummy {}
    }

    fn __lshift__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __rshift__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __and__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __xor__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __or__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __radd__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __rrsub__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __rmul__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __rtruediv__(&self, _other: &Self) -> crate::PyResult<()> {
        ::std::result::Result::Err(crate::exceptions::PyZeroDivisionError::new_err("boo"))
    }

    fn __rfloordiv__(&self, _other: &Self) -> crate::PyResult<()> {
        ::std::result::Result::Err(crate::exceptions::PyZeroDivisionError::new_err("boo"))
    }

    fn __rmod__(&self, _other: &Self) -> u32 {
        0
    }

    fn __rdivmod__(&self, _other: &Self) -> (u32, u32) {
        (0, 0)
    }

    fn __rpow__(&self, _other: &Self, modulo: ::std::option::Option<i32>) -> Dummy {
        Dummy {}
    }

    fn __rlshift__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __rrshift__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __rand__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __rxor__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __ror__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __iadd__(&mut self, other: &Self) {}

    fn __irsub__(&mut self, other: &Self) {}

    fn __imul__(&mut self, other: &Self) {}

    fn __itruediv__(&mut self, _other: &Self) {}

    fn __ifloordiv__(&mut self, _other: &Self) {}

    fn __imod__(&mut self, _other: &Self) {}

    fn __ipow__(&mut self, _other: &Self, modulo: ::std::option::Option<i32>) {}

    fn __ilshift__(&mut self, other: &Self) {}

    fn __irshift__(&mut self, other: &Self) {}

    fn __iand__(&mut self, other: &Self) {}

    fn __ixor__(&mut self, other: &Self) {}

    fn __ior__(&mut self, other: &Self) {}

    fn __neg__(slf: crate::pycell::PyRef<Self>) -> crate::pycell::PyRef<Self> {
        slf
    }

    fn __pos__(slf: crate::pycell::PyRef<Self>) -> crate::pycell::PyRef<Self> {
        slf
    }

    fn __abs__(slf: crate::pycell::PyRef<Self>) -> crate::pycell::PyRef<Self> {
        slf
    }

    fn __invert__(slf: crate::pycell::PyRef<Self>) -> crate::pycell::PyRef<Self> {
        slf
    }

    fn __complex__<'py>(&self, py: crate::Python<'py>) -> &'py crate::types::PyComplex {
        crate::types::PyComplex::from_doubles(py, 0.0, 0.0)
    }

    fn __int__(&self) -> u32 {
        0
    }

    fn __float__(&self) -> f64 {
        0.0
    }

    fn __index__(&self) -> u32 {
        0
    }

    fn __round__(&self, ndigits: ::std::option::Option<u32>) -> u32 {
        0
    }

    fn __trunc__(&self) -> u32 {
        0
    }

    fn __floor__(&self) -> u32 {
        0
    }

    fn __ceil__(&self) -> u32 {
        0
    }

    //////////////////////
    // With Statement Context Managers
    //////////////////////

    fn __enter__(&mut self) {}

    fn __exit__(
        &mut self,
        exc_type: &crate::PyAny,
        exc_value: &crate::PyAny,
        traceback: &crate::PyAny,
    ) {
    }

    //////////////////////
    // Awaitable Objects
    //////////////////////

    fn __await__(slf: crate::pycell::PyRef<Self>) -> crate::pycell::PyRef<Self> {
        slf
    }

    //////////////////////

    // Asynchronous Iterators
    //////////////////////

    fn __aiter__(slf: crate::pycell::PyRef<Self>, py: crate::Python) -> crate::Py<DummyIter> {
        crate::Py::new(py, DummyIter {}).unwrap()
    }

    fn __anext__(&mut self) -> ::std::option::Option<()> {
        ::std::option::Option::None
    }

    //////////////////////
    // Asynchronous Context Managers
    //////////////////////

    fn __aenter__(&mut self) {}

    fn __aexit__(
        &mut self,
        exc_type: &crate::PyAny,
        exc_value: &crate::PyAny,
        traceback: &crate::PyAny,
    ) {
    }

    // Things with attributes

    #[args(x = "1", "*", _z = "2")]
    fn test(&self, _y: &Dummy, _z: i32) {}
    #[staticmethod]
    fn staticmethod() {}
    #[classmethod]
    fn clsmethod(_: &crate::types::PyType) {}
    #[args(args = "*", kwds = "**")]
    fn __call__(
        &self,
        _args: &crate::types::PyTuple,
        _kwds: ::std::option::Option<&crate::types::PyDict>,
    ) -> crate::PyResult<i32> {
        ::std::panic!("unimplemented isn't hygienic before 1.50")
    }
    #[new]
    fn new(a: u8) -> Self {
        Dummy {}
    }
    #[getter]
    fn get(&self) -> i32 {
        0
    }
    #[setter]
    fn set(&mut self, _v: i32) {}
    #[classattr]
    fn class_attr() -> i32 {
        0
    }

    // Dunder methods invented for protocols

    fn __richcmp__(
        &self,
        other: &Self,
        op: crate::class::basic::CompareOp,
    ) -> crate::PyResult<bool> {
        ::std::result::Result::Ok(false)
    }
    // PyGcProtocol
    // Buffer protocol?
}

#[cfg(not(Py_3_8))]
#[crate::pymethods]
#[pyo3(crate = "crate")]
impl Dummy {
    //////////////////////
    // Basic customization
    //////////////////////
    fn __repr__(&self) -> &'static str {
        "Dummy"
    }

    fn __str__(&self) -> &'static str {
        "Dummy"
    }

    fn __bytes__<'py>(&self, py: crate::Python<'py>) -> &'py crate::types::PyBytes {
        crate::types::PyBytes::new(py, &[0])
    }

    fn __format__(&self, format_spec: ::std::string::String) -> ::std::string::String {
        ::std::panic!("unimplemented isn't hygienic before 1.50")
    }

    fn __lt__(&self, other: &Self) -> bool {
        false
    }

    fn __le__(&self, other: &Self) -> bool {
        false
    }
    fn __eq__(&self, other: &Self) -> bool {
        false
    }
    fn __ne__(&self, other: &Self) -> bool {
        false
    }
    fn __gt__(&self, other: &Self) -> bool {
        false
    }
    fn __ge__(&self, other: &Self) -> bool {
        false
    }

    fn __hash__(&self) -> u64 {
        42
    }

    fn __bool__(&self) -> bool {
        true
    }

    //////////////////////
    // Customizing attribute access
    //////////////////////

    fn __getattr__(&self, name: ::std::string::String) -> &crate::PyAny {
        ::std::panic!("unimplemented isn't hygienic before 1.50")
    }

    fn __getattribute__(&self, name: ::std::string::String) -> &crate::PyAny {
        ::std::panic!("unimplemented isn't hygienic before 1.50")
    }

    fn __setattr__(&mut self, name: ::std::string::String, value: ::std::string::String) {}

    fn __delattr__(&mut self, name: ::std::string::String) {}

    fn __dir__<'py>(&self, py: crate::Python<'py>) -> &'py crate::types::PyList {
        crate::types::PyList::new(py, ::std::vec![0_u8])
    }

    //////////////////////
    // Implementing Descriptors
    //////////////////////

    fn __get__(
        &self,
        instance: &crate::PyAny,
        owner: &crate::PyAny,
    ) -> crate::PyResult<&crate::PyAny> {
        ::std::panic!("unimplemented isn't hygienic before 1.50")
    }

    fn __set__(&self, instance: &crate::PyAny, owner: &crate::PyAny) {}

    fn __delete__(&self, instance: &crate::PyAny) {}

    fn __set_name__(&self, owner: &crate::PyAny, name: &crate::PyAny) {}

    //////////////////////
    // Implementing Descriptors
    //////////////////////

    fn __len__(&self) -> usize {
        0
    }

    fn __getitem__(&self, key: u32) -> crate::PyResult<u32> {
        ::std::result::Result::Err(crate::exceptions::PyKeyError::new_err("boo"))
    }

    fn __setitem__(&self, key: u32, value: u32) {}

    fn __delitem__(&self, key: u32) {}

    fn __iter__(_: crate::pycell::PyRef<Self>, py: crate::Python) -> crate::Py<DummyIter> {
        crate::Py::new(py, DummyIter {}).unwrap()
    }

    fn __next__(&mut self) -> ::std::option::Option<()> {
        ::std::option::Option::None
    }

    fn __reversed__(slf: crate::pycell::PyRef<Self>, py: crate::Python) -> crate::Py<DummyIter> {
        crate::Py::new(py, DummyIter {}).unwrap()
    }

    fn __contains__(&self, item: u32) -> bool {
        false
    }

    //////////////////////
    // Emulating numeric types
    //////////////////////

    fn __add__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __sub__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __mul__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __truediv__(&self, _other: &Self) -> crate::PyResult<()> {
        ::std::result::Result::Err(crate::exceptions::PyZeroDivisionError::new_err("boo"))
    }

    fn __floordiv__(&self, _other: &Self) -> crate::PyResult<()> {
        ::std::result::Result::Err(crate::exceptions::PyZeroDivisionError::new_err("boo"))
    }

    fn __mod__(&self, _other: &Self) -> u32 {
        0
    }

    fn __divmod__(&self, _other: &Self) -> (u32, u32) {
        (0, 0)
    }

    fn __pow__(&self, _other: &Self, modulo: ::std::option::Option<i32>) -> Dummy {
        Dummy {}
    }

    fn __lshift__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __rshift__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __and__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __xor__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __or__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __radd__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __rrsub__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __rmul__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __rtruediv__(&self, _other: &Self) -> crate::PyResult<()> {
        ::std::result::Result::Err(crate::exceptions::PyZeroDivisionError::new_err("boo"))
    }

    fn __rfloordiv__(&self, _other: &Self) -> crate::PyResult<()> {
        ::std::result::Result::Err(crate::exceptions::PyZeroDivisionError::new_err("boo"))
    }

    fn __rmod__(&self, _other: &Self) -> u32 {
        0
    }

    fn __rdivmod__(&self, _other: &Self) -> (u32, u32) {
        (0, 0)
    }

    fn __rpow__(&self, _other: &Self, modulo: ::std::option::Option<i32>) -> Dummy {
        Dummy {}
    }

    fn __rlshift__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __rrshift__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __rand__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __rxor__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __ror__(&self, other: &Self) -> Dummy {
        Dummy {}
    }

    fn __iadd__(&mut self, other: &Self) {}

    fn __irsub__(&mut self, other: &Self) {}

    fn __imul__(&mut self, other: &Self) {}

    fn __itruediv__(&mut self, _other: &Self) {}

    fn __ifloordiv__(&mut self, _other: &Self) {}

    fn __imod__(&mut self, _other: &Self) {}

    fn __ipow__(&mut self, _other: &Self, _modulo: ::std::option::Option<u32>) {}
    fn __ilshift__(&mut self, other: &Self) {}

    fn __irshift__(&mut self, other: &Self) {}

    fn __iand__(&mut self, other: &Self) {}

    fn __ixor__(&mut self, other: &Self) {}

    fn __ior__(&mut self, other: &Self) {}

    fn __neg__(slf: crate::pycell::PyRef<Self>) -> crate::pycell::PyRef<Self> {
        slf
    }

    fn __pos__(slf: crate::pycell::PyRef<Self>) -> crate::pycell::PyRef<Self> {
        slf
    }

    fn __abs__(slf: crate::pycell::PyRef<Self>) -> crate::pycell::PyRef<Self> {
        slf
    }

    fn __invert__(slf: crate::pycell::PyRef<Self>) -> crate::pycell::PyRef<Self> {
        slf
    }

    fn __complex__<'py>(&self, py: crate::Python<'py>) -> &'py crate::types::PyComplex {
        crate::types::PyComplex::from_doubles(py, 0.0, 0.0)
    }

    fn __int__(&self) -> u32 {
        0
    }

    fn __float__(&self) -> f64 {
        0.0
    }

    fn __index__(&self) -> u32 {
        0
    }

    fn __round__(&self, ndigits: ::std::option::Option<u32>) -> u32 {
        0
    }

    fn __trunc__(&self) -> u32 {
        0
    }

    fn __floor__(&self) -> u32 {
        0
    }

    fn __ceil__(&self) -> u32 {
        0
    }

    //////////////////////
    // With Statement Context Managers
    //////////////////////

    fn __enter__(&mut self) {}

    fn __exit__(
        &mut self,
        exc_type: &crate::PyAny,
        exc_value: &crate::PyAny,
        traceback: &crate::PyAny,
    ) {
    }

    //////////////////////
    // Awaitable Objects
    //////////////////////

    fn __await__(slf: crate::pycell::PyRef<Self>) -> crate::pycell::PyRef<Self> {
        slf
    }

    //////////////////////

    // Asynchronous Iterators
    //////////////////////

    fn __aiter__(slf: crate::pycell::PyRef<Self>, py: crate::Python) -> crate::Py<DummyIter> {
        crate::Py::new(py, DummyIter {}).unwrap()
    }

    fn __anext__(&mut self) -> ::std::option::Option<()> {
        ::std::option::Option::None
    }

    //////////////////////
    // Asynchronous Context Managers
    //////////////////////

    fn __aenter__(&mut self) {}

    fn __aexit__(
        &mut self,
        exc_type: &crate::PyAny,
        exc_value: &crate::PyAny,
        traceback: &crate::PyAny,
    ) {
    }

    // Things with attributes

    #[args(x = "1", "*", _z = "2")]
    fn test(&self, _y: &Dummy, _z: i32) {}
    #[staticmethod]
    fn staticmethod() {}
    #[classmethod]
    fn clsmethod(_: &crate::types::PyType) {}
    #[args(args = "*", kwds = "**")]
    fn __call__(
        &self,
        _args: &crate::types::PyTuple,
        _kwds: ::std::option::Option<&crate::types::PyDict>,
    ) -> crate::PyResult<i32> {
        ::std::panic!("unimplemented isn't hygienic before 1.50")
    }
    #[new]
    fn new(a: u8) -> Self {
        Dummy {}
    }
    #[getter]
    fn get(&self) -> i32 {
        0
    }
    #[setter]
    fn set(&mut self, _v: i32) {}
    #[classattr]
    fn class_attr() -> i32 {
        0
    }

    // Dunder methods invented for protocols

    fn __richcmp__(
        &self,
        other: &Self,
        op: crate::class::basic::CompareOp,
    ) -> crate::PyResult<bool> {
        ::std::result::Result::Ok(false)
    }
    // PyGcProtocol
    // Buffer protocol?
}
