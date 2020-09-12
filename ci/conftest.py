import pytest


def pytest_addoption(parser):
    parser.addoption(
        "--use-docker",
        action="store_true",
        help="use docker, default is disable",
    )


@pytest.fixture
def docker(request):
    if request.config.getoption("--use-docker"):
        return True
    else:
        return False
