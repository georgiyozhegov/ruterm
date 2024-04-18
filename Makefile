profile:
	cargo flamegraph --release --example car 
	# firefox has some troubles with opening local files
	flatpak run org.chromium.Chromium file://$(shell pwd)/flamegraph.svg

clean:
	rm perf.data* flamegraph.svg
