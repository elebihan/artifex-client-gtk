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
    'command_bar.blp',
    'commands_row.blp',
    'commands_view.blp',
    'connection_bar.blp',
    'connection_status_page.blp',
    'execution_page.blp',
    'inspection_page.blp',
    'preferences_dialog.blp',
    'operation_page.blp',
    'operations_row.blp',
    'upgrade_page.blp',
    'window.blp'
  ),
  output: '.',
  command: [
    find_program('blueprint-compiler'),
    'batch-compile', '@OUTPUT@', '@CURRENT_SOURCE_DIR@', '@INPUT@'
  ],
  build_always_stale: true,
)
