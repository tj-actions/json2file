[![Crates.io](https://img.shields.io/crates/v/json2file)](https://crates.io/crates/json2file)
[![Codacy Badge](https://app.codacy.com/project/badge/Grade/29ec338049e8410cbf7f858d84b22ab8)](https://www.codacy.com/gh/tj-actions/json2file/dashboard?utm_source=github.com\&utm_medium=referral\&utm_content=tj-actions/json2file\&utm_campaign=Badge_Grade)
[![codecov](https://codecov.io/github/tj-actions/json2file/branch/main/graph/badge.svg?token=FALQKDPDP2)](https://codecov.io/github/tj-actions/json2file)
[![CI](https://github.com/tj-actions/json2file/workflows/CI/badge.svg)](https://github.com/tj-actions/json2file/actions?query=workflow%3ACI)
[![Update release version.](https://github.com/tj-actions/json2file/actions/workflows/sync-release-version.yml/badge.svg)](https://github.com/tj-actions/json2file/actions/workflows/sync-release-version.yml)
[![Public workflows that use this action.](https://img.shields.io/endpoint?url=https%3A%2F%2Fused-by.vercel.app%2Fapi%2Fgithub-actions%2Fused-by%3Faction%3Dtj-actions%2Fjson2file%26badge%3Dtrue)](https://github.com/search?o=desc\&q=tj-actions+json2file+language%3AYAML\&s=\&type=Code)

[![Ubuntu](https://img.shields.io/badge/Ubuntu-E95420?logo=ubuntu\&logoColor=white)](https://docs.github.com/en/actions/reference/workflow-syntax-for-github-actions#jobsjob_idruns-on)
[![Mac OS](https://img.shields.io/badge/mac%20os-000000?logo=macos\&logoColor=F0F0F0)](https://docs.github.com/en/actions/reference/workflow-syntax-for-github-actions#jobsjob_idruns-on)
[![Windows](https://img.shields.io/badge/Windows-0078D6?logo=windows\&logoColor=white)](https://docs.github.com/en/actions/reference/workflow-syntax-for-github-actions#jobsjob_idruns-on)

# json2file

Generate file output from a JSON string.

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

|       INPUT       |  TYPE  | REQUIRED |       DEFAULT       |                         DESCRIPTION                         |
|-------------------|--------|----------|---------------------|-------------------------------------------------------------|
|     bin\_path      | string |  false   |                     |                     Path to the binary                      |
|     directory     | string |   true   | `".github/outputs"` |                    Directory to write to                    |
|     extension     | string |  false   |       `"txt"`       |                    File extension to use                    |
|       keys        | string |   true   |                     |   List of Keys to read from<br>the `outputs` JSON string    |
|      outputs      | string |   true   |                     |                         JSON string                         |
| skip\_missing\_keys | string |  false   |      `"false"`      | Skip missing keys not found in<br>the `outputs` JSON string |

<!-- AUTO-DOC-INPUT:END -->

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
  -e, --extension <EXTENSION>  The extension to use for the files [default: txt]
  -v, --verbose                
  -h, --help                   Print help information
  -V, --version                Print version information
```

### Example

```bash
$ json2file --keys="foo bar" --outputs="{\"foo\": \"value1\", \"bar\": \"value2\"}" --directory=/tmp --extension=txt
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
