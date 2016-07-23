[Documentation][2]

This crate provides an interface to the open source [Spacenav][0] daemon.

This daemon communicates with 3D mice made by [3DConnexion][1] such as the
SpaceNavigator.

The spacenavd daemon supports two protocols. An X11 protocol compatible with
the proprietary daemon as well as an alternative communication protocol that
does not require an X server. This crate communicates via the second, non X11
protocol. For now the X11 protocol is not implemented.

### Installation

```toml
[dependencies]
spacenav = "*"
```

### Example

```rust
extern crate spacenav;

use spacenav::SpaceNav;

fn main() {

    let mut spcnav = SpaceNav::new().unwrap();

    loop {
        let event = spcnav.read();

        println!("{:?}", event);
    }
}
```

See [example output][3] of the above code when used with a SpaceNavigator.

[0]: http://spacenav.sourceforge.net
[1]: http://www.3dconnexion.com
[2]: https://mfs.github.io/spacenav/spacenav/index.html
[3]: https://asciinema.org/a/80713
