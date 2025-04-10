#
# This file is part of artifex-client-gtk
#
# Copyright (c) 2025 Eric Le Bihan
#
# SPDX-License-Identifier: MIT
#

subdir('ui')

resources = gnome.compile_resources(
  'resources',
  'resources.gresource.xml',
  dependencies: blueprints,
  gresource_bundle: true,
  source_dir: meson.current_build_dir(),
  install: true,
  install_dir: pkgdatadir,
)
