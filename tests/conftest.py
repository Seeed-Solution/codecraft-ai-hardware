"""Collection contract for independent Python client/native-host tests.

The Rust CLI owns discovery and dispatch; pytest supplies Python fixture semantics.
"""
import inspect
import pytest


def pytest_collection_modifyitems(session, config, items):
    selected = []
    identities = set()
    objects = set()
    for item in items:
        function = getattr(item, "obj", None)
        module = getattr(item, "module", None)
        if module is not None and getattr(function, "__module__", None) != module.__name__:
            continue
        if inspect.iscoroutinefunction(function):
            raise pytest.UsageError("Async tests require an explicit synchronous runner: " + item.nodeid)
        identity = item.nodeid
        # Bound methods produce temporary objects; compare underlying definitions.
        object_key = (item.path, getattr(function, "__func__", function))
        if identity in identities or object_key in objects:
            raise pytest.UsageError("Duplicate test identity or alias: " + identity)
        identities.add(identity)
        objects.add(object_key)
        selected.append(item)
    items[:] = selected
