#
# This file is part of artifex-client-gtk
#
# Copyright (c) 2025 Eric Le Bihan
#
# SPDX-License-Identifier: MIT
#

blueprints = custom_target(
  'blueprints',
  input: files(
    'batch_execution_page.blp',
    'connection_bar.blp',
    'connection_status_page.blp',
    'inspection_page.blp',
    'operation_page.blp',
    'operations_row.blp',
    'window.blp'
  ),
  output: '.',
  command: [
    find_program('blueprint-compiler'),
    'batch-compile', '@OUTPUT@', '@CURRENT_SOURCE_DIR@', '@INPUT@'
  ],
  build_always_stale: true,
)
