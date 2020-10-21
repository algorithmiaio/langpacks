from . import oracle_connector_test

def test_oracle_connector_test():
    assert oracle_connector_test.apply("Jane") == "hello Jane"
