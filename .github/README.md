# Liquid Grammar Pest
[heading__top]:
  #liquid-grammar-pest
  "&#x2B06; Pest grammar for parsing Shopify Liquid"


Pest grammar for parsing Shopify Liquid

<!-- ## [![Byte size of Liquid Grammar Pest][badge__main__liquid_grammar_pest__source_code]][liquid_grammar_pest__main__source_code] [![Open Issues][badge__issues__liquid_grammar_pest]][issues__liquid_grammar_pest] [![Open Pull Requests][badge__pull_requests__liquid_grammar_pest]][pull_requests__liquid_grammar_pest] [![Latest commits][badge__commits__liquid_grammar_pest__main]][commits__liquid_grammar_pest__main]   [![GitHub Actions Build Status][badge__github_actions]][activity_log__github_actions] [![License][badge__license]][branch__current__license] -->


---


- [:arrow_up: Top of Document][heading__top]
- [:building_construction: Requirements][heading__requirements]
- [:zap: Quick Start][heading__quick_start]
- [&#x1F9F0; Usage][heading__usage]
- [&#x1F5D2; Notes][heading__notes]
- [:chart_with_upwards_trend: Contributing][heading__contributing]
  - [:trident: Forking][heading__forking]
  - [:currency_exchange: Sponsor][heading__sponsor]
- [:card_index: Attribution][heading__attribution]
- [:balance_scale: Licensing][heading__license]
  - [Commercial and/or proprietary use][heading__commercial_andor_proprietary_use]
  - [Non-commercial and FOSS use][heading__noncommercial_and_foss_use]


---



## Requirements
[heading__requirements]:
  #requirements
  "&#x1F3D7; Prerequisites and/or dependencies that this project needs to function properly"


This repository requires [Rust][rust_home] language/compiler to build from
source

As of last update to this ReadMe file, the recommended method of installing
Rust is via the installer script...

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```


______


## Quick Start
[heading__quick_start]:
  #quick-start
  "&#9889; Perhaps as easy as one, 2.0,..."


This repository is a Rust library, define it as a dependency within a project
`Cargo.toml` file...

**`Cargo.toml` (snip)**

```toml
[dependencies]
pest = "2.7.10"
liquid_grammar_pest = "0.0.1"
```

> Check
> [Rust -- Doc -- Specifying Dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
> for details about defining dependencies.

Then include within a source file via `use` statement...

**`src/main.rs`**

```Rust
use liquid_grammar_pest::{LiquidParser, Rule};
use pest::Parser;
```


______


## Usage
[heading__usage]:
  #usage
  "&#x1F9F0; How to utilize this repository"


Check the `tests/` and `examples/` files for sample code.


______


## Notes
[heading__notes]:
  #notes
  "&#x1F5D2; Additional things to keep in mind when developing"


This repository may not be feature complete and/or fully functional, Pull
Requests that add features or fix bugs are certainly welcomed.

This repository is focused on providing grammar and, through Pest, tokenizer
only.  It is up to crate consumers to implement interpreter, error
customization, or any additional features.


______


## Contributing
[heading__contributing]:
  #contributing
  "&#x1F4C8; Options for contributing to liquid-grammar-pest and rust-utilities"


Options for contributing to liquid-grammar-pest and rust-utilities


---


### Forking
[heading__forking]:
  #forking
  "&#x1F531; Tips for forking liquid-grammar-pest"


Start making a [Fork][liquid_grammar_pest__fork_it] of this repository to an
account that you have write permissions for.


- Add remote for fork URL. The URL syntax is
  _`git@github.com:<NAME>/<REPO>.git`_...

```bash
cd ~/git/hub/rust-utilities/liquid-grammar-pest

git remote add fork git@github.com:<NAME>/liquid-grammar-pest.git
```

- Commit your changes and push to your fork, eg. to fix an issue...

```bash
cd ~/git/hub/rust-utilities/liquid-grammar-pest


git commit -F- <<'EOF'
:bug: Fixes #42 Issue


**Edits**


- `<SCRIPT-NAME>` script, fixes some bug reported in issue
EOF


git push fork main
```


> Note, the `-u` option may be used to set `fork` as the default remote, eg.
> _`git push -u fork main`_ however, this will also default the `fork` remote
> for pulling from too! Meaning that pulling updates from `origin` must be done
> explicitly, eg. _`git pull origin main`_


- Then on GitHub submit a Pull Request through the Web-UI, the URL syntax is
  _`https://github.com/<NAME>/<REPO>/pull/new/<BRANCH>`_


> Note; to decrease the chances of your Pull Request needing modifications
> before being accepted, please check the
> [dot-github](https://github.com/rust-utilities/.github) repository for
> detailed contributing guidelines.


---


### Sponsor
  [heading__sponsor]:
  #sponsor
  "&#x1F4B1; Methods for financially supporting rust-utilities that maintains liquid-grammar-pest"


Thanks for even considering it!

Via Liberapay you may
<sub>[![sponsor__shields_io__liberapay]][sponsor__link__liberapay]</sub> on a
repeating basis.

Regardless of if you're able to financially support projects such as
liquid-grammar-pest that rust-utilities maintains, please consider sharing
projects that are useful with others, because one of the goals of maintaining
Open Source repositories is to provide value to the community.


______


## Attribution
[heading__attribution]:
  #attribution
  "&#x1F4C7; Resources that where helpful in building this project so far."


- [GitHub -- `github-utilities/make-readme`](https://github.com/github-utilities/make-readme)
- [Pest -- Editor](https://pest.rs/#editor)
- [LiquidJS -- Playground](https://liquidjs.com/playground.html)


______


## License
[heading__license]:
  #license
  "&#x2696; Legal side of Open Source"


This project is licensed based on use-case


---


### Commercial and/or proprietary use
[heading__commercial_andor_proprietary_use]: #commercial-andor-proprietary-use


If a project is **either** commercial or (`||`) proprietary, then please
contact the author for pricing and licensing options to make use of code and/or
features from this repository.


---


### Non-commercial and FOSS use
[heading__noncommercial_and_foss_use]: #noncommercial-and-foss-use


If a project is **both** non-commercial and (`&&`) published with a license
compatible with AGPL-3.0, then it may utilize code from this repository under
the following terms.

```
Pest grammar for parsing Shopify Liquid
Copyright (C) 2024 S0AndS0

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as published
by the Free Software Foundation, version 3 of the License.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```

... For further details review full length version of
[AGPL-3.0][branch__current__license] License.



[branch__current__license]:
  /LICENSE
  "&#x2696; Full length version of AGPL-3.0 License"

[badge__license]:
  https://img.shields.io/github/license/rust-utilities/liquid-grammar-pest

[badge__commits__liquid_grammar_pest__main]:
  https://img.shields.io/github/last-commit/rust-utilities/liquid-grammar-pest/main.svg

[commits__liquid_grammar_pest__main]:
  https://github.com/rust-utilities/liquid-grammar-pest/commits/main
  "&#x1F4DD; History of changes on this branch"


[liquid_grammar_pest__community]:
  https://github.com/rust-utilities/liquid-grammar-pest/community
  "&#x1F331; Dedicated to functioning code"


[issues__liquid_grammar_pest]:
  https://github.com/rust-utilities/liquid-grammar-pest/issues
  "&#x2622; Search for and _bump_ existing issues or open new issues for project maintainer to address."

[liquid_grammar_pest__fork_it]:
  https://github.com/rust-utilities/liquid-grammar-pest/fork
  "&#x1F531; Fork it!"

[pull_requests__liquid_grammar_pest]:
  https://github.com/rust-utilities/liquid-grammar-pest/pulls
  "&#x1F3D7; Pull Request friendly, though please check the Community guidelines"

[liquid_grammar_pest__main__source_code]:
  https://github.com/rust-utilities/liquid-grammar-pest/
  "&#x2328; Project source!"

[badge__issues__liquid_grammar_pest]:
  https://img.shields.io/github/issues/rust-utilities/liquid-grammar-pest.svg

[badge__pull_requests__liquid_grammar_pest]:
  https://img.shields.io/github/issues-pr/rust-utilities/liquid-grammar-pest.svg

[badge__main__liquid_grammar_pest__source_code]:
  https://img.shields.io/github/repo-size/rust-utilities/liquid-grammar-pest


[rust_home]:
  https://www.rust-lang.org/
  "Home page for Rust language"

[rust_github]:
  https://github.com/rust-lang
  "Source code for Rust on GitHub"

[sponsor__shields_io__liberapay]:
  https://img.shields.io/static/v1?logo=liberapay&label=Sponsor&message=rust-utilities

[sponsor__link__liberapay]:
  https://liberapay.com/rust-utilities
  "&#x1F4B1; Sponsor developments and projects that rust-utilities maintains via Liberapay"


[badge__github_actions]:
  https://github.com/rust-utilities/liquid-grammar-pest/actions/workflows/test.yaml/badge.svg?branch=main

[activity_log__github_actions]:
  https://github.com/rust-utilities/liquid-grammar-pest/deployments/activity_log

