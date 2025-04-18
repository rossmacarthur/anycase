import json
from dataclasses import dataclass
from pathlib import Path
import re
import os

import anycase
import pytest


COMMON = Path(__file__).parent.parent.parent / "testdata" / "common.json"


@dataclass(kw_only=True)
class TestCase:
    input: str
    snake: str
    camel: str
    pascal: str
    screaming_snake: str
    kebab: str
    screaming_kebab: str
    train: str
    lower: str
    title: str
    upper: str


TESTS = [
    TestCase(
        input=case["input"],
        snake=case["snake"],
        camel=case["camel"],
        pascal=case["pascal"],
        screaming_snake=case["screaming_snake"],
        kebab=case["kebab"],
        screaming_kebab=case["screaming_kebab"],
        train=case["train"],
        lower=case["lower"],
        title=case["title"],
        upper=case["upper"],
    )
    for case in json.loads(open(COMMON).read())
]


@pytest.mark.parametrize("case", TESTS, ids=lambda case: case.input or "empty")
def test_common(case: TestCase):
    assert anycase.to_snake(case.input) == case.snake
    assert anycase.to_camel(case.input) == case.camel
    assert anycase.to_pascal(case.input) == case.pascal
    assert anycase.to_screaming_snake(case.input) == case.screaming_snake
    assert anycase.to_kebab(case.input) == case.kebab
    assert anycase.to_screaming_kebab(case.input) == case.screaming_kebab
    assert anycase.to_train(case.input) == case.train
    assert anycase.to_lower(case.input) == case.lower
    assert anycase.to_title(case.input) == case.title
    assert anycase.to_upper(case.input) == case.upper


def test_to_camel_with_acronyms():
    assert (
        anycase.to_camel("xml_http_request", acronyms={"xml": "XML"})
        == "xmlHttpRequest"
    )
    assert (
        anycase.to_camel("xml_http_request", acronyms={"http": "HTTP"})
        == "xmlHTTPRequest"
    )


def test_to_pascal_with_acronyms():
    assert (
        anycase.to_pascal("xml_http_request", acronyms={"xml": "XML"})
        == "XMLHttpRequest"
    )
    assert (
        anycase.to_pascal("xml_http_request", acronyms={"xml": "XML", "http": "HTTP"})
        == "XMLHTTPRequest"
    )
    assert (
        anycase.to_pascal("xml_http_request", acronyms={"xml": "XML", "http": "Http"})
        == "XMLHttpRequest"
    )


def test_to_train_with_acronyms():
    assert (
        anycase.to_train("xml_http_request", acronyms={"xml": "XML"})
        == "XML-Http-Request"
    )
    assert (
        anycase.to_train("xml_http_request", acronyms={"xml": "XML", "http": "HTTP"})
        == "XML-HTTP-Request"
    )
    assert (
        anycase.to_train("xml_http_request", acronyms={"xml": "XML", "http": "Http"})
        == "XML-Http-Request"
    )


def test_to_title_with_acronyms():
    assert (
        anycase.to_title("xml_http_request", acronyms={"xml": "XML"})
        == "XML Http Request"
    )
    assert (
        anycase.to_title("xml_http_request", acronyms={"xml": "XML", "http": "HTTP"})
        == "XML HTTP Request"
    )
    assert (
        anycase.to_title("xml_http_request", acronyms={"xml": "XML", "http": "Http"})
        == "XML Http Request"
    )


def examples() -> list[tuple[str, str]]:
    pyi_file = os.path.join(os.path.dirname(__file__), "..", "anycase", "__init__.pyi")
    with open(pyi_file) as f:
        contents = f.read()
    examples = re.findall(r"^\s*>>> (.*)\n\s*(.*)$", contents, re.MULTILINE)
    assert len(examples) == 14
    return list(examples)


@pytest.mark.parametrize("case", examples())
def test_doc_example(case: tuple[str, str]):
    code, expected = case
    exec(f"""result = {code}\nassert result == {expected}""")
