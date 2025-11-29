# Makefile for Sphinx documentation
#
# Usage:
#   make html      - Build HTML documentation
#   make clean     - Remove build artifacts
#   make linkcheck - Check external links
#   make livehtml  - Build with auto-reload (requires sphinx-autobuild)
#   make rustdoc   - Generate and stage rustdoc
#   make full      - Full build with rustdoc

SPHINXOPTS    ?= -W
SPHINXBUILD   ?= sphinx-build
SOURCEDIR     = .
BUILDDIR      = _build

# Default target
.PHONY: help
help:
	@echo "Sphinx documentation build targets:"
	@echo ""
	@echo "  html       Build HTML documentation"
	@echo "  clean      Remove build artifacts"
	@echo "  linkcheck  Check external links"
	@echo "  livehtml   Build with auto-reload (requires sphinx-autobuild)"
	@echo "  rustdoc    Generate and stage rustdoc artifacts"
	@echo "  full       Full build including rustdoc"
	@echo ""

.PHONY: html
html:
	$(SPHINXBUILD) -b html $(SPHINXOPTS) $(SOURCEDIR) $(BUILDDIR)/html

.PHONY: clean
clean:
	rm -rf $(BUILDDIR)

.PHONY: linkcheck
linkcheck:
	$(SPHINXBUILD) -b linkcheck $(SPHINXOPTS) $(SOURCEDIR) $(BUILDDIR)/linkcheck

.PHONY: livehtml
livehtml:
	sphinx-autobuild -b html $(SPHINXOPTS) $(SOURCEDIR) $(BUILDDIR)/html

.PHONY: rustdoc
rustdoc:
	cd .. && cargo doc --no-deps --all-features
	cd .. && ./scripts/stage-rustdoc.sh

.PHONY: full
full: rustdoc html
	@echo "✓ Full documentation build complete"
