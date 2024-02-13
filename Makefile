.PHONY: make-release
make-release:
	@ if [ -z "${v}" ]; then (echo 'Missing version. e.g. v=0.1.0' >&2 && exit 1) fi
	
	@ args=""; \
	if [ -n "${v}" ]; then args="-v ${v}"; fi; \
	./.build/make-release.sh $${args}