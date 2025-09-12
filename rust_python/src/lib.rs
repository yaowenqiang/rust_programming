use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// 计算斐波那契数列 (Rust实现)
#[pyfunction]
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

/// 高性能斐波那契数列 (迭代实现)
#[pyfunction]
fn fast_fibonacci(n: u64) -> u64 {
    let (mut a, mut b) = (0, 1);
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    a
}

/// 字符串处理示例
#[pyfunction]
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

/// 注册Python模块
#[pymodule]
fn rust_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    m.add_function(wrap_pyfunction!(fast_fibonacci, m)?)?;
    m.add_function(wrap_pyfunction!(reverse_string, m)?)?;

    Ok(())
}
