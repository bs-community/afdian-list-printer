# afdian-list-printer

Fetch sponsors list on [afdian.net](https://afdian.net/) and update your files (such as READMEs).

## Usage

Copy the example configuration file as `config.toml`.

Then, add your account and password on [afdian.net](https://afdian.net/) to the configuration file.
Also, you need to specify which files should be updated. Only Markdown files are supported.

If you need to split different users as sponsors and backers, tweak the `amount` value. `0.0` means all users are sponsors.

Last, just run this program.

## License

MIT License

Copyright (c) 2016-present The Blessing Skin Team
