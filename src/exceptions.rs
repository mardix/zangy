use pyo3::{create_exception, exceptions::PyException};

create_exception!(zangy, ConnectionError, PyException);
create_exception!(zangy, ArgumentError, PyException);
create_exception!(zangy, RedisError, PyException);
