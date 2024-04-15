"""Python binds to a beancount clone in Rust."""

# The Rust bindings are in `_bean_rs` and we try to mediate all access
# through this file to make typing a bit easier
from bean_rs._bean_rs import load

__all__ = ["load"]
