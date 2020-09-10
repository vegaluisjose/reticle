import pytest


def pytest_addoption(parser):
    parser.addoption(
        "--docker",
        action="store",
        default="True",
        help="enable or disable docker for ci",
    )


@pytest.fixture
def docker(request):
    return request.config.getoption("--docker")
