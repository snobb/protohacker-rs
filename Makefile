FLY_APP := protohacker-rs
PKG := ${shell awk -F '"' '/name/ { print $$2 }' ./${TASK}/Cargo.toml 2>/dev/null}

check-env:
ifndef TASK
	$(error TASK variable is undefined)
endif

run:
	cargo run

build:
	cargo build

test:
	cargo test

launch:
	fly launch --copy-config --local-only --name ${FLY_APP} \
		--no-deploy -r lhr && \
	fly ips allocate-v6 -a ${FLY_APP}

deploy-fly: check-env
	fly deploy --local-only -c ./${TASK}/fly.toml --build-arg TASKDIR=${TASK} --build-arg PKG=${PKG}

deploy: deploy-fly clean

destroy:
	fly destroy protohacker-rs

clean:
	-rm -rf .project
	cargo clean
