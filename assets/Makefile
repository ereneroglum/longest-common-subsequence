PLANTUML ?= plantuml
OUTPUT_FOLDER := generated
DIAGRAMS :=

.PHONY: all clean

all: $(OUTPUT_FOLDER)/%.svg

$(OUTPUT_FOLDER)/%.svg: $(DIAGRAMS:=.puml)
	$(PLANTUML) -tsvg $^ -o $(OUTPUT_FOLDER)

clean:
	rm -rf $(OUTPUT_FOLDER)

