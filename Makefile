install: 
	pip install --upgrade pip && pip install -r requirements.txt 

format: 
	black *.py

lint:
	pylint --disable=R,C --ignore-patterns=test_.*?py $(wildcard *.py)

test: 
	python -m pytest -cov=main test_main.py --disable-warnings

all: install format lint test