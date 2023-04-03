import polyvalid
import pytest

polyvalid = polyvalid.bindings.polyvalid()


@pytest.mark.parametrize(
    "input_string, should_match",
    [
        # valid cases
        ("ValidName1", True),
        ("helloworld", True),
        ("HELLOWORLD", True),
        ("HelloworlD", True),
        ("d981273", True),
        ("Valid-Name", True),
        ("hello-world", True),
        ("hel__lo", True),
        # invalid cases
        ("1InvalidName", False),
        ("Inva@lidName", False),
        ("InvalidName!", False),
        (" hello ", False),
        ("-hello", False),
        ("hello-", False),
        ("-hello-", False),
        ("_hello", False),
        ("hello_", False),
        ("--hello", False),
        ("hello--", False),
        ("hel--lo", False),
        ("__hello", False),
        ("hello__", False),
        ("_hello_", False),
    ],
)
def test_validation(input_string, should_match):
    assert polyvalid.is_app_name_valid(input_string).is_valid == should_match
