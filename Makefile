TARGET_DIR=target/mos-atari8-none/release
DOS_ATR_TEMPLATE=data/dos2d64.atr
PROJECT=$(basename $(notdir $@))

all: $(addsuffix .done,$(addprefix $(TARGET_DIR)/,$(wildcard day09)))

clean:
	cargo clean
	rm -f examples.atr*

examples.atr.done:
	cp $(DOS_ATR_TEMPLATE) examples.atr
	touch $@

$(TARGET_DIR)/%.done: examples.atr.done %/src/*.rs
    # examples.atr.done examples/%.rs
	cargo build -p $(PROJECT) --release --target mos-atari8-none
	atr examples.atr put $(basename $@) $(PROJECT).com
	touch $@
