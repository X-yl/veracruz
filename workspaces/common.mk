PROFILE ?= release
PROFILE_PATH = release
PROFILE_FLAG = --release

export PROFILE

ifeq ($(PROFILE),dev)
    PROFILE_PATH = debug
    PROFILE_FLAG =
endif
