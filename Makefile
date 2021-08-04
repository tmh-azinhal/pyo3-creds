setup:
	pip install virtualenv
	virtualenv --python=python3.7 venv37

build:
	source venv37/bin/activate
	pip install -r requirements.txt
	maturin build --release
