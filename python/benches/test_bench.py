from pytest_benchmark.fixture import BenchmarkFixture


LEN = 100
INPUT = "thisIsACamelCaseString" * LEN
EXPECT = "this_is_a_camel_case_string" * LEN


def test_bench_to_snake_pure_python(benchmark: BenchmarkFixture):
    def to_snake(s: str) -> str:
        return "".join(["_" + c.lower() if c.isupper() else c for c in s]).lstrip("_")

    assert benchmark(to_snake, INPUT) == EXPECT


def test_bench_to_snake_python_re(benchmark: BenchmarkFixture):
    import re

    pattern = re.compile(r"(?<!^)(?=[A-Z])")

    def to_snake(s: str) -> str:
        return pattern.sub("_", s).lower()

    assert benchmark(to_snake, INPUT) == EXPECT


def test_bench_to_snake_anycase(benchmark: BenchmarkFixture):
    from anycase import to_snake

    assert benchmark(to_snake, INPUT) == EXPECT


def test_bench_to_snake_caseconversion(benchmark: BenchmarkFixture):
    from case_conversion import snakecase as to_snake

    assert benchmark(to_snake, INPUT) == EXPECT


def test_bench_to_snake_inflection(benchmark: BenchmarkFixture):
    from inflection import underscore as to_snake

    assert benchmark(to_snake, INPUT) == EXPECT


def test_bench_to_snake_pydantic(benchmark: BenchmarkFixture):
    from pydantic.alias_generators import to_snake

    assert benchmark(to_snake, INPUT) == EXPECT


def test_bench_to_snake_pyheck(benchmark: BenchmarkFixture):
    from pyheck import snake as to_snake

    assert benchmark(to_snake, INPUT) == EXPECT


def test_bench_to_snake_stringcase(benchmark: BenchmarkFixture):
    from stringcase import snakecase as to_snake

    assert benchmark(to_snake, INPUT) == EXPECT
