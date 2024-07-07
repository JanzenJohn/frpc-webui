#!/bin/bash

set -e

frpc-webui &
(cd frontend && npm run dev -- --host)