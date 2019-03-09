# fun memory violations

This is my website, running at [spacekookie.de](https://spacekookie.de). It's built with Pelican and my own theme called `crumbs` (because kookies...).

To understand the theme, you need to understand jinja because it's not very linear. Every sub-component is structured as such and then included. Sometimes parameters are given to the sub-component to distinguish between different page behaviour.

If you have questions, just ask :)

## How to build

You need to have python3 (not sure if it works with python2...) installed. Then you can build the website as follows...

```bash
pip install pelican markdown webassets
pelican content
make devserver
# The server is hosted on port 8000
```

I hope someone can learn from this :)

## License

It's all MIT. Fuck anything to do with the GNU project 💩
