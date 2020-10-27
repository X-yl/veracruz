# This makefile is used within the docker image generated by docker/Dockerfile
#
# AUTHORS
#
# The Veracruz Development Team.
#
# COPYRIGHT
#
# See the `LICENSE.markdown` file in the Veracruz root directory for licensing
# and copyright information.
 
.PHONY: all sdk test_cases sgx-durango-test trustzone-durango-test sgx trustzone sgx-sinaloa-test sgx-sinaloa-performance sgx-veracruz-test sgx-psa-attestation tz-psa-attestationtrustzone-sinaloa-test-setting  trustzone-veracruz-test-setting trustzone-env sgx-env tlaxcala trustzone-test-env clean clean-cargo-lock fmt 

 
WARNING_COLOR := "\e[1;33m"
INFO_COLOR := "\e[1;32m"
RESET_COLOR := "\e[0m"
OPTEE_DIR_SDK ?= /work/rust-optee-trustzone-sdk/
AARCH64_OPENSSL_DIR ?= /work/rust-optee-trustzone-sdk/optee-qemuv8-3.7.0/build/openssl-1.0.2s/
AARCH64_GCC ?= $(OPTEE_DIR)/toolchains/aarch64/bin/aarch64-linux-gnu-gcc
SGX_RUST_FLAG ?= "-L/work/sgxsdk/lib64 -L/work/sgxsdk/sdk_libs"
 
all:
	@echo $(WARNING_COLOR)"Please explicitly choose a target."$(RESET_COLOR)

# Build all of the SDK and examples
sdk:
	$(MAKE) -C sdk

# Generate all test policy
test_cases: sdk
	$(MAKE) -C test-collateral

# Test durango for sgx, due to the use of a mocked server with a fixed port, these tests must run in a single thread
sgx-durango-test: sgx test_cases 
	cd durango && RUSTFLAGS=$(SGX_RUST_FLAG) cargo test --lib --features "mock sgx" -- --test-threads=1

# Test durango for sgx, due to the use of a mocked server with a fixed port, these tests must run in a single thread
trustzone-durango-test: trustzone test_cases
	cd durango && cargo test --lib --features "mock tz" -- --test-threads=1

# Compile for sgx
# offset the CC OPENSSL_DIR, which might be used in compiling trustzone
sgx: sdk sgx-env
	cd mexico-city-bind && RUSTFLAGS=$(SGX_RUST_FLAG) cargo build
	cd sonora-bind && RUSTFLAGS=$(SGX_RUST_FLAG) cargo build
	cd durango && RUSTFLAGS=$(SGX_RUST_FLAG) cargo build --lib --features sgx

# Compile for trustzone, note: source the rust-optee-trustzone-sdk/environment first, however assume `unset CC`.
trustzone: sdk trustzone-env
	$(MAKE) -C mexico-city trustzone CC=$(AARCH64_GCC)
	$(MAKE) -C jalisco trustzone
	cd durango && RUSTFLAGS=$(SGX_RUST_FLAG) cargo build --lib --features tz

sgx-sinaloa-test: sgx test_cases
	cd sinaloa-test \
		&& RUSTFLAGS=$(SGX_RUST_FLAG) cargo test --features sgx \
		&& RUSTFLAGS=$(SGX_RUST_FLAG) cargo test test_debug --features sgx  -- --ignored --test-threads=1

sgx-sinaloa-performance: sgx test_cases
	cd sinaloa-test \
		&& RUSTFLAGS=$(SGX_RUST_FLAG) cargo test test_performance_ --features sgx -- --ignored 

sgx-veracruz-test: sgx test_cases
	cd veracruz-test \
		&& RUSTFLAGS=$(SGX_RUST_FLAG) cargo test --features sgx 

sgx-psa-attestation: sgx-env
	cd psa-attestation && cargo build --features sgx

tz-psa-attestation: trustzone-env
	cd psa-attestation && cargo build --target aarch64-unknown-linux-gnu --features tz

trustzone-sinaloa-test: trustzone test_cases trustzone-test-env
	cd sinaloa-test \
		&& export OPENSSL_DIR=$(AARCH64_OPENSSL_DIR) \
		&& cargo test --target aarch64-unknown-linux-gnu --no-run --features tz -- --test-threads=1 \
		&& ./cp_sinaloa_test_tz.sh
	chmod u+x run_sinaloa_test_tz.sh
	./run_sinaloa_test_tz.sh

trustzone-veracruz-test: trustzone test_cases trustzone-test-env
	cd veracruz-test \
		&& export OPENSSL_DIR=$(AARCH64_OPENSSL_DIR) \
		&& cargo test --target aarch64-unknown-linux-gnu --no-run --features tz -- --test-threads=1 \
		&& ./cp_veracruz_tz.sh
	chmod u+x run_veracruz_test_tz.sh
	./run_veracruz_test_tz.sh

trustzone-test-env: tz_test.sh run_tz_test.sh
	chmod u+x $^

trustzone-env:
	unset CC
	rustup target add aarch64-unknown-linux-gnu arm-unknown-linux-gnueabihf
	rustup component add rust-src
	chmod u+x tz_test.sh

sgx-env:
	unset CC

clean:
	cd mexico-city-bind && cargo clean 
	cd sonora-bind && cargo clean
	cd psa-attestation && cargo clean
	cd tabasco && cargo clean
	cd baja && cargo clean
	cd veracruz-utils && cargo clean
	cd sinaloa-test && cargo clean
	cd veracruz-test && cargo clean
	$(MAKE) clean -C mexico-city
	$(MAKE) clean -C jalisco
	$(MAKE) clean -C sinaloa
	$(MAKE) clean -C test-collateral 
	$(MAKE) clean -C sonora
	$(MAKE) clean -C sdk

# NOTE: this target deletes ALL cargo.lock.
clean-cargo-lock:
	$(MAKE) clean -C sdk
	rm -f $(addsuffix /Cargo.lock,baja chihuahua colima durango jalisco mexico-city-bind mexico-city psa-attestation sinaloa-test sinaloa sonora-bind sonora tabasco veracruz-test veracruz-util)

fmt:
	cd baja && cargo fmt
	cd chihuahua && cargo fmt
	cd colima && cargo fmt
	cd durango && cargo fmt
	cd jalisco && cargo fmt
	cd mexico-city && cargo fmt
	cd psa-attestation && cargo fmt
	cd sinaloa-test && cargo fmt
	cd sinaloa && cargo fmt
	cd veracruz-test && cargo fmt
	cd veracruz-utils && cargo fmt
	cd sonora && cargo fmt
	cd tabasco && cargo fmt
	$(MAKE) -C sdk fmt
