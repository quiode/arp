#!/bin/bash
mkdir -p "$HOME"/.local/share/glib-2.0/schemas
cp com.github.quiode.arp.gschema.xml "$HOME"/.local/share/glib-2.0/schemas/
glib-compile-schemas "$HOME"/.local/share/glib-2.0/schemas/
