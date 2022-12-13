# json2file

Generate file output from a JSON string.

## Usage

```yaml
    - name: Set output
      id: set-output
      run: |
        echo "color=green" >> $GITHUB_OUTPUT

    - name: Generate file output from json
      uses: tj-actions/json2file@v1
      with:
        directory: 'output'
        outputs:  ${{ steps.set-output.outputs }}
        keys: 'color'
        extension: 'txt'
```

The above example will create a file named `color.txt` in the `output` directory with the contents `green`.

## Inputs

<!-- AUTO-DOC-INPUT:START - Do not remove or modify this section -->

|   INPUT   |  TYPE  | REQUIRED |           DEFAULT            |                                  DESCRIPTION                                  |
|-----------|--------|----------|------------------------------|-------------------------------------------------------------------------------|
| bin\_path  | string |  false   | `"target/release/json2file"` |                              Path to the binary                               |
| directory | string |   true   |         `"outputs"`          |                             Directory to write to                             |
| extension | string |  false   |           `"txt"`            |                             File extension to use                             |
|   keys    | string |   true   |                              | Comma separated list of Keys<br>to read from the outputs.<br>Example: foo,bar |
|  outputs  | string |   true   |                              |                                  JSON string                                  |

<!-- AUTO-DOC-INPUT:END -->

## CLI Reference

Run `json2file --help` for more information.

```bash
$ json2file --help
json2file
Generate files from a JSON output.

Usage:
        json2file --keys=[keys] --outputs=[output] --directory=[directory] --extension=[EXTENSION]

Options:

-h, --help              Show this help message and exit.
-v, --VERSION           Show the VERSION and exit.
-k, --keys              The keys to use to generate the files. (Required)
-o, --outputs           The JSON output to use. (Required)
-d, --directory         The directory to output the files to. (Required)
-e, --extension         The extension to use for the files. (Optional, defaults to txt)

Example:
        json2file --keys=foo,bar --outputs="{\"foo\": \"value1\", \"bar\": \"value2\"}" --directory=/tmp --extension=tx
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
