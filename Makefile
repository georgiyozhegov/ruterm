profile:
	cargo flamegraph --example car
	flatpak run org.mozilla.firefox file://$(shell pwd)/flamegraph.svg

clean:
	rm perf.data* flamegraph.svg
