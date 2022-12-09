build:
	@cargo build --release

clean:
	@find . -name "target" -type d -prune -exec rm -rf '{}' + | xargs du -chs

.PHONY: build clean