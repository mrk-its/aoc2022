TARGET_DIR=target/mos-atari8-none/release
CARGO_OPTS=--release
DOS_ATR_TEMPLATE=data/dos2d64.atr
PROJECT=$(basename $(notdir $@))
SET1=day01 day02 day03 day04 day05 day06 day07 day08
SET2=day09 day10 day11 day12 day13 day14 day15 day17
SET3=day18

all: $(addsuffix .xex,$(addprefix $(TARGET_DIR)/,$(wildcard day??))) $(addprefix $(TARGET_DIR)/,part1.atr part2.atr part3.atr)

$(TARGET_DIR)/part1.atr: $(addprefix $(TARGET_DIR)/,$(SET1)) $(DOS_ATR_TEMPLATE)
	cp $(DOS_ATR_TEMPLATE) $@.tmp
	for name in $(SET1); do atr $@.tmp put $(TARGET_DIR)/$$name $$name.com; done
	mv $@.tmp $@

$(TARGET_DIR)/part2.atr: $(addprefix $(TARGET_DIR)/,$(SET2)) $(DOS_ATR_TEMPLATE)
	cp $(DOS_ATR_TEMPLATE) $@.tmp
	for name in $(SET2); do atr $@.tmp put $(TARGET_DIR)/$$name $$name.com; done
	mv $@.tmp $@

$(TARGET_DIR)/part3.atr: $(addprefix $(TARGET_DIR)/,$(SET3)) $(DOS_ATR_TEMPLATE)
	cp $(DOS_ATR_TEMPLATE) $@.tmp
	for name in $(SET3); do atr $@.tmp put $(TARGET_DIR)/$$name $$name.com; done
	mv $@.tmp $@

clean:
	cargo clean

examples.atr.done:
	cp $(DOS_ATR_TEMPLATE) examples.atr
	touch $@

$(TARGET_DIR)/%.xex: $(DOS_ATR_TEMPLATE) %/src/*.rs
	cargo build -p $(PROJECT) $(CARGO_OPTS) --target mos-atari8-none
	cp $(DOS_ATR_TEMPLATE) $(basename $@).atr
	atr $(basename $@).atr put $(basename $@) $(PROJECT).com
	cp $(basename $@) $@
