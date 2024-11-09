
.PHONY: deps
deps:
	pip install maturin
	pip install maturin[patchelf]

.PHONY: dev
dev:
	maturin develop
