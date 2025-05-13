l:
	uv run mypy .
	uv run ruff check . --fix

f:
	uv run ruff format .

test:
	maturin develop
	uv run pytest -vvv
