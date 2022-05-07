<br/>
<a href="https://github.com/hisamafahri/cyg" target="blank_">
    <img height="100" alt="cyg" src="https://raw.githubusercontent.com/hisamafahri/cyg/main/docs/assets/cyg-title.svg" />
</a>
<br/>


# cyg: Secure files in your repository

Cyg will help you to secure files in your repository directly using PGP encryption. The name "cyg" was inspired by the [Cygnus constellation](https://en.wikipedia.org/wiki/Cygnus_(constellation)).

## How It Works?

The current version of this tool is using the already existing and secure [GnuPG CLI](https://gnupg.org/).

Make sure it already installed in your system by running:

```bash
gpg --version
```

It should return the version installed in your system.

## Usage

- Lock a file

    ```bash
    cyg lock # or: cyg l

    # Then it will ask you which file you want to encrypt
    # and which email (make sure pgp public key are available in your system)
    # can decrypt it.
    ```

- Unlock a file

    ```bash
    cyg unlock # or: cyg u

    # As long as you have the private key of the email
    # inputted from `cyg lock` command, you will have
    # the access to unlock it.
    ```

## Author

[Hisam A Fahri](https://hisamafahri.com): [@hisamafahri](https://github.com/hisamafahri)

## License

[GNU GPLv3](LICENSE)