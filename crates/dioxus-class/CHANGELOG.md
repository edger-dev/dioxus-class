# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- update with Dioxus 0.5

### Fixed

### Removed

- classes.rs
- class! macro
- style! macro

## [0.3.0] - 2023-03-03

### Removed

- remove components, it's hard to get it right.

## [0.2.2] - 2023-02-27

### Fixed

- cleanup

## [0.2.1] - 2023-02-27

### Added

- add example (todomvc.rs from dioxus examples)

### Fixed

- make class! to use Class::from(), to support more types of expr
- bugfix with class!


## [0.2.0] - 2023-02-25

### Added

- macros (constant!) for easier extension writing

## [0.1.x]

### Added

- Class struct
- build::generate to write files with defined classes, which can be used for css generation when needed (e.g. used for tailwindcss)
- macros (class!, class!) for creating styles that includes classes
