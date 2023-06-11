
<p align="center">
<img src="https://github.com/nopeNoshishi/nss/blob/main/picture/logos.png" width="200">
</p>

<p align="center">
[![Test](https://github.com/nopeNoshishi/nss/actions/workflows/rust.yml/badge.svg)](https://github.com/nopeNoshishi/nss/actions/workflows/rust.yml)
[![Codecov](https://codecov.io/gh/nopeNoshishi/nss/branch/main/graph/badge.svg?token=TVQHRPZ1SN)](https://codecov.io/gh/nopeNoshishi/nss)
<img src="https://img.shields.io/badge/version-0.1.6-green?style=flat-square">
</p>

# nss (noshishi)
Original Version Management System based on Git.


Learning git and rust for good developer.

# Usage
### Install
```
cargo install nssi
```

### how to
```
nssi help
nssi <Commands> [Options]
```

### Basic Usage
First, create repository! (`git-init`)
```
nssi voyage
```

Edit file ....
Next snapshot the file! (`git-add`)
```
nssi snapshot `file_path`
```

Last you must refister version data into database! (`git-commit`)
```
nssi reg -m 'initial'
```

**GREAT!!**

### Advance
Yon can trace history of commit. (`git-log`)
```
nssi stroy
```

You may think to go back specific commit... (`git-checkout`)
```
nssi go-to <commit hash>
```

Of course, you can bookmark specific commit! (`git-branch`)
```
nssi bookmark <bookmarker> <commit hash>
```

### editiing......
