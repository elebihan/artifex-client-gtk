#
# This file is part of artifex-client-gtk
#
# Copyright (c) 2025 Eric Le Bihan
#
# SPDX-License-Identifier: MIT
#

scalable_dir = 'hicolor/scalable/apps'
install_data(
  scalable_dir / ('@0@.svg').format(application_id),
  install_dir: get_option('datadir') / 'icons' / scalable_dir,
)

symbolic_dir = 'hicolor/symbolic/apps'
install_data(
  symbolic_dir / ('@0@-symbolic.svg').format(application_id),
  install_dir: get_option('datadir') / 'icons' / symbolic_dir,
)
