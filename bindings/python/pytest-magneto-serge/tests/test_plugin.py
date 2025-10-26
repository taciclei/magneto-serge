"""
Tests for pytest-magneto-serge plugin
"""

import pytest
from pytest_magneto_serge import magneto_cassette, use_cassette


def test_config_fixture(magneto_config):
    """Test magneto_config fixture"""
    assert magneto_config.cassette_dir == 'tests/cassettes'
    assert magneto_config.mode == 'auto'
    assert magneto_config.record_mode == 'new_episodes'


def test_translate_record_mode(magneto_config):
    """Test record mode translation"""
    assert magneto_config.translate_record_mode('new_episodes') == 'auto'
    assert magneto_config.translate_record_mode('once') == 'replay'
    assert magneto_config.translate_record_mode('all') == 'record'
    assert magneto_config.translate_record_mode('none') == 'replay'
    assert magneto_config.translate_record_mode('unknown') == 'auto'


@pytest.mark.magneto_cassette('test_marker')
def test_marker_usage(magneto_proxy):
    """Test using magneto_cassette marker"""
    # Proxy should be auto-configured
    assert magneto_proxy is not None


def test_decorator_usage():
    """Test using @magneto_cassette decorator"""

    @magneto_cassette('test_decorator')
    def dummy_test():
        return True

    result = dummy_test()
    assert result is True


def test_context_manager():
    """Test using use_cassette context manager"""
    with use_cassette('test_context'):
        # Context manager should handle cassette lifecycle
        pass


def test_nested_context_managers():
    """Test nested use_cassette calls"""
    with use_cassette('outer'):
        with use_cassette('inner'):
            pass


class TestConfiguration:
    """Test configuration changes"""

    def test_config_cassette_dir(self, magneto_config):
        original = magneto_config.cassette_dir
        magneto_config.cassette_dir = 'custom/path'
        assert magneto_config.cassette_dir == 'custom/path'
        magneto_config.cassette_dir = original

    def test_config_mode(self, magneto_config):
        original = magneto_config.mode
        magneto_config.mode = 'replay'
        assert magneto_config.mode == 'replay'
        magneto_config.mode = original

    def test_config_verbose(self, magneto_config):
        original = magneto_config.verbose
        magneto_config.verbose = True
        assert magneto_config.verbose is True
        magneto_config.verbose = original
