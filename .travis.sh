#!/bin/bash

if [[ $TRAVIS_OS_NAME == 'linux' ]]; then
  echo Linux
elif [[ $TRAVIS_OS_NAME == 'osx' ]]; then
  echo OSX
  brew update
  brew outdated libidn || brew upgrade libidn
  brew outdated autoconf || brew upgrade autoconf
  brew outdated automake || brew upgrade automake
  brew outdated autopoint || brew upgrade autopoint
  brew outdated libtool || brew upgrade libtool
  brew outdated gettext || brew upgrade gettext
  brew outdated flex || brew upgrade flex
  brew outdated gtk-doc || brew upgrade gtk-doc
  brew outdated gnome-doc-utils || brew upgrade gnome-doc-utils
fi
