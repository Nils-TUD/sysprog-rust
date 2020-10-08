BUILD=build
PDF=$(BUILD)/main.pdf
SRC=$(wildcard *.tex)
IMG=$(wildcard img/*)

ifeq (, $(shell which ppdflatex 2>/dev/null))
	LATEX=pdflatex
else
	LATEX=ppdflatex
endif

.PHONY: all watch clean

all: $(PDF)

$(BUILD):
	mkdir -p $(BUILD)

$(PDF): $(SRC) $(IMG) $(BUILD)
	$(LATEX) -draftmode -shell-escape -output-directory=$(BUILD) -interaction=nonstopmode main.tex
	$(LATEX) -shell-escape -output-directory=$(BUILD) -interaction=nonstopmode main.tex

watch: $(BUILD)
	latexmk -outdir=$(BUILD) -pdflatex=$(LATEX) -pdf -pvc -r latexmk.rc main.tex

clean:
	rm -f $(BUILD)/*
