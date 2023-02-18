#!/bin/bash

VSCODE_WS="$1"
SSH_REMOTE="$2"
GDBPORT="$3"

APP="accel_model"
TARGET_ARCH="aarch64-unknown-linux-gnu"
BUILD_BIN_FILE="${VSCODE_WS}/target/${TARGET_ARCH}/debug/${APP}"
TARGET_USER="mendel"
TARGET_BIN_FILE="/home/${TARGET_USER}/Projects/rust_acc/${APP}"
TARGET_CWD="/home/${TARGET_USER}/Projects/rust_acc"

ssh "${TARGET_USER}@${SSH_REMOTE}" "killall lldb-server ${APP}"

if ! rsync -avz "${BUILD_BIN_FILE}" "${TARGET_USER}@${SSH_REMOTE}:${TARGET_BIN_FILE}"; then
    # If rsync doesn't work, it may not be available on target. Fallback to trying SSH copy.
    if ! scp "${BUILD_BIN_FILE}" "${TARGET_USER}@${SSH_REMOTE}:${TARGET_BIN_FILE}"; then
        exit 2
    fi
fi

ssh -f "${TARGET_USER}@${SSH_REMOTE}" "sh -c 'cd ${TARGET_CWD}; nohup lldb-server platform --listen *:${GDBPORT} --server > /dev/null 2>&1 &'"
