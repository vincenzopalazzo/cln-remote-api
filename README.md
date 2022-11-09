<div align="center">
  <h1>cln-remote-api</h1>

  <img src="https://image.winudf.com/v2/image1/Y29tLmFwcC5yZXN0Y2xpZW50X2ljb25fMTU1NDg4MTMwM18wMjY/icon.png?w=&fakeurl=1" width="250" height="250" />

  <p>
    <strong> Rust Remote API for core lightning provided as library and a plugin </strong>
  </p>

  <h4>
    <a href="https://github.com/vincenzopalazzo/cln-rest-api-keet">Project Homepage</a>
  </h4>
</div>

## Table of Content

- Introduction
- How to Use
- How to Contribute
- License 

## Introduction

A simple and fast plugin for core lightning to talk through the following API:

- Rest API: Not authenticate (Support for Rune in the pipeline) API that support some basic API to talk with
            core lightning node.

N.B: The code is written to be easily extensible to support new API, so please consider to contribute.

## How to Use

To use the plugin you can download the binary from the Github release inside the `.lightning/plugins` directory, or 
link the plugin with core `lightningd` with `--plugin=<path>`.

For the moment it is not possible restart the plugin at runtime, but it is possible to implement is someone will required it.

## How to Contribute

Read our [Hacking guide](https://github.com/laanwj/cln4rust/blob/master/docs/MAINTAINERS.md)

## License

    Rust Remote API for core lightning provided as library and a plugin
    Copyright (C) 2022 Vincenzo Palazzo <vincenzopalazzodev@gmail.com>

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License along
    with this program; if not, write to the Free Software Foundation, Inc.,
    51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

