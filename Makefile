install: 
	pip install --upgrade pip && pip install -r requirements.txt 

python_format: 
	black *.py

python_lint:
	pylint --disable=R,C --ignore-patterns=test_.*?py $(wildcard *.py)

python_test: 
	python -m pytest -cov=main test_main.py --disable-warnings


rust_format: 
	cargo fmt

rust_lint:
	cargo clippy -- -D warnings

rust_test: 
	cargo test

rust_run:
	cargo run

release:
	cargo build --release