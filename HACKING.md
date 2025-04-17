# Hacking on artifex-client-gtk

## Development

To build for development (i.e. without installing):

```sh
mkdir _build
meson setup . _build
meson compile -C _build
```

To test for development:

```sh
meson devenv -C _build
./src/artifex-client-gtk
```

## Translation

Translating requires the following tools:

- [xtr](https://crates.io/crates/xtr)
- [gettext-tools](https://www.gnu.org/software/gettext/)

To generate the template:

```sh
find src -name '*.rs' -exec xtr -o po/code.pot
xgettext --from-code=UTF-8 _build/data/resources/ui/*.ui data/resources/ui/*.ui -o po/ui.pot
msgcat po/code.pot po/ui.pot > po/artifex-client-gtk.pot
```

To create a translation (e.g. french):

```sh
msginit --input po/artifex-client-gtk.pot --output po/fr.po --locale fr
```

To update the french translation:

```sh
msgmerge --update po/fr.po po/artifex-client-gtk.pot
```
