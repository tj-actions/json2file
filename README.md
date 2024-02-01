[![Crates.io](https://img.shields.io/crates/v/json2file)](https://crates.io/crates/json2file)
[![Codacy Badge](https://app.codacy.com/project/badge/Grade/b60a9e369c774e11a813baa11362b99d)](https://app.codacy.com/gh/tj-actions/json2file/dashboard?utm_source=gh\&utm_medium=referral\&utm_content=\&utm_campaign=Badge_grade)
[![codecov](https://codecov.io/gh/tj-actions/json2file/branch/main/graph/badge.svg?token=FALQKDPDP2)](https://codecov.io/gh/tj-actions/json2file)
[![CI](https://github.com/tj-actions/json2file/workflows/CI/badge.svg)](https://github.com/tj-actions/json2file/actions?query=workflow%3ACI)
[![Update release version.](https://github.com/tj-actions/json2file/actions/workflows/sync-release-version.yml/badge.svg)](https://github.com/tj-actions/json2file/actions/workflows/sync-release-version.yml)
[![Public workflows that use this action.](https://img.shields.io/endpoint?url=https%3A%2F%2Fused-by.vercel.app%2Fapi%2Fgithub-actions%2Fused-by%3Faction%3Dtj-actions%2Fjson2file%26badge%3Dtrue)](https://github.com/search?o=desc\&q=tj-actions+json2file+language%3AYAML\&s=\&type=Code)

[![Ubuntu](https://img.shields.io/badge/Ubuntu-E95420?logo=ubuntu\&logoColor=white)](https://docs.github.com/en/actions/reference/workflow-syntax-for-github-actions#jobsjob_idruns-on)
[![Mac OS](https://img.shields.io/badge/mac%20os-000000?logo=macos\&logoColor=F0F0F0)](https://docs.github.com/en/actions/reference/workflow-syntax-for-github-actions#jobsjob_idruns-on)
[![Windows](https://img.shields.io/badge/Windows-0078D6?logo=windows\&logoColor=white)](https://docs.github.com/en/actions/reference/workflow-syntax-for-github-actions#jobsjob_idruns-on)

# json2file

Simplifies the process of serializing JSON data to a file.

## Usage

```yaml
...
    - name: Set output
      id: set-output
      run: |
        echo "color=green" >> $GITHUB_OUTPUT

    - name: Generate file output from json
      uses: tj-actions/json2file@v1
      with:
        directory: 'output'
        outputs:  ${{ toJSON(steps.set-output.outputs) }}
        keys: 'color'
        extension: 'txt'
...
```

The above example will create a file named `color.txt` in the `output` directory with the contents `green`.

```bash
$ cat ./output/color.txt
green
```

## Inputs

<!-- AUTO-DOC-INPUT:START - Do not remove or modify this section -->

|                                        INPUT                                        |  TYPE  | REQUIRED |       DEFAULT       |                          DESCRIPTION                          |
|-------------------------------------------------------------------------------------|--------|----------|---------------------|---------------------------------------------------------------|
|              <a name="input_bin_path"></a>[bin\_path](#input_bin_path)               | string |  false   |                     |                      Path to the binary                       |
|             <a name="input_directory"></a>[directory](#input_directory)             | string |   true   | `".github/outputs"` |                     Directory to write to                     |
|             <a name="input_extension"></a>[extension](#input_extension)             | string |  false   |       `"txt"`       |    File extension to use, possible <br>values: txt, json      |
|                    <a name="input_keys"></a>[keys](#input_keys)                     | string |   true   |                     |   List of Keys to read <br>from the `outputs` JSON string     |
|                <a name="input_outputs"></a>[outputs](#input_outputs)                | string |   true   |                     |                          JSON string                          |
| <a name="input_skip_missing_keys"></a>[skip\_missing\_keys](#input_skip_missing_keys) | string |  false   |      `"false"`      | Skip missing keys not found <br>in the `outputs` JSON string  |

<!-- AUTO-DOC-INPUT:END -->


## Installation

To install `json2file`, open a terminal window (Command Prompt or PowerShell on Windows) and run the following command:

```shell
cargo install json2file
```

### Building from Source

If you prefer to build from the source, follow these steps:

```shell
git clone https://github.com/tj-actions/json2file
cd json2file
cargo build --release
```

## CLI Reference

Run `json2file --help` for more information.

```bash
$ json2file --help
Generate file output from JSON

Usage: json2file [OPTIONS] --keys <KEYS> --outputs <OUTPUTS> --directory <DIRECTORY>

Options:
  -k, --keys <KEYS>            Space delimited list of keys to extract from the JSON output
  -o, --outputs <OUTPUTS>      The JSON output to use
  -d, --directory <DIRECTORY>  The directory to output the files to
  -s, --skip-missing-keys      Skip missing keys
  -e, --extension <EXTENSION>  The extension to use for the files [default: txt] [possible values: txt, json, csv]
  -v, --verbose                
  -h, --help                   Print help
  -V, --version                Print version
```

### Example

Run

```bash
json2file --keys="foo bar" --outputs="{\"foo\": \"value1\", \"bar\": \"value2\"}" --directory=/tmp --extension=txt
```

This creates two files with the following contents:

`foo.txt`

```txt
value1
```

`bar.txt`

```txt
value2
```

*   Free software: [MIT license](LICENSE)

If you feel generous and want to show some extra appreciation:

[![Buy me a coffee][buymeacoffee-shield]][buymeacoffee]

[buymeacoffee]: https://www.buymeacoffee.com/jackton1

[buymeacoffee-shield]: https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png

## Credits

This package was created with [Cookiecutter](https://github.com/cookiecutter/cookiecutter) using [cookiecutter-action](https://github.com/tj-actions/cookiecutter-action)

## Report Bugs

Report bugs at https://github.com/tj-actions/json2file/issues.

If you are reporting a bug, please include:

*   Your operating system name and VERSION.
*   Any details about your workflow that might be helpful in troubleshooting.
*   Detailed steps to reproduce the bug.
