l:
	uv run mypy .
	uv run ruff check . --fix

f:
	uv run ruff format .

lf: l f

test:
	maturin develop
	uv run pytest -vvv
