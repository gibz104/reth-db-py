all: develop
	@echo "✅ Python package installation completed!"

develop:
	@echo "🛠 Installing the Python package for development..."
	maturin develop


test: rust_tests
	@echo "✅ All tests successfully completed!"

rust_tests:
	@echo "🏃🦀🧪 Running Rust tests..."
	cargo test


benchmark: rust_benchmarks python_benchmarks
	@echo "✅ Speed tests completed!"

rust_benchmarks:
	@echo "🏃🦀⏱ Running Rust speed tests..."
	cargo +nightly bench

python_benchmarks:
	@echo "🏃🐍⏱ Running Python speed tests..."
	pytest tests/test_benchmarks.py


clean:
	@echo "🧹 Cleaning..."
	cargo clean
	pip uninstall -y reth-db-py
