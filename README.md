# json2file

Generate file OUTPUT from a JSON string.

## Usage

```yaml
    - name: Set OUTPUT
      id: set-OUTPUT
      run: |
        echo "color=green" >> $GITHUB_OUTPUT

    - name: Generate file OUTPUT from json
      uses: tj-actions/json2file@v1
      with:
        DIRECTORY: 'OUTPUT'
        OUTPUTS:  ${{ steps.set-OUTPUT.OUTPUTS }}
        KEYS: 'color'
        extension: 'txt'
```

The above example will create a file named `color.txt` in the `OUTPUT` DIRECTORY with the contents `green`.


## Inputs


## CLI Reference
Run `json2file --help` for more information.

```bash
$ json2file --help
json2file
Generate files from a JSON OUTPUT.

Usage:
        json2file --KEYS=[KEYS] --OUTPUTS=[OUTPUT] --DIRECTORY=[DIRECTORY] --extension=[EXTENSION]

Options:

-h, --help              Show this help message and exit.
-v, --VERSION           Show the VERSION and exit.
-k, --KEYS              The KEYS to use to generate the files. (Required)
-o, --OUTPUTS           The JSON OUTPUT to use. (Required)
-d, --DIRECTORY         The DIRECTORY to OUTPUT the files to. (Required)
-e, --extension         The extension to use for the files. (Optional, defaults to txt)

Example:
        json2file --KEYS=foo,bar --OUTPUTS="{\"foo\": \"value1\", \"bar\": \"value2\"}" --DIRECTORY=/tmp --extension=tx
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

