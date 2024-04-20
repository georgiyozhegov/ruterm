profile:
	export CARGO_PROFILE_RELEASE_DEBUG=true
	cargo flamegraph --example car
	export CARGO_PROFILE_RELEASE_DEBUG=false
	gtk-launch org.mozilla.firefox.desktop flamegraph.svg

clean:
	rm perf.data* flamegraph.svg
